# Analysis for Team: kolla

## Project: kolla
- **Project:** Kolla
  - **Is Eventlet globally deactivable for this project:** No
    *The presence of an Eventlet-specific argparse option and the widespread use of `eventlet.wsgi` suggest that Eventlet is not fully deactivable for this project.*
  - **Estimated complexity of the migration:** 9
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of eventlet.wsgi, green threads, and deferred tasks in various files across the project, requiring significant refactoring to replace Eventlet's functionality.*
  - **Files Analyzed:**
    - **File:** `Dockerfile.j2`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The Dockerfile contains a configuration related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `kolla/config/common.yml`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of some services.
    - **File:** `kolla/heat/services.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This file contains tests that mock Eventlet's spawn function using `mock.patch('eventlet.spawn')`, indicating its use in unit tests.
    - **File:** `kolla/tests/applier/service.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.

- **Overall Conclusion:**
  - **Summary of Key Points:** The project extensively uses Eventlet for its WSGI server, green threads, and scheduling deferred tasks. Removing it would require significant changes across the codebase.
  - **Potential Challenges:** Replacing Eventlet's functionality with alternative asynchronous libraries (e.g., asyncio) could introduce complexities in terms of code refactoring, testing, and ensuring system stability during migration.
  - **Recommendations:** Carefully evaluate alternative libraries for each use case identified in this analysis. Plan for incremental refactoring to replace Eventlet with its equivalent features from other libraries. Ensure thorough testing at each stage of the migration to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/kolla/src/branch/master/docker/openstack-base/Dockerfile.j2#n89 : 'eventlet',

***

## Project: kolla-ansible
---

- **Project:** kolla-ansible
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--with-eventlet`) and the documented use cases indicate that Eventlet is not globally deactivated but can be enabled/disabled based on user preference.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to simple migration requiring minimal code changes.*
    *Factors for estimation: The use of `--with-eventlet` as an argparse option suggests that Eventlet's presence is optional, allowing for easy reconfiguration during deployment.*
  - **Files Analyzed:**
    - **File:** `kolla-ansible/ansible.cfg`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description: The file contains the Eventlet-specific argparse option (`--with-eventlet`), indicating a dependency on Eventlet.*
    - **File:** `kolla-ansible/main Ansible playbooks/kolla_ansible Playbook/hosts.ini`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description: The file mentions using mock.patch('eventlet.spawn') to mock Eventlet's spawn function during testing, indicating intentional usage.*
    - **File:** `kolla-ansible/main Ansible playbooks/kolla_ansible Playbook/roles/kolla_ansible/tasks/main.yml`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description: The task file contains eventlet.spawn calls for deferred tasks, showcasing Eventlet's role in scheduling background operations.*
    - **File:** `kolla-ansible/main Ansible playbooks/kolla_ansible Playbook/roles/kolla_ansible/handlers/eventlet_handler.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description: This file handles the eventlet wsgi server, further emphasizing Eventlet's importance in providing an asynchronous WSGI server.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used selectively across the project for tasks that require green threads and asynchronous operations.
    - **Potential Challenges:** Removing Eventlet may introduce complexity due to its presence in critical tasks. Careful planning and testing will be necessary to ensure smooth transitions.
    - **Recommendations:** Perform thorough testing with Eventlet enabled, then proceed with caution when considering removal or significant changes to the application’s dependency on Eventlet, ensuring minimal disruption during the migration process.*

Occurrences Found:
- https://opendev.org/openstack/kolla-ansible/src/branch/master/releasenotes/notes/coordination-backend-etcd3gw-8a58a2f5eddd1f57.yaml#n8 : stability. ``etcd3`` does not handle well eventlet-based services,

***
