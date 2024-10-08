# Analysis for Team: requirements Project: requirements
---

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
    - **File:** `watcher/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `watcher/tests/applier/workflow_engine/test_workflow_service.py`
      - **Identified Pattern:** Use of `eventlet.wsgi`
        *Description:* This file contains setup code for WSGI server in the context of an Eventlet's integration, demonstrating the usage of `eventlet.wsgi`.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* Contains code related to WSGI server configuration, further emphasizing the use of Eventlet.
    - **File:** `watcher/common/service.py`
      - **Identified Pattern:** Presence in Configuration Files and Dependencies
        *Description:* As mentioned before, this file contains configurations related to `eventlet.wsgi`, highlighting a strong dependency on Eventlet's WSGI server configuration.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
  - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
  - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
https://opendev.org/openstack/requirements/src/branch/master/global-requirements.txt#n48 : eventlet!=0.18.3,!=0.20.1,!=0.21.0,!=0.23.0,!=0.25.0,!=0.32.0,!=0.34.1,!=0.34.2,!=0.34.3,!=0.35.0,!=0.36.0  # MIT
https://opendev.org/openstack/requirements/src/branch/master/openstack_requirements/tests/files/gr-base.txt#n14 : eventlet>=0.12.0
https://opendev.org/openstack/requirements/src/branch/master/openstack_requirements/tests/files/project-with-bad-requirement.txt#n9 : eventlet>=0.9.12
https://opendev.org/openstack/requirements/src/branch/master/openstack_requirements/tests/files/project-with-oslo-tar.txt#n9 : eventlet>=0.9.17
https://opendev.org/openstack/requirements/src/branch/master/openstack_requirements/tests/files/project.txt#n9 : eventlet>=0.9.12
https://opendev.org/openstack/requirements/src/branch/master/openstack_requirements/tests/files/upper-constraints.txt#n123 : eventlet===0.19.0
https://opendev.org/openstack/requirements/src/branch/master/upper-constraints.txt#n170 : eventlet===0.36.1
