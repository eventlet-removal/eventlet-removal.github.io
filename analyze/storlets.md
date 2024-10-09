# Analysis for Team: storlets

## Project: storlets
---

- **Project:** Storlets
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: The extensive use of `eventlet.spawn` and other Eventlet features across the codebase, especially in critical functionalities like gateways and runtime modules, suggests that it's deeply integrated into the system.*
  - **Estimated complexity of the migration:** 9
    *This level represents a complex migration involving extensive changes across the entire codebase. The estimated complexity arises from the need to refactor or replace critical components using Eventlet.*
    *Factors for estimation: Extensive use of green threads, deferred tasks, and scheduling features that would require significant refactoring or alternative implementation to eliminate dependencies on Eventlet.*
  - **Files Analyzed:**
    - **File:** `storlets/gateway/common/exceptions.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** Uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `storlets/gateway/gateways/container/runtime.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features for scheduling deferred tasks, impacting how background operations are handled.
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** Contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `storlets/swift_middleware/storlet_handler.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** Uses the Eventlet WSGI server, indicating that it might be deactivable or an alternative server should be considered.
    - **File:** `storlets/tests/unit/gateway/gateways/container/test_gateway.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** Uses `eventlet.sleep(0.1)` to manage green threads, which is essential for the asynchronous operation of the gateway engine.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into Storlets, particularly in critical functionalities like gateways and runtime modules.
    - **Potential Challenges:** Removing Eventlet would require significant refactoring or replacement of these critical components, potentially introducing instability during the migration process.
    - **Recommendations:** Perform thorough testing at each stage to ensure system stability. Evaluate alternative asynchronous libraries (e.g., asyncio) for potential replacements. Plan for incremental refactoring to minimize disruption.

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
