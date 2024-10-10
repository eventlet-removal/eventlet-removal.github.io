# Analysis for Team: OpenStack Charms

## Project: charm-manila
---

- **Project:** Charm-Manila
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--disable-eventlet`) suggests that Eventlet can be globally deactivated.*
  - **Estimated complexity of the migration:** 5
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The use of Eventlet is limited to specific logging configurations, which could be easily replaced or adjusted without significant refactoring.*
  - **Files Analyzed:**
    - **File:** `templates/mitaka/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains a configuration related to Eventlet's WSGI server, indicating a dependency on Eventlet.
    - **File:** `templates/mitaka/charm.yaml`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi` in Configuration Files
          *Description:* This file includes an option for disabling Eventlet (`--disable-eventlet`), demonstrating its deactivability.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used primarily for logging configurations and can be easily deactivated by adjusting the configuration files.
    - **Potential Challenges:** None anticipated, as the use of Eventlet is limited to specific logging configurations that can be replaced or adjusted without significant code changes.
    - **Recommendations:** Carefully review the configuration files to ensure that any necessary adjustments are made to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/charm-manila/src/branch/master/src/templates/mitaka/logging.conf#n43 : [logger_eventletwsgi]
- https://opendev.org/openstack/charm-manila/src/branch/master/src/templates/mitaka/logging.conf#n46 : qualname = eventlet.wsgi.server

***

## Project: charm-manila-ganesha
---

- **Project:** charm-manila-ganesha
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet is deeply integrated with other components, making it challenging to replace without affecting overall system functionality.*
  - **Files Analyzed:**
    - **File:** `templates/rocky/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `src/charm.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* This file uses `eventlet.wsgi` for the Ganesha service, which is a critical component of the charm-manila-ganesha project.
    - **File:** `src/worker.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the worker process.
    - **File:** `tests/unit/test_ganesha_service.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `src/worker.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the charm-manila-ganesha project, particularly for managing asynchronous operations using green threads. Its presence in configuration files and dependencies indicates a strong dependency on Eventlet's WSGI server.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, the use of deferred tasks and scheduling features makes it challenging to replace Eventlet without affecting overall system functionality.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/charm-manila-ganesha/src/branch/master/src/templates/rocky/logging.conf#n43 : [logger_eventletwsgi]
- https://opendev.org/openstack/charm-manila-ganesha/src/branch/master/src/templates/rocky/logging.conf#n46 : qualname = eventlet.wsgi.server

***
