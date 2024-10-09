# Analysis for Team: zun

## Project: zun
---

- **Project:** zun
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: Although Eventlet is not explicitly mentioned in the configuration files and dependencies, the presence of `eventlet>=0.28.0 # MIT` in the requirements.txt file suggests that Eventlet can be deactivated.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The majority of Eventlet usage is encapsulated within modules and classes, allowing for easy replacement or deactivation without affecting critical functionalities.*
  - **Files Analyzed:**
    - **File:** `zun/cmd/__init__.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description: The file contains the `eventlet.monkey_patch()` function, indicating a dependency on Eventlet.*
    - **File:** `zun/common/rpc_service.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description: The file uses `executor='eventlet'` in the `rpc_service.py`, which is an indicator of Eventlet's use in WSGI server management.*
    - **File:** `zun/container/docker/driver.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description: The file utilizes `eventlet.Timeout(CONF.docker.execute_timeout)` to manage timeouts, impacting deferred tasks scheduling.*
    - **File:** `zun/websocket/websocketproxy.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description: This test file imports `hubs` from Eventlet's `eventlet` module using `from eventlet import hubs`, demonstrating its use in testing.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is widely used within the zun project, primarily for managing asynchronous operations and WSGI server management.
    - **Potential Challenges:** Deactivating or replacing Eventlet requires careful consideration to avoid breaking core functionality. Incremental refactoring and thorough testing should be emphasized to ensure system stability during migration.*
    - **Recommendations:**

- zun's reliance on eventlet makes it an ideal candidate for exploring alternative libraries (e.g., asyncio). A step-by-step approach, including testing at each stage, will help maintain system stability while deactivating Eventlet.

Occurrences Found:
- https://opendev.org/openstack/zun/src/branch/master/contrib/nova-docker/nova/virt/zun/driver.py#n27 : import eventlet
- https://opendev.org/openstack/zun/src/branch/master/contrib/nova-docker/nova/virt/zun/driver.py#n467 : except eventlet.timeout.Timeout:
- https://opendev.org/openstack/zun/src/branch/master/contrib/nova-docker/nova/virt/zun/opencontrail.py#n16 : import eventlet
- https://opendev.org/openstack/zun/src/branch/master/contrib/nova-docker/nova/virt/zun/opencontrail.py#n34 : self._vrouter_semaphore = eventlet.semaphore.Semaphore()
- https://opendev.org/openstack/zun/src/branch/master/requirements.txt#n10 : eventlet>=0.28.0 # MIT
- https://opendev.org/openstack/zun/src/branch/master/zun/cmd/__init__.py#n18 : import eventlet
- https://opendev.org/openstack/zun/src/branch/master/zun/cmd/__init__.py#n20 : eventlet.monkey_patch()
- https://opendev.org/openstack/zun/src/branch/master/zun/common/rpc_service.py#n55 : executor='eventlet',
- https://opendev.org/openstack/zun/src/branch/master/zun/common/utils.py#n22 : import eventlet
- https://opendev.org/openstack/zun/src/branch/master/zun/common/utils.py#n194 : """Passthrough method for eventlet.spawn_n.
- https://opendev.org/openstack/zun/src/branch/master/zun/common/utils.py#n213 : eventlet.spawn_n(context_wrapper, *args, **kwargs)
- https://opendev.org/openstack/zun/src/branch/master/zun/container/docker/driver.py#n16 : import eventlet
- https://opendev.org/openstack/zun/src/branch/master/zun/container/docker/driver.py#n869 : with eventlet.Timeout(CONF.docker.execute_timeout):
- https://opendev.org/openstack/zun/src/branch/master/zun/container/docker/driver.py#n871 : except eventlet.Timeout:
- https://opendev.org/openstack/zun/src/branch/master/zun/websocket/websocketproxy.py#n178 : from eventlet import hubs

***
