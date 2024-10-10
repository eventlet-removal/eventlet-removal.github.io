# Analysis for Team: watcher

## Project: watcher
- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `watcher/applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* The code utilizes Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `watcher/cmd/__init__.py`
      - **Identified Pattern:**
        - **Pattern:** Eventlet Monkey Patching
          *Description:* The file imports and patches Eventlet using monkey patching, indicating its presence in the project's core functionality.
    - **File:** `watcher/common/rpc.py`
      - **Identified Patterns:**
        - **Pattern:** Eventlet Executor
          *Description:* The code uses an executor from Eventlet, indicating its use for asynchronous operations.
        - **Pattern:** Eventlet Executor
          *Description:* Another instance of using an executor from Eventlet, further emphasizing its presence in the project's core functionality.
    - **File:** `watcher/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Eventlet WSGI Server
          *Description:* The code specifies a warning for the use of Eventlet's WSGI server, indicating its integration with the project.
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Eventlet Timeout
          *Description:* The file uses Eventlet's timeout feature to manage asynchronous operations.
        - **Pattern:** Eventlet TPool
          *Description:* It imports and utilizes Eventlet's thread pool (tpool) for managing threads in the background.
    - **File:** `watcher/decision_engine/scheduling.py`
      - **Identified Patterns:**
        - **Pattern:** Eventlet Timeout
          *Description:* The code uses an eventlet timeout, emphasizing its use of asynchronous operations.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Mocking Eventlet Spawn
          *Description:* The file mocks the spawn function from Eventlet to test its functionality, indicating its presence in the project's testing framework.
    - **File:** `watcher/tests/decision_engine/test_scheduling.py`
      - **Identified Pattern:**
        - **Pattern:** Eventlet Sleep
          *Description:* The code uses an eventlet sleep function, further emphasizing its use of asynchronous operations.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
  - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
  - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Data for project watcher: https://opendev.org/openstack/watcher/src/branch/master/watcher/applier/workflow_engine/base.py#n22 : import eventlet
https://opendev.org/openstack/watcher/src/branch/master/watcher/applier/workflow_engine/base.py#n205 : et = eventlet.spawn(_do_execute_action, *args, **kwargs)
https://opendev.org/openstack/watcher/src/branch/master/watcher/applier/workflow_engine/base.py#n236 : except eventlet.greenlet.GreenletExit:
https://opendev.org/openstack/watcher/src/branch/master/watcher/cmd/__init__.py#n18 : import eventlet
https://opendev.org/openstack/watcher/src/branch/master/watcher/cmd/__init__.py#n20 : eventlet.monkey_patch()
https://opendev.org/openstack/watcher/src/branch/master/watcher/common/rpc.py#n140 : executor='eventlet',
https://opendev.org/openstack/watcher/src/branch/master/watcher/common/rpc.py#n154 : executor='eventlet',
https://opendev.org/openstack/watcher/src/branch/master/watcher/common/service.py#n61 : 'eventlet.wsgi.server=WARN', 'iso8601=WARN',
https://opendev.org/openstack/watcher/src/branch/master/watcher/common/utils.py#n27 : import eventlet
https://opendev.org/openstack/watcher/src/branch/master/watcher/common/utils.py#n28 : from eventlet import tpool
https://opendev.org/openstack/watcher/src/branch/master/watcher/common/utils.py#n199 : with eventlet.timeout.Timeout(
https://opendev.org/openstack/watcher/src/branch/master/watcher/decision_engine/scheduling.py#n19 : import eventlet
https://opendev.org/openstack/watcher/src/branch/master/watcher/decision_engine/scheduling.py#n58 : with eventlet.Timeout(
https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n12 : import mock
https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n15 : from unittest.mock import patch
https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n17 : with mock.patch.object(eventlet, 'spawn') as mock_spawn:
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Pattern:**
        - **Pattern:** Mocking Eventlet Spawn
          *Description:* The file mocks the spawn function from Eventlet to test its functionality, indicating its presence in the project's testing framework.

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
