import argparse
from bcc import BPF
import socket
import struct

parser = argparse.ArgumentParser(
    description="Trace HTTP long living calls.")
parser.add_argument("-t", "--threshold", type=int, default=100,
                    help="Duration threshold (default to: 100 ms)")
parser.add_argument("-p", "--process", type=str,
    action='append', required=True,
    help="Process name")
args = parser.parse_args()

DURATION_THRESHOLD_MS = args.threshold

PROCESS_FILTERS = args.process

process_filters = args.process
filter_count = len(process_filters)

filters_code = ""
for i, proc in enumerate(process_filters):
    proc_escaped = proc.replace('\\', '\\\\').replace('"', '\\"')
    filters_code += f'static const char filter_{i}[] = "{proc_escaped}";\n'

filters_code += f'#define FILTER_COUNT {filter_count}\n'
filters_code += 'static const char *process_filters[FILTER_COUNT] = {\n'
for i in range(filter_count):
    filters_code += f'    filter_{i},\n'
filters_code += '};\n'

filters_code += """
#define MY_COMM_LEN 16

static inline int COMM_COMPARE(char *comm, const char *filter) {
    int i;
    #pragma unroll
    for (i = 0; i < MY_COMM_LEN; i++) {
        if (filter[i] == '\\0') {
            return 1;
        }
        if (comm[i] != filter[i]) {
            return 0;
        }
    }
    return 1;
}
"""

bpf_text = """
#include <uapi/linux/ptrace.h>
#include <uapi/linux/in.h>
#include <uapi/linux/in6.h>

// Définition simplifiée de sock_common
struct sock_common {
    u32 skc_rcv_saddr;
    u32 skc_daddr;
    u16 skc_dport;
    // Autres champs si nécessaire
};

struct sock {
    struct sock_common __sk_common;
    // Autres champs si nécessaire
};

struct start_t {
    u64 ts;
    u32 stack_id;
};

struct data_t {
    u32 pid;
    u64 duration;
    int size;
    char comm[MY_COMM_LEN];
    u32 saddr;
    u32 daddr;
    u16 dport;
    u32 stack_id;
};

BPF_HASH(start, u64, struct start_t);
BPF_STACK_TRACE(stack_traces, 1024);
BPF_PERF_OUTPUT(events);

int trace_tcp_sendmsg(struct pt_regs *ctx, struct sock *sk) {
    u32 pid = bpf_get_current_pid_tgid() >> 32;
    char comm[MY_COMM_LEN];

    bpf_get_current_comm(&comm, sizeof(comm));

    // Appliquer les filtres de processus
    bool matched = false;
    int i;
    #pragma unroll
    for (i = 0; i < FILTER_COUNT; i++) {
        if (COMM_COMPARE(comm, process_filters[i])) {
            matched = true;
            break;
        }
    }
    if (!matched) {
        return 0;
    }

    u64 id = bpf_get_current_pid_tgid();
    u32 stack_id = stack_traces.get_stackid(ctx, BPF_F_USER_STACK);
    if ((int)stack_id < 0) {
        return 0;
    }

    struct start_t start_data = {};
    start_data.ts = bpf_ktime_get_ns();
    start_data.stack_id = stack_id;
    start.update(&id, &start_data);

    return 0;
}

int trace_tcp_recvmsg(struct pt_regs *ctx, struct sock *sk) {
    u32 pid = bpf_get_current_pid_tgid() >> 32;
    char comm[MY_COMM_LEN];

    bpf_get_current_comm(&comm, sizeof(comm));

    // Appliquer les filtres de processus
    bool matched = false;
    int i;
    #pragma unroll
    for (i = 0; i < FILTER_COUNT; i++) {
        if (COMM_COMPARE(comm, process_filters[i])) {
            matched = true;
            break;
        }
    }
    if (!matched) {
        return 0;
    }

    u64 id = bpf_get_current_pid_tgid();
    struct start_t *start_data = start.lookup(&id);
    if (!start_data) {
        return 0;
    }

    struct data_t data = {};
    data.pid = pid;
    data.duration = bpf_ktime_get_ns() - start_data->ts;
    data.size = PT_REGS_RC(ctx);
    data.stack_id = start_data->stack_id;
    bpf_probe_read_kernel(&data.comm, sizeof(data.comm), comm);

    // Lire les adresses et les ports
    struct sock_common sk_common = {};
    bpf_probe_read_kernel(&sk_common, sizeof(sk_common), &sk->__sk_common);

    data.saddr = sk_common.skc_rcv_saddr;
    data.daddr = sk_common.skc_daddr;
    data.dport = sk_common.skc_dport;

    // Envoyer les données à l'espace utilisateur
    events.perf_submit(ctx, &data, sizeof(data));

    start.delete(&id);

    return 0;
}
"""

bpf_text = filters_code + bpf_text

b = BPF(text=bpf_text)
b.attach_kprobe(event="tcp_sendmsg", fn_name="trace_tcp_sendmsg")
b.attach_kprobe(event="tcp_recvmsg", fn_name="trace_tcp_recvmsg")

def print_event(cpu, data, size):
    event = b["events"].event(data)
    duration_ms = event.duration / 1e6

    if duration_ms < DURATION_THRESHOLD_MS:
        return

    saddr = socket.inet_ntoa(struct.pack(">I", event.saddr))
    daddr = socket.inet_ntoa(struct.pack(">I", event.daddr))
    dport = socket.ntohs(event.dport)

    stack = []
    for addr in b["stack_traces"].walk(event.stack_id):
        sym = b.sym(addr, event.pid)
        if isinstance(sym, bytes):
            sym = sym.decode('utf-8', 'replace')
        stack.append(sym)

    print(f"PID: {event.pid}, Comm: {event.comm.decode('utf-8', 'replace')}, Stack ID: {event.stack_id}")
    print(f"Duration: {duration_ms:.2f} ms, Size: {event.size} bytes")
    print(f"Src IP: {saddr}, Dst IP: {daddr}, Dst Port: {dport}")
    print("Stack trace:")
    for frame in stack:
        print(f"    {frame}")
    print("")

    with open("stack_traces.txt", "a") as f:
        f.write(f"PID: {event.pid}, Comm: {event.comm.decode('utf-8', 'replace')}, Stack ID: {event.stack_id}\n")
        f.write(f"Duration: {duration_ms:.2f} ms, Size: {event.size} bytes\n")
        f.write(f"Src IP: {saddr}, Dst IP: {daddr}, Dst Port: {dport}\n")
        f.write("Stack trace:\n")
        for frame in stack:
            f.write(f"    {frame}\n")
        f.write("\n")

    with open("stacks.folded", "a") as f:
        stack_line = ";".join(reversed(stack))
        f.write(f"{stack_line} {duration_ms}\n")

b["events"].open_perf_buffer(print_event)

print(f"Tracing... Hit Ctrl-C to end. (duration threshold : {DURATION_THRESHOLD_MS} ms)")
print(f"Processes filter: {', '.join(PROCESS_FILTERS)}")
while True:
    try:
        b.perf_buffer_poll()
    except KeyboardInterrupt:
        break
