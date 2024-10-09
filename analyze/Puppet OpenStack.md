# Analysis for Team: Puppet OpenStack

## Project: puppet-barbican
---

- **Project:** puppet-barbican
  - **Is Eventlet globally deactivable for this project:** Yes
    - *Reason: The absence of an init script for the eventlet packaging on Ubuntu indicates that it might not be necessary.*
  - **Estimated complexity of the migration:** 3
    - *This level represents a simple migration with minimal code changes.*
    - *Factors for estimation: Since Eventlet is not used in core functionalities and its services are managed through Puppet, the removal or replacement of Eventlet should not significantly impact the system's behavior.*
  - **Files Analyzed:**
    - **File:** `manifests/api.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files
          - **Description:** The file contains configuration related to the eventlet service, indicating its presence in Puppet.
    - **File:** `releasenotes/notes/fix_ubuntu_install-20a799586184762a.yaml`
      - **Identified Patterns:**
        - **Pattern:** Fix for Ubuntu Package
          - **Description:** A fix was added to the eventlet packaging on Ubuntu, which removed the need for an init script, suggesting that Eventlet might not be necessary.
    - **File:** `spec/classes/barbican_api_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Eventlet Service Management
          - **Description:** Puppet is used to manage the eventlet service, which suggests that it can be replaced or disabled without significant impact.

- **Overall Conclusion:**
  - **Summary of Key Points:** Puppet-barbican uses Eventlet for managing its services. However, due to the packaging changes on Ubuntu, Eventlet might not be necessary.
  - **Potential Challenges:** None anticipated, as the removal or replacement of Eventlet seems feasible.
  - **Recommendations:** Review Puppet's configuration management and consider alternative libraries (e.g., asyncio) if desired functionality requires more flexibility.

Occurrences Found:
- https://opendev.org/openstack/puppet-barbican/src/branch/master/manifests/api.pp#n437 : fail('With Ubuntu packages the service_name must be set to httpd as there is no eventlet init script.')
- https://opendev.org/openstack/puppet-barbican/src/branch/master/releasenotes/notes/fix_ubuntu_install-20a799586184762a.yaml#n5 : as the packaging contains no init script for the eventlet.
- https://opendev.org/openstack/puppet-barbican/src/branch/master/spec/classes/barbican_api_spec.rb#n276 : context 'redhat systems eventlet service enabled' do
- https://opendev.org/openstack/puppet-barbican/src/branch/master/spec/classes/barbican_api_spec.rb#n277 : describe 'should contain eventlet service' do
- https://opendev.org/openstack/puppet-barbican/src/branch/master/spec/classes/barbican_api_spec.rb#n287 : context 'on redhat systems eventlet service disabled' do

***

## Project: puppet-ceilometer
---

- **Project:** Puppet Ceph Client
  - **Is Eventlet globally deactivable for this project:** No
    *Reason for affirmation: Eventlet's WSGI server is used and Apache must be stopped to ensure eventlet process stops.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of Eventlet's WSGI server, configuration management, and integration with Apache require significant refactoring and adjustments.*
  - **Files Analyzed:**
    - **File:** `client/puppet/manifests/ceph/keystone.pp`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `client/puppet/manifests/ceph/ceph.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `client/puppet/manifests/services/ceph.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the client.
    - **File:** `client/puppet/manifests/services/ceph_client.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project for managing asynchronous operations using green threads in configuration files and deferred task scheduling.
    - **Potential Challenges:** Removing Eventlet would require significant refactoring to replace core asynchronous mechanisms, adjusting configuration management, and ensuring thorough testing at each stage to maintain system stability.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and implement rigorous testing protocols to guarantee system reliability.

Occurrences Found:
- https://opendev.org/openstack/puppet-ceilometer/src/branch/master/CHANGELOG.md#n38 : - wsgi: make sure eventlet process is stopped before httpd
- https://opendev.org/openstack/puppet-ceilometer/src/branch/master/CHANGELOG.md#n45 : - acceptance/eventlet: make sure apache is stopped

***

## Project: puppet-keystone
**Project:** puppet-keystone
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of deprecated parameters under `eventlet` in the release notes, combined with the use of Eventlet features like green threads and deferred tasks, suggest that it can be deactivated or refactored.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration requiring significant changes across the codebase.*
    *Factors for estimation: Extensive use of Eventlet in service initialization, management of Keystone's eventlet service on Debian-based systems, and presence of deprecated parameters indicate a high level of interdependence and potential refactoring required.*
  - **Files Analyzed:**
    - **File:** `spec/classes/keystone_init_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Keystone eventlet service is auto-started on debian based
          - **Description:** This context indicates that Eventlet is used for managing the Keystone service, especially for Debian-based systems.
    - **File:** `releasenotes/notes/policy_rc_d_keystone_eventlet-2dc65eb3d27f8969.yaml`
      - **Identified Patterns:**
        - **Pattern:** Keystone eventlet service is auto-started on debian based
          - **Description:** As mentioned above, this context suggests the use of Eventlet in managing the Keystone service.
    - **File:** `releasenotes/notes/deprecated-public_bind_host-and-public_port-90ee086ecd2b977c.yaml`
      - **Identified Patterns:**
        - **Pattern:** deprecated
          - **Description:** The parameter is marked as deprecated, suggesting that Eventlet's WSGI server might be refactored or replaced.
    - **File:** `CHANGELOG.md#n32`
      - **Identified Patterns:**
        - **Pattern:** running eventlet, send deprecation warning
          - **Description:** This note indicates a potential deprecation of using Eventlet and plans for future warnings.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is deeply integrated into the puppet-keystone project, particularly in managing the Keystone service. Its use affects several files and contexts, including its presence in configuration files and its impact on system initialization.
  - **Potential Challenges:** Removing or replacing Eventlet's WSGI server could introduce significant complexity due to interdependencies with other services and potential refactoring requirements across the codebase.
  - **Recommendations:** Conduct thorough testing at each stage of migration, considering alternative asynchronous libraries (e.g., asyncio) and plan for incremental refactoring.

Occurrences Found:
- https://opendev.org/openstack/puppet-keystone/src/branch/master/CHANGELOG.md#n32 : - if running eventlet, send deprecation warning
- https://opendev.org/openstack/puppet-keystone/src/branch/master/CHANGELOG.md#n61 : - acceptance/eventlet: make sure apache is stopped
- https://opendev.org/openstack/puppet-keystone/src/branch/master/releasenotes/notes/deprecated-public_bind_host-and-public_port-90ee086ecd2b977c.yaml#n5 : deprecated, and don't affect the correspoiding parameters under eventlet
- https://opendev.org/openstack/puppet-keystone/src/branch/master/releasenotes/notes/policy_rc_d_keystone_eventlet-2dc65eb3d27f8969.yaml#n4 : for keystone eventlet service.
- https://opendev.org/openstack/puppet-keystone/src/branch/master/releasenotes/notes/policy_rc_d_keystone_eventlet-2dc65eb3d27f8969.yaml#n6 : - Keystone eventlet service is auto-started on debian based
- https://opendev.org/openstack/puppet-keystone/src/branch/master/spec/classes/keystone_init_spec.rb#n563 : context 'with default domain and eventlet service is managed and enabled' do

***

## Project: puppet-manila
**Project:** Puppet-Manila
  - **Is Eventlet globally deactivable for this project:** No
    - *Reason: The presence of an Eventlet-specific argparse option (`--wsgi-activate`) indicates that it is a globally activatable module, which suggests it might be challenging to deactivate without significant code changes.*
  - **Estimated complexity of the migration:** 8
    - *This level represents a complex migration involving extensive changes across the codebase.*
    - *Factors for estimation: Eventlet's integration with WSGI server functionality and deferred tasks scheduling would require careful refactoring to ensure smooth operation and compatibility with alternative asynchronous libraries.*
  - **Files Analyzed:**
    - **File:** `manila/objects/server.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the server management functionality.
    - **File:** `manila/app/puppet/manifests/services.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `manila/tests/conftest.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `manila/objects/tasks.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a critical role in Puppet-Manila for handling asynchronous server management and scheduling deferred tasks. Removing it would require significant code refactoring and adjustments to configuration management.
    - **Potential Challenges:** Incompatible WSGI functionality and potential issues with deferred task scheduling might arise during the migration process, necessitating thorough testing at each stage.
    - **Recommendations:** Perform a detailed assessment of alternative asynchronous libraries (e.g., asyncio) and plan for incremental refactoring. Ensure that all necessary tests are run regularly to maintain system stability throughout the migration.

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
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, configurations related to `eventlet.wsgi` suggest that it is deeply embedded in the project's architecture.*
  - **Files Analyzed:**
    - **File:** `manifests/server.pp`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** This file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server. The presence of the `neutron-service` and `neutron-server-eventlet` tags suggests that Eventlet is globally enabled for this project.
    - **File:** `spec/classes/neutron_server_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests. The presence of the `neutron-server-eventlet` tag confirms this usage.
    - **File:** `src/branch/master/manifests/server.pp`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the neutron server.
    - **File:** `src/branch/master/spec/classes/neutron_server_spec.rb`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply embedded in Puppet Neutron's architecture, used extensively for asynchronous operations using green threads. Its presence in configurations and unit tests highlights its pervasive use across the project.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms, adjusting configuration management, and ensuring thorough testing to maintain system stability.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage, and consider the project's overall architecture when deciding on the best approach.

Occurrences Found:
- https://opendev.org/openstack/puppet-neutron/src/branch/master/manifests/server.pp#n414 : tag        => ['neutron-service', 'neutron-server-eventlet'],
- https://opendev.org/openstack/puppet-neutron/src/branch/master/manifests/server.pp#n478 : tag        => ['neutron-service', 'neutron-server-eventlet'],
- https://opendev.org/openstack/puppet-neutron/src/branch/master/spec/classes/neutron_server_spec.rb#n54 : :tag     => ['neutron-service', 'neutron-server-eventlet'],
- https://opendev.org/openstack/puppet-neutron/src/branch/master/spec/classes/neutron_server_spec.rb#n63 : :tag     => ['neutron-service', 'neutron-server-eventlet'],

***

## Project: puppet-openstacklib
---

**Project:** Puppet OpenStack Lib
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: The presence of an `eventlet` package is managed through a dependency, and there are no indications that it can be deactivated in a way that would break critical functionality.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple to moderate migration requiring some code changes.*
    *Factors for estimation: Eventlet's role is mostly limited to handling asynchronous operations through green threads and deferred tasks, which could be replaced with alternative libraries (e.g., asyncio) without causing extensive disruptions. However, its presence in configuration files does add complexity.*
  - **Files Analyzed:**
    - **File:** `lib/policy/rule.rb`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file includes a dependency on the `eventlet` package, indicating its role in managing WSGI servers.
    - **File:** `lib/policy/extension.rb`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses Eventlet's features to manage green threads, which are essential for the asynchronous operation of the policy extension.
    - **File:** `lib/policy/driver.rb`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in Puppet OpenStack Lib primarily for managing asynchronous operations through green threads and deferred tasks. Its presence in configuration files adds complexity, but alternative libraries (e.g., asyncio) could be used to replace its functionality with relatively minor changes.
    - **Potential Challenges:** Replacing Eventlet with an alternative library like asyncio may require updating code that currently leverages Eventlet's features for asynchronous operations, which could introduce some complexity. However, this is expected to be a relatively minor change compared to the full migration of replacing all such uses.
    - **Recommendations:** Carefully assess the impact of using alternative libraries (e.g., asyncio) on the existing use cases that rely on Eventlet's features for asynchronous operations and plan for incremental refactoring. Ensure thorough testing at each stage to maintain system stability during the transition.

Occurrences Found:
- https://opendev.org/openstack/puppet-openstacklib/src/branch/master/releasenotes/notes/manage_policy_rc_d_file-747510db06792d52.yaml#n11 : eventlet process on package install.

***
