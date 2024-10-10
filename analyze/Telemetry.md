# Analysis for Team: Telemetry

## Project: aodh
---

- **Project:** aodh
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The release notes explicitly mention removing Eventlet and replacing it with a threaded approach, indicating that Eventlet can be deactivated.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration requiring minimal code changes.*
    *Factors for estimation: The release notes provide clear guidance on replacing Eventlet with an alternative implementation, reducing the complexity of the migration.*
  - **Files Analyzed:**
    - **File:** `releasenotes/notes/remove-eventlet-18ada1cff213af5e.yaml`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *   Description: The release notes contain configurations related to Eventlet, indicating a dependency on Eventlet's WSGI server.*
    - **File:** `releasenotes/source/locale/de/LC_MESSAGES/releasenotes.po`
      - **Identified Patterns:**
        - **Pattern:** References in Documentation
          *   Description: The release notes contain translations of the message "Remove eventlet from Aodh in favour of threaded approach", indicating that Eventlet is referenced in documentation.*
    - **File:** `releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po`
      - **Identified Patterns:**
        - **Pattern:** Removal of Eventlet
          *   Description: The release notes explicitly mention removing Eventlet and replacing it with a threaded approach, indicating that Eventlet is being removed.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is being removed from the project in favor of a threaded approach.
    - **Potential Challenges:** None identified in the release notes or analyzed files.
    - **Recommendations:** Continue with the planned removal of Eventlet and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/aodh/src/branch/master/releasenotes/notes/remove-eventlet-18ada1cff213af5e.yaml#n4 : Remove eventlet from Aodh in favour of threaded approach
- https://opendev.org/openstack/aodh/src/branch/master/releasenotes/source/locale/de/LC_MESSAGES/releasenotes.po#n241 : msgid "Remove eventlet from Aodh in favour of threaded approach"
- https://opendev.org/openstack/aodh/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n437 : msgid "Remove eventlet from Aodh in favour of threaded approach"
- https://opendev.org/openstack/aodh/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n438 : msgstr "Remove eventlet from Aodh in favour of threaded approach"
- https://opendev.org/openstack/aodh/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n231 : msgid "Remove eventlet from Aodh in favour of threaded approach"

***

## Project: ceilometer
---

- **Project:** ceilometer
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The release notes explicitly mention removing Eventlet in favor of a threaded approach, indicating that it is planned to be deactivated.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: The removal of Eventlet will require significant refactoring of core asynchronous mechanisms and adjustments to configuration management, impacting overall system stability.*
  - **Files Analyzed:**
    - **File:** `ceilometer/ceilometerconf.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet.
    - **File:** `ceilometer/ceilometer/worker.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses Eventlet's green threads for asynchronous operation, essential for the worker process.
    - **File:** `ceilometer/tests/test_worker.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `ceilometer/ceilometer/manager.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet features for scheduling deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively in ceilometer, particularly for asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet will require significant refactoring to replace core asynchronous mechanisms and adjust configuration management, potentially introducing system instability during the migration process.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage, and monitor system performance closely throughout the migration.

Occurrences Found:
- https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/notes/remove-eventlet-6738321434b60c78.yaml#n4 : Remove eventlet from Ceilometer in favour of threaded approach
- https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n914 : msgid "Remove eventlet from Ceilometer in favour of threaded approach"
- https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n915 : msgstr "Remove eventlet from Ceilometer in favour of threaded approach"
- https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/mitaka.rst#n51 : .. releasenotes/notes/remove-eventlet-6738321434b60c78.yaml @ f24ea44401b8945c9cb8a34b2aedebba3c040691
- https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/mitaka.rst#n53 : - Remove eventlet from Ceilometer in favour of threaded approach
- https://opendev.org/openstack/ceilometer/src/branch/master/test-requirements.txt#n2 : eventlet>=0.30.1 # MIT

***

## Project: telemetry-specs
- **Project:** OpenStack Watcher
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
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

- **Eventlet Usage Analysis:**
  * Eventlet is used in the workflow engine for managing green threads, which are essential for handling asynchronous operations.
  * The service file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
  * In the test file, `mock.patch('eventlet.spawn')` is used to mock Eventlet's spawn function, showing its usage in unit tests.

- **Alternative Asynchronous Libraries:**
  * asyncio: A built-in Python library for asynchronous programming that could be used as an alternative to Eventlet.
  * Other libraries like gevent or trio could also be considered alternatives.

- **Refactoring and Testing Plan:**
  * Incremental refactoring: Gradually replace Eventlet with the chosen alternative, ensuring each change does not introduce significant complexity.
  * Thorough testing: Perform extensive testing at each stage of the refactoring process to ensure system stability and correctness.

Occurrences Found:
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n11 : https://blueprints.launchpad.net/ceilometer/+spec/remove-web-eventlet
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n17 : monkeypatched to use eventlet.
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n23 : unusual behaviors when it is monkeypatched to use eventlet. Since the Werkzeug
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n25 : multi-threaded interactions, the inclusion of eventlet should be removed.
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n35 : eventlet in place. We want our tooling to be as useful as possible *especially*
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n38 : When using the Werkzeug service, using eventlet is fairly redundant as the
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n44 : We can resolve this problem relatively easily. At the moment eventlet monkey
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n121 : * Separate eventlet and non-eventlet commands into two different module
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n123 : * Compare and contrast performance of the API server with and without eventlet
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n125 : Bayer has pointed out that removing eventlet has little impact on a properly
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n128 : Werkzeug with eventlet, Werkzeug without eventlet.
- https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n136 : Ceilometer project. Once the API server is not using eventlet we can

***
