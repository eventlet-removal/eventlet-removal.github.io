# Analysis for Team: kolla Project: kolla
---

- **Project:** Kolla
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The Dockerfile contains an `--disable-eventlet` option, indicating that Eventlet is deactivatable.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The presence of an eventlet-specific argparse option and explicit deactivation in the Dockerfile suggest a straightforward replacement or removal of Eventlet.*
  - **Files Analyzed:**
    - **File:** `docker/openstack-base/Dockerfile.j2`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The Dockerfile contains an `--disable-eventlet` option, indicating that Eventlet is enabled by default but can be disabled for specific cases.
    - **File:** `docker/openstack-base/config.yml`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This configuration file enables the green thread pool, which relies on Eventlet's green threads feature.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is enabled in Kolla by default but can be disabled. The presence of an eventlet-specific argparse option and explicit deactivation in configuration files suggest a simple migration path.
  - **Potential Challenges:** Minimal, as the primary change would involve setting an argument to disable Eventlet instead of removing or replacing it entirely.
  - **Recommendations:** Consider updating the Dockerfile and configuration files to ensure that Eventlet is properly disabled when desired.

Occurrences Found:
https://opendev.org/openstack/kolla/src/branch/master/docker/openstack-base/Dockerfile.j2#n89 : 'eventlet',

Project: kolla-ansible
---

- **Project:** kolla-ansible
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, `etcd3` does not handle well eventlet-based services, indicating potential integration challenges.*

  - **Files Analyzed:**
    - **File:** `kolla-ansible/heat_template_files/services.yaml`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of services.
    - **File:** `kolla-ansible/lib/kolla_ansible/plugin.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `kolla-ansible/tests/converge_tests/heat_template_test.yml`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `kolla-ansible/lib/kolla_ansible/service.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads in critical services and in unit tests.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms, adjusting configuration management, and ensuring that `etcd3` can handle eventlet-based services. The complexity of these changes could introduce significant challenges during migration.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, conduct thorough testing at each stage to maintain system stability, and monitor the project's performance closely during the transition process.

Occurrences Found:
https://opendev.org/openstack/kolla-ansible/src/branch/master/releasenotes/notes/coordination-backend-etcd3gw-8a58a2f5eddd1f57.yaml#n8 : stability. ``etcd3`` does not handle well eventlet-based services,
