# Analysis for Team: neutron

## Project: networking-sfc
---

- **Project:** Networking-SFC
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of Eventlet in configuration files and dependencies adds complexity.*
  - **Files Analyzed:**
    - **File:** `networking_sfc/services/sfc/agent/__init__.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the sfc agent.
    - **File:** `networking_sfc/tests/unit/services/sfc/drivers/ovs/test_driver.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `networking_sfc/requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `networking_sfc/services/sfc/agent/__init__.py` (again)
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/networking-sfc/src/branch/master/networking_sfc/services/sfc/agent/__init__.py#n1 : from neutron.common import eventlet_utils
- https://opendev.org/openstack/networking-sfc/src/branch/master/networking_sfc/services/sfc/agent/__init__.py#n2 : eventlet_utils.monkey_patch()
- https://opendev.org/openstack/networking-sfc/src/branch/master/networking_sfc/tests/unit/services/sfc/drivers/ovs/test_driver.py#n18 : from eventlet import greenthread
- https://opendev.org/openstack/networking-sfc/src/branch/master/requirements.txt#n7 : eventlet>=0.27.0 # MIT

***

## Project: neutron
The code snippet provided appears to be a test case for the Nova (OpenStack Compute) service, specifically testing the eventlet library's handling of asynchronous tasks. The test is checking if the `eventlet.GreenPool` class can handle multiple threads concurrently without blocking.

Here's a breakdown of the code:

1. Import statements:
   - `eventlet`: This imports the eventlet library, which provides an asynchronous I/O framework for Python.
   - `eventlet.timeout`: This imports the timeout functionality from eventlet, used to raise a `Timeout` exception after a specified time period.
2. Test setup:
   - A `GreenPool` object is created with 2 threads (`pool = eventlet.GreenPool(2)`). The `GreenPool` class is designed to manage a pool of worker threads that can be reused for I/O operations.

3. Test execution:
   - An asynchronous task is executed using the `eventlet.sleep(0)` function, which blocks for zero seconds and allows other tasks to run concurrently.
   - Another `GreenPool` object is created with 2 threads (`pool = eventlet.GreenPool(2)`), but this time it's used in a different context.

4. Test assertion:
   - The test checks if the second `GreenPool` object can handle multiple concurrent requests without blocking by raising an exception after a short period of time using `eventlet.timeout`.

The purpose of this test is to ensure that the eventlet library can efficiently manage asynchronous tasks and prevent blocking when dealing with multiple threads concurrently.

Occurrences Found:
- https://opendev.org/openstack/neutron/src/branch/master/.coveragerc#n6 : neutron/cmd/eventlet/agents/dhcp.py
- https://opendev.org/openstack/neutron/src/branch/master/.coveragerc#n7 : neutron/cmd/eventlet/agents/l3.py
- https://opendev.org/openstack/neutron/src/branch/master/.coveragerc#n8 : neutron/cmd/eventlet/agents/metadata.py
- https://opendev.org/openstack/neutron/src/branch/master/.coveragerc#n9 : neutron/cmd/eventlet/agents/ovn_metadata.py
- https://opendev.org/openstack/neutron/src/branch/master/.coveragerc#n10 : neutron/cmd/eventlet/agents/ovn_neutron_agent.py
- https://opendev.org/openstack/neutron/src/branch/master/.coveragerc#n11 : neutron/cmd/eventlet/plugins/linuxbridge_neutron_agent.py
- https://opendev.org/openstack/neutron/src/branch/master/.coveragerc#n12 : neutron/cmd/eventlet/plugins/macvtap_neutron_agent.py
- https://opendev.org/openstack/neutron/src/branch/master/.coveragerc#n13 : neutron/cmd/eventlet/plugins/ovs_neutron_agent.py
- https://opendev.org/openstack/neutron/src/branch/master/.coveragerc#n14 : neutron/cmd/eventlet/plugins/sriov_nic_neutron_agent.py
- https://opendev.org/openstack/neutron/src/branch/master/doc/source/contributor/contribute.rst#n479 : neutron-foo-agent = networking_foo.cmd.eventlet.agents.foo:main
- https://opendev.org/openstack/neutron/src/branch/master/doc/source/contributor/effective_neutron.rst#n197 : Document common pitfalls as well as good practices done when using eventlet
- https://opendev.org/openstack/neutron/src/branch/master/doc/source/contributor/effective_neutron.rst#n205 : https://wiki.openstack.org/wiki/OpenStack_and_SQLAlchemy#MySQLdb_.2B_eventlet_.3D_sad
- https://opendev.org/openstack/neutron/src/branch/master/doc/source/contributor/internals/api_layer.rst#n45 : .. _Eventlet: http://eventlet.net/
- https://opendev.org/openstack/neutron/src/branch/master/doc/source/contributor/internals/api_layer.rst#n47 : .. _GreenPool: http://eventlet.net/doc/modules/greenpool.html
- https://opendev.org/openstack/neutron/src/branch/master/doc/source/contributor/internals/services_and_agents.rst#n61 : Neutron extensively utilizes the eventlet library to provide asynchronous
- https://opendev.org/openstack/neutron/src/branch/master/doc/source/contributor/internals/services_and_agents.rst#n65 : If a service utilizes the eventlet library, then it should not call
- https://opendev.org/openstack/neutron/src/branch/master/doc/source/contributor/internals/services_and_agents.rst#n66 : eventlet.monkey_patch() directly but instead maintain its entry point main()
- https://opendev.org/openstack/neutron/src/branch/master/doc/source/contributor/internals/services_and_agents.rst#n67 : function under neutron/cmd/eventlet/... If that is the case, the standard
- https://opendev.org/openstack/neutron/src/branch/master/doc/source/contributor/internals/services_and_agents.rst#n70 : <http://opendev.org/openstack/neutron/src/neutron/cmd/eventlet/__init__.py>`_).
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n17 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n18 : import eventlet.event
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n19 : from eventlet.green import subprocess
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n20 : import eventlet.queue
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n115 : self._stdout_lines = eventlet.queue.LightQueue()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n116 : self._stderr_lines = eventlet.queue.LightQueue()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n169 : self._kill_event = eventlet.event.Event()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n178 : watcher = eventlet.spawn(self._watch_process,
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n244 : eventlet.sleep(self.respawn_interval)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n267 : eventlet.sleep()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/async_process.py#n310 : except eventlet.queue.Empty:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/polling.py#n18 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/polling.py#n75 : eventlet.sleep()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n18 : from eventlet import patcher
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n22 : from oslo_utils import eventletutils
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n70 : """Determines if socket module is patched by eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n75 : class is to workaround eventlet bug
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n76 : https://github.com/eventlet/eventlet/issues/764 and for the
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n78 : with socket mocks as it abstracts eventlet under the hood module
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n80 : TODO(mtomaska): This class(workaround) can be removed once eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n84 : if eventletutils.is_monkey_patched(socket.__name__):
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n85 : LOG.debug("Std library socket module is patched by eventlet. "
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n86 : "Requesting std library socket module from eventlet.")
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/common/utils.py#n89 : LOG.debug("Std library socket module is not patched by eventlet. "
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/dhcp/agent.py#n21 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/dhcp/agent.py#n136 : self._pool = eventlet.GreenPool(1)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/dhcp/agent.py#n180 : eventlet.spawn_n(self._process_loop)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/dhcp/agent.py#n182 : eventlet.spawn_n(self._reload_bulk_allocations)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/dhcp/agent.py#n195 : eventlet.greenthread.sleep(self.conf.bulk_reload_interval)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/dhcp/agent.py#n293 : eventlet.greenthread.sleep(0)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/dhcp/agent.py#n302 : pool = eventlet.GreenPool(self.conf.num_sync_threads)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/dhcp/agent.py#n369 : eventlet.sleep(0.2)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/dhcp/agent.py#n374 : eventlet.spawn(self._dhcp_ready_ports_loop)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/dhcp/agent.py#n406 : eventlet.spawn(self._periodic_resync_helper)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/l3/agent.py#n18 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/l3/agent.py#n331 : self._pool = eventlet.GreenPool(size=self._pool_size)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/l3/agent.py#n980 : eventlet.spawn_n(self._process_routers_loop)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/l3/agent.py#n1068 : eventlet.spawn_n(self._process_routers_loop)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/l3/ha.py#n19 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/l3/ha.py#n91 : eventlet.spawn(self._start_keepalived_notifications_server)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/l3/ha.py#n164 : eventlet.spawn_n(self._enqueue_state_change, router_id, state)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/l3/ha.py#n165 : eventlet.sleep(0)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/l3/ha.py#n170 : eventlet.sleep(self.conf.ha_vrrp_advert_int)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/external_process.py#n19 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/external_process.py#n257 : eventlet.spawn(self._periodic_checking_thread)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/external_process.py#n278 : eventlet.sleep(self._config.AGENT.check_child_processes_interval)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/external_process.py#n279 : eventlet.spawn(self._check_child_processes)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_conntrack.py#n16 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_conntrack.py#n74 : self._queue = eventlet.queue.LightQueue()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_conntrack.py#n76 : eventlet.spawn_n(self._process_queue_worker)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_lib.py#n22 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_lib.py#n1122 : use_eventlet=True):
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_lib.py#n1140 : :param use_eventlet: (Optional) True if the arping command will be spawned
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_lib.py#n1141 : using eventlet, False to use Python threads
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_lib.py#n1148 : if use_eventlet:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_lib.py#n1149 : eventlet.spawn_n(arping)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_lib.py#n1543 : _queue = eventlet.Queue()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/ip_lib.py#n1560 : except eventlet.queue.Empty:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/of_monitor.py#n18 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/of_monitor.py#n55 : self._queue = eventlet.queue.Queue()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/of_monitor.py#n56 : eventlet.spawn(self._read_and_enqueue, self.iter_stdout)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/of_monitor.py#n57 : eventlet.spawn(self._read_and_enqueue, self.iter_stderr)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/openvswitch_firewall/firewall.py#n22 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/openvswitch_firewall/firewall.py#n885 : eventlet.sleep(0)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/pd.py#n19 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/pd.py#n228 : eventlet.spawn_n(self._ensure_lla_task,
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/utils.py#n26 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/utils.py#n27 : from eventlet.green import subprocess
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/utils.py#n417 : class UnixDomainHttpProtocol(eventlet.wsgi.HttpProtocol):
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/utils.py#n433 : eventlet.wsgi.HttpProtocol.__init__(self, conn_state, server)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/utils.py#n444 : eventlet.wsgi.HttpProtocol.__init__(
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/utils.py#n447 : eventlet.wsgi.HttpProtocol.__init__(self, *args)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/utils.py#n459 : self._socket = eventlet.listen(file_socket,
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/utils.py#n469 : logger = logging.getLogger('eventlet.wsgi.server')
- https://opendev.org/openstack/neutron/src/branch/master/neutron/agent/linux/utils.py#n470 : eventlet.wsgi.server(socket,
- https://opendev.org/openstack/neutron/src/branch/master/neutron/api/wsgi.py#n24 : import eventlet.wsgi
- https://opendev.org/openstack/neutron/src/branch/master/neutron/api/wsgi.py#n89 : if isinstance(self._server, eventlet.greenthread.GreenThread):
- https://opendev.org/openstack/neutron/src/branch/master/neutron/api/wsgi.py#n93 : if isinstance(self._server, eventlet.greenthread.GreenThread):
- https://opendev.org/openstack/neutron/src/branch/master/neutron/api/wsgi.py#n107 : eventlet.wsgi.MAX_HEADER_LINE = CONF.max_header_line
- https://opendev.org/openstack/neutron/src/branch/master/neutron/api/wsgi.py#n111 : self.pool = eventlet.GreenPool(1)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/api/wsgi.py#n141 : sock = eventlet.listen(bind_addr,
- https://opendev.org/openstack/neutron/src/branch/master/neutron/api/wsgi.py#n148 : eventlet.sleep(0.1)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/api/wsgi.py#n223 : eventlet.wsgi.server(socket, application,
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/__init__.py#n13 : from neutron.common import eventlet_utils
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/__init__.py#n15 : eventlet_utils.monkey_patch()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n14 : from neutron.server import api_eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n16 : from neutron.server import periodic_eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n17 : from neutron.server import rpc_eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n18 : from neutron.server import wsgi_eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n22 : server.boot_server(wsgi_eventlet.eventlet_wsgi_server)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n25 : def main_rpc_eventlet():
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n26 : server.boot_server(rpc_eventlet.eventlet_rpc_server)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n29 : def main_api_eventlet():
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n30 : return server.boot_server(api_eventlet.eventlet_api_server)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n33 : def main_periodic_eventlet():
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n34 : server.boot_server(periodic_eventlet.eventlet_periodic_workers)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n37 : def main_ovn_maintenance_eventlet():
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/eventlet/server/__init__.py#n38 : return server.boot_server(ovn_maintenance.eventlet_ovn_maintenance_worker)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/cmd/ovn/neutron_ovn_db_sync_util.py#n173 : logging.setup(conf, 'neutron_ovn_db_sync_util', fix_eventlet=False)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/config.py#n118 : logging.setup(cfg.CONF, product_name, fix_eventlet=False)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/eventlet_utils.py#n16 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/eventlet_utils.py#n21 : eventlet.monkey_patch()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n36 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n37 : from eventlet.green import subprocess
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n113 : eventlet.sleep(time_to_wait)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n738 : with eventlet.Timeout(timeout):
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n740 : eventlet.sleep(sleep)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n741 : except eventlet.Timeout:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n920 : """As eventlet.spawn() but with osprofiler initialized in the new threads
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n923 : in new threads (including eventlet threads) osprofiler comes uninitialized
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n924 : by default. This spawn() is a stand-in replacement for eventlet.spawn()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n936 : return eventlet.spawn(wrapper, *args, **kwargs)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/common/utils.py#n950 : return eventlet.spawn_n(wrapper, *args, **kwargs)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/db/agents_db.py#n19 : from eventlet import greenthread
- https://opendev.org/openstack/neutron/src/branch/master/neutron/notifiers/batch_notifier.py#n15 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/notifiers/batch_notifier.py#n22 : self._pending_events = eventlet.Queue()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/notifiers/batch_notifier.py#n61 : eventlet.sleep(self.batch_interval)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/notifiers/nova.py#n18 : from eventlet.green import threading
- https://opendev.org/openstack/neutron/src/branch/master/neutron/plugins/ml2/drivers/openvswitch/agent/openflow/native/ofswitch.py#n20 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/plugins/ml2/drivers/openvswitch/agent/openflow/native/ofswitch.py#n93 : timeout = eventlet.Timeout(seconds=timeout_sec)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/plugins/ml2/drivers/openvswitch/agent/openflow/native/ofswitch.py#n109 : except eventlet.Timeout as e:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/plugins/ml2/drivers/ovn/mech_driver/ovsdb/ovn_db_sync.py#n17 : from eventlet import greenthread
- https://opendev.org/openstack/neutron/src/branch/master/neutron/plugins/ml2/plugin.py#n18 : from eventlet import greenthread
- https://opendev.org/openstack/neutron/src/branch/master/neutron/privileged/agent/linux/utils.py#n19 : from eventlet.green import subprocess
- https://opendev.org/openstack/neutron/src/branch/master/neutron/server/api_eventlet.py#n27 : def eventlet_api_server():
- https://opendev.org/openstack/neutron/src/branch/master/neutron/server/ovn_maintenance.py#n27 : def eventlet_ovn_maintenance_worker():
- https://opendev.org/openstack/neutron/src/branch/master/neutron/server/periodic_eventlet.py#n26 : def eventlet_periodic_workers():
- https://opendev.org/openstack/neutron/src/branch/master/neutron/server/rpc_eventlet.py#n30 : def eventlet_rpc_server():
- https://opendev.org/openstack/neutron/src/branch/master/neutron/server/wsgi_eventlet.py#n14 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/server/wsgi_eventlet.py#n23 : def eventlet_wsgi_server():
- https://opendev.org/openstack/neutron/src/branch/master/neutron/server/wsgi_eventlet.py#n32 : pool = eventlet.GreenPool()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/services/trunk/drivers/openvswitch/agent/ovsdb_handler.py#n18 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/services/trunk/drivers/openvswitch/agent/ovsdb_handler.py#n144 : eventlet.spawn_n(self.handle_trunk_add, port_name)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/services/trunk/drivers/openvswitch/agent/ovsdb_handler.py#n149 : eventlet.spawn_n(
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/base.py#n31 : import eventlet.timeout
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/base.py#n179 : except eventlet.Timeout as e:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/base.py#n310 : with eventlet.Timeout(max_execution_time, False):
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/common/__init__.py#n13 : from neutron.common import eventlet_utils
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/common/__init__.py#n16 : eventlet_utils.monkey_patch()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/common/agents/ovs_agent.py#n21 : from neutron.cmd.eventlet.plugins.ovs_neutron_agent import main as _main
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/fullstack/__init__.py#n13 : from neutron.common import eventlet_utils
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/fullstack/__init__.py#n16 : eventlet_utils.monkey_patch()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/fullstack/agents/dhcp_agent.py#n24 : from neutron.cmd.eventlet.agents import dhcp as dhcp_agent
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/fullstack/agents/l3_agent.py#n21 : from neutron.common import eventlet_utils
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/fullstack/agents/l3_agent.py#n24 : eventlet_utils.monkey_patch()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/__init__.py#n23 : from neutron.common import eventlet_utils
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/__init__.py#n26 : eventlet_utils.monkey_patch()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/l2/base.py#n20 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/l2/base.py#n57 : self.init_done_ev = eventlet.event.Event()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/l2/base.py#n58 : self.main_ev = eventlet.event.Event()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/l2/base.py#n72 : self._main_thread = eventlet.spawn(self._kick_main)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/l2/base.py#n234 : self.agent_thread = eventlet.spawn(agent.rpc_loop,
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/linux/bin/ipt_binname.py#n36 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/linux/bin/ipt_binname.py#n39 : eventlet.spawn(print_binary_name).wait()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/linux/test_async_process.py#n15 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/linux/test_async_process.py#n40 : eventlet.sleep(0.01)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/linux/test_iptables.py#n187 : def test_binary_name_eventlet_spawn(self):
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/test_dhcp_agent.py#n20 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/agent/test_dhcp_agent.py#n247 : eventlet.sleep(10)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/cmd/test_netns_cleanup.py#n20 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/functional/cmd/test_netns_cleanup.py#n80 : except eventlet.Timeout:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/__init__.py#n16 : from neutron.common import eventlet_utils
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/__init__.py#n19 : eventlet_utils.monkey_patch()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n19 : import eventlet.event
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n20 : from eventlet.green import subprocess
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n21 : import eventlet.queue
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n45 : with mock.patch('eventlet.spawn') as mock_spawn:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n49 : self.assertIsInstance(proc._kill_event, eventlet.event.Event)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n87 : with mock.patch('eventlet.sleep') as sleep:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n120 : self._test__watch_process(lambda: None, eventlet.event.Event())
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n124 : eventlet.event.Event())
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n132 : kill_event = eventlet.event.Event()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n137 : queue = eventlet.queue.LightQueue()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n168 : result = list(self.proc._iter_queue(eventlet.queue.LightQueue(),
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/common/test_async_process.py#n173 : queue = eventlet.queue.LightQueue()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n24 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n340 : eventlet.greenthread.sleep(1)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n357 : with mock.patch.object(dhcp_agent.eventlet,
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n504 : with mock.patch.object(dhcp_agent.eventlet.GreenPool, 'waitall') as w:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n543 : with mock.patch.object(dhcp_agent.eventlet, 'spawn') as spawn:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n549 : with mock.patch.object(dhcp_agent.eventlet, 'spawn') as spawn:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n560 : with mock.patch.object(dhcp_agent.eventlet, 'sleep',
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n574 : with mock.patch.object(dhcp_agent.eventlet, 'sleep',
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n590 : with mock.patch.object(dhcp_agent.eventlet, 'sleep',
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n618 : with mock.patch.object(dhcp_agent.eventlet, 'sleep',
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/dhcp/test_agent.py#n639 : with mock.patch.object(dhcp_agent.eventlet, 'sleep',
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/l3/test_agent.py#n25 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/l3/test_agent.py#n81 : mock.patch('eventlet.spawn').start()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/l3/test_agent.py#n257 : eventlet.sleep(0.2)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/l3/test_agent.py#n261 : eventlet.sleep(self.conf.ha_vrrp_advert_int + 2)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/l3/test_agent.py#n298 : eventlet.sleep(self.conf.ha_vrrp_advert_int + 2)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/l3/test_agent.py#n314 : eventlet.sleep(self.conf.ha_vrrp_advert_int + 2)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/l3/test_agent.py#n441 : mock.patch.object(eventlet, 'spawn_n'):
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/l3/test_dvr_local_router.py#n52 : mock.patch('eventlet.spawn').start()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_external_process.py#n54 : self.spawn_patch = mock.patch("eventlet.spawn")
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_ip_lib.py#n1081 : @mock.patch('eventlet.spawn_n')
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_ip_lib.py#n1121 : @mock.patch('eventlet.spawn_n')
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_ip_lib.py#n1142 : @mock.patch('eventlet.spawn_n')
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_iptables_firewall.py#n73 : mock.patch('eventlet.spawn_n').start()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_utils.py#n40 : self.process = mock.patch('eventlet.green.subprocess.Popen').start()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_utils.py#n492 : self.ewhi = mock.patch('eventlet.wsgi.HttpProtocol.__init__').start()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_utils.py#n526 : self.eventlet_p = mock.patch.object(utils, 'eventlet')
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_utils.py#n527 : self.eventlet = self.eventlet_p.start()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_utils.py#n534 : self.eventlet.assert_has_calls([
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_utils.py#n547 : self.eventlet.wsgi.server.assert_called_once_with(
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/linux/test_utils.py#n562 : self.eventlet.wsgi.server.assert_called_once_with(
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/metadata/test_driver.py#n95 : mock.patch('eventlet.spawn').start()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/agent/ovn/metadata/test_driver.py#n57 : mock.patch('eventlet.spawn').start()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/api/test_wsgi.py#n122 : with mock.patch.object(wsgi.eventlet, 'listen') as mock_listen:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/api/test_wsgi.py#n198 : @mock.patch.object(wsgi, 'eventlet')
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/api/test_wsgi.py#n199 : def test__run(self, eventlet_mock):
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/api/test_wsgi.py#n202 : eventlet_mock.wsgi.server.assert_called_once_with(
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/common/test_utils.py#n22 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/common/test_utils.py#n23 : from eventlet import queue
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/common/test_utils.py#n123 : gt = eventlet.spawn(func)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/common/test_utils.py#n135 : gt = eventlet.spawn(func)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/common/test_utils.py#n147 : pool = eventlet.GreenPool(4)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/common/test_utils.py#n167 : pool = eventlet.GreenPool(4)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/common/test_utils.py#n413 : sleep = utils.eventlet.sleep
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/common/test_utils.py#n419 : with mock.patch.object(utils.eventlet, "sleep",
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/common/test_utils.py#n556 : eventlet.spawn(thread_with_no_leaked_profiler)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/db/test_db_base_plugin_v2.py#n23 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/db/test_db_base_plugin_v2.py#n6913 : In this test we start an update of the name on a model in an eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/db/test_db_base_plugin_v2.py#n6933 : coro = eventlet.spawn(_lock_blocked_name_update)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/db/test_db_base_plugin_v2.py#n6937 : eventlet.sleep(0)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/db/test_db_base_plugin_v2.py#n7076 : eventlet.sleep(0)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/db/test_db_base_plugin_v2.py#n7083 : coro = eventlet.spawn(_lock_blocked_object_delete)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/db/test_db_base_plugin_v2.py#n7087 : eventlet.sleep(0)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_batch_notifier.py#n18 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_batch_notifier.py#n28 : self._received_events = eventlet.Queue()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_batch_notifier.py#n30 : self.spawn_n_p = mock.patch.object(eventlet, 'spawn_n')
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_batch_notifier.py#n58 : eventlet.sleep(0)  # yield to let coro execute
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_batch_notifier.py#n72 : eventlet.sleep(0)  # yield to let coro execute
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_ironic.py#n18 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_ironic.py#n170 : @mock.patch.object(eventlet, 'spawn_n', autospec=True)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_nova.py#n19 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_nova.py#n411 : _queue = eventlet.queue.Queue()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_nova.py#n416 : eventlet.sleep(0)  # Next thread execution.
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_nova.py#n422 : eventlet.sleep(0)  # Next thread execution.
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/notifiers/test_nova.py#n428 : pool = eventlet.GreenPool(num_threads)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/plugins/ml2/drivers/ovn/agent/test_neutron_agent.py#n18 : import eventlet
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/plugins/ml2/drivers/ovn/agent/test_neutron_agent.py#n54 : eventlet.sleep(0)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/plugins/ml2/drivers/ovn/agent/test_neutron_agent.py#n68 : pool = eventlet.GreenPool(2)
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/tests/test_base.py#n21 : import eventlet.timeout
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/tests/test_base.py#n81 : raise eventlet.Timeout()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/tests/unit/tests/test_base.py#n90 : except eventlet.Timeout:
- https://opendev.org/openstack/neutron/src/branch/master/neutron/wsgi/api.py#n19 : from neutron.common import eventlet_utils
- https://opendev.org/openstack/neutron/src/branch/master/neutron/wsgi/api.py#n20 : eventlet_utils.monkey_patch()
- https://opendev.org/openstack/neutron/src/branch/master/neutron/wsgi/api.py#n25 : from neutron.server import api_eventlet  # noqa:E402
- https://opendev.org/openstack/neutron/src/branch/master/neutron/wsgi/api.py#n31 : application = server.boot_server(api_eventlet.eventlet_api_server)
- https://opendev.org/openstack/neutron/src/branch/master/requirements.txt#n11 : eventlet>=0.36.1 # MIT
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n34 : neutron-api = neutron.cmd.eventlet.server:main_api_eventlet
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n37 : neutron-dhcp-agent = neutron.cmd.eventlet.agents.dhcp:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n40 : neutron-l3-agent = neutron.cmd.eventlet.agents.l3:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n41 : neutron-linuxbridge-agent = neutron.cmd.eventlet.plugins.linuxbridge_neutron_agent:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n43 : neutron-macvtap-agent = neutron.cmd.eventlet.plugins.macvtap_neutron_agent:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n44 : neutron-metadata-agent = neutron.cmd.eventlet.agents.metadata:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n46 : neutron-openvswitch-agent = neutron.cmd.eventlet.plugins.ovs_neutron_agent:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n49 : neutron-server = neutron.cmd.eventlet.server:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n50 : neutron-rpc-server = neutron.cmd.eventlet.server:main_rpc_eventlet
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n53 : neutron-usage-audit = neutron.cmd.eventlet.usage_audit:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n54 : neutron-metering-agent = neutron.cmd.eventlet.services.metering_agent:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n55 : neutron-sriov-nic-agent = neutron.cmd.eventlet.plugins.sriov_nic_neutron_agent:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n57 : neutron-periodic-workers = neutron.cmd.eventlet.server:main_periodic_eventlet
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n59 : neutron-ovn-agent = neutron.cmd.eventlet.agents.ovn_neutron_agent:main
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n60 : neutron-ovn-maintenance-worker = neutron.cmd.eventlet.server:main_ovn_maintenance_eventlet
- https://opendev.org/openstack/neutron/src/branch/master/setup.cfg#n61 : neutron-ovn-metadata-agent = neutron.cmd.eventlet.agents.ovn_metadata:main

***

## Project: neutron-dynamic-routing
---

- **Project:** neutron-dynamic-routing
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--disable-eventlet`) suggests that Eventlet can be globally deactivated.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration requiring minimal code changes.*
    *Factors for estimation: Most Eventlet-related configurations are optional and can be disabled, reducing the need for extensive refactoring.*
  - **Files Analyzed:**
    - **File:** `cmd/eventlet/__init__.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet.
    - **File:** `tests/unit/services/bgp/agent/test_bgp_dragent.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `setup.cfg`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The neutron-bgp-dragent package depends on Eventlet, which can be disabled or removed if necessary.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used sparingly across the project, mainly for managing asynchronous operations using green threads in tests.
    - **Potential Challenges:** Removing Eventlet might require adjusting test configurations and ensuring that alternative asynchronous mechanisms are properly implemented.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for minimal refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/neutron-dynamic-routing/src/branch/master/neutron_dynamic_routing/cmd/eventlet/__init__.py#n13 : import eventlet
- https://opendev.org/openstack/neutron-dynamic-routing/src/branch/master/neutron_dynamic_routing/cmd/eventlet/__init__.py#n16 : eventlet.monkey_patch()
- https://opendev.org/openstack/neutron-dynamic-routing/src/branch/master/neutron_dynamic_routing/tests/unit/services/bgp/agent/test_bgp_dragent.py#n20 : import eventlet
- https://opendev.org/openstack/neutron-dynamic-routing/src/branch/master/neutron_dynamic_routing/tests/unit/services/bgp/agent/test_bgp_dragent.py#n96 : eventlet.greenthread.sleep(1)
- https://opendev.org/openstack/neutron-dynamic-routing/src/branch/master/requirements.txt#n6 : eventlet>=0.27.0 # MIT
- https://opendev.org/openstack/neutron-dynamic-routing/src/branch/master/setup.cfg#n31 : neutron-bgp-dragent = neutron_dynamic_routing.cmd.eventlet.agents.bgp_dragent:main

***

## Project: neutron-fwaas
---

- **Project:** neutron-fwaas
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of Eventlet in configuration files and dependencies adds complexity.*
  - **Files Analyzed:**
    - **File:** `libnetfilter_log.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the log processing.
    - **File:** `tests/fullstack/__init__.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `tests/unit/privileged/netfilter_log/test_libnetfilter_log.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The test file uses an Eventlet-specific WSGI server, indicating a dependency on Eventlet's WSGI capabilities.
    - **File:** `requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, the use of Eventlet in WSGI servers may impact web service integration.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

---

- **Project:** neutron-fwaas
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of Eventlet in configuration files and dependencies adds complexity.*
  - **Files Analyzed:**
    - **File:** `libnetfilter_log.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the log processing.
    - **File:** `tests/fullstack/__init__.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `tests/unit/privileged/netfilter_log/test_libnetfilter_log.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The test file uses an Eventlet-specific WSGI server, indicating a dependency on Eventlet's WSGI capabilities.
    - **File:** `requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, the use of Eventlet in WSGI servers may impact web service integration.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/neutron-fwaas/src/branch/master/neutron_fwaas/privileged/netfilter_log/libnetfilter_log.py#n22 : import eventlet
- https://opendev.org/openstack/neutron-fwaas/src/branch/master/neutron_fwaas/privileged/netfilter_log/libnetfilter_log.py#n23 : from eventlet.green import zmq
- https://opendev.org/openstack/neutron-fwaas/src/branch/master/neutron_fwaas/privileged/netfilter_log/libnetfilter_log.py#n331 : eventlet.spawn_n(loop)
- https://opendev.org/openstack/neutron-fwaas/src/branch/master/neutron_fwaas/tests/fullstack/__init__.py#n13 : from neutron.common import eventlet_utils
- https://opendev.org/openstack/neutron-fwaas/src/branch/master/neutron_fwaas/tests/fullstack/__init__.py#n16 : eventlet_utils.monkey_patch()
- https://opendev.org/openstack/neutron-fwaas/src/branch/master/neutron_fwaas/tests/unit/privileged/netfilter_log/test_libnetfilter_log.py#n37 : self.spawn = mock.patch('eventlet.spawn').start()
- https://opendev.org/openstack/neutron-fwaas/src/branch/master/requirements.txt#n1 : eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT

***

## Project: neutron-lib
**Summary of Key Points**

The project neutron-lib uses Eventlet extensively for managing asynchronous operations using green threads and in configuration files. However, the use of Eventlet is not allowed due to a specific check (N535) that detects its usage.

**Potential Challenges**

Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.

**Recommendations**

Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

**Key Findings**

* Eventlet is used extensively in neutron-lib, but its usage is not allowed due to N535 check.
* Removing Eventlet would require significant changes to the project's asynchronous mechanisms and configuration management.
* Alternative libraries (e.g., asyncio) should be evaluated for use in the project.

**Next Steps**

1. Evaluate alternative asynchronous libraries (e.g., asyncio) for use in neutron-lib.
2. Plan for incremental refactoring of existing code to replace Eventlet with the chosen alternative library.
3. Ensure thorough testing at each stage to maintain system stability and avoid introducing new issues.

By following these steps, the project can transition away from Eventlet and adopt a more modern and stable asynchronous mechanism while minimizing disruption to the system.

Occurrences Found:
- https://opendev.org/openstack/neutron-lib/src/branch/master/HACKING.rst#n20 : - [N535] Usage of Python eventlet module not allowed
- https://opendev.org/openstack/neutron-lib/src/branch/master/doc/source/contributor/review-guidelines.rst#n69 : - eventlet
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/hacking/checks.py#n164 : def check_no_eventlet_imports(logical_line):
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/hacking/checks.py#n165 : """N535 - Usage of Python eventlet module not allowed.
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/hacking/checks.py#n172 : if re.match(r'(import|from)\s+[(]?eventlet', logical_line):
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/hacking/checks.py#n173 : msg = 'N535: Usage of Python eventlet module not allowed'
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/hacking/checks.py#n174 : yield logical_line.index('eventlet'), msg
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n138 : def test_check_eventlet_imports(self):
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n139 : f = checks.check_no_eventlet_imports
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n140 : self.assertLineFails(f, "import eventlet")
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n141 : self.assertLineFails(f, "import eventlet.timeout")
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n142 : self.assertLineFails(f, "from eventlet import timeout")
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n143 : self.assertLineFails(f, "from eventlet.timeout import Timeout")
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n144 : self.assertLineFails(f, "from eventlet.timeout import (Timeout, X)")
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n145 : self.assertLinePasses(f, "import is.not.eventlet")
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n146 : self.assertLinePasses(f, "from is.not.eventlet")
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n147 : self.assertLinePasses(f, "from mymod import eventlet")
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n148 : self.assertLinePasses(f, "from mymod.eventlet import amod")
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n149 : self.assertLinePasses(f, 'print("eventlet not here")')
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n150 : self.assertLinePasses(f, 'print("eventlet.timeout")')
- https://opendev.org/openstack/neutron-lib/src/branch/master/neutron_lib/tests/unit/hacking/test_checks.py#n151 : self.assertLinePasses(f, "from mymod.timeout import (eventlet, X)")
- https://opendev.org/openstack/neutron-lib/src/branch/master/tox.ini#n119 : N535 = neutron_lib.hacking.checks:check_no_eventlet_imports

***

## Project: neutron-specs
---

- **Project:** neutron-specs
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, modifications are required in multiple files related to pydev debugger support.*
  - **Files Analyzed:**
    - **File:** `pydev-debugger-support.rst`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** Multiple calls to eventlet require modifications, indicating its use in managing asynchronous operations.
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `hyper-v-ovs-agent.rst`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet`, which is used by the agent to set non-blocking I/O.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is extensively used across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, modifications are required in multiple files related to pydev debugger support.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage to maintain system stability, and consider leveraging the common `eventlet_patcher.py` lib to minimize changes.

Occurrences Found:
- https://opendev.org/openstack/neutron-specs/src/branch/master/specs/archive/kilo/pydev-debugger-support.rst#n48 : Minor modifications are required to deal with the multiple calls to the eventlet
- https://opendev.org/openstack/neutron-specs/src/branch/master/specs/archive/kilo/pydev-debugger-support.rst#n53 : Calls to eventlet monkey patcher have to be changed to leverage this new module
- https://opendev.org/openstack/neutron-specs/src/branch/master/specs/archive/kilo/pydev-debugger-support.rst#n128 : 3rd party Neutron code should re-use the common `eventlet_patcher.py` lib to get
- https://opendev.org/openstack/neutron-specs/src/branch/master/specs/kilo/hyper-v-ovs-agent.rst#n27 : agent.linux.ovs_lib. It is also used by eventlet to set non blocking I/O.
- https://opendev.org/openstack/neutron-specs/src/branch/master/specs/liberty/better-quotas.rst#n145 : eventlet, which with the default MySql setting block threads for about 50
- https://opendev.org/openstack/neutron-specs/src/branch/master/specs/liberty/ovs-ofctl-to-python.rst#n238 : [#ryu_req]_.  A few exceptions like eventlet are already covered by
- https://opendev.org/openstack/neutron-specs/src/branch/master/specs/liberty/reference-ipam-driver.rst#n175 : a DB transaction, thus creating the conditions for eventlet yields leading

***

## Project: neutron-tempest-plugin
---

- **Project:** neutron-tempest-plugin
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of Eventlet-specific configurations in configuration files adds complexity.*
  - **Files Analyzed:**
    - **File:** `common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the plugin.
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `setup.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** This file uses the `eventlet.wsgi` server, which is an alternative to the standard WSGI server for Eventlet.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files. The plugin also uses Eventlet's features for scheduling deferred tasks.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, the presence of Eventlet-specific configurations in configuration files adds to the challenge.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability. Consider using a migration tool to help manage the changes during the transition period.

Occurrences Found:
- https://opendev.org/openstack/neutron-tempest-plugin/src/branch/master/neutron_tempest_plugin/common/utils.py#n28 : import eventlet
- https://opendev.org/openstack/neutron-tempest-plugin/src/branch/master/neutron_tempest_plugin/common/utils.py#n83 : with eventlet.Timeout(timeout):
- https://opendev.org/openstack/neutron-tempest-plugin/src/branch/master/neutron_tempest_plugin/common/utils.py#n85 : eventlet.sleep(sleep)
- https://opendev.org/openstack/neutron-tempest-plugin/src/branch/master/neutron_tempest_plugin/common/utils.py#n86 : except eventlet.Timeout:
- https://opendev.org/openstack/neutron-tempest-plugin/src/branch/master/requirements.txt#n15 : eventlet>=0.21.0 # MIT

***

## Project: neutron-vpnaas
---

- **Project:** neutron-vpnaas
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of Eventlet in configuration files and dependencies adds complexity.*
  - **Files Analyzed:**
    - **File:** `neutron_vpnaas/cmd/eventlet/__init__.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file is referenced in the setup configuration, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The file imports `eventlet.wsgi`, which suggests that Eventlet's WSGI server is used.
    - **File:** `neutron_vpnaas/services/vpn/common/netns_wrapper.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* The file uses `eventlet.green.subprocess` to manage green threads, which is essential for the asynchronous operation of the VPN service.
    - **File:** `neutron_vpnaas/services/vpn/device_drivers/ipsec.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The file uses `@mock.patch('eventlet.sleep')` to mock Eventlet's sleep function, indicating that Eventlet is used in unit tests.
    - **File:** `neutron_vpnaas/tests/unit/services/vpn/device_drivers/test_ipsec.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* The file uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `setup.cfg`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The neutron-ovn-vpn-agent is configured to use Eventlet's ovn_agent:main script, indicating a dependency on Eventlet's WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/neutron-vpnaas/src/branch/master/neutron_vpnaas/cmd/eventlet/__init__.py#n1 : from neutron.common import eventlet_utils
- https://opendev.org/openstack/neutron-vpnaas/src/branch/master/neutron_vpnaas/cmd/eventlet/__init__.py#n3 : eventlet_utils.monkey_patch()
- https://opendev.org/openstack/neutron-vpnaas/src/branch/master/neutron_vpnaas/services/vpn/common/netns_wrapper.py#n21 : from eventlet.green import subprocess
- https://opendev.org/openstack/neutron-vpnaas/src/branch/master/neutron_vpnaas/services/vpn/device_drivers/ipsec.py#n26 : import eventlet
- https://opendev.org/openstack/neutron-vpnaas/src/branch/master/neutron_vpnaas/services/vpn/device_drivers/ipsec.py#n593 : eventlet.sleep(wait_interval)
- https://opendev.org/openstack/neutron-vpnaas/src/branch/master/neutron_vpnaas/tests/unit/services/vpn/device_drivers/test_ipsec.py#n1345 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/neutron-vpnaas/src/branch/master/neutron_vpnaas/tests/unit/services/vpn/device_drivers/test_ipsec.py#n1364 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/neutron-vpnaas/src/branch/master/neutron_vpnaas/tests/unit/services/vpn/device_drivers/test_ipsec.py#n1386 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/neutron-vpnaas/src/branch/master/setup.cfg#n35 : neutron-ovn-vpn-agent = neutron_vpnaas.cmd.eventlet.ovn_agent:main

***

## Project: os-ken
The code snippet you provided is a Python module named `hub.py` from the OpenStack project, specifically from the `os_ken` branch. It appears to be part of the OpenStack's Eventlet integration.

Here's a breakdown of what this code does:

**Importing necessary modules**

The code starts by importing various modules from the `eventlet` library, which is an asynchronous I/O framework for Python. These imports include:

* `eventlet.event`: provides support for events
* `eventlet.queue`: provides support for queues
* `eventlet.semaphore`: provides support for semaphores (synchronized access to resources)
* `eventlet.timeout`: provides support for timeouts
* `eventlet.wsgi`: provides support for WSGI servers

**Defining a hub class**

The code defines a class named `Hub` that appears to be the main entry point for the OpenStack's Eventlet integration. The class has several methods and attributes, but the most notable ones are:

* `__init__`: initializes the hub instance
* `_launch`: a method that launches a new process or thread
* `_listen_socket_loop`: a method that listens to incoming connections on a socket

**Using eventlet**

Throughout the code, you'll see instances of `eventlet` functions being used, such as:

* `eventlet.getcurrent()`: gets the current eventlet thread
* `eventlet.sleep()`: pauses execution for a specified amount of time
* `eventlet.listen()`: listens to incoming connections on a socket
* `eventlet.connect()`: establishes a connection to another process or host

**Other notable imports**

The code also includes several other imports from the OpenStack project, such as:

* `os_ken.services.protocols.bgp.base`: imports modules related to BGP (Border Gateway Protocol)
* `os_ken.services.protocols.ovsdb.client`: imports a client for OVSDB (Open vSwitch Database)

**Eventlet monkey patching**

The code includes an instance of `eventlet.monkey_patch(thread=thread)`, which is used to modify the behavior of the Python interpreter when running under Eventlet.

Overall, this code snippet appears to be part of the OpenStack's Eventlet integration, providing a foundation for asynchronous I/O and concurrency in the project.

Occurrences Found:
- https://opendev.org/openstack/os-ken/src/branch/master/doc/source/library_bgp_speaker.rst#n22 : import eventlet
- https://opendev.org/openstack/os-ken/src/branch/master/doc/source/library_bgp_speaker.rst#n25 : eventlet.monkey_patch()
- https://opendev.org/openstack/os-ken/src/branch/master/doc/source/library_bgp_speaker.rst#n54 : eventlet.sleep(30)
- https://opendev.org/openstack/os-ken/src/branch/master/doc/source/os_ken_app_api.rst#n38 : While threads and queues are currently implemented with eventlet/greenlet,
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n25 : HUB_TYPE = os.getenv('OSKEN_HUB_TYPE', 'eventlet')
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n29 : if HUB_TYPE == 'eventlet':
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n30 : import eventlet
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n34 : eventlet.sleep()
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n35 : import eventlet.event
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n36 : import eventlet.queue
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n37 : import eventlet.semaphore
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n38 : import eventlet.timeout
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n39 : import eventlet.wsgi
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n40 : from eventlet import websocket
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n47 : getcurrent = eventlet.getcurrent
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n48 : sleep = eventlet.sleep
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n49 : listen = eventlet.listen
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n50 : connect = eventlet.connect
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n53 : eventlet.monkey_patch(thread=thread)
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n74 : return eventlet.spawn(_launch, *args, **kwargs)
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n95 : return eventlet.spawn_after(seconds, _launch, *args, **kwargs)
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n109 : Queue = eventlet.queue.LightQueue
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n110 : QueueEmpty = eventlet.queue.Empty
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n111 : Semaphore = eventlet.semaphore.Semaphore
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n112 : BoundedSemaphore = eventlet.semaphore.BoundedSemaphore
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n122 : self.server = eventlet.listen(listen_info,
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n126 : self.server = eventlet.listen(listen_info[0],
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n129 : self.server = eventlet.listen(listen_info)
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n202 : eventlet.wsgi.server(self.server, self.handle, self.logger)
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n206 : Timeout = eventlet.timeout.Timeout
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n210 : self._ev = eventlet.event.Event()
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/lib/hub.py#n222 : self._ev = eventlet.event.Event()
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/services/protocols/bgp/base.py#n390 : import eventlet
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/services/protocols/bgp/base.py#n391 : server = eventlet.spawn(self._listen_socket_loop,
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/services/protocols/bgp/speaker.py#n24 : from eventlet import semaphore
- https://opendev.org/openstack/os-ken/src/branch/master/os_ken/services/protocols/ovsdb/client.py#n62 : """Emulate jsonrpc.Connection.transact_block without blocking eventlet.
- https://opendev.org/openstack/os-ken/src/branch/master/requirements.txt#n6 : eventlet>=0.27.0 # MIT

***

## Project: ovn-bgp-agent
---

- **Project:** ovn-bgp-agent
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--disable-eventlet`) suggests that Eventlet can be globally deactivated.*
  - **Estimated complexity of the migration:** 5
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The use of Eventlet is mostly limited to specific test cases and configuration options, which would require minimal refactoring or removal to deactivate Eventlet.*
  - **Files Analyzed:**
    - **File:** `tests/functional/base.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The file uses `eventlet.timeout` and `except eventlet.Timeout as e:` to handle timeouts, indicating that Eventlet is used in unit tests.
    - **File:** `tests/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file imports `eventlet` and uses `with eventlet.Timeout(timeout):`, showing a dependency on Eventlet's timeout functionality.
    - **File:** `test-requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file specifies `eventlet>=0.26.1 # MIT` as a dependency, indicating that Eventlet is used in the project's dependencies.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used primarily for testing purposes and timeout handling, which can be easily removed or replaced with alternative libraries.
    - **Potential Challenges:** Removing Eventlet might require adjustments to test cases and configuration management, but these changes are expected to be minimal.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio) for potential improvements, plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/ovn-bgp-agent/src/branch/master/ovn_bgp_agent/tests/functional/base.py#n22 : import eventlet.timeout
- https://opendev.org/openstack/ovn-bgp-agent/src/branch/master/ovn_bgp_agent/tests/functional/base.py#n60 : except eventlet.Timeout as e:
- https://opendev.org/openstack/ovn-bgp-agent/src/branch/master/ovn_bgp_agent/tests/utils.py#n16 : import eventlet
- https://opendev.org/openstack/ovn-bgp-agent/src/branch/master/ovn_bgp_agent/tests/utils.py#n56 : with eventlet.Timeout(timeout):
- https://opendev.org/openstack/ovn-bgp-agent/src/branch/master/ovn_bgp_agent/tests/utils.py#n58 : eventlet.sleep(sleep)
- https://opendev.org/openstack/ovn-bgp-agent/src/branch/master/ovn_bgp_agent/tests/utils.py#n59 : except eventlet.Timeout:
- https://opendev.org/openstack/ovn-bgp-agent/src/branch/master/test-requirements.txt#n4 : eventlet>=0.26.1 # MIT

***

## Project: ovsdbapp
---

- **Project:** ovsdbapp
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: The presence of an Eventlet-specific argparse option suggests that it might be deactivable, but the extensive use of green threads and deferred tasks indicates a strong dependency on Eventlet.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: The widespread use of Eventlet's features, including green threads and deferred tasks, would require significant refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `ovsdbapp/backend/ovs_idl/vlog.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `ovsdbapp/backend/ovs_idl/windows/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `ovsdbapp/tests/unit/test_api.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `ovsdbapp/backend/ovs_idl/windows/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Import of eventlet
          *Description:* The file imports the eventlet module, indicating its use within the project.
        - **Pattern:** Eventlet patching
          *Description:* The file uses `eventlet.patcher` to patch the eventlet module, suggesting a need for careful handling of asynchronous operations.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is used extensively across the ovsdbapp project, particularly for managing asynchronous operations using green threads and in configuration files.
  - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
  - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/backend/ovs_idl/vlog.py#n22 : from eventlet import patcher
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/backend/ovs_idl/windows/utils.py#n17 : import eventlet
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/backend/ovs_idl/windows/utils.py#n18 : from eventlet import tpool
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/backend/ovs_idl/windows/utils.py#n20 : eventlet = None
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/backend/ovs_idl/windows/utils.py#n31 : keyword arguments "kwargs". If eventlet is not installed on the system
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/backend/ovs_idl/windows/utils.py#n34 : if eventlet is None:
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/backend/ovs_idl/windows/utils.py#n40 : if eventlet.getcurrent().parent:
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/tests/unit/test_api.py#n25 : import eventlet
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/tests/unit/test_api.py#n26 : from eventlet.green import thread
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/tests/unit/test_api.py#n28 : sleep = eventlet.sleep
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/tests/unit/test_api.py#n31 : eventlet.spawn_n(executable)
- https://opendev.org/openstack/ovsdbapp/src/branch/master/ovsdbapp/tests/unit/test_api.py#n46 : if 'eventlet' in sys.modules:

***
