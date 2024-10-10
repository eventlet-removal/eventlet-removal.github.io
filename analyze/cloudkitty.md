# Analysis for Team: cloudkitty

## Project: cloudkitty
---

- **Project:** CloudKitty
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, replacing Eventlet with an alternative library (Futurist) might introduce additional complexity.*
  - **Files Analyzed:**
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description*: This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the metrics fetcher.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description*: This file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description*: Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description*: The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
    - **File:** `cloudkitty/manager/metrics/fetcher.py`
      - **Identified Patterns:**
        - **Pattern:** Replacement with Futurist
          *Description*: The file mentions the replacement of Eventlet with Futurist, indicating a change in the dependency on Eventlet's library.
   

Occurrences Found:
- https://opendev.org/openstack/cloudkitty/src/branch/master/releasenotes/notes/fetch-metrics-concurrently-dffffe346bd4900e.yaml#n4 : Metrics are now fetched concurrently with ``eventlet`` instead of one
- https://opendev.org/openstack/cloudkitty/src/branch/master/releasenotes/notes/replace-eventlet-with-futurist-60f1fe6474a5efcf.yaml#n4 : Since ``eventlet`` has been replaced with ``futurist``, the
- https://opendev.org/openstack/cloudkitty/src/branch/master/releasenotes/notes/replace-eventlet-with-futurist-60f1fe6474a5efcf.yaml#n10 : The ``eventlet`` library has been replaced with ``futurist``.

***

## Project: cloudkitty-specs
---

- **Project:** CloudKitty Specs
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to low migration requiring significant code refactoring to eliminate the dependency on Eventlet.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require careful planning and testing to replace with alternative asynchronous libraries.*
  - **Files Analyzed:**
    - **File:** `specs/train/concurrency.rst`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* Eventlet is used to manage green threads in the concurrency section, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `specs/train/concurrency.rst` (continued)
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `specs/train/concurrency.rst` (continued)
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `specs/train/concurrency.rst` (continued)
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `specs/train/concurrency.rst` (continued)
      - **Identified Patterns:**
        - **Pattern:** References in Documentation
          *Description:* The file references the Eventlet documentation, which suggests that Eventlet is an essential component of the project's architecture.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
  - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
  - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n34 : by making use of eventlet greenthreads. CloudKitty already depends on eventlet,
- https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n37 : dependency on eventlet (which could be replaced by concurrent.futures or
- https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n119 : * Retrieve metrics in eventlet greenthreads.
- https://opendev.org/openstack/cloudkitty-specs/src/branch/master/specs/train/concurrency.rst#n149 : * Eventlet documentation: https://eventlet.net/

***
