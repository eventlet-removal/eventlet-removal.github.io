# Analysis for Team: blazar

## Project: blazar
---

- **Project:** blazar
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--disable-eventlet`) suggests that Eventlet can be globally deactivated.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate migration requiring some code changes.*
    *Factors for estimation: Some minor adjustments to configuration management and occasional use of `eventlet.monkey_patch`*
  - **Files Analyzed:**
    - **File:** `blazar/api/v1/app.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `blazar/cmd/api.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* This file imports and uses the `wsgi.server` function from `eventlet`, indicating its direct usage.
    - **File:** `blazar/cmd/manager.py`
      - **Identified Patterns:**
        - **Pattern:** Eventlet Monkey Patching
          *Description:* The file uses `eventlet.monkey_patch()` to monkey patch the `__import__` function, suggesting global modifications for compatibility reasons.
    - **File:** `blazar/requirements.txt`
      - **Identified Pattern:** Dependency on Eventlet
        *Description:* The file lists an eventlet version constraint in the MIT license range (>=0.18.2), indicating Eventlet's dependency.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet's usage is mainly limited to configuration and compatibility modifications, with direct instances of `wsgi.server` and a monkey patching of the import function.
  - **Potential Challenges:** The global deactivation of Eventlet could introduce minor issues related to event-driven handling and require adjustments in some code segments for better stability.
  - **Recommendations:** Carefully review event-driven operations, plan for the replacement of `eventlet` with an alternative async runtime if necessary (e.g., asyncio), and ensure that testing covers these modifications thoroughly.

Occurrences Found:
- https://opendev.org/openstack/blazar/src/branch/master/blazar/api/v1/app.py#n16 : import eventlet
- https://opendev.org/openstack/blazar/src/branch/master/blazar/api/v1/app.py#n17 : eventlet.monkey_patch(
- https://opendev.org/openstack/blazar/src/branch/master/blazar/cmd/api.py#n16 : import eventlet
- https://opendev.org/openstack/blazar/src/branch/master/blazar/cmd/api.py#n17 : eventlet.monkey_patch(
- https://opendev.org/openstack/blazar/src/branch/master/blazar/cmd/api.py#n23 : from eventlet import wsgi
- https://opendev.org/openstack/blazar/src/branch/master/blazar/cmd/api.py#n59 : wsgi.server(eventlet.listen((CONF.host, CONF.port), backlog=500), app)
- https://opendev.org/openstack/blazar/src/branch/master/blazar/cmd/manager.py#n16 : import eventlet
- https://opendev.org/openstack/blazar/src/branch/master/blazar/cmd/manager.py#n17 : eventlet.monkey_patch()
- https://opendev.org/openstack/blazar/src/branch/master/blazar/manager/service.py#n20 : import eventlet
- https://opendev.org/openstack/blazar/src/branch/master/blazar/manager/service.py#n162 : event_thread = eventlet.spawn(
- https://opendev.org/openstack/blazar/src/branch/master/blazar/monitor/notification_monitor.py#n38 : executor='eventlet'
- https://opendev.org/openstack/blazar/src/branch/master/blazar/rpc.py#n46 : executor='eventlet',
- https://opendev.org/openstack/blazar/src/branch/master/blazar/tests/manager/test_service.py#n21 : import eventlet
- https://opendev.org/openstack/blazar/src/branch/master/blazar/tests/manager/test_service.py#n123 : self.eventlet = eventlet
- https://opendev.org/openstack/blazar/src/branch/master/blazar/tests/manager/test_service.py#n286 : self.patch(eventlet, 'spawn')
- https://opendev.org/openstack/blazar/src/branch/master/blazar/tests/manager/test_service.py#n352 : spawn = self.patch(eventlet, 'spawn')
- https://opendev.org/openstack/blazar/src/branch/master/blazar/tests/manager/test_service.py#n363 : self.patch(eventlet, 'spawn').side_effect = Exception
- https://opendev.org/openstack/blazar/src/branch/master/requirements.txt#n11 : eventlet!=0.18.3,!=0.20.1,!=0.21.0,!=0.23.0,!=0.25.0,>=0.18.2 # MIT

***
