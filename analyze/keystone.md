# Analysis for Team: keystone

## Project: keystone
The changes made in the `f35fc5f83c16ea1c.yaml` file are related to the removal of certain sections from the `keystone.conf` file, specifically:

* The `[eventlet_server]` section
* The `[eventlet_server_ssl]` section
* The `public_bind_host`, `public_bind_port`, `public_admin_host`, and `public_admin_port` settings

These changes are part of the Newton release, which marks a significant change in the way Keystone is configured and run. Specifically:

* The use of eventlet for running Keystone has been deprecated and will be removed in future releases.
* The `[eventlet_server]` and `[eventlet_server_ssl]` sections have been removed from the `keystone.conf` file.
* The `public_bind_host`, `public_bind_port`, `public_admin_host`, and `public_admin_port` settings are no longer recommended for use.

The changes aim to simplify the configuration of Keystone and make it easier to run in a standard HTTP server, rather than relying on eventlet.

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

- **Project:** Keystone Specs
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Keystone's deployment options include eventlet, Apache, federation, etc., indicating potential complexity in replacing or adjusting these configurations.*
  - **Files Analyzed:**
    - **File:** `specs/functional-testing-setup.rst`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** The file mentions deploying on eventlet, indicating a direct dependency on Eventlet's WSGI server.
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file references Keystone's deployment options, including eventlet, which suggests that Eventlet is deeply integrated into the project's configuration management.
    - **File:** `specs/keystone-tokenless-authz-with-x509-ssl-client-cert.rst`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The file uses mock.patch('eventlet.spawn') to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `specs/functional-testing-setup.rst#n69`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** The file mentions eventlet as a deployment option for Keystone, which implies the use of green threads for asynchronous operations.
    - **File:** `specs/functional-testing-setup.rst#n99`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** The file references eventlet's features for scheduling deferred tasks, impacting how background operations are handled in Keystone.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the project's configuration management and used extensively across various files, particularly for managing asynchronous operations using green threads and in unit tests.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms, adjusting configuration management, and potentially introducing compatibility issues with Keystone's deployment options. Careful evaluation of alternative asynchronous libraries (e.g., asyncio) is necessary to ensure a smooth migration.
    - **Recommendations:** Perform thorough testing at each stage, plan for incremental refactoring, and consider the potential impact on Keystone's deployment options when migrating away from Eventlet.

Occurrences Found:
- https://opendev.org/openstack/keystone-specs/src/branch/master/specs/keystone/liberty/keystone-tokenless-authz-with-x509-ssl-client-cert.rst#n157 : standalone eventlet-based Keystone does not parse or convey X.509 client
- https://opendev.org/openstack/keystone-specs/src/branch/master/superseded/functional-testing-setup.rst#n69 : type of deployment (eventlet, Apache, federation, etc)
- https://opendev.org/openstack/keystone-specs/src/branch/master/superseded/functional-testing-setup.rst#n99 : * eventlet - deploy on eventlet

***

## Project: keystonemiddleware
- **Project:** keystonemiddleware
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--global-deactivate`) suggests that Eventlet can be globally deactivated.*
  - **Estimated complexity of the migration:** 5
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The use of `eventlet.wsgi` is limited to specific configurations, and most of the project's asynchronous operations are handled using eventlet-safe memcached client pools.*
  - **Files Analyzed:**
    - **File:** `_cache.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `_opts.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the memcached client pool.
    - **File:** `external_oauth2_token.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce minimal complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

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
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `common/cms.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The file uses `from eventlet.green import subprocess` and `from eventlet.patcher import patcher`, indicating that Eventlet is used in unit tests.
    - **File:** `common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi` 
          - **Description:** This test file uses `subprocess = eventlet.green.subprocess`, indicating that Eventlet's WSGI server is used.
    - **File:** `tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n16 : subprocess or eventlet.green.subprocess can be used.
- https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n19 : or eventlet.green.subprocess based on if os module is patched by eventlet.
- https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n60 : from eventlet import patcher
- https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n62 : from eventlet.green import subprocess
- https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n78 : The subprocess could be eventlet.green.subprocess if using eventlet,

***
