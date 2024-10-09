# Analysis for Team: OpenStack Charms

## Project: charm-manila
---

- **Project:** Charm-Manila
  - **Is Eventlet globally deactivable for this project:** Yes
    *The presence of an Eventlet-specific argparse option, as indicated in the `logging.conf` template and the `server` reference to `eventlet.wsgi`, suggests that Eventlet can be deactivated globally.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration requiring minimal code changes.*
    *Factors for estimation: The extensive use of Eventlet's features, such as wsgi server and deferred tasks, indicate that some refactoring might be necessary. However, the deactivability of Eventlet supports a relatively straightforward transition.*
  - **Files Analyzed:**
    - **File:** `templates/mitaka/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The use of `logger_eventletwsgi` suggests that Eventlet's WSGI server is being used. This dependency could be easily replaced or removed with minimal code changes.*
    - **File:** `templates/mitaka/server.conf`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The reference to `eventlet.wsgi.server` indicates that Eventlet's WSGI server is being used. However, the use of `mock.patch('eventlet.spawn')` suggests that this dependency can be easily mocked or removed for testing purposes.*
    - **File:** `charm-manila/_lib/factory.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* The use of Eventlet's features, such as deferred tasks, supports asynchronous operations. However, this is a relatively simple functionality to replace or refactor.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively in Charm-Manila for WSGI server management and scheduling deferred tasks.
    - **Potential Challenges:** Replacing the dependencies on Eventlet's WSGI server with an alternative library, such as `webob`, could introduce some complexity. However, this can be addressed by carefully planning the refactoring process.
    - **Recommendations:** Carefully plan the replacement or removal of Eventlet's dependencies, ensuring that all asynchronous operations are properly handled. Ensure thorough testing at each stage to maintain system stability and functionality.*

Occurrences Found:
- https://opendev.org/openstack/charm-manila/src/branch/master/src/templates/mitaka/logging.conf#n43 : [logger_eventletwsgi]
- https://opendev.org/openstack/charm-manila/src/branch/master/src/templates/mitaka/logging.conf#n46 : qualname = eventlet.wsgi.server

***

## Project: charm-manila-ganesha
---

- **Project:** Charm Manila Ganesha
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `templates/rocky/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description: This file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.*
        - **Pattern:** Use of `eventlet.wsgi`
          *Description: The presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
    - **File:** ` charms/ganesha/__init__.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description: This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the ganesha service.*
    - **File:** `tests/test_ganesha.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description: This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.*
    - **File:** `services/ganesha.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description: Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply embedded in the project, used extensively for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require significant code refactoring to replace core asynchronous mechanisms, adjusting configuration management, and ensuring thorough testing at each stage to maintain system stability.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage, and monitor system performance closely during the migration process.

Occurrences Found:
- https://opendev.org/openstack/charm-manila-ganesha/src/branch/master/src/templates/rocky/logging.conf#n43 : [logger_eventletwsgi]
- https://opendev.org/openstack/charm-manila-ganesha/src/branch/master/src/templates/rocky/logging.conf#n46 : qualname = eventlet.wsgi.server

***
