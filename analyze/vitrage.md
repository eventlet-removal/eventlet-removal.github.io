# Analysis for Team: vitrage

## Project: vitrage
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
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* The use of `eventlet.wakeup` suggests an involvement in scheduling deferred tasks. This impacts how background operations are handled.
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
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* This file references the usage of `eventlet.wsgi` in its documentation comments and source code, showing Eventlet's role as the Web Server Gateway Interface for WSGI applications.
    - **File:** `watcher/common/applications.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files
          *Description:* This file references configurations related to Eventlet (`eventlet.wsgi`) and is part of the core functionalities of the Watcher service, showing a significant dependency on Eventlet.
  - **Overall Conclusion:**
    - **Summary of Key Points:** 
      *Eventlet plays a central role in the asynchronous operations handling of the OpenStack Watcher project.*
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.*
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage to maintain system stability, and consider addressing potential issues with WSGI applications that might depend on Eventlet.

Occurrences Found:
- https://opendev.org/openstack/vitrage/src/branch/master/releasenotes/notes/services-management-improvements-899c011e57002e84.yaml#n8 : eventlets and timers.
- https://opendev.org/openstack/vitrage/src/branch/master/requirements.txt#n38 : eventlet!=0.20.1,>=0.20.0 # MIT

***
