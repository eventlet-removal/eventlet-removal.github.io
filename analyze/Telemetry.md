# Analysis for Team: Telemetry

## Project: aodh
- **Project:** aodh
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive removal of Eventlet's features in favour of threaded approach, which would require significant code refactoring to ensure compatibility and reliability.*
  - **Files Analyzed:**
    - **File:** `aodh/management/commands/init.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** This file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `aodh/management/commands/init.py` (same as above)
      - **Identified Patterns:**
        - **Pattern:** Remove eventlet from Aodh in favour of threaded approach
          - **Description:** The presence of this message indicates that Eventlet will be removed, suggesting a significant change.
    - **File:** `aodh/management/commands/init.py` (same as above)
      - **Identified Patterns:**
        - **Pattern:** Remove eventlet from Aodh in favour of threaded approach
          - **Description:** This message suggests that Eventlet will be removed, indicating a major change.
    - **File:** `aodh/tests/integration/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `aodh/management/commands/init.py` (same as above)
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** This file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in various parts of the project, including configuration files and tests, suggesting significant changes ahead for its removal.
    - **Potential Challenges:** Replacing core asynchronous mechanisms with threaded approaches will require extensive code refactoring, and ensuring compatibility across different modules will introduce complexity.
    - **Recommendations:** Plan a phased approach to refactoring code, incrementally replacing Eventlet's features with threaded alternatives, while maintaining system stability through thorough testing at each stage.

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
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate migration requiring significant code refactoring to maintain system stability and functionality.*
    *Factors for estimation: Extensive use of green threads in the Ceilometer architecture, which would require adjustments to handle asynchronous operations without Eventlet.*
  - **Files Analyzed:**
    - **File:** `ceilometer/ceilometer.conf`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The configuration file contains settings related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `ceilometer/api/v2/notifications.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `ceilometer/processors/collectors.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads for collecting metrics, which is essential for the asynchronous operation of the collector.
    - **File:** `ceilometer/test-requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Dependencies
          - **Description:** The file lists Eventlet as a dependency, indicating that it is part of the Ceilometer's requirement specifications.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a crucial role in managing asynchronous operations and green threads in the Ceilometer architecture.
    - **Potential Challenges:** Removing Eventlet would require adjusting the code to handle background operations without its features, which could introduce complexity.
    - **Recommendations:** Carefully evaluate alternative libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/notes/remove-eventlet-6738321434b60c78.yaml#n4 : Remove eventlet from Ceilometer in favour of threaded approach
- https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n914 : msgid "Remove eventlet from Ceilometer in favour of threaded approach"
- https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n915 : msgstr "Remove eventlet from Ceilometer in favour of threaded approach"
- https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/mitaka.rst#n51 : .. releasenotes/notes/remove-eventlet-6738321434b60c78.yaml @ f24ea44401b8945c9cb8a34b2aedebba3c040691
- https://opendev.org/openstack/ceilometer/src/branch/master/releasenotes/source/mitaka.rst#n53 : - Remove eventlet from Ceilometer in favour of threaded approach
- https://opendev.org/openstack/ceilometer/src/branch/master/test-requirements.txt#n2 : eventlet>=0.30.1 # MIT

***

## Project: telemetry-specs
- **Project:** {{project_name}}
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration requiring extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet is used in tests with `mock`, indicating that its removal may impact testing frameworks.*
  - **Files Analyzed:**
    - **File:** `{{project_name}}/applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `{{project_name}}/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `{{project_name}}/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock the behavior of Eventlet, impacting how background operations are handled.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** Monkeypatched to use eventlet
          *Description:* The file is monkeypatched to use Eventlet, indicating that its functionality has been altered for testing purposes.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** Unusual behaviors when it is monkeypatched to use eventlet
          *Description:* This file mentions unusual behaviors when Eventlet is used, suggesting that its removal may impact the project's functionality.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** Multi-threaded interactions
          *Description:* The file mentions multi-threaded interactions, indicating that Eventlet is used to handle concurrent requests. Removing Eventlet may impact performance.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** eventlet in place
          *Description:* The file states that Eventlet should be included when using the Werkzeug service, indicating its importance for this specific use case.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** When using the Werkzeug service, using eventlet is fairly redundant
          *Description:* The file mentions that Eventlet is not necessary when using the Werkzeug service, suggesting that its removal may impact performance.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** We can resolve this problem relatively easily
          *Description:* The file states that resolving issues related to Eventlet is possible, but does not provide details on how to do so.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** Separate eventlet and non-eventlet commands into two different module
          *Description:* The file suggests separating Eventlet-related code from non-Eventlet related code, indicating a potential solution for removing Eventlet.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** Compare and contrast performance of the API server with and without eventlet
          *Description:* The file mentions comparing the performance of the API server with and without Eventlet, suggesting that its removal may impact performance.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** Bayer has pointed out that removing eventlet has little impact on a properly
          *Description:* The file mentions that removing Eventlet has little impact on the project, but also notes this was stated by someone named Bayer.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** Werkzeug with eventlet, Werkzeug without eventlet
          *Description:* The file compares the performance of the API server with and without Eventlet, indicating that its removal may impact performance.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** Ceilometer project
          *Description:* The file mentions the Ceilometer project, which is likely a related project to {{project_name}}, and notes that once the API server is not using Eventlet, removing it may be feasible.
    - **File:** `{{project_name}}/lib/openstack/common/ceilometer/main.py`
      - **Identified Patterns:**
        - **Pattern:** Once the API server is not using eventlet we can
          *Description:* The file suggests that once the API server no longer relies on Eventlet, removing it may be feasible.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, tests with `mock` suggest that removing Eventlet may impact testing frameworks.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

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
