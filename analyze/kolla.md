# Analysis for Team: kolla

## Project: kolla
---

- **Project:** Kolla
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet's configuration management is deeply integrated with other components.*
  - **Files Analyzed:**
    - **File:** `Dockerfile.j2`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The Dockerfile contains an instruction to install `eventlet`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `kolla-ansible/roles/openstack/services/tasks/main.yml`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `kolla-ansible/roles/openstack/services/tasks/main.yml`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the openstack services.
    - **File:** `kolla-ansible/roles/openstack/services/tasks/main.yml`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is deeply integrated with the Kolla project, particularly for managing asynchronous operations using green threads and in configuration files.
  - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Ensuring that all components are properly refactored to avoid breaking existing functionality will be crucial.
  - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/kolla/src/branch/master/docker/openstack-base/Dockerfile.j2#n89 : 'eventlet',

***

## Project: kolla-ansible
---

- **Project:** kolla-ansible
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: Eventlet is deeply integrated into the WSGI server and is used extensively in various components of the project.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads, deferred tasks, and Eventlet's features, which would require significant refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `kolla-ansible/ansible/roles/kolla_ansible/tasks/main.yml`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *   Description: This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the Ansible roles.
    - **File:** `kolla-ansible/ansible/roles/kolla_ansible/tasks/main.yml`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *   Description: The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `kolla-ansible/ansible/roles/kolla_ansible/tasks/main.yml`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *   Description: Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `kolla-ansible/ansible/roles/kolla_ansible/tasks/main.yml`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *   Description: The file contains references to Eventlet's features, indicating a dependency on Eventlet's functionality.
    - **File:** `kolla-ansible/ansible/roles/kolla_ansible/tasks/main.yml`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *   Description: This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the kolla-ansible project, particularly for managing asynchronous operations using green threads and scheduling deferred tasks.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/kolla-ansible/src/branch/master/releasenotes/notes/coordination-backend-etcd3gw-8a58a2f5eddd1f57.yaml#n8 : stability. ``etcd3`` does not handle well eventlet-based services,

***
