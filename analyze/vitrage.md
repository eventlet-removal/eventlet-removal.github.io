# Analysis for Team: vitrage Project: vitrage
---

- **Project:** vitrage
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet, as well as configuration adjustments related to the WSGI server.*
  - **Files Analyzed:**
    - **File:** `vitrage/engines/services.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file utilizes Eventlet's green threads for efficient service management, which is essential for the asynchronous operation of the engine.
    - **File:** `vitrage/common/timers.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file includes references to eventlet.wsgi, indicating a dependency on Eventlet's WSGI server for configuration purposes.
    - **File:** `vitrage/tests/engines/services/test_services.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file employs mock.patch('eventlet.spawn') to mock Eventlet's spawn function, signifying the project's reliance on Eventlet for testing asynchronous operations.
    - **File:** `vitrage/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Utilizes Eventlet features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is widely used across vitrage for efficient asynchronous service management and scheduling.
    - **Potential Challenges:** Migrating away from Eventlet could require significant refactoring of critical functionality involving green threads and deferred tasks, as well as adjustments to configuration related to the WSGI server.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring to minimize disruptions, ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
https://opendev.org/openstack/vitrage/src/branch/master/releasenotes/notes/services-management-improvements-899c011e57002e84.yaml#n8 : eventlets and timers.
https://opendev.org/openstack/vitrage/src/branch/master/requirements.txt#n38 : eventlet!=0.20.1,>=0.20.0 # MIT
