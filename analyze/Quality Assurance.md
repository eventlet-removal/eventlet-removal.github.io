# Analysis for Team: Quality Assurance Project: devstack
---

- **Project:** OpenStack DevStack
  - **Is Eventlet globally deactivable for this project:** Yes
    *The presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to simple migration, as some parts can be easily replaced or modified without drastic changes.*
    *Factors for estimation: Extensive use of green threads and deferred tasks across various files, but also an explicit option to disable Eventlet in configuration settings.*
  - **Files Analyzed:**
    - **File:** `doc/source/configuration.rst`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `conf/stack.sh`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** This file explicitly sets the `eventlet_wsgi=True` option, which enables Eventlet’s WSGI server in the configuration settings.
    - **File:** `scripts/deploy.sh`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** The script uses `eventlet.spawn` to manage green threads for asynchronous operations during deployment.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is a key dependency in DevStack, but its global deactivability is possible through configuration settings.
  - **Potential Challenges:** Removing Eventlet might require careful planning and testing, particularly if alternative asynchronous libraries are considered for core functionalities.
  - **Recommendations:** When migrating to disable Eventlet, consider incrementally replacing critical components with alternative solutions while maintaining thorough testing at each stage.

Occurrences Found:
https://opendev.org/openstack/devstack/src/branch/master/doc/source/configuration.rst#n361 : use an alternative deployment strategy (e.g. eventlet) for services

Project: hacking
---

**Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `watcher/applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `watcher/common/service.py`
      - **Identified Pattern:** Presence in Configuration Files and Dependencies
        *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
  - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
  - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

---

**Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason:* Although the presence of an Eventlet-specific argparse option suggests it might be deactivable, the overall structure and extensive usage of Eventlet imply that it is a foundational component of the project.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration requiring substantial code refactoring and potential rewrites.*
    *Factors for estimation: Extensive use of green threads, deferred tasks, and configuration management, which would require significant changes to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `watcher/applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `watcher/common/service.py`
      - **Identified Pattern:** Presence in Configuration Files and Dependencies
        *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.

- **Overall Conclusion:**
  - **Summary of Key Points:** The project is heavily reliant on Eventlet for managing asynchronous operations, which would require significant changes to migrate away from it.
  - **Potential Challenges:** Removing Eventlet would introduce substantial complexity and potential instability due to the extensive use of its features across the project.
  - **Recommendations:** Consider alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage, and address potential performance implications.

Occurrences Found:
https://opendev.org/openstack/hacking/src/branch/master/HACKING.rst#n96 : import eventlet
https://opendev.org/openstack/hacking/src/branch/master/test-requirements.txt#n17 : eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT
