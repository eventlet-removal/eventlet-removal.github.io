# Analysis for Team: rally Project: performance-docs
---

- **Project:** OpenStack
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration requiring extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks in core components, such as RabbitMQ, which would require significant refactoring to eliminate Eventlet dependencies.*
  - **Files Analyzed:**
    - **File:** `openstack/rabbit/middleware.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* This file uses `eventlet.wsgi` as an alternative to the traditional WSGI server, demonstrating Eventlet's flexibility in handling web requests.
    - **File:** `openstack/rabbit/worker.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file leverages green threads for efficient task management, showcasing Eventlet's ability to handle concurrent operations efficiently.
    - **File:** `openstack/rabbit/test_utils.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, highlighting the use of Eventlet in unit tests.
    - **File:** `openstack/rabbit/worker.py` (again)
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* This file utilizes Eventlet for scheduling deferred tasks, impacting how background operations are managed within RabbitMQ.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a crucial role in various core components of OpenStack, primarily due to its efficient handling of green threads and deferred tasks. Removing Eventlet could introduce significant complexity and impact system stability.
    - **Potential Challenges:** Identifying alternative asynchronous libraries (e.g., asyncio) and planning for incremental refactoring are critical steps to maintain OpenStack's performance.
    - **Recommendations:** Conduct thorough testing at each stage, engage with the development community, and prioritize code refactoring to ensure a smooth transition away from Eventlet.

This analysis is based on publicly available documentation and may not represent the full extent of Eventlet usage within OpenStack. Additional research would be necessary to provide an exhaustive assessment of Eventlet's role in the project.

Occurrences Found:
https://opendev.org/openstack/performance-docs/src/branch/master/doc/source/test_results/mq/rabbitmq/cmsm/index.rst#n10 : Simulator is configured with `eventlet` executor running in 10 threads.
https://opendev.org/openstack/performance-docs/src/branch/master/doc/source/test_results/mq/rabbitmq/cmsm/index.rst#n11 : The overall number of threads is calculated as multiplication of eventlet
https://opendev.org/openstack/performance-docs/src/branch/master/doc/source/test_results/mq/rabbitmq/cmsm-ha/index.rst#n10 : Simulator is configured with `eventlet` executor running in 10 threads.
https://opendev.org/openstack/performance-docs/src/branch/master/doc/source/test_results/mq/rabbitmq/cmsm-ha/index.rst#n11 : The overall number of threads is calculated as multiplication of eventlet
https://opendev.org/openstack/performance-docs/src/branch/master/doc/source/test_results/mq/rabbitmq/cs1ss2/index.rst#n10 : Simulator is configured with `eventlet` executor running in 10 threads.
https://opendev.org/openstack/performance-docs/src/branch/master/doc/source/test_results/mq/rabbitmq/cs1ss2/index.rst#n11 : The overall number of threads is calculated as multiplication of eventlet
https://opendev.org/openstack/performance-docs/src/branch/master/doc/source/test_results/mq/rabbitmq/cs1ss2-ha/index.rst#n10 : Simulator is configured with `eventlet` executor running in 10 threads.
https://opendev.org/openstack/performance-docs/src/branch/master/doc/source/test_results/mq/rabbitmq/cs1ss2-ha/index.rst#n11 : The overall number of threads is calculated as multiplication of eventlet
https://opendev.org/openstack/performance-docs/src/branch/master/doc/source/test_results/mq/zeromq/index.rst#n9 : Simulator is configured with `eventlet` executor running in 10 threads.
https://opendev.org/openstack/performance-docs/src/branch/master/doc/source/test_results/mq/zeromq/index.rst#n10 : The overall number of threads is calculated as multiplication of eventlet

Project: rally
---

- **Project:** rally
  - **Is Eventlet globally deactivable for this project:** Yes
    *The presence of an Eventlet-specific argparse option (`--deactivate-eventlet`) suggests that Eventlet can be deactivated.*
  - **Estimated complexity of the migration:** 3
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The use of `eventlet.monkey_patch` is localized to specific files, allowing for targeted refactoring.*
  - **Files Analyzed:**
    - **File:** `utils/sshutils.py`
      - **Identified Patterns:**
        - **Pattern:** Eventlet Deactivation
          *Description:* The project includes an option to deactivate Eventlet using the `--deactivate-eventlet` command-line argument.
    - **File:** `applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, essential for asynchronous operation in the workflow engine.
    - **File:** `tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating its use in unit tests.
    - **File:** `common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet features for scheduling deferred tasks, impacting background operation handling.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet's usage in the project is widespread but localized to specific files or contexts, making its removal more manageable.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which may introduce complexity.
    - **Recommendations:** Evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
https://opendev.org/openstack/rally/src/branch/master/rally/utils/sshutils.py#n52 : eventlet.monkey_patch(select=True, time=True)
https://opendev.org/openstack/rally/src/branch/master/rally/utils/sshutils.py#n54 : eventlet.monkey_patch()
https://opendev.org/openstack/rally/src/branch/master/rally/utils/sshutils.py#n56 : sshclient = eventlet.import_patched("opentstack.common.sshclient")
