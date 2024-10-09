# Analysis for Team: monasca

## Project: monasca-agent
---

- **Project:** monasca-agent
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: Although there is a dependency on Eventlet in the requirements.txt file, it's specifically set to a range of versions (>=0.18.2), which suggests that it might be possible to use a different version or library without breaking the functionality.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving significant changes across the codebase.*
    *Factors for estimation: Extensive use of Eventlet's green threads, timeout handling, and scheduler features require careful refactoring to eliminate dependencies on Eventlet.*
  - **Files Analyzed:**
    - **File:** `collector/checks/services_checks.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** Uses `eventlet.timeout.Timeout` and `except eventlet.Timeout` to manage green threads, which are essential for the asynchronous operation of the service checks.
    - **File:** `collector/virt/xenapi/inspector.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `from eventlet import timeout`
          - **Description:** Imports Eventlet's timeout module to use its features in the virtualization inspector.
    - **File:** `requirements.txt`
      - **Identified Pattern:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains an explicit dependency on Eventlet, which is set to a specific range of versions (>=0.18.2).
    - **File:** `tests/detection/test_mon.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** Uses ` -k eventlet --worker-connections=2000` to run the test with Eventlet's green thread pool, indicating its use in functional testing.
    - **File:** `tests/test_services_check.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Utilizes Eventlet's features to schedule deferred tasks, impacting how background operations are handled in the service checks.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into monasca-agent's functionality, particularly for handling asynchronous operations using green threads and in configuration management files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, potentially introducing significant complexity and requiring careful planning.
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
- **Project:** monasca-api
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option may suggest that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration requiring extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the project uses Eventlet's WSGI server in multiple places, indicating a potential need for careful replacement or configuration adjustments.*
  - **Files Analyzed:**
    - **File:** `monasca_api/app/wsgi.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *   Description: This file uses the Eventlet WSGI server, indicating its necessity for running the application.
    - **File:** `monasca_api/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *   Description: The file contains configurations related to the Eventlet WSGI server, showing its dependency on Eventlet.
    - **File:** `monasca_api/tests/api/test_api.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   Description: This test file uses the mock library to replace certain dependencies, including `eventlet.spawn`, indicating a need to manage or replace Eventlet's asynchronous capabilities.
    - **File:** `monasca_api/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *   Description: Uses Eventlet's features for scheduling deferred tasks, impacting how background operations are handled in the application.
    - **File:** `Dockerfile` (docker/api-config.ini.j2)
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *   Description: The Dockerfile contains a configuration that uses Eventlet's worker class, demonstrating its presence in the project.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is deeply integrated into monasca-api, primarily for managing green threads, deferred tasks, and running the WSGI server.
  - **Potential Challenges:** Removing Eventlet could necessitate significant code refactoring to manage asynchronous operations differently and may require careful adjustments to the project's configuration management.
  - **Recommendations:**
    *   Perform a thorough assessment of the current dependency on Eventlet, weighing its advantages against potential migration complexities.
    *   Develop strategies for managing or replacing Eventlet's functionalities with alternative libraries (e.g., asyncio) to ensure smooth transition and maintain system stability.

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
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `monasca-events-api/etc/monasca/events-api-paste.ini`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** The line "worker-class = eventlet" explicitly indicates Eventlet's use in managing worker classes.
    - **File:** `monasca-events-api/lower-constraints.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence of Eventlet Version
          - **Description:** The constraint `eventlet==0.18.2` directly references the version of Eventlet used in this project.
    - **File:** `monasca-events-api/requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Eventlet Dependency
          - **Description:** The dependency on Eventlet, excluding versions 0.18.3 and 0.20.1, indicates its necessity for the project's stability.
    - **File:** `monasca-events-api/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The use of `mock.patch('eventlet.spawn')` indicates Eventlet's presence in unit tests, indicating its critical role.
    - **File:** `monasca-events-api/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, showing Eventlet's dependency on WSGI server features.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a vital role across the monasca-events-api project, particularly for managing green threads and ensuring event handling stability.
    - **Potential Challenges:** Removing or replacing Eventlet could introduce issues with event scheduling and management due to its extensive use in the system's core functionality.
    - **Recommendations:** Conduct thorough testing and analysis before attempting a global removal of Eventlet. Consider transitioning to an alternative library, such as asyncio, that can replace Eventlet's features while ensuring minimal disruption to existing operations.

Occurrences Found:
- https://opendev.org/openstack/monasca-events-api/src/branch/master/etc/monasca/events-api-paste.ini#n60 : worker-class = eventlet
- https://opendev.org/openstack/monasca-events-api/src/branch/master/lower-constraints.txt#n4 : eventlet==0.18.2
- https://opendev.org/openstack/monasca-events-api/src/branch/master/requirements.txt#n17 : eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT

***
