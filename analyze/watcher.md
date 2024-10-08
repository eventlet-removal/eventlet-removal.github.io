# Analysis for Team: watcher Project: watcher
---

- **Project:** watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `cmd/__init__.py`
      - **Identified Pattern:** Presence of Eventlet in the project
        *Description:* The import statement indicates that Eventlet is being used elsewhere in the project, suggesting that it plays a crucial role in the overall functionality.
    - **File:** `common/rpc.py`
      - **Identified Pattern:** Use of eventlet in execution
        *Description:* The executor parameter is set to 'eventlet', indicating that Eventlet is being utilized for asynchronous task execution.
    - **File:** `common/service.py`
      - **Identified Pattern:** Configuration setting involving Eventlet
        *Description:* The configuration option for Eventlet.wsgi.server and iso8601 highlights its presence in the project's configuration files.
    - **File:** `common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Import statement indicating Eventlet usage
          *Description:* The import statement indicates that Eventlet is being used elsewhere in the project, suggesting that it plays a crucial role in the overall functionality.
        - **Pattern:** Use of eventlet.timeout.Timeout
          *Description:* This suggests that Eventlet's timeout feature is being utilized to manage asynchronous tasks.
    - **File:** `scheduling.py`
      - **Identified Pattern:** Use of eventlet in task scheduling
        *Description:* The with eventlet.Timeout() block indicates that Eventlet's timing mechanisms are being used for task management.
    - **File:** `tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Mocking of eventlet.spawn
          *Description:* This suggests that Eventlet is being mocked in tests to isolate the behavior of asynchronous tasks, indicating its importance in the project's testing framework.
        - **Pattern:** Usage of mock_eventlet_spawn.return_value = et
          *Description:* This line highlights the role of Eventlet in managing asynchronous task execution and completion.
    - **File:** `tests/decision_engine/test_scheduling.py`
      - **Identified Pattern:** Use of eventlet.sleep in test scheduling
        *Description:* The use of eventlet.sleep(.5) as a side effect suggests that Eventlet's timing mechanisms are being utilized to manage the flow of tasks in the decision engine.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the watcher project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Documentation on the project can be found here: https://opendev.org/openstack/watcher/src/branch/master

Occurrences Found:
https://opendev.org/openstack/watcher/src/branch/master/watcher/applier/workflow_engine/base.py#n22 : import eventlet
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
https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n19 : import eventlet
https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n94 : @mock.patch('eventlet.spawn')
https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n95 : def test_execute_with_cancel_action_plan(self, mock_eventlet_spawn):
https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n112 : et = eventlet.spawn(empty_test)
https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/applier/workflow_engine/test_taskflow_action_container.py#n113 : mock_eventlet_spawn.return_value = et
https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/decision_engine/test_scheduling.py#n21 : import eventlet
https://opendev.org/openstack/watcher/src/branch/master/watcher/tests/decision_engine/test_scheduling.py#n120 : side_effect=lambda: eventlet.sleep(.5))
