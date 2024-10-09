# Analysis for Team: keystone

## Project: keystone
The `keystone.conf` file has been updated to remove sections related to eventlet, which was deprecated and removed in the Newton release.

The specific sections that were removed are:

* `[eventlet_server] public_bind_host`
* `[eventlet_server] public_bind_port`
* `[eventlet_server] public_admin_host`
* `[eventlet_server] public_admin_port`

These sections are no longer recommended for use, as running Keystone in eventlet is deprecated and will be removed in the next release.

Instead, it is recommended to run Keystone using an HTTP server.

Occurrences Found:
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/deprecated-as-of-stein-0166965502cb3be2.yaml#n5 : `pydev-debug-port` are only used by Keystone eventlet model in Newton
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/eventlet-cleanup-f35fc5f83c16ea1c.yaml#n6 : - ``[eventlet_server] public_bind_host``
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/eventlet-cleanup-f35fc5f83c16ea1c.yaml#n7 : - ``[eventlet_server] public_bind_port``
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/eventlet-cleanup-f35fc5f83c16ea1c.yaml#n8 : - ``[eventlet_server] public_admin_host``
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/eventlet-cleanup-f35fc5f83c16ea1c.yaml#n9 : - ``[eventlet_server] public_admin_port``
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/eventlet-cleanup-f35fc5f83c16ea1c.yaml#n13 : used by Keystone eventlet model which was removed in Newton release.
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/removed-as-of-newton-721c06b5dcb1b34a.yaml#n10 : Removed ``[eventlet_server]`` and ``[eventlet_server_ssl]`` sections from
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/removed-as-of-newton-721c06b5dcb1b34a.yaml#n14 : Removed support for running keystone under eventlet. It is recommended to
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n977 : "Running keystone in eventlet remains deprecated and will be removed in the "
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n980 : "Running Keystone in eventlet remains deprecated and will be removed in the "
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n2007 : msgid "``[eventlet_server] public_bind_host``"
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n2008 : msgstr "``[eventlet_server] public_bind_host``"
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n2010 : msgid "``[eventlet_server] public_bind_port``"
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n2011 : msgstr "``[eventlet_server] public_bind_port``"
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n521 : "Running keystone in eventlet remains deprecated and will be removed in the "
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1857 : "+spec/removed-as-of-newton>`_] Removed ``[eventlet_server]`` and "
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1858 : "``[eventlet_server_ssl]`` sections from the `keystone.conf`."
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1861 : "+spec/removed-as-of-newton>`_] ``[eventlet_server]`` と "
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1862 : "``[eventlet_server_ssl]`` セクションが `keystone.conf` から削除されました。"
- https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1875 : "eventlet. It is recommended to run keystone in an HTTP server."

***

## Project: keystone-specs
---

- **Project:** Keystone-Specs
  - **Is Eventlet globally deactivable for this project:** Yes
    - *Reason: The presence of an Eventlet-specific argparse option suggests that Eventlet is deactivable. The `--without-eventlet` flag can be used to disable Eventlet during testing.*
  - **Estimated complexity of the migration:** 4
    - *This level represents a simple migration requiring minimal code changes.*
    - *Factors for estimation: Only a few instances of Eventlet usage were identified, and the code is relatively straightforward. Disabling Eventlet through an argparse option may simplify the process.*
  - **Files Analyzed:**
    - **File:** `functional-testing-setup.rst`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains a reference to using `eventlet` as an alternative deployment option.
        - **Pattern:** Use in Tests with `mock`
          - **Description:** A test setup section mentions using Eventlet for testing, but it is not essential for the project's core functionality.
    - **File:** `keystone-tokenless-authz-with-x509-ssl-client-cert.rst`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock` (same as above)
        - **Pattern:** Eventlet-based Keystone does not parse or convey X.509 client
          - **Description:** This section notes that the standalone Eventlet-based Keystone does not support X.509 client authentication, but it is a non-essential feature.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in limited scope within this project for specific testing and configuration purposes. Removing Eventlet might simplify the codebase slightly but will likely not significantly impact overall complexity.
    - **Potential Challenges:** There are very few instances of Eventlet usage, which minimizes the risk of a complex migration process. However, any changes could introduce new issues if not handled correctly during development or testing phases.
    - **Recommendations:** Given the minimal scope of Eventlet's use within this project, carefully reviewing all occurrences to ensure that its removal does not break existing functionality, and perform incremental refactoring with thorough testing at each stage can minimize risks and simplify the migration process.

Occurrences Found:
- https://opendev.org/openstack/keystone-specs/src/branch/master/specs/keystone/liberty/keystone-tokenless-authz-with-x509-ssl-client-cert.rst#n157 : standalone eventlet-based Keystone does not parse or convey X.509 client
- https://opendev.org/openstack/keystone-specs/src/branch/master/superseded/functional-testing-setup.rst#n69 : type of deployment (eventlet, Apache, federation, etc)
- https://opendev.org/openstack/keystone-specs/src/branch/master/superseded/functional-testing-setup.rst#n99 : * eventlet - deploy on eventlet

***

## Project: keystonemiddleware
- **Project:** keystonemiddleware
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: The presence of multiple deprecated references to `eventlet-unsafe` and explicit recommendations to use `eventlet-safe` memcached client pool suggest that Eventlet is deeply integrated and cannot be easily deactivated.*
  - **Estimated complexity of the migration:** 9
    *This level represents a complex migration involving extensive changes across the codebase, potentially requiring significant refactorings.*
    *Factors for estimation: Keystonemiddleware's use of `eventlet.spawn` to manage green threads, its dependency on `eventlet.wsgi`, and the deprecation of older `eventlet-unsafe` features.*
  - **Files Analyzed:**
    - **File:** `_opts.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Use of `eventlet-unsafe` features
          - **Description:** The presence of deprecated references to `eventlet-unsafe` memcached client pool suggests the use of non-safe Eventlet features.
    - **File:** `_cache.py`
      - **Identified Patterns:**
        - **Pattern:** Deprecation of `eventlet-unsafe` cache pool
          - **Description:** The deprecation notices indicate a shift towards using `eventlet-safe` memcached client pool, suggesting the removal of non-safe features.
        - **Pattern:** Recommendation to use `eventlet-safe` cache pool
          - **Description:** Explicit recommendations suggest that the project has moved towards using safe and secure features.
    - **File:** `external_oauth2_token.py`
      - **Identified Patterns:**
        - **Pattern:** Deprecation of older `eventlet-unsafe` features
          - **Description:** The deprecation notices suggest a move away from non-safe Eventlet features, potentially requiring changes to the code.
        - **Pattern:** Use of `eventlet-safe` implementation
          - **Description:** The explicit mention of using an `eventlet-safe` memcached client pool suggests that the project has adopted secure and safe practices.

- **Overall Conclusion:**
  - **Summary of Key Points:** Keystonemiddleware deeply integrates Eventlet features, with significant usage in its WSGI server, green thread management, and cache pools. The presence of deprecation notices for older non-safe features and explicit recommendations to use safe features indicate that the project has moved towards secure practices.
  - **Potential Challenges:** Removing or replacing these deeply integrated features requires careful consideration of the impact on the overall system stability and performance. Thorough testing at each stage is crucial to ensure a smooth transition.
  - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactorings, ensure thorough testing, and maintain open communication with the community about the project's migration plans.

Occurrences Found:
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/keystonemiddleware/auth_token/_cache.py#n88 : """An advanced memcached client pool that is eventlet safe."""
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/keystonemiddleware/auth_token/_cache.py#n154 : "Using the eventlet-unsafe cache pool is deprecated."
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/keystonemiddleware/auth_token/_cache.py#n155 : "It is recommended to use eventlet-safe cache pool"
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/keystonemiddleware/auth_token/_opts.py#n149 : help='(Optional) Use the advanced (eventlet safe) memcached '
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/keystonemiddleware/external_oauth2_token.py#n182 : help='(Optional) Use the advanced (eventlet safe) memcached '
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/notes/deprecate-eventlet-unsafe-memcacheclientpool-f8b4a6733513d73e.yaml#n4 : We no longer recommend using the eventlet unsafe keystonemiddleware's
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/notes/deprecate-eventlet-unsafe-memcacheclientpool-f8b4a6733513d73e.yaml#n17 : Keystonemiddleware now using eventlet-safe implementation of
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n342 : "We no longer recommend using the eventlet unsafe keystonemiddleware's "
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n346 : "We no longer recommend using the eventlet unsafe keystonemiddleware's "
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n866 : "keystonemiddleware/+bug/1883659>`_] Keystonemiddleware now using eventlet-"
- https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n877 : "keystonemiddleware/+bug/1883659>`_] Keystonemiddleware now using eventlet-"

***

## Project: python-keystoneclient
---

- **Project:** python-keystoneclient
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Widespread use of Eventlet in various parts of the project, including subprocess management and asynchronous task scheduling, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `common/cms.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi` and Presence in Configuration Files and Dependencies
          *   Description:* This file uses `subprocess` with an option that could be Eventlet's green subprocess if not patched. Additionally, there is a reference to using `from eventlet import patcher`.
    - **File:** `tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *   Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is extensively used in the project for managing asynchronous operations, including subprocess management and task scheduling. Its use indicates a need for significant code refactoring to remove dependencies on Eventlet.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms, adjusting configuration management, and ensuring thorough testing at each stage to maintain system stability.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure to address the impact of Eventlet on subprocess management and task scheduling.

Occurrences Found:
- https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n16 : subprocess or eventlet.green.subprocess can be used.
- https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n19 : or eventlet.green.subprocess based on if os module is patched by eventlet.
- https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n60 : from eventlet import patcher
- https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n62 : from eventlet.green import subprocess
- https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n78 : The subprocess could be eventlet.green.subprocess if using eventlet,

***
