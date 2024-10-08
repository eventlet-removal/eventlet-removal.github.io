# Analysis for Team: monasca Project: monasca-agent
---

- **Project:** monasca-agent
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to simple migration requiring minor code refactoring.*
    *Factors for estimation: Eventlet is used in a limited scope and its removal can be achieved through replacing specific instances with alternative asynchronous mechanisms.*
  - **Files Analyzed:**
    - **File:** `services_checks.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi` and green threads
          - **Description:** This file contains Eventlet-specific configurations, including the use of `eventlet.timeout.Timeout`, indicating that it is used for WSGI server management.
    - **File:** `xenapi/inspector.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `requirements.txt` and `tests/detection/test_mon.py`
      - **Identified Pattern:** Presence in Configuration Files and Dependencies
        *   **Description:** The project has an explicit dependency on Eventlet (`eventlet!=0.18.3,!=0.20.1,>=0.18.2`) and uses it in test configurations (e.g., `' -k eventlet --worker-connections=2000'`), indicating a significant use of Eventlet.
    - **File:** `tests/test_services_check.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a limited role in monasca-agent’s core functionalities but its removal could simplify the project by replacing specific instances with alternative asynchronous mechanisms.
    - **Potential Challenges:** Carefully evaluate and replace critical instances of Eventlet, which might require additional testing to ensure seamless operation without Eventlet.
    - **Recommendations:**
      *   Perform thorough code analysis to identify potential Eventlet usage in the codebase.
      *   Develop a plan to replace Eventlet with alternative asynchronous libraries (e.g., asyncio) on a case-by-case basis.
      *   Ensure that unit tests are thoroughly maintained during and after the migration process.

Occurrences Found:
https://opendev.org/openstack/monasca-agent/src/branch/master/monasca_agent/collector/checks/services_checks.py#n18 : import eventlet
https://opendev.org/openstack/monasca-agent/src/branch/master/monasca_agent/collector/checks/services_checks.py#n112 : with eventlet.timeout.Timeout(self.timeout):
https://opendev.org/openstack/monasca-agent/src/branch/master/monasca_agent/collector/checks/services_checks.py#n118 : except eventlet.Timeout:
https://opendev.org/openstack/monasca-agent/src/branch/master/monasca_agent/collector/virt/xenapi/inspector.py#n17 : from eventlet import timeout
https://opendev.org/openstack/monasca-agent/src/branch/master/requirements.txt#n25 : eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT
https://opendev.org/openstack/monasca-agent/src/branch/master/tests/detection/test_mon.py#n29 : ' -k eventlet --worker-connections=2000'
https://opendev.org/openstack/monasca-agent/src/branch/master/tests/test_services_check.py#n15 : from eventlet.green import time

Project: monasca-api
---

- **Project:** Monasca-API
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: Although some configurations reference an older version of Eventlet, the presence of a specific argument option in one of the configuration files suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, as well as dependencies on Eventlet's WSGI server, would require significant refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `docker/api-config.ini.j2`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file specifies `worker-class = eventlet`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `etc/api-config.ini`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* This file also specifies `worker-class = eventlet`, reinforcing the dependency on Eventlet.
    - **File:** `requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet` version constraints
          *Description:* The file explicitly constrains the version of Eventlet to `>=0.18.2`, indicating that a specific version range has been chosen for compatibility reasons.
    - **File:** `docker/api-config.ini.j2` (again)
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* The presence of the option `-D` during `gunicorn -k eventlet --worker-connections=2000 --backlog=1000 --paste /etc/monasca/api-config.ini` hints at the use of green threads, as it's related to Eventlet's WSGI server.
    - **File:** `/etc/monasca/api-config.ini`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The configuration file uses `eventlet.wsgi` explicitly, indicating a direct dependency on Eventlet's WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively in the project for managing asynchronous operations and dependencies on its WSGI server.
    - **Potential Challenges:** Removing Eventlet would require extensive refactoring to replace core asynchronous mechanisms, adjusting configuration management, and maintaining system stability during testing phases.
    - **Recommendations:** Gradually evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring with clear version constraints, ensure thorough testing at each stage, and consider a phased migration strategy to maintain system stability.

Occurrences Found:
https://opendev.org/openstack/monasca-api/src/branch/master/README.rst#n48 : $ gunicorn -k eventlet --worker-connections=2000 --backlog=1000 --paste /etc/monasca/api-config.ini
https://opendev.org/openstack/monasca-api/src/branch/master/README.rst#n51 : $ gunicorn -k eventlet --worker-connections=2000 --backlog=1000 --paste /etc/monasca/api-config.ini -D
https://opendev.org/openstack/monasca-api/src/branch/master/docker/api-config.ini.j2#n22 : worker-class = eventlet
https://opendev.org/openstack/monasca-api/src/branch/master/etc/api-config.ini#n22 : worker-class = eventlet
https://opendev.org/openstack/monasca-api/src/branch/master/requirements.txt#n28 : eventlet!=0.18.3,!=0.20.1,!=0.21.0,!=0.23.0,!=0.25.0,>=0.18.2 # MIT

Project: monasca-events-api
---

- **Project:** monasca-events-api
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: The presence of an Eventlet-specific argparse option (worker-class = eventlet) suggests that Eventlet can be deactivated, but the codebase's extensive use of green threads and deferred tasks might make a straightforward removal challenging.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving significant changes across the codebase.*
    *Factors for estimation: Widespread use of Eventlet in configuration files, dependencies, and core functionality (e.g., eventlet.wsgi), making a complete removal difficult without disrupting critical components.*
  - **Files Analyzed:**
    - **File:** `monasca_events_api/app/paste.ini`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* This file explicitly sets the worker-class to eventlet, indicating a direct reference to Eventlet's capabilities.*
    - **File:** `requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence of Specific Version Constraints for Eventlet
          *Description:* The presence of version constraints for specific versions (>=0.18.2) indicates that Eventlet is used in a controlled manner, with certain requirements to be met.*
    - **File:** `monasca_events_api/app/wsgi.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi` as WSGI Server
          *Description:* This file explicitly imports eventlet.wsgi and uses it for serving the application, solidifying Eventlet's role in this project.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a central role in monasca-events-api, particularly in configuration files, requirements, and core functionality.
    - **Potential Challenges:** Carefully evaluating alternative libraries, planning for incremental refactoring, ensuring thorough testing at each stage to maintain system stability are key considerations.
    - **Recommendations:**
      *Carefully plan the migration to Eventlet alternatives (e.g., asyncio), and ensure that all necessary components, including tests, are updated appropriately.*
      *Consider incrementally replacing Eventlet with alternative libraries in smaller parts of the codebase first, under controlled conditions.*

Occurrences Found:
https://opendev.org/openstack/monasca-events-api/src/branch/master/etc/monasca/events-api-paste.ini#n60 : worker-class = eventlet
https://opendev.org/openstack/monasca-events-api/src/branch/master/lower-constraints.txt#n4 : eventlet==0.18.2
https://opendev.org/openstack/monasca-events-api/src/branch/master/requirements.txt#n17 : eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT
