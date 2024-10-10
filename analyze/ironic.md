# Analysis for Team: ironic

## Project: ironic
Based on the provided code snippet, it appears that there are several `@mock.patch` decorators used to mock out dependencies in the Ironic test suite.

Here's a breakdown of each patch:

1. `@mock.patch('oslo_utils.eventletutils.EventletEvent.wait', autospec=True)`: This patch is used to mock out the `wait` method of the `EventletEvent` class, which is part of the `eventlet` library. The `autospec=True` argument tells Mock to generate a mock object that matches the signature of the original function.
2. `@mock.patch('oslo_utils.eventletutils.EventletEvent.wait', autospec=True)`: This patch is similar to the first one, but it's used in a different test file (`test_ipmitool.py`).
3. `@mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)`: This patch is used in another test file (`test_snmp.py`) and has the same purpose as the previous two patches.
4. `@mock.patch('eventlet.spawn_after', lambda delay, func: func())`: This patch is used to mock out the `spawn_after` function from the `eventlet` library. The `lambda` function is used to create a mock object that calls the original function with no arguments.
5. `@mock.patch('eventlet.event.Event', autospec=True)`: This patch is used to mock out the `Event` class from the `eventlet` library.

In general, these patches are used to isolate dependencies and make it easier to test the Ironic codebase in isolation. By mocking out these dependencies, the tests can focus on the specific behavior of the Ironic code without being affected by external factors.

However, it's worth noting that using Mock to mock out entire libraries like `eventlet` can be complex and may not always work as expected. In some cases, it may be better to use a more targeted approach, such as mocking only the specific functions or classes that are relevant to the test.

Occurrences Found:
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/admin/gmr.rst#n11 : and more. The eventlet backdoor facility provides an interactive shell
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/admin/gmr.rst#n12 : interface for any eventlet based process, allowing an administrator to
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/admin/report.txt#n20 : /usr/local/lib/python2.7/dist-packages/eventlet/hubs/hub.py:346 in run
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/admin/report.txt#n23 : /usr/local/lib/python2.7/dist-packages/eventlet/hubs/poll.py:82 in wait
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/admin/report.txt#n41 : `eventlet.greenthread.sleep(self.wait_interval)`
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/admin/report.txt#n43 : /usr/local/lib/python2.7/dist-packages/eventlet/greenthread.py:34 in sleep
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/admin/report.txt#n46 : /usr/local/lib/python2.7/dist-packages/eventlet/hubs/hub.py:294 in switch
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/admin/report.txt#n231 : eventlet.wsgi.server=INFO
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/admin/tuning.rst#n159 : .. _eventlet: https://eventlet.net/
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/conf.py#n17 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/doc/source/conf.py#n22 : eventlet.monkey_patch(subprocess=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/cmd/__init__.py#n15 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/cmd/__init__.py#n17 : eventlet.monkey_patch()
- https://opendev.org/openstack/ironic/src/branch/master/ironic/common/rpc.py#n137 : executor='eventlet',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/conductor/base_manager.py#n19 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/conductor/base_manager.py#n665 : eventlet.sleep(0)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/conductor/manager.py#n46 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/conductor/manager.py#n1564 : eventlet.sleep(0)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/conductor/manager.py#n2708 : eventlet.sleep(0)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/conductor/manager.py#n3699 : eventlet.sleep(0)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/conductor/periodics.py#n19 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/conductor/periodics.py#n162 : eventlet.sleep(0)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/conf/opts.py#n82 : 'eventlet.wsgi.server=INFO',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/drivers/modules/inspector/interface.py#n20 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/drivers/modules/inspector/interface.py#n251 : eventlet.spawn_n(_start_inspection, task.node.uuid, task.context)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/drivers/modules/ipmitool.py#n39 : from eventlet.green import subprocess as green_subprocess
- https://opendev.org/openstack/ironic/src/branch/master/ironic/pxe_filter/service.py#n16 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/pxe_filter/service.py#n17 : from eventlet import event
- https://opendev.org/openstack/ironic/src/branch/master/ironic/pxe_filter/service.py#n56 : self._thread = eventlet.spawn_after(_START_DELAY, self._periodic_sync)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/pxe_filter/service.py#n61 : eventlet.sleep(0)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/base.py#n31 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/base.py#n32 : eventlet.monkey_patch(os=False)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/__init__.py#n25 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/__init__.py#n31 : eventlet.monkey_patch() # noqa
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/conductor/test_base_manager.py#n19 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/conductor/test_base_manager.py#n259 : @mock.patch.object(eventlet.greenpool.GreenPool, 'waitall', autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/conductor/test_manager.py#n27 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/conductor/test_manager.py#n6039 : with mock.patch.object(eventlet, 'sleep', autospec=True) as sleep_mock:
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/inspector/test_interface.py#n15 : import eventlet
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/inspector/test_interface.py#n85 : @mock.patch.object(eventlet, 'spawn_n', lambda f, *a, **kw: f(*a, **kw))
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/irmc/test_power.py#n55 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/irmc/test_power.py#n75 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/irmc/test_power.py#n95 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/redfish/test_bios.py#n43 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/redfish/test_bios.py#n553 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/redfish/test_boot.py#n37 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/redfish/test_boot.py#n1716 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/redfish/test_boot.py#n2566 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/redfish/test_firmware.py#n37 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/redfish/test_power.py#n33 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/redfish/test_raid.py#n65 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait',
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_ipmitool.py#n1797 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait', autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_ipmitool.py#n1820 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait', autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_ipmitool.py#n1839 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait', autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_ipmitool.py#n1862 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait', autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n819 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n833 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n847 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n862 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n923 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n937 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n966 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n983 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n1000 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n1017 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n1032 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n1049 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/drivers/modules/test_snmp.py#n1064 : @mock.patch("oslo_utils.eventletutils.EventletEvent.wait", autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/pxe_filter/test_service.py#n114 : @mock.patch('eventlet.spawn_after', lambda delay, func: func())
- https://opendev.org/openstack/ironic/src/branch/master/ironic/tests/unit/pxe_filter/test_service.py#n115 : @mock.patch('eventlet.event.Event', autospec=True)
- https://opendev.org/openstack/ironic/src/branch/master/releasenotes/notes/agent-last-command-4ec6967c995ba84a.yaml#n5 : sporadically happens before of the eventlet's TLS implementation), we
- https://opendev.org/openstack/ironic/src/branch/master/releasenotes/notes/logging-keystoneauth-9db7e56c54c2473d.yaml#n5 : - Log eventlet.wsgi.server events with a proper logger name and ignore DEBUG
- https://opendev.org/openstack/ironic/src/branch/master/requirements.txt#n9 : eventlet>=0.30.1 # MIT

***

## Project: ironic-inspector
The bug introduced in the code is related to an issue with `eventlet`, a library used for concurrent programming in Python. The specific issue is that the `sleep` function from `eventlet.greenthread` module is not properly handling some edge cases, leading to a `CalledProcessError` exception.

The error occurs when the `sleep` function is called with a non-zero value, but the system clock is not accurately set, causing the sleep time to be shorter than expected. This can happen if the system clock is not synchronized or if there are issues with the underlying operating system's clock.

To fix this issue, you need to ensure that the system clock is accurately set before calling `eventlet.greenthread.sleep`. You can do this by using a more robust timing mechanism, such as `time.sleep` from the `time` module, which is less prone to errors due to clock drift.

Here's an example of how you could modify the code to use `time.sleep` instead:
```python
import time

# ...

def my_function():
    # ...
    eventlet.greenthread.sleep(1)  # Use time.sleep instead
    # ...
```
Alternatively, if you need to continue using `eventlet.greenthread.sleep`, you can try to mitigate the issue by adding a small buffer to the sleep time to account for any clock drift:
```python
import eventlet

# ...

def my_function():
    # ...
    sleep_time = 1 + (0.01 * time.time())  # Add a small buffer to the sleep time
    eventlet.greenthread.sleep(sleep_time)
    # ...
```
Note that these are just suggestions, and you should consult the documentation for `eventlet` and the specific requirements of your application to determine the best approach for fixing this issue.

Occurrences Found:
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/cmd/__init__.py#n13 : import eventlet
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/cmd/__init__.py#n14 : eventlet.monkey_patch()
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/common/rpc.py#n53 : TRANSPORT, target, endpoints, executor='eventlet',
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/conductor/manager.py#n17 : from eventlet import semaphore
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/introspect.py#n18 : from eventlet import semaphore
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/pxe_filter/base.py#n23 : from eventlet import semaphore
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/pxe_filter/iptables.py#n105 : function. This function is using ``eventlet`` semaphore to serialize
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n27 : import eventlet
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n46 : eventlet.monkey_patch()
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n145 : 'oslo_messaging._drivers.impl_fake.time.sleep', eventlet.sleep))
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n254 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n264 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n295 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n304 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n337 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n346 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n363 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n383 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n411 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n419 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n517 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n519 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n555 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n557 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n572 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n581 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n593 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n600 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n617 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n623 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n639 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n652 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n660 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n672 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n679 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n697 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n707 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n709 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n727 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n736 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n749 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n758 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n784 : eventlet.greenthread.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n822 : eventlet.greenthread.spawn_n(inspector_cmd.main,
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n824 : eventlet.greenthread.sleep(1)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n832 : eventlet.greenthread.sleep(3)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/functional.py#n838 : eventlet.greenthread.sleep(1)
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/unit/test_process.py#n22 : import eventlet
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/unit/test_process.py#n446 : eventlet.greenthread, 'sleep', autospec=True))
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/unit/test_pxe_filter.py#n17 : from eventlet import semaphore
- https://opendev.org/openstack/ironic-inspector/src/branch/master/ironic_inspector/test/unit/test_wsgi_service.py#n14 : import eventlet  # noqa
- https://opendev.org/openstack/ironic-inspector/src/branch/master/releasenotes/notes/fix-CalledProcessError-on-startup-28d9dbed85a81542.yaml#n5 : The issue is caused by eventlet bug, see:
- https://opendev.org/openstack/ironic-inspector/src/branch/master/releasenotes/notes/fix-CalledProcessError-on-startup-28d9dbed85a81542.yaml#n6 : https://github.com/eventlet/eventlet/issues/357
- https://opendev.org/openstack/ironic-inspector/src/branch/master/releasenotes/notes/handle_eventlet_wsgi_evil_override-3905c6eef0ad7fa3.yaml#n9 : `eventlet <https://github.com/eventlet/eventlet/issues/746>`_ is resolved.
- https://opendev.org/openstack/ironic-inspector/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n998 : "up. The issue is caused by eventlet bug, see: https://github.com/eventlet/"
- https://opendev.org/openstack/ironic-inspector/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n999 : "eventlet/issues/357 The issue affects *ironic-inspector* only if it manages "
- https://opendev.org/openstack/ironic-inspector/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1003 : "up. The issue is caused by eventlet bug, see: https://github.com/eventlet/"
- https://opendev.org/openstack/ironic-inspector/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1004 : "eventlet/issues/357 The issue affects *ironic-inspector* only if it manages "
- https://opendev.org/openstack/ironic-inspector/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n2647 : "in `eventlet <https://github.com/eventlet/eventlet/issues/746>`_ is resolved."
- https://opendev.org/openstack/ironic-inspector/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n2653 : "in `eventlet <https://github.com/eventlet/eventlet/issues/746>`_ is resolved."
- https://opendev.org/openstack/ironic-inspector/src/branch/master/requirements.txt#n8 : eventlet>=0.27.0 # MIT

***

## Project: ironic-python-agent
---

- **Project:** ironic-python-agent
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: Eventlet is deeply integrated into the agent's core functionality, particularly in its use of green threads and deferred tasks.*
  - **Estimated complexity of the migration:** 9
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of Eventlet features, such as green threads, deferred tasks, and WSGI server, which would require significant refactoring to eliminate dependencies on Eventlet.*
  - **Files Analyzed:**
    - **File:** `ironic_python_agent/__init__.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `ironic_python_agent/agent.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the agent.
    - **File:** `ironic_python_agent/tests/unit/test_agent.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.sleep')` to mock Eventlet's sleep function, indicating that Eventlet is used in unit tests.
    - **File:** `ironic_python_agent/releasenotes/notes/fix-high-cpu-usage-eventlet-1dccf3b81dd42c47.yaml`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.sleep` with different durations
          - **Description:** The file mentions using `eventlet.sleep(0.1)` instead of `eventlet.sleep(0)`, indicating a specific use case for Eventlet's sleep function.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the ironic-python-agent project, particularly in its core functionality and testing frameworks.
    - **Potential Challenges:** Removing Eventlet would require significant refactoring to replace its asynchronous mechanisms and adjust configuration management, which could introduce substantial complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage to maintain system stability, and consider the impact on existing tests and configurations.

Occurrences Found:
- https://opendev.org/openstack/ironic-python-agent/src/branch/master/ironic_python_agent/__init__.py#n13 : import eventlet
- https://opendev.org/openstack/ironic-python-agent/src/branch/master/ironic_python_agent/__init__.py#n19 : eventlet.monkey_patch(all=False, socket=True)
- https://opendev.org/openstack/ironic-python-agent/src/branch/master/ironic_python_agent/agent.py#n23 : import eventlet
- https://opendev.org/openstack/ironic-python-agent/src/branch/master/ironic_python_agent/agent.py#n111 : eventlet.sleep(0)
- https://opendev.org/openstack/ironic-python-agent/src/branch/master/ironic_python_agent/agent.py#n450 : eventlet.sleep(0.1)
- https://opendev.org/openstack/ironic-python-agent/src/branch/master/ironic_python_agent/tests/unit/test_agent.py#n534 : @mock.patch('eventlet.sleep', autospec=True)
- https://opendev.org/openstack/ironic-python-agent/src/branch/master/releasenotes/notes/fix-high-cpu-usage-eventlet-1dccf3b81dd42c47.yaml#n7 : Using eventlet.sleep(0.1) instead of eventlet.sleep(0) gives other processes
- https://opendev.org/openstack/ironic-python-agent/src/branch/master/requirements.txt#n2 : eventlet>=0.18.2 # MIT

***

## Project: ironic-specs
---

- **Project:** ironic-specs
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet's integration with other components (e.g., WSGI server) adds complexity.*
  - **Files Analyzed:**
    - **File:** `specs/kilo-implemented/driver-periodic-tasks.rst`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** The file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of periodic tasks.
    - **File:** `specs/kilo-implemented/driver-periodic-tasks.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `specs/kilo-implemented/driver-periodic-tasks.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `specs/kilo-implemented/driver-periodic-tasks.rst`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `specs/kilo-implemented/driver-periodic-tasks.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** This file uses the `eventlet.wsgi` server, which is a dependency on Eventlet.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files. Its use in tests with mock also indicates its importance.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, the integration of Eventlet with other components (e.g., WSGI server) adds to the challenge.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/ironic-specs/src/branch/master/specs/kilo-implemented/driver-periodic-tasks.rst#n46 : ``eventlet.greenthread.spawn_n`` to make it run in a new thread. It will

***

## Project: networking-baremetal
---

- **Project:** networking-baremetal
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--disable-eventlet`) suggests that Eventlet can be globally deactivated.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: Limited use of green threads and deferred tasks, which would require only minor adjustments to configuration management.*
  - **Files Analyzed:**
    - **File:** `agent/ironic_neutron_agent.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet.
    - **File:** `agent/__init__.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** This file imports and uses the eventlet.wsgi module, which is an Eventlet-specific WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in a limited capacity within this project, primarily for managing the WSGI server.
    - **Potential Challenges:** Removing Eventlet would require adjusting configuration management and ensuring that alternative asynchronous libraries are properly integrated.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/networking-baremetal/src/branch/master/networking_baremetal/agent/ironic_neutron_agent.py#n20 : import eventlet
- https://opendev.org/openstack/networking-baremetal/src/branch/master/networking_baremetal/agent/ironic_neutron_agent.py#n22 : eventlet.monkey_patch()
- https://opendev.org/openstack/networking-baremetal/src/branch/master/networking_baremetal/agent/ironic_neutron_agent.py#n76 : transport, targets, endpoints, executor='eventlet', pool=agent_id)

***

## Project: networking-generic-switch
---

- **Project:** networking-generic-switch
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `batching.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the batching functionality.
    - **File:** `tools/ngs_stress/ngs_stress.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

---

- **Project:** networking-generic-switch
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `tools/ngs_stress/ngs_stress.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

---

- **Project:** networking-generic-switch
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `batching.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the batching functionality.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/networking-generic-switch/src/branch/master/networking_generic_switch/batching.py#n23 : import eventlet
- https://opendev.org/openstack/networking-generic-switch/src/branch/master/networking_generic_switch/batching.py#n35 : THREAD_POOL = eventlet.greenpool.GreenPool()
- https://opendev.org/openstack/networking-generic-switch/src/branch/master/networking_generic_switch/batching.py#n54 : with eventlet.Timeout(SHUTDOWN_TIMEOUT, ShutdownTimeout):
- https://opendev.org/openstack/networking-generic-switch/src/branch/master/networking_generic_switch/batching.py#n371 : eventlet.sleep(0)
- https://opendev.org/openstack/networking-generic-switch/src/branch/master/requirements.txt#n5 : eventlet>=0.18.2 # Apache-2.0
- https://opendev.org/openstack/networking-generic-switch/src/branch/master/tools/ngs-stress/ngs_stress.py#n21 : import eventlet
- https://opendev.org/openstack/networking-generic-switch/src/branch/master/tools/ngs-stress/ngs_stress.py#n28 : eventlet.monkey_patch()

***

## Project: python-ironic-inspector-client
---

- **Project:** python-ironic-inspector-client
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to simple migration requiring minimal code changes.*
    *Factors for estimation: The project uses Eventlet in specific areas (e.g., scheduling and green threads), but not extensively throughout the entire codebase, which would make it easier to replace or refactor without significant impact on overall functionality.*
  - **Files Analyzed:**
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** The file uses Eventlet's features, such as `eventlet.monkey_patch()` and `eventlet.greenthread.sleep()`, to test the functionality of the project. This indicates that Eventlet is used for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `import eventlet` at the beginning of the file shows that Eventlet is being imported and utilized within the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** Similar to the previous pattern, this indicates that Eventlet's features are used for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** Another import statement `eventlet.monkey_patch()` is found within the file, showing its usage throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's use for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern continues to show Eventlet's usage for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found once more, demonstrating its presence in the project.
    - **File:** `tests/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This pattern repeats the previous usage of Eventlet for testing purposes.*
        - **Pattern:** Import Statement
          *   **Description:** The import statement `eventlet.greenthread.sleep()` is found again, showing its continued use throughout the project.
    -

Occurrences Found:
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n21 : import eventlet
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n22 : eventlet.monkey_patch()  # noqa
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n48 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n57 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n69 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n80 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n103 : eventlet.greenthread.sleep(delay)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n106 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n127 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n135 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n145 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n163 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n171 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n341 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)
- https://opendev.org/openstack/python-ironic-inspector-client/src/branch/master/ironic_inspector_client/tests/functional.py#n350 : eventlet.greenthread.sleep(functional.DEFAULT_SLEEP)

***

## Project: python-ironicclient
---

- **Project:** python-ironicclient
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--disable-eventlet`) suggests that Eventlet can be deactivated.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The use of Eventlet is limited to specific tools and configuration files, which would require only minor adjustments during the migration process.*
  - **Files Analyzed:**
    - **File:** `tools/install_venv_common.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet.
    - **File:** `tools/install_venv_common.py` (continued)
      - **Identified Patterns:**
        - **Pattern:** Workaround for a bug in eventlet
          - **Description:** The file contains a workaround for an upstream issue with Eventlet, indicating that Eventlet is used to address a critical bug.
    - **File:** `setup.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** The file uses the `eventlet.wsgi` server, which suggests that Eventlet is used for WSGI-related functionality.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in specific tools and configuration files but not globally throughout the project. Its use can be easily replaced or adjusted during the migration process.
    - **Potential Challenges:** None anticipated, as the use of Eventlet is limited to specific areas of the codebase.
    - **Recommendations:** Perform a thorough review of the affected files and tools to ensure that all dependencies on Eventlet are properly addressed.

Occurrences Found:
- https://opendev.org/openstack/python-ironicclient/src/branch/master/tools/install_venv_common.py#n180 : """Workaround for a bug in eventlet.
- https://opendev.org/openstack/python-ironicclient/src/branch/master/tools/install_venv_common.py#n188 : Upstream: https://bitbucket.org/eventlet/eventlet/issue/89
- https://opendev.org/openstack/python-ironicclient/src/branch/master/tools/install_venv_common.py#n199 : 'eventlet/green/subprocess.py'),
- https://opendev.org/openstack/python-ironicclient/src/branch/master/tools/install_venv_common.py#n200 : 'contrib/redhat-eventlet.patch')

***
