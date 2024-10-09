# Analysis for Team: watcher

## Project: watcher
---

- **Project:** watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt*: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase. Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description*: This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Configuration Management
          *Description*: 'eventlet.wsgi.server=WARN', 'iso8601=WARN' are configuration options that indicate Eventlet's presence in the system.
    - **File:** `utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Timers
          *Description*: Uses Eventlet's features to schedule deferred tasks and manage timers, impacting how background operations are handled.
    - **File:** `cmd/__init__.py` and `tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Mocking Eventlet for Testing
          *Description*: The tests import eventlet and use mocking to test the workflow engine, indicating a need to maintain compatibility with Eventlet.
    - **File:** `decision_engine/scheduling.py` and `tests/decision_engine/test_scheduling.py`
      - **Identified Patterns:**
        - **Pattern:** Scheduling and Timers
          *Description*: Uses Eventlet's features to schedule tasks and manage timers, impacting how background operations are handled.
  - **Overall Conclusion:**
    *Summary of Key Points*: Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.*
    *Potential Challenges*: Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.*
    *Recommendations*: Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/watcher/src/branch/master/watcher/applier/workflow_engine/base.py#n22 : import eventlet
- https://opendev.org/openstack/watcher/src/branch/master/watcher/applier/workflow_engine/base.py#n205 : et = eventlet.spawn(_do_execute_action, *args, **kwargs)
- https://opendev.org/openstack/watcher/src/branch/master/watcher/applier/workflow_engine/base.py#n236 : except eventlet.greenlet.GreenletExit:
- https://opendev.org/openstack/watcher/src/branch/master/watcher/cmd/__init__.py#n18 : import eventlet
- https://opendev.org/openstack/watcher/src/branch/master/watcher/cmd/__init__.py#n20 : eventlet.monkey_patch()
- https://opendev.org/openstack/watcher/src/branch/master/watcher/common/rpc.py#n140 : executor='eventlet',
- https://opendev.org/openstack/watcher/src/branch/master/watcher/common/rpc.py#n154 : executor='eventlet',
- https://opendev.org/openstack/watcher/src/branch/master/watcher/common/service.py#n61 : 'eventlet.wsgi.server=WARN', 'iso8601=WARN',
- https://opendev.org/openstack/watcher/src/branch/master/watcher/common/utils.py#n27 : import eventlet
- https://opendev.org/openstack/watcher/src/branch/master/watcher/common/utils.py#n28 : from eventlet import tpool
- https://opendev.org/openstack/watcher/src/branch/master/watcher/common/utils.py#n199 : with eventlet.timeout.Timeout(
- https://opendev.org/openstack/watcher/src/branch/master/watcher/decision_engine/scheduling.py#n19 : import eventlet
- https://opendev.org/openstack/watcher/src/branch/master/watcher/decision_engine/scheduling.py#n58 : with eventlet.Timeout(
- https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n19 : import eventlet
- https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n94 : @mock.patch('eventlet.spawn')
- https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n95 : def test_execute_with_cancel_action_plan(self, mock_eventlet_spawn):
- https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n112 : et = eventlet.spawn(empty_test)
- https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n113 : mock_eventlet_spawn.return_value = et
- https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/decision_engine/test_scheduling.py#n21 : import eventlet
- https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/decision_engine/test_scheduling.py#n120 : side_effect=lambda: eventlet.sleep(.5))

***
