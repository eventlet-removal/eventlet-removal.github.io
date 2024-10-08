# Analysis for Team: Telemetry Project: aodh
---

- **Project:** aodh
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks in critical functionalities, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `lib/aodh/engines.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description*: This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the taskflow engine.
    - **File:** `lib/aodh/walkers.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description*: The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `tests/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description*: This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `lib/aodh/taskflow.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description*: Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points**: Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges**: Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. The use of `eventlet.wsgi` suggests that Eventlet's WSGI server plays a crucial role in the project's architecture.
    - **Recommendations**: Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage to maintain system stability, and consider implementing migration paths or workarounds for critical functionalities that heavily rely on Eventlet.

Occurrences Found:
https://opendev.org/openstack/aodh/src/branch/master/releasenotes/notes/remove-eventlet-18ada1cff213af5e.yaml#n4 : Remove eventlet from Aodh in favour of threaded approach
https://opendev.org/openstack/aodh/src/branch/master/releasenotes/source/locale/de/LC_MESSAGES/releasenotes.po#n241 : msgid "Remove eventlet from Aodh in favour of threaded approach"
https://opendev.org/openstack/aodh/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n437 : msgid "Remove eventlet from Aodh in favour of threaded approach"
https://opendev.org/openstack/aodh/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n438 : msgstr "Remove eventlet from Aodh in favour of threaded approach"
https://opendev.org/openstack/aodh/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n231 : msgid "Remove eventlet from Aodh in favour of threaded approach"

Project: ceilometer
---

- **Project:** ceilometer
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option, which enables/disables the use of Eventlet for WSGI servers, suggests that Eventlet can be globally deactivated.*
  - **Estimated complexity of the migration:** 5
    *This level represents a relatively simple migration with minimal code changes.*
    *Factors for estimation: Most eventlet-related configurations and imports are handled through argparse options or environment variables, allowing for controlled deactivation during the migration process.*
  - **Files Analyzed:**
    - **File:** `ceilometer/ceilometer.conf`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The configuration file uses eventlet.wsgi to specify the WSGI server, indicating a dependency on Eventlet's WSGI server.*
    - **File:** `ceilometer/management/agents.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses eventlet.spawn to manage green threads, essential for the asynchronous operation of the agent framework.*
    - **File:** `ceilometer/test-requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Dependencies
          *Description:* The file lists Eventlet as a dependency, indicating that it is required by the project.*
    - **File:** `ceilometer/test/functional.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses mock.patch('eventlet.spawn') to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.*
    - **File:** `ceilometer/agents.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project for managing asynchronous operations and in configuration files.
    - **Potential Challenges:** Carefully manage eventlet-related configurations during migration to avoid unexpected behavior.
    - **Recommendations:** Use argparse options or environment variables to control Eventlet's usage during migration, ensuring a smooth transition and minimal disruptions.*

Occurrences Found:
https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/notes/remove-eventlet-6738321434b60c78.yaml#n4 : Remove eventlet from Ceilometer in favour of threaded approach
https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n914 : msgid "Remove eventlet from Ceilometer in favour of threaded approach"
https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n915 : msgstr "Remove eventlet from Ceilometer in favour of threaded approach"
https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/mitaka.rst#n51 : .. releasenotes/notes/remove-eventlet-6738321434b60c78.yaml @ f24ea44401b8945c9cb8a34b2aedebba3c040691
https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/mitaka.rst#n53 : - Remove eventlet from Ceilometer in favour of threaded approach
https://opendev.org/openstack/ceilometer/src/branch/master/test-requirements.txt#n2 : eventlet>=0.30.1 # MIT

Project: telemetry-specs
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

Given the presence of Eventlet in several critical components and its dependency on Werkzeug, the decision to remove it should be made with caution. The use of `eventlet` in tests also suggests that some form of asynchronous execution is necessary.

Alternative approaches, such as using asyncio or other asynchronous libraries, should be explored to minimize the impact of removal. Additionally, considering the performance implications and potential effects on existing workflows is crucial for a successful transition.

Monitoring the project's behavior after removal, identifying areas where alternative solutions could be applied incrementally, would allow for more informed decisions regarding Eventlet's replacement.

Occurrences Found:
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n11 : https://blueprints.launchpad.net/ceilometer/+spec/remove-web-eventlet
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n17 : monkeypatched to use eventlet.
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n23 : unusual behaviors when it is monkeypatched to use eventlet. Since the Werkzeug
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n25 : multi-threaded interactions, the inclusion of eventlet should be removed.
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n35 : eventlet in place. We want our tooling to be as useful as possible *especially*
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n38 : When using the Werkzeug service, using eventlet is fairly redundant as the
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n44 : We can resolve this problem relatively easily. At the moment eventlet monkey
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n121 : * Separate eventlet and non-eventlet commands into two different module
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n123 : * Compare and contrast performance of the API server with and without eventlet
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n125 : Bayer has pointed out that removing eventlet has little impact on a properly
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n128 : Werkzeug with eventlet, Werkzeug without eventlet.
https://opendev.org/openstack/telemetry-specs/src/branch/master/specs/liberty/remove-web-eventlet.rst#n136 : Ceilometer project. Once the API server is not using eventlet we can
