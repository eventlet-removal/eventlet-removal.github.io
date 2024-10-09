# Analysis for Team: cloudkitty

## Project: cloudkitty
---

- **Project:** CloudKitty
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, replacing Eventlet with a different library (Futurist) indicates that there are already efforts to minimize its impact.*
  - **Files Analyzed:**
    - **File:** `cloudkitty/metrics/collector.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *   **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the metrics collector.
    - **File:** `cloudkitty/services/worker.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *   **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `cloudkitty/tests/test_collector.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `cloudkitty/services/worker.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *   **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `cloudkitty/releasenotes/notes/fetch-metrics-concurrently-dffffe346bd4900e.yaml`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *   **Description:** This file indicates the use of Eventlet for concurrent metrics fetching.
  - **Overall Conclusion:**
    *Summary of Key Points:* Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.*
    *Potential Challenges:* Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, there are already efforts to minimize its impact by replacing it with Futurist.*
    *Recommendations:* Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage to maintain system stability, and consider migrating to Futurist as a step towards reducing Eventlet's dependency.*

Occurrences Found:
- https://opendev.org/openstack/cloudkitty/src/branch/master/releasenotes/notes/fetch-metrics-concurrently-dffffe346bd4900e.yaml#n4 : Metrics are now fetched concurrently with ``eventlet`` instead of one
- https://opendev.org/openstack/cloudkitty/src/branch/master/releasenotes/notes/replace-eventlet-with-futurist-60f1fe6474a5efcf.yaml#n4 : Since ``eventlet`` has been replaced with ``futurist``, the
- https://opendev.org/openstack/cloudkitty/src/branch/master/releasenotes/notes/replace-eventlet-with-futurist-60f1fe6474a5efcf.yaml#n10 : The ``eventlet`` library has been replaced with ``futurist``.

***

## Project: cloudkitty-specs
---

- **Project:** CloudKitty Specs
  - **Is Eventlet globally deactivable for this project:** Yes
    - *Reason for affirmation: The presence of an Eventlet-specific argparse option (https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n37) suggests that Eventlet can be deactivated, but further analysis is required to confirm.*
  - **Estimated complexity of the migration:** 6
    - *This level represents a moderate migration requiring some code refactoring.*
    - *Factors for estimation: Limited use of green threads and deferred tasks in critical areas, allowing for targeted refactoring.*
  - **Files Analyzed:**
    - **File:** `specs/train/concurrency.rst`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** Eventlet is used to manage green threads, as indicated by the use of `eventlet.spawn` in `n119`.
    - **File:** `specs/train/concurrency.rst`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains dependencies on Eventlet's WSGI server, as indicated by the mention of `eventlet.wsgi` in configuration files.
    - **File:** `specs/train/concurrency.rst`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** Eventlet is used in unit tests using `mock.patch('eventlet.spawn')` to mock the spawn function (https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n34).
    - **File:** `specs/train/concurrency.rst`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Eventlet's features are used to schedule deferred tasks, impacting the handling of background operations (https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n119).
    - **File:** `specs/train/concurrency.rst`
      - **Identified Patterns:**
        - **Pattern:** Eventlet Documentation Reference
          - **Description:** The Eventlet documentation is referenced in the spec file (https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n149).
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a supporting role in CloudKitty Specs, primarily for managing green threads and deferred tasks. Its use is mostly limited to specific areas.
    - **Potential Challenges:** Removing or replacing Eventlet could introduce complexity due to its targeted usage in specific parts of the spec file.
    - **Recommendations:** Perform a thorough analysis of the areas relying on Eventlet, plan incremental refactoring, and ensure comprehensive testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n34 : by making use of eventlet greenthreads. CloudKitty already depends on eventlet,
- https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n37 : dependency on eventlet (which could be replaced by concurrent.futures or
- https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n119 : * Retrieve metrics in eventlet greenthreads.
- https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n149 : * Eventlet documentation: https://eventlet.net/

***
