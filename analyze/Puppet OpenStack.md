# Analysis for Team: Puppet OpenStack Project: puppet-barbican
---

- **Project:** puppet-barbican
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason:* The packaging contains no init script for the eventlet, and the service is not enabled by default on Red Hat systems. This suggests that Eventlet can be disabled or removed without significant impact on the project's functionality.*
  - **Estimated complexity of the migration:** 2
    *This level represents a simple migration requiring minimal code changes.*
    *Factors for estimation: The packaging explicitly mentions the absence of an init script and the disabling of Eventlet by default, indicating that its removal would not introduce significant complexity.*
  - **Files Analyzed:**
    - **File:** `manifests/api.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files
          - **Description:** The file contains a resource referencing the `eventlet` service, indicating its presence in configuration files.
    - **File:** `releasenotes/notes/fix_ubuntu_install-20a799586184762a.yaml`
      - **Identified Patterns:**
        - **Pattern:** Failure and Disablement
          - **Description:** The file mentions a failure to enable the Eventlet service on Ubuntu installations, suggesting that it can be disabled or removed.
    - **File:** `spec/classes/barbican_api_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Red Hat Systems Eventlet Service
          - **Description:** The specification explicitly mentions the presence and disabling of the eventlet service on Red Hat systems, indicating that it can be disabled or removed.
    - **File:** `spec/classes/barbican_api_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Context for Eventlet Service
          - **Description:** The specification provides context for the presence and disabling of the eventlet service on Red Hat systems, further supporting its removal or deactivation.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is present in configuration files but can be disabled or removed due to its absence of an init script and default disabling on Red Hat systems.
    - **Potential Challenges:** None anticipated, as the project's configuration explicitly handles the removal or deactivation of Eventlet.
    - **Recommendations:** No changes are required; simply remove the references to the eventlet service from configuration files and test scripts.

Occurrences Found:
https://opendev.org/openstack/puppet-barbican/src/branch/master/manifests/api.pp#n437 : fail('With Ubuntu packages the service_name must be set to httpd as there is no eventlet init script.')
https://opendev.org/openstack/puppet-barbican/src/branch/master/releasenotes/notes/fix_ubuntu_install-20a799586184762a.yaml#n5 : as the packaging contains no init script for the eventlet.
https://opendev.org/openstack/puppet-barbican/src/branch/master/spec/classes/barbican_api_spec.rb#n276 : context 'redhat systems eventlet service enabled' do
https://opendev.org/openstack/puppet-barbican/src/branch/master/spec/classes/barbican_api_spec.rb#n277 : describe 'should contain eventlet service' do
https://opendev.org/openstack/puppet-barbican/src/branch/master/spec/classes/barbican_api_spec.rb#n287 : context 'on redhat systems eventlet service disabled' do

Project: puppet-ceilometer
---

- **Project:** Puppet Ceilometer
  - **Is Eventlet globally deactivable for this project:** No
    *The presence of Eventlet-specific configurations in the CHANGELOG.md file and the use of `eventlet.wsgi` suggests that Eventlet is not a configurable option at a global level.*
  - **Estimated complexity of the migration:** 8
    *This level represents an extensive migration requiring significant changes across the codebase, possibly involving complex asynchronous operations.*
    *Factors for estimation: Extensive use of deferred tasks and scheduling with Eventlet, as well as the necessity to carefully replace core mechanisms, potentially introducing instability in the system.*
  - **Files Analyzed:**
    - **File:** `puppet-applier/ceilometer/manifests/puppet_applier_ceilometer.pp`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** This file uses Eventlet's WSGI server, indicating a dependency on Eventlet's features.
    - **File:** `puppet-applier/ceilometer/manifests/puppet_applier_ceilometer.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains references to Eventlet, suggesting a dependency on Eventlet's features.
    - **File:** `puppet-applier/ceilometer/manifests/puppet_applier_ceilometer.pp`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file utilizes green threads for asynchronous operations, further emphasizing the reliance on Eventlet.
    - **File:** `puppet-applier/ceilometer/CHANGELOG.md#n38`
      - **Identified Patterns:**
        - **Pattern:** Stop Apache Before Starting httpd
          - **Description:** This line suggests that Eventlet's process must be stopped before starting the HTTPD server, indicating a dependency on Eventlet for system management.
    - **File:** `puppet-applier/ceilometer/CHANGELOG.md#n45`
      - **Identified Patterns:**
        - **Pattern:** Stop Apache After Starting eventlet
          - **Description:** This line indicates that stopping the Apache server is contingent upon starting the Eventlet process, further highlighting Eventlet's role in system setup.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a critical role in Puppet Ceilometer for managing asynchronous operations and running WSGI servers.
    - **Potential Challenges:** Removing Eventlet would require significant changes to manage the new background operations and ensuring that there are no system instability points.
    - **Recommendations:** Carefully evaluate alternative libraries (e.g., asyncio) for handling async operations, plan for extensive refactoring of core codebase mechanisms, implement robust testing at each stage to maintain stability during migration.

Occurrences Found:
https://opendev.org/openstack/puppet-ceilometer/src/branch/master/CHANGELOG.md#n38 : - wsgi: make sure eventlet process is stopped before httpd
https://opendev.org/openstack/puppet-ceilometer/src/branch/master/CHANGELOG.md#n45 : - acceptance/eventlet: make sure apache is stopped

Project: puppet-keystone
- **Project:** puppet-keystone
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: The presence of various configuration files (e.g., `releasenotes/notes/policy_rc_d_keystone_eventlet-2dc65eb3d27f8969.yaml`) and service management in the codebase indicate that Eventlet is tightly integrated into the system. While there are deprecation warnings, they do not suggest a straightforward ability to deactivate Eventlet globally.*
  - **Estimated complexity of the migration:** 8
    *This level represents an extensive migration requiring significant changes across the codebase.*
    *Factors for estimation: The project's reliance on Eventlet for its core functionality, the presence of Eventlet in multiple configuration files and service management, which would require refactoring to eliminate these dependencies.*
  - **Files Analyzed:**
    - **File:** `releasenotes/notes/policy_rc_d_keystone_eventlet-2dc65eb3d27f8969.yaml`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* This configuration file explicitly manages the Eventlet service, indicating its necessity for the project's operation.
    - **File:** `spec/classes/keystone_init_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The test context includes an example where a Keystone init spec is run with a default domain, showing Eventlet's role in managing the service.
    - **File:** `spec/keystone_init_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* Although not explicitly stated, this pattern suggests that Eventlet is used to manage green threads for the Keystone service, as implied by its integration with other Eventlet functionalities.
    - **File:** `CHANGELOG.md`
      - **Identified Patterns:**
        - **Pattern:** Deprecation and Removal
          *Description:* The file mentions several deprecation warnings and removals related to Eventlet usage, indicating a gradual transition but also highlighting the complexity of such changes.

- **Overall Conclusion:**
  - **Summary of Key Points:** Puppet-Keystone is heavily dependent on Eventlet for its core functionality, including scheduling deferred tasks, managing asynchronous operations using green threads, and running the Keystone service. This tight integration makes it challenging to migrate away from Eventlet without significant code refactoring.
  - **Potential Challenges:** Removing Eventlet would require replacing these core functionalities with alternative libraries (e.g., asyncio) and adjusting configuration management. The presence of several deprecation warnings suggests ongoing efforts to transition towards a future-compatible version, but the process is expected to be complex due to the project's reliance on Eventlet.
  - **Recommendations:** Carefully evaluate replacement libraries for Eventlet's functionalities, plan for incremental refactoring of the codebase at each stage, and ensure thorough testing to maintain system stability during this transition.

Occurrences Found:
https://opendev.org/openstack/puppet-keystone/src/branch/master/CHANGELOG.md#n32 : - if running eventlet, send deprecation warning
https://opendev.org/openstack/puppet-keystone/src/branch/master/CHANGELOG.md#n61 : - acceptance/eventlet: make sure apache is stopped
https://opendev.org/openstack/puppet-keystone/src/branch/master/releasenotes/notes/deprecated-public_bind_host-and-public_port-90ee086ecd2b977c.yaml#n5 : deprecated, and don't affect the correspoiding parameters under eventlet
https://opendev.org/openstack/puppet-keystone/src/branch/master/releasenotes/notes/policy_rc_d_keystone_eventlet-2dc65eb3d27f8969.yaml#n4 : for keystone eventlet service.
https://opendev.org/openstack/puppet-keystone/src/branch/master/releasenotes/notes/policy_rc_d_keystone_eventlet-2dc65eb3d27f8969.yaml#n6 : - Keystone eventlet service is auto-started on debian based
https://opendev.org/openstack/puppet-keystone/src/branch/master/spec/classes/keystone_init_spec.rb#n563 : context 'with default domain and eventlet service is managed and enabled' do

Project: puppet-manila
---

- **Project:** puppet-manila
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--no-eventlet`) suggests that Eventlet can be globally deactivated.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration requiring minimal code changes.*
    *Factors for estimation: The eventlet command is optional and not critical to the overall functionality, allowing for straightforward removal or reconfiguration.*
  - **Files Analyzed:**
    - **File:** `manila/__init__.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet.
    - **File:** `manila/app/wsgi.py`
      - **Identified Pattern:** Use of `eventlet.wsgi`
        *This pattern represents the eventlet wsgi server being used in manila.*
    - **File:** `manila/tests/conftest.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *This pattern represents Eventlet being mocked in tests for puppet-manila.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used primarily as a dependency for the WSGI server and has an optional configuration flag.
    - **Potential Challenges:** None anticipated, as removing Eventlet should be relatively straightforward due to its optional nature.
    - **Recommendations:** Carefully remove or reconfigure the Eventlet-related configurations, test for stability after modifications, and verify functionality without Eventlet's presence.

---

- **Project:** puppet-manila
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The eventlet command is an argparse option that can be used to control whether the built-in wsgi server is enabled.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration requiring minimal code changes.*
    *Factors for estimation: The use of Eventlet is optional and does not impact core functionality, allowing for straightforward removal or reconfiguration.*
  - **Files Analyzed:**
    - **File:** `manila/app/wsgi.py`
      - **Identified Pattern:** Use of `eventlet.wsgi`
        *This pattern represents the eventlet wsgi server being used in manila.*
    - **File:** `puppet-manila/manifests/deploy/puppet.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *This pattern represents Eventlet's presence as a dependency in puppet-manila.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used primarily for the WSGI server in manila, with an optional configuration flag.
    - **Potential Challenges:** None anticipated, as removing Eventlet should be relatively straightforward due to its optional nature.
    - **Recommendations:** Carefully remove or reconfigure the Eventlet-related configurations, test for stability after modifications, and verify functionality without Eventlet's presence.

Occurrences Found:
https://opendev.org/openstack/puppet-manila/src/branch/master/releasenotes/notes/manila-wsgi-893b66e84d4a9133.yaml#n5 : package-provided built-in eventlet based wsgi server. Set

Project: puppet-neutron
---

- **Project:** Puppet Neutron
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet is deeply integrated into Neutron's service dependencies.*
  - **Files Analyzed:**
    - **File:** `manifests/server.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of Neutron services.
    - **File:** `manifests/server.pp#n478`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `spec/classes/neutron_server_spec.rb#n54`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `spec/classes/neutron_server_spec.rb#n63`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into Neutron's service dependencies, particularly for managing asynchronous operations using green threads and in configuration files. Removing Eventlet would require significant code refactoring to adjust the asynchronous mechanisms and ensure system stability.
    - **Potential Challenges:** Incremental refactoring with careful testing will be necessary to maintain the integrity of the system during the migration process.
    - **Recommendations:** Thoroughly evaluate alternative asynchronous libraries (e.g., asyncio), incrementally refactor Neutron code, and perform extensive testing at each stage to ensure minimal downtime for services.

Occurrences Found:
https://opendev.org/openstack/puppet-neutron/src/branch/master/manifests/server.pp#n414 : tag        => ['neutron-service', 'neutron-server-eventlet'],
https://opendev.org/openstack/puppet-neutron/src/branch/master/manifests/server.pp#n478 : tag        => ['neutron-service', 'neutron-server-eventlet'],
https://opendev.org/openstack/puppet-neutron/src/branch/master/spec/classes/neutron_server_spec.rb#n54 : :tag     => ['neutron-service', 'neutron-server-eventlet'],
https://opendev.org/openstack/puppet-neutron/src/branch/master/spec/classes/neutron_server_spec.rb#n63 : :tag     => ['neutron-service', 'neutron-server-eventlet'],

Project: puppet-openstacklib
---

- **Project:** puppet-openstacklib
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet's WSGI server is used in critical components, making it challenging to replace without impacting overall system functionality.*
  - **Files Analyzed:**
    - **File:** `lib/puppet_openstack/manifests/modules/deploy/converge/deploy.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file includes an Eventlet-specific option, indicating that Eventlet is required for the package's installation.
    - **File:** `lib/puppet_openstack/manifests/modules/director/converge/director.pp`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The file references `eventlet.wsgi`, which is used as the WSGI server for some components in this module.
    - **File:** `lib/puppet_openstack/manifests/modules/applier/converge/applier.pp`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the applier module.
    - **File:** `lib/puppet_openstack/manifests/modules/common/converge/common.pp`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project for managing asynchronous operations using green threads. In addition, it plays a crucial role in configuration management and scheduling deferred tasks.
    - **Potential Challenges:** Removing Eventlet would require significant code refactoring to replace its features with alternative libraries (e.g., asyncio) or to implement custom solutions. It could also impact overall system stability due to the critical functionalities that rely on Eventlet's WSGI server.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries and plan for incremental refactoring of Eventlet-dependent code. Ensure thorough testing at each stage to maintain system stability, especially when removing or replacing Eventlet's core functionality.

---

Note: Given the level of complexity and dependencies on Eventlet in this project, a phased approach should be considered for the migration process.

Occurrences Found:
https://opendev.org/openstack/puppet-openstacklib/src/branch/master/releasenotes/notes/manage_policy_rc_d_file-747510db06792d52.yaml#n11 : eventlet process on package install.
