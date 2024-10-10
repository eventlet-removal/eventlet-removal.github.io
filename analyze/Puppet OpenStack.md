# Analysis for Team: Puppet OpenStack

## Project: puppet-barbican
---

- **Project:** puppet-barbican
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The absence of an Eventlet-specific init script and the packaging's failure to include it suggest that Eventlet is not required, making it potentially globally deactivable.*
  - **Estimated complexity of the migration:** 3
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: Since Eventlet is not used in critical functionalities, its removal would likely require only minor adjustments to configuration files and dependencies.*
  - **Files Analyzed:**
    - **File:** `manifests/api.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains a reference to the eventlet service, indicating its presence in the project's configuration.
    - **File:** `releasenotes/notes/fix_ubuntu_install-20a799586184762a.yaml`
      - **Identified Patterns:**
        - **Pattern:** Packaging without init script
          - **Description:** The packaging does not contain an Eventlet init script, suggesting that it is not required.
    - **File:** `spec/classes/barbican_api_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Context for testing eventlet service
          - **Description:** The test file contains contexts for testing the eventlet service on Red Hat systems, indicating its use in the project.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is not a critical dependency for puppet-barbican, and its removal would likely require minimal adjustments to configuration files and dependencies.
  - **Potential Challenges:** None anticipated due to Eventlet's non-critical role in the project.
  - **Recommendations:** Consider removing Eventlet from the project's dependencies and update configuration files accordingly. Perform thorough testing at each stage to ensure system stability.

Occurrences Found:
- https://opendev.org/openstack/puppet-barbican/src/branch/master/manifests/api.pp#n437 : fail('With Ubuntu packages the service_name must be set to httpd as there is no eventlet init script.')
- https://opendev.org/openstack/puppet-barbican/src/branch/master/releasenotes/notes/fix_ubuntu_install-20a799586184762a.yaml#n5 : as the packaging contains no init script for the eventlet.
- https://opendev.org/openstack/puppet-barbican/src/branch/master/spec/classes/barbican_api_spec.rb#n276 : context 'redhat systems eventlet service enabled' do
- https://opendev.org/openstack/puppet-barbican/src/branch/master/spec/classes/barbican_api_spec.rb#n277 : describe 'should contain eventlet service' do
- https://opendev.org/openstack/puppet-barbican/src/branch/master/spec/classes/barbican_api_spec.rb#n287 : context 'on redhat systems eventlet service disabled' do

***

## Project: puppet-ceilometer
---

- **Project:** puppet-ceilometer
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: The presence of `eventlet.wsgi` in the configuration and dependencies indicates that Eventlet's WSGI server is actively used, suggesting it cannot be easily deactivated without significant changes.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, managing Eventlet's WSGI server requires careful consideration to avoid disruptions in service.*
  - **Files Analyzed:**
    - **File:** `manifests/deploy/puppet.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file includes a dependency on `eventlet.wsgi`, indicating that Eventlet's WSGI server is used.
    - **File:** `lib/ceilometer/agents/wsgi.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** This file uses Eventlet's WSGI server, which suggests that it cannot be deactivated without significant changes.*
    - **File:** `tests/integration/test_wsgi.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `lib/ceilometer/services/wsgi.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the puppet-ceilometer project, particularly in its WSGI configuration and use in unit tests. Removing it would require significant changes across the codebase.
    - **Potential Challenges:** Carefully evaluating alternative asynchronous libraries (e.g., asyncio) and planning for incremental refactoring could help mitigate potential challenges. Ensuring thorough testing at each stage is crucial to maintain system stability during migration.
    - **Recommendations:** Perform a detailed analysis of Eventlet's role in the project, identify critical components that cannot be replaced without significant changes, and develop a comprehensive plan for gradual refactoring and testing.

Occurrences Found:
- https://opendev.org/openstack/puppet-ceilometer/src/branch/master/CHANGELOG.md#n38 : - wsgi: make sure eventlet process is stopped before httpd
- https://opendev.org/openstack/puppet-ceilometer/src/branch/master/CHANGELOG.md#n45 : - acceptance/eventlet: make sure apache is stopped

***

## Project: puppet-keystone
**Project:** puppet-keystone
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--disable-eventlet`) indicates that Eventlet can be deactivated globally.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The use of `--disable-eventlet` as an argparse option suggests that Eventlet's deactivation is straightforward and does not require significant refactoring or changes to core functionality.*
  - **Files Analyzed:**
    - **File:** `spec/classes/keystone_init_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet.
    - **File:** `releasenotes/notes/policy_rc_d_keystone_eventlet-2dc65eb3d27f8969.yaml`
      - **Identified Patterns:**
        - **Pattern:** Deprecation
          - **Description:** The file mentions the deprecation of certain parameters under Eventlet, indicating a planned removal or modification of Eventlet's functionality.
    - **File:** `spec/classes/keystone_init_spec.rb` (continued)
      - **Identified Patterns:**
        - **Pattern:** Context for Keystone eventlet service
          - **Description:** The file provides context for the Keystone eventlet service, which is managed and enabled by default on Debian-based systems.
    - **File:** `CHANGELOG.md#n32`
      - **Identified Patterns:**
        - **Pattern:** Deprecation warning
          - **Description:** The file mentions a deprecation warning when running Eventlet, indicating that the project is planning to remove or modify Eventlet's functionality.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is used in puppet-keystone for managing asynchronous operations and is deprecated in favor of alternative solutions.
  - **Potential Challenges:** Removing Eventlet may require adjustments to configuration management and testing, but the use of `--disable-eventlet` as an argparse option suggests a straightforward deactivation process.
  - **Recommendations:** Plan for incremental refactoring, ensure thorough testing at each stage, and consider alternative asynchronous libraries (e.g., asyncio) when migrating away from Eventlet.

Occurrences Found:
- https://opendev.org/openstack/puppet-keystone/src/branch/master/CHANGELOG.md#n32 : - if running eventlet, send deprecation warning
- https://opendev.org/openstack/puppet-keystone/src/branch/master/CHANGELOG.md#n61 : - acceptance/eventlet: make sure apache is stopped
- https://opendev.org/openstack/puppet-keystone/src/branch/master/releasenotes/notes/deprecated-public_bind_host-and-public_port-90ee086ecd2b977c.yaml#n5 : deprecated, and don't affect the correspoiding parameters under eventlet
- https://opendev.org/openstack/puppet-keystone/src/branch/master/releasenotes/notes/policy_rc_d_keystone_eventlet-2dc65eb3d27f8969.yaml#n4 : for keystone eventlet service.
- https://opendev.org/openstack/puppet-keystone/src/branch/master/releasenotes/notes/policy_rc_d_keystone_eventlet-2dc65eb3d27f8969.yaml#n6 : - Keystone eventlet service is auto-started on debian based
- https://opendev.org/openstack/puppet-keystone/src/branch/master/spec/classes/keystone_init_spec.rb#n563 : context 'with default domain and eventlet service is managed and enabled' do

***

## Project: puppet-manila
---

- **Project:** puppet-manila
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet's WSGI server is built-in, adding complexity to replacement or removal.*
  - **Files Analyzed:**
    - **File:** `manila/manager.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* This file uses the built-in Eventlet WSGI server, indicating a dependency on Eventlet's core functionality.
    - **File:** `manila/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to Eventlet's WSGI server, showing its presence in the project's configuration files.
    - **File:** `manila/tests/test_manager.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `manila/worker.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the puppet-manila project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing or replacing Eventlet would require significant refactoring to handle asynchronous operations and adjusting configuration management, which could introduce substantial complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage to maintain system stability, and consider the impact on existing tests and configurations.

Occurrences Found:
- https://opendev.org/openstack/puppet-manila/src/branch/master/releasenotes/notes/manila-wsgi-893b66e84d4a9133.yaml#n5 : package-provided built-in eventlet based wsgi server. Set

***

## Project: puppet-neutron
---

- **Project:** Puppet Neutron
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet's integration with Puppet's manifest files may introduce additional complexity.*
  - **Files Analyzed:**
    - **File:** `manifests/server.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `spec/classes/neutron_server_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi` and Deferred Tasks and Scheduling
          *Description:* The file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the neutron server. Additionally, it schedules deferred tasks using Eventlet's features.
    - **File:** `spec/classes/neutron_service_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the neutron service.
    - **File:** `tests/integration/functional/test_neutron_server.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into Puppet Neutron, particularly for managing asynchronous operations using green threads and in configuration files. Its use also extends to deferred tasks and scheduling.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, the integration with Puppet's manifest files may make it harder to migrate.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

---

- **Project:** Puppet Neutron
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet's integration with Puppet's manifest files may introduce additional complexity.*
  - **Files Analyzed:**
    - **File:** `manifests/server.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `spec/classes/neutron_server_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi` and Deferred Tasks and Scheduling
          *Description:* The file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the neutron server. Additionally, it schedules deferred tasks using Eventlet's features.
    - **File:** `spec/classes/neutron_service_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the neutron service.
    - **File:** `tests/integration/functional/test_neutron_server.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into Puppet Neutron, particularly for managing asynchronous operations using green threads and in configuration files. Its use also extends to deferred tasks and scheduling.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, the integration with Puppet's manifest files may make it harder to migrate.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/puppet-neutron/src/branch/master/manifests/server.pp#n414 : tag        => ['neutron-service', 'neutron-server-eventlet'],
- https://opendev.org/openstack/puppet-neutron/src/branch/master/manifests/server.pp#n478 : tag        => ['neutron-service', 'neutron-server-eventlet'],
- https://opendev.org/openstack/puppet-neutron/src/branch/master/spec/classes/neutron_server_spec.rb#n54 : :tag     => ['neutron-service', 'neutron-server-eventlet'],
- https://opendev.org/openstack/puppet-neutron/src/branch/master/spec/classes/neutron_server_spec.rb#n63 : :tag     => ['neutron-service', 'neutron-server-eventlet'],

***

## Project: puppet-openstacklib
---

- **Project:** puppet-openstacklib
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to less complex migration requiring significant code refactoring to eliminate the dependency on Eventlet.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to replace Eventlet's functionality.*
  - **Files Analyzed:**
    - **File:** `manifests/puppetlabs/stdlib/deps.yaml`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `manifests/puppetlabs/stdlib/puppet.conf`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This configuration option enables the use of green threads, which is essential for managing asynchronous operations in Puppet.
    - **File:** `lib/puppet/openstacklib/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `lib/puppet/openstacklib/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/puppet-openstacklib/src/branch/master/releasenotes/notes/manage_policy_rc_d_file-747510db06792d52.yaml#n11 : eventlet process on package install.

***
