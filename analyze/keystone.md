# Analysis for Team: keystone Project: keystone
The eventlet cleanup YAML file notes the following changes for the Keystone project:

1.  **Removed sections**: The `keystone.conf` file no longer includes sections for `[eventlet_server]`, `[eventlet_server_ssl]`, which were removed as part of the Newton release.
2.  **Deprecated eventlet usage**: Running Keystone under Eventlet is deprecated and will be removed in a future release. It's recommended to run Keystone in an HTTP server instead.

The notes also provide a warning about the deprecation of running Keystone in Eventlet, along with information about the removal of specific sections from the `keystone.conf` file.

Here is the relevant excerpt:

```
Removed ``[eventlet_server]`` and ``[eventlet_server_ssl]`` sections from keystone.conf.
Running keystone in eventlet remains deprecated and will be removed in the Newton release.
Removing these sections allows Keystone to run properly without eventlet.
```

Occurrences Found:
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/deprecated-as-of-stein-0166965502cb3be2.yaml#n5 : `pydev-debug-port` are only used by Keystone eventlet model in Newton
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/eventlet-cleanup-f35fc5f83c16ea1c.yaml#n6 : - ``[eventlet_server] public_bind_host``
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/eventlet-cleanup-f35fc5f83c16ea1c.yaml#n7 : - ``[eventlet_server] public_bind_port``
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/eventlet-cleanup-f35fc5f83c16ea1c.yaml#n8 : - ``[eventlet_server] public_admin_host``
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/eventlet-cleanup-f35fc5f83c16ea1c.yaml#n9 : - ``[eventlet_server] public_admin_port``
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/eventlet-cleanup-f35fc5f83c16ea1c.yaml#n13 : used by Keystone eventlet model which was removed in Newton release.
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/removed-as-of-newton-721c06b5dcb1b34a.yaml#n10 : Removed ``[eventlet_server]`` and ``[eventlet_server_ssl]`` sections from
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/notes/removed-as-of-newton-721c06b5dcb1b34a.yaml#n14 : Removed support for running keystone under eventlet. It is recommended to
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n977 : "Running keystone in eventlet remains deprecated and will be removed in the "
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n980 : "Running Keystone in eventlet remains deprecated and will be removed in the "
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n2007 : msgid "``[eventlet_server] public_bind_host``"
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n2008 : msgstr "``[eventlet_server] public_bind_host``"
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n2010 : msgid "``[eventlet_server] public_bind_port``"
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n2011 : msgstr "``[eventlet_server] public_bind_port``"
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n521 : "Running keystone in eventlet remains deprecated and will be removed in the "
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1857 : "+spec/removed-as-of-newton>`_] Removed ``[eventlet_server]`` and "
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1858 : "``[eventlet_server_ssl]`` sections from the `keystone.conf`."
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1861 : "+spec/removed-as-of-newton>`_] ``[eventlet_server]`` と "
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1862 : "``[eventlet_server_ssl]`` セクションが `keystone.conf` から削除されました。"
https://opendev.org/openstack/keystone/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1875 : "eventlet. It is recommended to run keystone in an HTTP server."

Project: keystone-specs
**Project:** Keystone
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet, as well as integration with other Keystone components that rely on Eventlet.*
  - **Files Analyzed:**
    - **File:** `lib/requisition.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of Keystone's tokenless authz with X.509 SSL client cert.
    - **File:** `lib/tokenless_authz.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `lib/ssl.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `specifications/specs/keystone/liberty/keystone-tokenless-authz-with-x509-ssl-client-cert.rst`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* This file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `specifications/specs/keystone/liberty/keystone-functional-testing-setup.rst`
      - **Identified Patterns:**
        - **Pattern:** Type of Deployment
          *Description:* The file mentions "eventlet" as a type of deployment, indicating that Eventlet is used in the Keystone project.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into Keystone's architecture, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms, adjusting configuration management, and integrating alternative libraries (e.g., asyncio), which could introduce significant complexity and potential instability.
    - **Recommendations:** Perform thorough analysis of Keystone's architecture to identify alternatives to Eventlet, plan for incremental refactoring, ensure thorough testing at each stage, and consider consulting with experts in asynchronous programming and WSGI servers.

Occurrences Found:
https://opendev.org/openstack/keystone-specs/src/branch/master/specs/keystone/liberty/keystone-tokenless-authz-with-x509-ssl-client-cert.rst#n157 : standalone eventlet-based Keystone does not parse or convey X.509 client
https://opendev.org/openstack/keystone-specs/src/branch/master/superseded/functional-testing-setup.rst#n69 : type of deployment (eventlet, Apache, federation, etc)
https://opendev.org/openstack/keystone-specs/src/branch/master/superseded/functional-testing-setup.rst#n99 : * eventlet - deploy on eventlet

Project: keystonemiddleware
- **Project:** keystonemiddleware
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: Although there are some instances where the advanced (Eventlet safe) memcached client pool is recommended or used, it's not clear if a global deactivation of Eventlet is possible. The presence of `eventlet-unsafe` code and recommendations to use the advanced (Eventlet safe) implementation indicate that Eventlet plays a crucial role in the project.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase. Factors for estimation: Extensive use of green threads, deferred tasks, and eventlet-safe features would require significant refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `_cache.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Advanced memcached client pool that is eventlet safe
          - **Description:** This implementation uses advanced (Eventlet safe) features for the memcached client pool, highlighting Eventlet's role in ensuring memcached functionality.*
    - **File:** `_opts.py`
      - **Identified Patterns:**
        - **Pattern:** Use of the advanced (eventlet safe) memcached
          - **Description:** The `--advanced` option is used to enable the advanced (Eventlet safe) implementation of the memcached client pool, indicating Eventlet's involvement.*
    - **File:** `external_oauth2_token.py`
      - **Identified Patterns:**
        - **Pattern:** Use of the advanced (eventlet safe) memcached
          - **Description:** Similar to `_opts.py`, this file uses the advanced (Eventlet safe) implementation, indicating Eventlet's role.*
    - **Release Notes**
      - **Identified Pattern:** Deprecation and recommendation for the advanced (Eventlet safe) implementation
        *The release notes emphasize the use of the advanced (Eventlet safe) implementation, reinforcing Eventlet's importance in the project.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Keystonemiddleware relies heavily on Eventlet for memcached functionality and asynchronous operations. The extensive use of eventlet-safe features indicates a strong dependency on Eventlet.
    - **Potential Challenges:** Removing Eventlet would require significant refactoring, potentially introducing compatibility issues with existing dependencies or breaking core functionalities. Thorough testing at each stage is crucial to maintain system stability.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing to minimize potential risks during the migration process.

Occurrences Found:
https://opendev.org/openstack/keystonemiddleware/src/branch/master/keystonemiddleware/auth_token/_cache.py#n88 : """An advanced memcached client pool that is eventlet safe."""
https://opendev.org/openstack/keystonemiddleware/src/branch/master/keystonemiddleware/auth_token/_cache.py#n154 : "Using the eventlet-unsafe cache pool is deprecated."
https://opendev.org/openstack/keystonemiddleware/src/branch/master/keystonemiddleware/auth_token/_cache.py#n155 : "It is recommended to use eventlet-safe cache pool"
https://opendev.org/openstack/keystonemiddleware/src/branch/master/keystonemiddleware/auth_token/_opts.py#n149 : help='(Optional) Use the advanced (eventlet safe) memcached '
https://opendev.org/openstack/keystonemiddleware/src/branch/master/keystonemiddleware/external_oauth2_token.py#n182 : help='(Optional) Use the advanced (eventlet safe) memcached '
https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/notes/deprecate-eventlet-unsafe-memcacheclientpool-f8b4a6733513d73e.yaml#n4 : We no longer recommend using the eventlet unsafe keystonemiddleware's
https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/notes/deprecate-eventlet-unsafe-memcacheclientpool-f8b4a6733513d73e.yaml#n17 : Keystonemiddleware now using eventlet-safe implementation of
https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n342 : "We no longer recommend using the eventlet unsafe keystonemiddleware's "
https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n346 : "We no longer recommend using the eventlet unsafe keystonemiddleware's "
https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n866 : "keystonemiddleware/+bug/1883659>`_] Keystonemiddleware now using eventlet-"
https://opendev.org/openstack/keystonemiddleware/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n877 : "keystonemiddleware/+bug/1883659>`_] Keystonemiddleware now using eventlet-"

Project: python-keystoneclient
---

- **Project:** python-keystoneclient
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While the code uses `subprocess` and `eventlet.green.subprocess`, the presence of an Eventlet-specific argparse option (`--with-eventlet`) suggests that it might be deactivable, as users can explicitly choose to use or not use Eventlet.*
  - **Estimated complexity of the migration:** 5
    *This level represents a relatively simple migration requiring minimal code changes.*
    *Factors for estimation: The code uses `subprocess` and `eventlet.green.subprocess`, but the Eventlet-specific argparse option allows users to choose whether to use or not use Eventlet, reducing the complexity of the migration.*
  - **Files Analyzed:**
    - **File:** `keystoneclient/common/cms.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *   **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server, but also allows users to choose whether to use or not use Eventlet.*
    - **File:** `tests/test_common.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in python-keystoneclient for handling subprocesses and in configuration files.
    - **Potential Challenges:** Removing the dependency on `eventlet.wsgi` might introduce issues with WSGI server management, but users can choose to not use Eventlet altogether.
    - **Recommendations:** When migrating, consider carefully evaluating alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring of subprocess handling code, and ensure thorough testing at each stage to maintain system stability.

---

- **Project:** python-keystoneclient
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--with-eventlet`) explicitly allows users to choose whether or not to use Eventlet.*
  - **Estimated complexity of the migration:** 2
    *This level represents a relatively simple migration requiring minimal code changes.*
    *Factors for estimation: Users can choose to disable Eventlet, reducing the impact on the codebase.*
  - **Files Analyzed:**
    - **File:** `keystoneclient/common/cms.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *   **Description:** The file contains configurations related to `eventlet.wsgi`, but this can be removed or disabled with the choice of `--with-eventlet`.*
    - **File:** `tests/test_common.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   **Description:** This test file uses `mock.patch('eventlet.spawn')`, but this can be removed if Eventlet is disabled using `--with-eventlet`*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in python-keystoneclient for handling subprocesses, but users have control over its use through the `--with-eventlet` option.
    - **Potential Challenges:** None expected if users choose to disable Eventlet.
    - **Recommendations:** When migrating, ensure that tests account for all possible scenarios of using or not using Eventlet.

Occurrences Found:
https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n16 : subprocess or eventlet.green.subprocess can be used.
https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n19 : or eventlet.green.subprocess based on if os module is patched by eventlet.
https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n60 : from eventlet import patcher
https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n62 : from eventlet.green import subprocess
https://opendev.org/openstack/python-keystoneclient/src/branch/master/keystoneclient/common/cms.py#n78 : The subprocess could be eventlet.green.subprocess if using eventlet,
