# Script Documentation: Tracing Long-Duration HTTP Calls and Generating FlameGraphs

## Overview

The `trace_http.py` script is designed to trace long-duration HTTP calls made
by specific processes on a Linux system. It utilizes eBPF (extended Berkeley
Packet Filter) via the BCC (BPF Compiler Collection) to monitor TCP send and
receive events at the kernel level. By capturing stack traces of these events,
the script allows you to identify performance bottlenecks and analyze the
call stacks of processes of interest.

The script:

- Monitors `tcp_sendmsg` and `tcp_recvmsg` kernel functions.
- Filters events based on specified process names.
- Captures stack traces when TCP send/receive operations exceed a specified duration threshold.
- Outputs detailed information to the console and writes stack traces to files for further analysis.
- Generates data compatible with [FlameGraph](https://github.com/brendangregg/FlameGraph) for visualizing call stacks.

## Requirements

To use this script, you need the following:

- **Linux Kernel with eBPF Support**: Kernel version 4.1 or higher is recommended.
- **Python 3**: Ensure Python 3 is installed on your system.
- **BCC (BPF Compiler Collection)**: Install BCC and its Python bindings.

  - **On Ubuntu/Debian**:

    ```bash
    sudo apt-get install bpfcc-tools linux-headers-$(uname -r)
    ```

  - **On Fedora/CentOS/RHEL**:

    ```bash
    sudo dnf install bcc bcc-tools python3-bcc kernel-devel-$(uname -r)
    ```

- **FlameGraph**: For generating flame graphs from the collected stack traces.

  ```bash
  git clone https://github.com/brendangregg/FlameGraph.git
  ```

- **Debug Symbols for Python and Other Relevant Libraries**: To get detailed stack traces.

  - **On Ubuntu/Debian**:

    ```bash
    sudo apt-get install python3-dbg
    ```

  - **On Fedora/CentOS/RHEL**:

    ```bash
    sudo dnf debuginfo-install python3
    ```

- **Root Privileges**: The script requires root privileges to attach eBPF probes to kernel functions.

## How to Use the Script

### 1. Make the Script Executable

Ensure the script has executable permissions:

```bash
chmod +x trace_http.py
```

### 2. Run the Script with Appropriate Options

The script accepts command-line arguments to customize its behavior:

- **`-t` or `--threshold`**: Set the duration threshold in milliseconds. Only events exceeding this duration will be captured. The default is 100 ms.
- **`-p` or `--process`**: Specify the process names to filter on. You can provide multiple `-p` options to monitor multiple processes.

#### Example Usage

- **Monitor process named `nova` with a threshold of 200 ms**:

  ```bash
  sudo ./trace_http.py -t 200 -p nova
  ```

- **Monitor a process named `python` with the default threshold**:

  ```bash
  sudo ./trace_http.py -p python
  ```

### 4. Understanding the Output

When the script is running, it will display messages on the console whenever
it captures an event that meets the specified criteria.

Example output:

```
Tracing... Hit Ctrl-C to end. (Duration threshold: 200 ms)
Process filters: nova

PID: 12345, Comm: nova-api, Stack ID: 256
Duration: 250.00 ms, Size: 512 bytes
Src IP: 192.168.1.10, Dst IP: 192.168.1.20, Dst Port: 80
Stack trace:
    function_a
    function_b
    function_c
```

### 5. Output Files

The script generates two files:

- **`stack_traces.txt`**: Contains detailed information about each captured event, including the stack trace.
- **`stacks.folded`**: Contains stack traces in a folded format suitable for generating flame graphs.

### 6. Generating FlameGraphs

After running the script and collecting data, you can generate a flame graph
to visualize the stack traces.

#### Steps to Generate a FlameGraph

1. **Ensure FlameGraph is Cloned**

If you haven't already, clone the FlameGraph repository:

```bash
git clone https://github.com/brendangregg/FlameGraph.git
```

2. **Generate the FlameGraph**

Run the `flamegraph.pl` script provided by FlameGraph:

```bash
./FlameGraph/flamegraph.pl stacks.folded > flamegraph.svg
```

**Note**: Replace `stacks.folded` with the path to your folded stack trace file if it's in a different directory.

3. **View the FlameGraph**

Open the generated `flamegraph.svg` file in a web browser to interactively explore the stack traces.

```bash
firefox flamegraph.svg
```

### 7. Analyzing the FlameGraph

- **Understanding the Visualization**:
  - **X-Axis**: Represents the stack trace's cumulative time spent in functions.
  - **Y-Axis**: Represents the call stack depth.
  - **Blocks**: Each block is a function in the call stack. The width represents the total time spent in that function (including child calls).
- **Navigating**:
  - **Hover**: Hover over a block to see function names and additional details.
  - **Zoom**: Click on a block to zoom into that portion of the call stack.
- **Identifying Performance Bottlenecks**:
  - Look for wide blocks near the top of the graph, indicating functions where significant time is spent.

## Script Details

### Functionality

- **Attaches eBPF Probes**: Hooks into `tcp_sendmsg` and `tcp_recvmsg` kernel functions to monitor TCP events.
- **Process Filtering**: Only considers events from processes whose names match the specified filters.
- **Duration Measurement**: Calculates the duration between send and receive events to identify long-duration calls.
- **Stack Trace Collection**: Captures user-space stack traces at the time of the event.
- **Data Output**: Prints event information to the console and writes detailed data to files.

### Command-Line Arguments

- **`-t` or `--threshold`**: Duration threshold in milliseconds (default: 100 ms).
- **`-p` or `--process`**: Process name(s) to filter on. Can be specified multiple times.

## Tips and Caveats

- **Root Privileges**: Running the script with `sudo` is necessary due to the requirement to attach eBPF probes.
- **Process Name Matching**:
  - The script matches process names exactly as provided.
  - Process names in Linux are limited to 15 characters.
  - Matching is case-sensitive.
- **Debug Symbols**:
  - Installing debug symbols enhances the quality of stack traces.
  - Without debug symbols, function names may not be fully resolved.
- **Performance Impact**:
  - Monitoring with eBPF is efficient, but capturing stack traces can add overhead.
  - Use duration thresholds to limit the amount of data collected.
- **Python Version**:
  - Ensure compatibility between the Python version used to run the script and the version for which debug symbols are installed.
- **FlameGraph Compatibility**:
  - The `stacks.folded` file must be properly formatted for FlameGraph.
  - If you encounter issues, verify the file contents and format.

## Troubleshooting

- **No Output or Empty Files**:
  - Lower the duration threshold to capture more events.
  - Verify that the specified process names are correct and that the processes are running.
- **Symbol Resolution Issues**:
  - Ensure that debug symbols are installed for all relevant binaries and libraries.
  - Check that the script has permission to read symbol information.
- **Errors When Running `flamegraph.pl`**:
  - Ensure Perl is installed and properly configured on your system.
  - Verify that the `stacks.folded` file is not empty and is correctly formatted.
- **Permission Denied Errors**:
  - Confirm that you are running the script with root privileges.
  - Check file permissions for the output files.

## Examples

### Monitoring a Single Process

Monitor HTTP calls from the `python` process with a threshold of 50 ms:

```bash
sudo ./trace_http.py -t 50 -p python
```

### Monitoring Multiple Processes

Monitor `nova-api` process with the default threshold:

```bash
sudo ./trace_http.py -p nova-api 
```

### Generating a FlameGraph with a Custom Title

```bash
cd FlameGraph
./flamegraph.pl --title "HTTP Call Stack" ../stacks.folded > ../flamegraph.svg
```

## Conclusion

This script is a powerful tool for tracing long-duration HTTP calls and
analyzing the performance of specific processes on a Linux system. By
leveraging eBPF and FlameGraph, you can gain deep insights into your
application's behavior and identify potential bottlenecks or areas for optimization.

For further customization or advanced usage, you may modify the script to suit
your specific needs, such as adjusting the stack trace depth or integrating
additional filters.

## References

- **BCC Documentation**: [iovisor.github.io/bcc](https://iovisor.github.io/bcc/)
- **FlameGraph Repository**: [github.com/brendangregg/FlameGraph](https://github.com/brendangregg/FlameGraph)
- **Brendan Gregg's Blog**: [brendangregg.com](http://www.brendangregg.com/)
- **eBPF Tracing Tools**: [ebpf.io](https://ebpf.io/)
