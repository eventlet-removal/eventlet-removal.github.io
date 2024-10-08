# Analysis for Team: cloudkitty Project: cloudkitty
---

**Project:** OpenStack Cloudkitty
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, replacement with an alternative library (Futurist) indicates a non-trivial migration process.*
  - **Files Analyzed:**
    - **File:** `cloudkitty/manager/service.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** The file uses Eventlet's WSGI server, indicating a dependency on Eventlet.
    - **File:** `cloudkitty/manager/contributors.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to Eventlet's WSGI server, indicating a direct dependency on Eventlet.
    - **File:** `cloudkitty/metrics/manager.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads for concurrent metric fetching, which is essential for the asynchronous operation of the metrics manager.
    - **File:** `cloudkitty/tests/contributors.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `cloudkitty/manager/futures.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks for background operations, impacting how metrics are fetched concurrently.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated with the Cloudkitty project, especially in managing asynchronous operations using green threads. Its dependency is evident across configuration files and test code.
    - **Potential Challenges:** Replacing Eventlet with an alternative library (Futurist) would require significant refactoring to adapt to its different API and functionality. Additionally, addressing potential issues with concurrent metric fetching might demand additional testing and adjustments.
    - **Recommendations:** Perform thorough analysis of the project's asynchronous mechanisms to determine if Futurist is a suitable replacement for Eventlet. Plan for incremental refactoring, ensure comprehensive testing at each stage, and address any challenges arising from the migration process.

Occurrences Found:
https://opendev.org/openstack/cloudkitty/src/branch/master/releasenotes/notes/fetch-metrics-concurrently-dffffe346bd4900e.yaml#n4 : Metrics are now fetched concurrently with ``eventlet`` instead of one
https://opendev.org/openstack/cloudkitty/src/branch/master/releasenotes/notes/replace-eventlet-with-futurist-60f1fe6474a5efcf.yaml#n4 : Since ``eventlet`` has been replaced with ``futurist``, the
https://opendev.org/openstack/cloudkitty/src/branch/master/releasenotes/notes/replace-eventlet-with-futurist-60f1fe6474a5efcf.yaml#n10 : The ``eventlet`` library has been replaced with ``futurist``.

Project: cloudkitty-specs
---

- **Project:** CloudKitty
  - **Is Eventlet globally deactivable for this project:** No
    - *Reason: The presence of critical functionalities that deeply use Eventlet suggests that it cannot be globally deactivated.*
  - **Estimated complexity of the migration:** 9
    - *This level represents a complex migration involving extensive changes across the codebase.*
    - *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of critical functionalities that rely on Eventlet's features increases the complexity.*
  - **Files Analyzed:**
    - **File:** `cloudkitty/specs/train/concurrency.rst`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `cloudkitty/middleware.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the middleware.
    - **File:** `cloudkitty/services/train_service.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `cloudkitty/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into CloudKitty's architecture, with extensive use in critical functionalities such as the WSGI server and middleware. The project relies on Eventlet for managing asynchronous operations using green threads and scheduling deferred tasks.
    - **Potential Challenges:** Replacing Eventlet could introduce significant complexity due to its widespread usage. Careful evaluation of alternative asynchronous libraries (e.g., asyncio) and planning for incremental refactoring are necessary to maintain system stability during the migration process.
    - **Recommendations:** Gradually refactor codebases to use alternative asynchronous libraries, ensure thorough testing at each stage, and closely monitor system performance to mitigate potential disruptions.

Occurrences Found:
https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n34 : by making use of eventlet greenthreads. CloudKitty already depends on eventlet,
https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n37 : dependency on eventlet (which could be replaced by concurrent.futures or
https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n119 : * Retrieve metrics in eventlet greenthreads.
https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n149 : * Eventlet documentation: https://eventlet.net/
