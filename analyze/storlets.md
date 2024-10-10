# Analysis for Team: storlets

## Project: storlets
---

- **Project:** OpenStack Storlets
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of Eventlet in multiple files and modules indicates that removing it would require careful consideration of alternative asynchronous mechanisms.*
  - **Files Analyzed:**
    - **File:** `storlets/gateway/common/exceptions.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The file imports the `Timeout` class from Eventlet's WSGI server, indicating a dependency on this feature.
    - **File:** `storlets/gateway/gateways/container/runtime.py`
      - **Identified Patterns:**
        - **Pattern 1:** Green Threads and GreenPool
          *Description:* The file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the gateway.
        - **Pattern 2:** Presence in Configuration Files and Dependencies
          *Description:* The file imports Eventlet's features, indicating a dependency on this library.
    - **File:** `storlets/swift_middleware/storlet_handler.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The file imports the `Timeout` class from Eventlet's WSGI server, indicating a dependency on this feature.
    - **File:** `storlets/tests/unit/gateway/gateways/container/test_gateway.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `storlets/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/storlets/src/branch/master/requirements.txt#n3 : eventlet>=0.17.4 # MIT
- https://opendev.org/openstack/storlets/src/branch/master/storlets/gateway/common/exceptions.py#n17 : from eventlet import Timeout
- https://opendev.org/openstack/storlets/src/branch/master/storlets/gateway/gateways/container/runtime.py#n24 : import eventlet
- https://opendev.org/openstack/storlets/src/branch/master/storlets/gateway/gateways/container/runtime.py#n40 : eventlet.monkey_patch()
- https://opendev.org/openstack/storlets/src/branch/master/storlets/gateway/gateways/container/runtime.py#n691 : eventlet.spawn_n(self._write_input_data,
- https://opendev.org/openstack/storlets/src/branch/master/storlets/gateway/gateways/container/runtime.py#n700 : eventlet.spawn_n(self._write_input_data,
- https://opendev.org/openstack/storlets/src/branch/master/storlets/swift_middleware/storlet_handler.py#n17 : from eventlet import Timeout
- https://opendev.org/openstack/storlets/src/branch/master/tests/unit/gateway/gateways/container/test_gateway.py#n16 : import eventlet
- https://opendev.org/openstack/storlets/src/branch/master/tests/unit/gateway/gateways/container/test_gateway.py#n497 : eventlet.sleep(0.1)

***
