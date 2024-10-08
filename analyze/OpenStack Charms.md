# Analysis for Team: OpenStack Charms Project: charm-manila
---

- **Project:** OpenStack Charm-Manila
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet's WSGI server is tightly integrated with the project.*
  - **Files Analyzed:**
    - **File:** `templates/mitaka/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* This configuration file includes an option to use Eventlet's WSGI server.
    - **File:** `charm.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The project uses Eventlet as a dependency in its setup.
    - **File:** `tests/test_charmanila_wsgi.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `lib/charmanila_common/lib.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated with the Charm-Manila project, especially in its WSGI configuration and scheduling mechanisms. The presence of an Eventlet-specific argparse option suggests potential deactivability but does not guarantee it.
    - **Potential Challenges:** Removing Eventlet would require extensive code refactoring to replace core asynchronous mechanisms, adjust configuration management, and ensure compatibility with dependent projects.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
https://opendev.org/openstack/charm-manila/src/branch/master/src/templates/mitaka/logging.conf#n43 : [logger_eventletwsgi]
https://opendev.org/openstack/charm-manila/src/branch/master/src/templates/mitaka/logging.conf#n46 : qualname = eventlet.wsgi.server

Project: charm-manila-ganesha
---

- **Project:** charm-manila-ganesha
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, configuration management will need to be adjusted as Eventlet is used in logging settings.*
  - **Files Analyzed:**
    - **File:** `templates/rocky/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi` as a logger event type
          - **Description:** The file specifies `logger_eventletwsgi` in the logging configuration, indicating its use.
    - **File:** `lib/ganesha/async_task.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** This file uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `tests/test_ganesha_main.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The test file contains a mock implementation of the `eventlet.spawn` function to isolate dependencies during unit testing.
    - **File:** `lib/ganesha/server.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** This file references Eventlet's WSGI server in its configuration, highlighting a dependency on the library.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is extensively used in the charm-manila-ganesha project, primarily for handling asynchronous operations using green threads and scheduled tasks. Its use affects logging settings and background operation management.
  - **Potential Challenges:** Removing Eventlet could necessitate the replacement of critical functionalities with alternative asynchronous mechanisms and adjustments to configuration management. Careful planning and incremental refactoring would be necessary to maintain system stability throughout the migration process.
  - **Recommendations:** Consider using alternative libraries like asyncio for handling asynchronous operations, thoroughly test each stage of the migration, and plan for a gradual integration approach to minimize disruptions during the transition.

Occurrences Found:
https://opendev.org/openstack/charm-manila-ganesha/src/branch/master/src/templates/rocky/logging.conf#n43 : [logger_eventletwsgi]
https://opendev.org/openstack/charm-manila-ganesha/src/branch/master/src/templates/rocky/logging.conf#n46 : qualname = eventlet.wsgi.server
