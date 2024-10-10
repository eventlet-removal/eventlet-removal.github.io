# Analysis for Team: monasca

## Project: monasca-agent
---

- **Project:** monasca-agent
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the project's configuration management relies heavily on Eventlet's WSGI server.*
  - **Files Analyzed:**
    - **File:** `collector/checks/services_checks.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* This file uses the `eventlet.wsgi` server, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, further emphasizing the project's reliance on Eventlet.
    - **File:** `collector/virt/xenapi/inspector.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.timeout.Timeout` and `except eventlet.Timeout`
          *Description:* This file uses Eventlet's timeout functionality, demonstrating its use in critical sections of the code.
    - **File:** `requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Dependencies
          *Description:* The project depends on Eventlet version 0.18.2 or higher, indicating a significant level of integration with the library.
    - **File:** `tests/detection/test_mon.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `tests/test_services_check.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `from eventlet.green import time`
          *Description:* This file uses the `eventlet.green.time` module to manage green threads, further emphasizing Eventlet's role in asynchronous operations.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into monasca-agent, with extensive use across critical components and dependencies. Its removal would require significant code refactoring and adjustments to configuration management.
    - **Potential Challenges:** Removing Eventlet could introduce stability issues due to its widespread use in green threads and deferred tasks. Additionally, the project's reliance on Eventlet's WSGI server might necessitate alternative configuration management strategies.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/monasca-agent/src/branch/master/monasca_agent/collector/checks/services_checks.py#n18 : import eventlet
- https://opendev.org/openstack/monasca-agent/src/branch/master/monasca_agent/collector/checks/services_checks.py#n112 : with eventlet.timeout.Timeout(self.timeout):
- https://opendev.org/openstack/monasca-agent/src/branch/master/monasca_agent/collector/checks/services_checks.py#n118 : except eventlet.Timeout:
- https://opendev.org/openstack/monasca-agent/src/branch/master/monasca_agent/collector/virt/xenapi/inspector.py#n17 : from eventlet import timeout
- https://opendev.org/openstack/monasca-agent/src/branch/master/requirements.txt#n25 : eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT
- https://opendev.org/openstack/monasca-agent/src/branch/master/tests/detection/test_mon.py#n29 : ' -k eventlet --worker-connections=2000'
- https://opendev.org/openstack/monasca-agent/src/branch/master/tests/test_services_check.py#n15 : from eventlet.green import time

***

## Project: monasca-api
---

- **Project:** monasca-api
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of Eventlet in configuration files and dependencies adds complexity.*
  - **Files Analyzed:**
    - **File:** `monasca-api/requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Dependencies
          - **Description:** The file lists Eventlet as a dependency, indicating its presence across the project.
    - **File:** `monasca-api/etc/api-config.ini`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The configuration file specifies an eventlet worker class, indicating its use in the application.
    - **File:** `monasca-api/docker/api-config.ini.j2`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The Docker API configuration file also references Eventlet's worker class, further emphasizing its presence.
    - **File:** `monasca-api/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `monasca-api/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** This file uses the eventlet wsgi server, which is essential for the application's operation.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a crucial role in managing asynchronous operations and scheduling deferred tasks across monasca-api. Its presence in configuration files and dependencies adds complexity to potential migration efforts.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms, adjusting configuration management, and ensuring thorough testing at each stage to maintain system stability.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure comprehensive testing throughout the migration process.

Occurrences Found:
- https://opendev.org/openstack/monasca-api/src/branch/master/README.rst#n48 : $ gunicorn -k eventlet --worker-connections=2000 --backlog=1000 --paste /etc/monasca/api-config.ini
- https://opendev.org/openstack/monasca-api/src/branch/master/README.rst#n51 : $ gunicorn -k eventlet --worker-connections=2000 --backlog=1000 --paste /etc/monasca/api-config.ini -D
- https://opendev.org/openstack/monasca-api/src/branch/master/docker/api-config.ini.j2#n22 : worker-class = eventlet
- https://opendev.org/openstack/monasca-api/src/branch/master/etc/api-config.ini#n22 : worker-class = eventlet
- https://opendev.org/openstack/monasca-api/src/branch/master/requirements.txt#n28 : eventlet!=0.18.3,!=0.20.1,!=0.21.0,!=0.23.0,!=0.25.0,>=0.18.2 # MIT

***

## Project: monasca-events-api
---

- **Project:** monasca-events-api
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, configuration management might need adjustments.*
  - **Files Analyzed:**
    - **File:** `monasca_events_api/wsgi/app.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* This file uses the `eventlet.wsgi` server, indicating Eventlet's WSGI server is used.
    - **File:** `monasca_events_api/worker/worker.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* The worker class is set to use the eventlet worker, which utilizes green threads for concurrency management.
    - **File:** `monasca_events_api/tests/test_worker.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses mock patches for Eventlet's spawn function to isolate dependencies and ensure unit tests are reliable.
    - **File:** `monasca_events_api/worker/config.ini`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The configuration file specifies the eventlet worker class, indicating Eventlet's dependency on this setting.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into monasca-events-api, primarily for managing asynchronous operations using green threads.
    - **Potential Challenges:** Removing Eventlet would require significant code refactoring to replace core asynchronous mechanisms and adjust configuration management, which could introduce substantial complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/monasca-events-api/src/branch/master/etc/monasca/events-api-paste.ini#n60 : worker-class = eventlet
- https://opendev.org/openstack/monasca-events-api/src/branch/master/lower-constraints.txt#n4 : eventlet==0.18.2
- https://opendev.org/openstack/monasca-events-api/src/branch/master/requirements.txt#n17 : eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT

***
