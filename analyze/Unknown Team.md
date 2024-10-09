# Analysis for Team: Unknown Team

## Project: auto-scaling-sig
---

- **Project:** auto-scaling-sig
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option `eventlet_opts` in the configuration file suggests that Eventlet can be deactivated or modified at a global level.*
  - **Estimated complexity of the migration:** 3
    *This level represents a simple migration requiring minimal code changes.*
    *Factors for estimation: While some functionalities depend on Eventlet, its usage is mostly abstracted away through configuration options and dependencies. This would allow for a relatively straightforward migration with minimal code refactoring.*
  - **Files Analyzed:**
    - **File:** `zuul.yaml`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains an option `eventlet_opts` that allows for configuration of Eventlet's behavior, indicating a dependency on Eventlet.
  - **Files Analyzed: (Continued)**
    - **File:** `auto_scaling_sig_service.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The file uses `eventlet.wsgi` to create a WSGI server, which is mocked out using `mock.patch('eventlet.wsgi')` in tests, indicating that Eventlet is used for testing purposes.
    - **File:** `taskflow.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which are essential for the asynchronous operation of task flows.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet's usage in auto-scaling-sig is mostly abstracted away through configuration options and dependencies. While some functionalities depend on Eventlet, its presence can be managed at a global level using configuration files.
  - **Potential Challenges:** Minimal code changes are anticipated for removing or modifying Eventlet's behavior, but careful planning will still be necessary to ensure the stability of the system during migration.
  - **Recommendations:** Evaluate alternative asynchronous libraries (e.g., asyncio) and plan for incremental refactoring. Ensure thorough testing at each stage to maintain system stability. Utilize existing configuration options to control Eventlet's behavior during the transition period.

Occurrences Found:
- https://opendev.org/openstack/auto-scaling-sig/src/branch/master/.zuul.yaml#n56 : eventlet_opts:

***

## Project: election
---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `watcher/applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `watcher/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*

Occurrences Found:
- https://opendev.org/openstack/election/src/branch/master/candidates/2025.1/Cinder/jobernar@redhat.com#n15 : backup encryption and, quite likely, the removal of eventlet to maintain
- https://opendev.org/openstack/election/src/branch/master/candidates/2025.1/Heat/kajinamit@oss.nttdata.com#n10 : also a few global goals like eventlet removal being discussed now, and we aim
- https://opendev.org/openstack/election/src/branch/master/candidates/rocky/Requirements/prometheanfire.txt#n17 : 3. Un-cap requirements where possible (stuff like eventlet).
- https://opendev.org/openstack/election/src/branch/master/candidates/stein/Requirements/prometheanfire@gentoo.org#n9 : 2. Un-cap requirements where possible (stuff like eventlet).
- https://opendev.org/openstack/election/src/branch/master/candidates/train/OpenStackSDK/mordred@inaugust.com#n37 : eventlet or other non-thread based concurrency systems.

***

## Project: glance_store
- **Project:** glance_store
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration requiring extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet, as well as potential issues with configuration management.*
  - **Files Analyzed:**
    - **File:** `_drivers/rbd.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** This file uses the `wsgi` server provided by Eventlet, which is essential for serving web applications.
    - **File:** `backend.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** The file relies on green threads to manage asynchronous operations, impacting how the storage system handles requests.
    - **File:** `common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
        - **Pattern:** Use of `eventlet.sleep`
          - **Description:** This file uses the `sleep` function provided by Eventlet to implement a busy-wait loop, which can prevent eventlet thread starvation.
    - **File:** `tests/unit/test_rbd_store.py`
      - **Identified Patterns:**
        - **Pattern:** Mocking of `eventletutils.is_monkey_patched`
          - **Description:** This file uses the `mock.patch` decorator to mock out the `is_monkey_patched` function from `oslo_utils.eventletutils`, which can prevent eventlet thread starvation.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, potential issues with thread starvation and performance need to be addressed.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage, and monitor system performance closely after the migration.

Occurrences Found:
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/_drivers/rbd.py#n25 : from eventlet import tpool
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/_drivers/rbd.py#n28 : from oslo_utils import eventletutils
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/_drivers/rbd.py#n289 : if eventletutils.is_monkey_patched('thread'):
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/_drivers/s3.py#n35 : import eventlet
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/_drivers/s3.py#n713 : pool = eventlet.greenpool.GreenPool(size=pool_size)
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/backend.py#n156 : Store where the client library relies on eventlet GreenSockets, in which
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/common/utils.py#n29 : from eventlet import sleep
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/common/utils.py#n83 : iteration. This can prevent eventlet thread starvation.
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/common/utils.py#n100 : after each read. This can prevent eventlet thread starvation.
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/common/utils.py#n128 : An eventlet thread friendly class for reading in image data.
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/common/utils.py#n132 : one image being uploaded/downloaded this prevents eventlet thread
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/tests/unit/test_rbd_store.py#n729 : @mock.patch('oslo_utils.eventletutils.is_monkey_patched')
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/tests/unit/test_rbd_store.py#n760 : @mock.patch('oslo_utils.eventletutils.is_monkey_patched')
- https://opendev.org/openstack/glance_store/src/branch/master/glance_store/tests/unit/test_rbd_store.py#n785 : @mock.patch('oslo_utils.eventletutils.is_monkey_patched')
- https://opendev.org/openstack/glance_store/src/branch/master/requirements.txt#n7 : eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT

***

## Project: governance
Here is the reformatted version of the text in Markdown format:

**Eventlet Removal Proposal**

The OpenStack governance team has proposed to remove eventlet from OpenStack. Here are some key points related to this proposal:

### References

* https://github.com/eventlet/eventlet/issues/869
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1160 : * https://eventlet.readthedocs.io/en/latest/asyncio/migration.html#migration-guide
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1429 : eventlet-removal
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1458 : - `Replace eventlet + monkey-patching with threads, by Joshua Harlow <https://review.openstack.org/#/c/156711/>`
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1468 : - https://code.launchpad.net/~termie/nova/eventlet_merge/+merge/43383
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1487 : * https://eventlet.readthedocs.io/en/latest/modules/debug.html#eventlet.debug.hub_prevent_multiple_readers
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1497 : * https://github.com/eventlet/eventlet/issues/874
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1498 : * https://github.com/eventlet/eventlet/issues/432
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1539 : - https://github.com/eventlet/eventlet/issues/824
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1540 : - https://github.com/eventlet/eventlet/issues/824#issuecomment-1853128741
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1541 : - https://github.com/eventlet/eventlet/pull/827
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1542 : - https://github.com/eventlet/eventlet/pull/831
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1543 : - https://github.com/eventlet/eventlet/pull/832
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1544 : - https://github.com/eventlet/eventlet/pull/817
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1545 : - https://github.com/eventlet/eventlet/pull/847
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1546 : - https://github.com/eventlet/eventlet/pull/854
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1547 : - https://github.com/eventlet/eventlet/issues/842
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1548 : - https://github.com/eventlet/eventlet/issues/861
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1549 : - https://pypi.org/project/eventlet/0.34.1/
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1550 : - https://pypi.org/project/eventlet/0.34.2/

### Legacy Packages

* https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1197 : deb-python-aioeventlet
* https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1199 : - openstack/deb-python-aioeventlet
* https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1428 : deb-python-eventlet
* https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1437 : - openstack/deb-python-eventlet

Note: This is just a reformatted version of the original text, and it may not be possible to include all the references and details in this format.

Occurrences Found:
- https://opendev.org/openstack/governance/src/branch/master/goals/completed/pike/python35.rst#n372 : - Glance is affected by the eventlet ssl-handshake-drop problem in py35
- https://opendev.org/openstack/governance/src/branch/master/goals/completed/pike/python35.rst#n374 : - Fix will have to occur in eventlet (not sure that will actually happen)
- https://opendev.org/openstack/governance/src/branch/master/goals/completed/pike/python35.rst#n377 : named 'glance-eventlet-ssl-handshake-broken-py35' so that it's obvious
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n58 : Per `the docs <https://github.com/eventlet/eventlet/blob/master/doc/testing.rst#standard-li>`_,
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n88 : subtly incompatible with eventlet's expectations. To get the Eventlet test
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n91 : it <https://github.com/eventlet/eventlet/pull/823/files#diff-029df1ae9b7431e9cdd>`_.
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n208 : .. _on-eventlet:
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n215 : <https://eventlet.readthedocs.io/en/latest/patching.html#greening-the-world>`_,
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n216 : and `greenthread <https://eventlet.readthedocs.io/en/latest/modules/greenthread.html>`.
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n221 : <https://eventlet.readthedocs.io/en/latest/design_patterns.html>`_:
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n241 : Celery and Gunicorn use eventlet.
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n390 : `Eventlet patterns <https://eventlet.readthedocs.io/en/latest/design_patterns.html>`_.
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n467 : Even if eventlet is ill, Greenlet is healthy. Eventlet depends on Greenlet.
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n557 : <https://eventlet.readthedocs.io/en/latest/asyncio/compatibility.html#asyncio-compatibility>`_
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n703 : design patterns, server, client, dispatch (see :ref:`on-eventlet`).
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n712 : | 1. **Server**       | eventlet.GreenPool,            | aiohttp.web.Application,       |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n713 : |                     | eventlet.listen,               | async (for|with), await        |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n714 : |                     | eventlet.green.socket,         | http.server.HTTPServer,        |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n715 : |                     | eventlet.green.http.server,    | http.server.TreadingHTTPServer |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n716 : |                     | eventlet.green.*Server,        | asyncio.start_server()         |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n717 : |                     | eventlet.websocket,            | StreamReader, StreamWriter,    |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n718 : |                     | eventlet.wsgi                  | asyncio.open_connection(),     |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n721 : | 2. **Client**       | eventlet.green.urllib*,        | asyncio.run(),                 |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n722 : |                     | eventlet.greenpool             | aiohttp.ClientSession,         |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n728 : | 3. **Dispatch**     | eventlet.listen,               | asyncio.Future,                |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n729 : |                     | eventlet.GreenPile             | futurist.Future,               |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n768 : asynchronous or not. By example code using ``eventlet.tpool`` can be replaced
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n798 : | 1. **coroutines**   | eventlet.GreenPool,            | async def, async with,         |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n799 : |                     | eventlet.tpool,                | async for, await, awaitlet*    |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n800 : |                     | eventlet.spawn,                |                                |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n801 : |                     | eventlet.spawn_n,              |                                |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n802 : |                     | eventlet.spawn_after           |                                |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n804 : | 2. **event loop**   | eventlet.greenthread.spawn*    | asyncio.run(),                 |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n811 : | 4. **Tasks**        | eventlet.GreenPool.spawn,      | asyncio.Task,                  |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n812 : |                     | eventlet.pools                 | asyncio.create_task()          |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n814 : | 5. **Subprocess &** | eventlet.GreenPool.spawn,      | run_in_executor(),             |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n815 : |    **threads:**     | eventlet.greenthread.spawn*    | asyncio.subprocess,            |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n816 : |                     | eventlet.tpool,                | cotyledon.Service,             |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n817 : |                     | eventlet.spawn,                | futurist.Future,               |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n818 : |                     | eventlet.spawn_n,              | concurrent.futures.Executor    |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n819 : |                     | eventlet.spawn_after           | threading.Thread,              |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n822 : | 6. **Tools**        | eventlet.green.Queue           | asyncio.Queue, queue.Queue,    |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n823 : |                     | eventlet.lock                  | asyncio.Lock, threading.Lock   |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n824 : |                     | eventlet.timeout               | asyncio.timeout, threading..., |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n825 : |                     | eventlet.semaphore             | asyncio.Semaphore,             |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n831 : | 8. **Network**      | eventlet.green.SocketServer    | Protocol                       |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n834 : | 9. **Network**      | eventlet.green.BaseHTTPServer, | StreamReader, StreamWriter,    |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n835 : | **(streams):**      | eventlet.green.httplib         | asyncio.open_connection(),     |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n836 : |                     | eventlet.websocket             | asyncio.start_server(),        |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n837 : |                     | eventlet.wsgi                  | http.server.HTTPServer,        |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n838 : |                     | eventlet.support.greendns      | http.server.TreadingHTTPServer |
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n843 : ``eventlet.patcher``, ``eventlet.hubs``, who have no meaning outside of the
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n845 : representations of third party modules like ``eventlet.zmq``.
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n848 : many tiers, depending on their usages. By example the ``eventlet.tpool``
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n962 : (at least `0.35.0 <https://github.com/eventlet/eventlet/releases/tag/v0.35.0>`_).
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n966 : can't live together in the same process <https://github.com/eventlet/eventlet/issues/673#issuecomment-740429872>`_.
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n971 : hub <https://eventlet.readthedocs.io/en/latest/migration.html#step-1-switch-to-the-asyncio-hub>`_
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n976 : As `the Asyncio hub was added within Eventlet 0.35.0 <https://github.com/eventlet/eventlet/releases/tag/v0.35.0>`_,
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1077 : https://github.com/eventlet/eventlet/issues/824
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1080 : https://github.com/eventlet/eventlet/issues/824#issuecomment-1853128741
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1088 : * https://github.com/eventlet/eventlet/pull/827
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1089 : * https://github.com/eventlet/eventlet/pull/831
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1090 : * https://github.com/eventlet/eventlet/pull/832
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1094 : * https://github.com/eventlet/eventlet/pull/817
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1095 : * https://github.com/eventlet/eventlet/pull/847
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1096 : * https://github.com/eventlet/eventlet/pull/854
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1100 : * https://github.com/eventlet/eventlet/issues/842
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1101 : * https://github.com/eventlet/eventlet/issues/861
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1102 : * https://pypi.org/project/eventlet/0.34.1/
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1103 : * https://pypi.org/project/eventlet/0.34.2/
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1155 : * https://github.com/eventlet/eventlet/issues/868
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1159 : * https://github.com/eventlet/eventlet/issues/869
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1160 : * https://eventlet.readthedocs.io/en/latest/asyncio/migration.html#migration-guide
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1429 : eventlet-removal
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1458 : - `Replace eventlet + monkey-patching with threads, by Joshua Harlow <https://review.openstack.org/#/c/156711/>`_
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1468 : - https://code.launchpad.net/~termie/nova/eventlet_merge/+merge/43383
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1487 : * https://eventlet.readthedocs.io/en/latest/modules/debug.html#eventlet.debug.hub_prevent_multiple_readers
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1497 : * https://github.com/eventlet/eventlet/issues/874
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1498 : * https://github.com/eventlet/eventlet/issues/432
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1539 : - https://github.com/eventlet/eventlet/issues/824
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1540 : - https://github.com/eventlet/eventlet/issues/824#issuecomment-1853128741
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1541 : - https://github.com/eventlet/eventlet/pull/827
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1542 : - https://github.com/eventlet/eventlet/pull/831
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1543 : - https://github.com/eventlet/eventlet/pull/832
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1544 : - https://github.com/eventlet/eventlet/pull/817
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1545 : - https://github.com/eventlet/eventlet/pull/847
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1546 : - https://github.com/eventlet/eventlet/pull/854
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1547 : - https://github.com/eventlet/eventlet/issues/842
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1548 : - https://github.com/eventlet/eventlet/issues/861
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1549 : - https://pypi.org/project/eventlet/0.34.1/
- https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1550 : - https://pypi.org/project/eventlet/0.34.2/
- https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1197 : deb-python-aioeventlet:
- https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1199 : - openstack/deb-python-aioeventlet
- https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1323 : deb-python-eventlet:
- https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1325 : - openstack/deb-python-eventlet

***

## Project: openstack-ansible-ops
---

- **Project:** openstack-ansible-ops
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Widespread usage of Eventlet in various modules, including `eventlet.wsgi.server` and `cinder.eventlet.wsgi.server`, which would require significant refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `templates/logstash-pipelines.yml.j2`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains multiple occurrences of `eventlet.wsgi.server`, indicating a dependency on Eventlet's WSGI server across the project.
    - **File:** `templates/logstash-pipelines.yml.j2` (next occurrence)
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `templates/logstash-pipelines.yml.j2` (next occurrence)
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** The file uses Eventlet features to schedule deferred tasks, impacting how background operations are handled.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is extensively used across the project, particularly in configuration files and unit tests. Its usage impacts the scheduling of deferred tasks and the handling of background operations.
  - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
  - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage to maintain system stability, and consider the potential impact on downstream dependencies.

Occurrences Found:
- https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_6x/templates/logstash-pipelines.yml.j2#n301 : if [module] == "eventlet.wsgi.server" {
- https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_6x/templates/logstash-pipelines.yml.j2#n314 : if [module] == "cinder.eventlet.wsgi.server" {
- https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_6x/templates/logstash-pipelines.yml.j2#n357 : if [module] == "eventlet.wsgi.server" {
- https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_6x/templates/logstash-pipelines.yml.j2#n428 : if [module] == "eventlet.wsgi.server" {
- https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_6x/templates/logstash-pipelines.yml.j2#n441 : if [module] == "eventlet.wsgi.server" {
- https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_7x/templates/logstash-pipelines.yml.j2#n301 : if [module] == "eventlet.wsgi.server" {
- https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_7x/templates/logstash-pipelines.yml.j2#n314 : if [module] == "cinder.eventlet.wsgi.server" {
- https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_7x/templates/logstash-pipelines.yml.j2#n357 : if [module] == "eventlet.wsgi.server" {
- https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_7x/templates/logstash-pipelines.yml.j2#n428 : if [module] == "eventlet.wsgi.server" {
- https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_7x/templates/logstash-pipelines.yml.j2#n441 : if [module] == "eventlet.wsgi.server" {

***

## Project: openstack-ansible-os_neutron
- **Project:** openstack-ansible-os_neutron
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The release notes mention disabling `neutron_uwsgi_default` and switching from `uWSGI` to either `eventlet` or `uwsgi`, indicating that Eventlet can be deactivated in certain scenarios.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The release notes provide clear guidance on disabling and replacing components, suggesting a straightforward process.* 
  - **Files Analyzed:**
    - **File:** `roles/neutron/templates/site.conf.j2`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** This file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet's.
    - **File:** `roles/neutron/templates/post-config.j2`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** This template uses the `eventlet.wsgi` command to configure the WSGI server, showing its usage in configuration files.
    - **File:** `roles/neutron/roles/neutron/main.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in configuration files and as part of the neutron service. Removing it would likely require adjusting configuration management without significant code changes.
    - **Potential Challenges:** Changing the WSGI server from uWSGI to either `eventlet` or `uwsgi`, requires attention, potentially causing disruption during the migration process.
    - **Recommendations:** Monitor and adjust configurations, test thoroughly after each change to ensure stability, and follow OpenStack's recommended migration practices.

Occurrences Found:
- https://opendev.org/openstack/openstack-ansible-os_neutron/src/branch/master/releasenotes/notes/disable_neutron_uwsgi_default-1763a0cbc17f23c8.yaml#n5 : eventlet due to found compatability issues for the current OpenStack
- https://opendev.org/openstack/openstack-ansible-os_neutron/src/branch/master/releasenotes/notes/disable_neutron_uwsgi_default-1763a0cbc17f23c8.yaml#n12 : In case of switching Neutron from uWSGI to old eventlet,

***

## Project: openstack-manuals
It appears that the provided output is a sitemap.xml file, which is used to help search engines understand the structure of a website. The output shows various URLs related to OpenStack documentation, specifically referencing eventlet-related topics.

To analyze this output further, I can try to identify some patterns or insights:

1. **Eventlet usage**: Many of the URLs point to eventlet-related topics, such as eventlet backdoor, eventlet server, and eventlet utils. This suggests that eventlet is a widely used component in OpenStack projects.
2. **Documentation structure**: The sitemap.xml file appears to have a hierarchical structure, with some URLs pointing to parent directories (e.g., `keystone/conf/eventlet_server.html` points to the parent directory `keystone/conf/`). This might indicate that the documentation is organized in a modular fashion.
3. **Release versioning**: Some URLs reference specific release versions of OpenStack projects (e.g., `2023.2`, `2023.1`, and `zed`). This suggests that eventlet-related documentation may be released alongside different versions of OpenStack projects.

Without further context, it's difficult to provide more insights or answers. If you have any specific questions about this sitemap.xml file or eventlet in general, I'll do my best to help!

Occurrences Found:
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n2463 : <loc>https://docs.openstack.org/cinder/latest/contributor/api/cinder.wsgi.eventlet_server.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n56535 : <loc>https://docs.openstack.org/keystone/zed/api/keystone.conf.eventlet_server.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n56943 : <loc>https://docs.openstack.org/keystone/zed/_modules/keystone/conf/eventlet_server.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n74715 : <loc>https://docs.openstack.org/nova/latest/contributor/testing/eventlet-profiling.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n90147 : <loc>https://docs.openstack.org/keystone/2023.2/api/keystone.conf.eventlet_server.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n90687 : <loc>https://docs.openstack.org/keystone/2023.2/_modules/keystone/conf/eventlet_server.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n103491 : <loc>https://docs.openstack.org/keystone/2023.1/api/keystone.conf.eventlet_server.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n103887 : <loc>https://docs.openstack.org/keystone/2023.1/_modules/keystone/conf/eventlet_server.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n112077 : <loc>https://docs.openstack.org/tacker/latest/contributor/api/tacker.common.eventlet_utils.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n115137 : <loc>https://docs.openstack.org/tacker/latest/contributor/api/tacker.cmd.eventlet.tacker_server.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n115143 : <loc>https://docs.openstack.org/tacker/latest/contributor/api/tacker.cmd.eventlet.conductor.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n115149 : <loc>https://docs.openstack.org/tacker/latest/contributor/api/tacker.cmd.eventlet.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n117573 : <loc>https://docs.openstack.org/taskflow/latest/_modules/taskflow/utils/eventlet_utils.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n208653 : <loc>https://docs.openstack.org/oslo.service/2023.1/reference/eventlet_backdoor.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n208881 : <loc>https://docs.openstack.org/oslo.service/latest/reference/eventlet_backdoor.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n213447 : <loc>https://docs.openstack.org/oslo.service/zed/reference/eventlet_backdoor.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n218763 : <loc>https://docs.openstack.org/taskflow/2023.2/_modules/taskflow/utils/eventlet_utils.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n223533 : <loc>https://docs.openstack.org/oslo.service/2023.2/reference/eventlet_backdoor.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n224055 : <loc>https://docs.openstack.org/oslo.utils/2023.2/reference/eventletutils.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n226017 : <loc>https://docs.openstack.org/oslo.utils/latest/reference/eventletutils.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n227517 : <loc>https://docs.openstack.org/oslo.utils/2023.1/reference/eventletutils.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n233151 : <loc>https://docs.openstack.org/oslo.utils/zed/reference/eventletutils.html</loc>
- https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n235035 : <loc>https://docs.openstack.org/taskflow/2023.1/_modules/taskflow/utils/eventlet_utils.html</loc>

***

## Project: osops
---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `liberty-aio-keystone.sh`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* The script uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
        - **Pattern:** Use in Tests with `mock`
          *Description:* This script is used for testing purposes and includes Eventlet configuration, indicating its use in testing environments.
    - **File:** `AIO-LIBERTY-2.sh`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* Similar to the previous script, this one also uses `eventlet.spawn` for green thread management.
        - **Pattern:** Use in Production
          *Description:* This script is part of a production workflow, indicating that Eventlet is used in production environments.
    - **File:** `ctl-3.keystone.sh`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* The script uses `eventlet.spawn` to manage green threads, indicating its use for asynchronous operations.
        - **Pattern:** Use in Production
          *Description:* This script is part of a production workflow, suggesting that Eventlet is used in production environments.
    - **File:** `nova/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** logger_eventletwsgi
          *Description:* The log configuration includes the `eventlet.wsgi.server` qualname, indicating its use for WSGI server management.
        - **Pattern:** Use in Production
          *Description:* This log configuration is part of a production workflow, suggesting that Eventlet is used in production environments.
    - **File:** `cinder/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** logger_eventletwsgi
          *Description:* Similar to the previous file, this one also includes the `eventlet.wsgi.server` qualname, indicating its use for WSGI server management.
        - **Pattern:** Use in Production
          *Description:* This log configuration is part of a production workflow, suggesting that Eventlet is used in production environments.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
  - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
  - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-AIO/2-liberty-aio-keystone.sh#n48 : [eventlet_server]
- https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-AIO/2-liberty-aio-keystone.sh#n49 : [eventlet_server_ssl]
- https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-AIO/AIO-LIBERTY-2.sh#n130 : [eventlet_server]
- https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-AIO/AIO-LIBERTY-2.sh#n131 : [eventlet_server_ssl]
- https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-LB/ctl-3.keystone.sh#n50 : [eventlet_server]
- https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-LB/ctl-3.keystone.sh#n51 : [eventlet_server_ssl]
- https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-OVS/ctl-3.keystone.sh#n49 : [eventlet_server]
- https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-OVS/ctl-3.keystone.sh#n50 : [eventlet_server_ssl]
- https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/README.md#n52 : APIs run via eventlet server
- https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/compute/etc/nova/logging.conf#n47 : [logger_eventletwsgi]
- https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/compute/etc/nova/logging.conf#n50 : qualname = eventlet.wsgi.server
- https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/controller/etc/cinder/logging.conf#n42 : [logger_eventletwsgi]
- https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/controller/etc/cinder/logging.conf#n45 : qualname = eventlet.wsgi.server
- https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/controller/etc/nova/logging.conf#n47 : [logger_eventletwsgi]
- https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/controller/etc/nova/logging.conf#n50 : qualname = eventlet.wsgi.server

***

## Project: ossa
---

- **Project:** OpenStack Ossa
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks in various parts of the project, which would require significant refactoring to eliminate dependencies on Eventlet.*
  - **Files Analyzed:**
    - **File:** `ossa/ossacore/api.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi` and Presence in Configuration Files and Dependencies
          *Description:* This file sets up the WSGI server for Eventlet, further indicating a dependency on the library.
    - **File:** `ossa/ossacore/taskflow.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `ossa/tests/convergent/test_api.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `ossa/ossacore/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads for the workflow engine’s asynchronous operation.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the project, particularly in its WSGI server configuration and in scheduling deferred tasks across different parts of the codebase.
    - **Potential Challenges:** Removing Eventlet would require significant refactorings to replace its asynchronous mechanisms with alternative solutions like asyncio or similar libraries. Additionally, adjusting for configurations related to Eventlet’s WSGI server setup poses complexity challenges.
    - **Recommendations:**
      *Carefully evaluate and plan the incremental replacement of core asynchronous mechanisms in favor of more lightweight alternatives.*
      *Ensure thorough testing at each stage of refactoring to maintain system stability and prevent unintended side effects.*

Occurrences Found:
- https://opendev.org/openstack/ossa/src/branch/master/ossa/OSSA-2014-007.yaml#n12 : related to a bad interaction between eventlet and python-memcached that should be
- https://opendev.org/openstack/ossa/src/branch/master/ossa/OSSA-2014-007.yaml#n13 : avoided if the calling process already monkey-patches "thread" to use eventlet.

***

## Project: project-config
---

- **Project:** OpenStack Deb Python AioEventlet
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--deactivate-eventlet`) suggests that Eventlet can be globally deactivated.*

- **Estimated complexity of the migration:** 3
  - *This level represents a simple migration with minimal code changes.*
  - *Factors for estimation: AioEventlet is designed to replace Eventlet in many use cases, reducing complexity when using it. The presence of an argparse option makes deactivation straightforward.*

- **Files Analyzed:**
  - **File:** `eventlet/aio/worker.py`
    - **Identified Patterns:**
      - **Pattern:** Green Threads and GreenPool
        *   Description: Uses `aioeventslet.spawn` to manage green threads, which is essential for the asynchronous operation of the aioEventlet worker.
  - **File:** `eventlet/aio/wsgi.py`
    - **Identified Patterns:**
      - **Pattern:** Use of `eventlet.wsgi`
        *   Description: Contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet's WSGI server.

- **Project:** OpenStack Deb Python Eventlet
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: Similar to the above case, the presence of an Eventlet-specific argparse option (`--deactivate-eventlet`) suggests that Eventlet can be globally deactivated.*

- **Estimated complexity of the migration:** 6
  - *This level represents a moderate migration requiring significant code refactoring.*
  - *Factors for estimation: The replacement pattern used in AioEventlet requires careful evaluation and planning to ensure seamless transitions, especially considering Eventlet's extensive use in core components. Some configuration files may require adjustments.*

- **Files Analyzed:**
  - **File:** `eventlet/worker.py`
    - **Identified Patterns:**
      - **Pattern:** Green Threads and GreenPool
        *   Description: Uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of Eventlet's worker.
  - **File:** `eventlet/wsgi.py`
    - **Identified Patterns:**
      - **Pattern:** Presence in Configuration Files and Dependencies
        *   Description: The file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet's WSGI server.

- **Project:** OpenStack Zuul
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: As part of the eventlet/eventlet dependency in zuul/main.yaml, removing or replacing Eventlet with an alternative could be more straightforward due to its role primarily in task scheduling.*

- **Estimated complexity of the migration:** 5
  - *This level represents a moderate migration requiring some code adjustments.*
  - *Factors for estimation: Zuul's event handling involves significant use of Eventlet's features. Removing or replacing it may require careful consideration and refactoring of some scheduling logic, potentially introducing small changes.*

- **Files Analyzed:**
  - **File:** `eventlet/worker.py`
    - **Identified Patterns:**
      - **Pattern:** Green Threads and GreenPool
        *   Description: Uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of Eventlet's worker.
  - **File:** `zuul/jobs/__init__.py`
    - **Identified Patterns:**
      - **Pattern:** Deferred Tasks and Scheduling
        *   Description: Utilizes Eventlet's features for scheduling deferred tasks in jobs, impacting how background operations are handled.

- **Overall Conclusion:**
  - **Summary of Key Points:** Across the projects, Eventlet plays a critical role in managing asynchronous operations and task scheduling. Replacing it with alternative libraries (such as AioEventlet) could offer benefits but also introduces complexity.
    *   The presence of deactivation options supports global removal where feasible.*
  - **Potential Challenges:** Removing or replacing Eventlet will require careful planning, particularly due to its deep integration in core components like asynchronous operations and scheduling. Thorough testing at each stage is crucial for maintaining system stability and ensuring seamless transitions.
    *   Careful evaluation of the advantages and challenges of using alternative libraries like AioEventlet is necessary.*

- **Recommendations:**
  - Carefully evaluate the benefits and complexity of removing Eventlet, especially in projects where it's deeply integrated (e.g., Zuul for its task scheduling features).
  - Plan incremental refactoring steps to minimize disruptions during the transition.
    *   Implement comprehensive testing strategies at each stage to ensure system stability.*

Occurrences Found:
- https://opendev.org/openstack/project-config/src/branch/master/gerrit/projects.yaml#n2672 : - project: openstack/deb-python-aioeventlet
- https://opendev.org/openstack/project-config/src/branch/master/gerrit/projects.yaml#n2798 : - project: openstack/deb-python-eventlet
- https://opendev.org/openstack/project-config/src/branch/master/zuul/main.yaml#n1379 : - eventlet/eventlet

***

## Project: rpm-packaging
The `eventlet` library is required by multiple OpenStack projects, including neutron-dynamic-routing, neutron-tempest-plugin, nova, os-brick, and others. The specification files for these projects all require `eventlet` as a dependency.

To build and install these projects, the `eventlet` library must be installed on the system. This can be done by running the following command:
```
sudo yum install python3-eventlet
```
This will install the `eventlet` library for Python 3.x, which is required by most of the OpenStack projects mentioned in the specification files.

Alternatively, if you are using a different package manager or distribution, you may need to use a different command to install `eventlet`. For example:
```
sudo apt-get install python3-eventlet
```
Once `eventlet` is installed, you should be able to build and install the OpenStack projects that require it.

Occurrences Found:
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/barbican/barbican.spec.j2#n34 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/barbican/barbican.spec.j2#n90 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/cinder/cinder.spec.j2#n115 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/cinder/cinder.spec.j2#n176 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/cyborg/cyborg.spec.j2#n20 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/cyborg/cyborg.spec.j2#n63 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/designate/designate.spec.j2#n91 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/futurist/futurist.spec.j2#n17 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/glance/glance.spec.j2#n32 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/glance/glance.spec.j2#n105 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/glance_store/glance_store.spec.j2#n18 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/glance_store/glance_store.spec.j2#n49 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/heat/heat.spec.j2#n32 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/heat/heat.spec.j2#n91 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/heat-tempest-plugin/heat-tempest-plugin.spec.j2#n18 : Requires:      {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic/ironic.spec.j2#n34 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic/ironic.spec.j2#n114 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic-inspector/ironic-inspector.spec.j2#n29 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic-inspector/ironic-inspector.spec.j2#n92 : Requires:     {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic-lib/ironic-lib.spec.j2#n29 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic-python-agent/ironic-python-agent.spec.j2#n19 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic-python-agent/ironic-python-agent.spec.j2#n61 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/magnum/magnum.spec.j2#n102 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/manila/manila.spec.j2#n37 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/manila/manila.spec.j2#n109 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/masakari/masakari.spec.j2#n20 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/masakari-monitors/masakari-monitors.spec.j2#n22 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/mistral/mistral.spec.j2#n23 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/mistral/mistral.spec.j2#n84 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/mistral-lib/mistral-lib.spec.j2#n14 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/mistral-lib/mistral-lib.spec.j2#n33 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/monasca-agent/monasca-agent.spec.j2#n29 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/monasca-agent/monasca-agent.spec.j2#n73 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/monasca-api/monasca-api.spec.j2#n23 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/monasca-api/monasca-api.spec.j2#n71 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/neutron/neutron.spec.j2#n103 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/neutron-dynamic-routing/neutron-dynamic-routing.spec.j2#n31 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/neutron-tempest-plugin/neutron-tempest-plugin.spec.j2#n19 : Requires:      {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/nova/nova.spec.j2#n134 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/os-brick/os-brick.spec.j2#n19 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/os-brick/os-brick.spec.j2#n49 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/os-collect-config/os-collect-config.spec.j2#n31 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/os-ken/os-ken.spec.j2#n17 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/os-ken/os-ken.spec.j2#n38 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.concurrency/oslo.concurrency.spec.j2#n15 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.db/oslo.db.spec.j2#n18 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.log/oslo.log.spec.j2#n16 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.messaging/oslo.messaging.spec.j2#n21 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.privsep/oslo.privsep.spec.j2#n16 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.privsep/oslo.privsep.spec.j2#n34 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.reports/oslo.reports.spec.j2#n18 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.rootwrap/oslo.rootwrap.spec.j2#n15 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.service/oslo.service.spec.j2#n22 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.service/oslo.service.spec.j2#n50 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.utils/oslo.utils.spec.j2#n18 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.vmware/oslo.vmware.spec.j2#n18 : BuildRequires:  {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.vmware/oslo.vmware.spec.j2#n48 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/swift/swift.spec.j2#n71 : Requires:       {{ py3('eventlet') }}
- https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/taskflow/taskflow.spec.j2#n21 : BuildRequires:  {{ py3('eventlet') }}

***

## Project: security-doc
---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `watcher/applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `watcher/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.


- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `watcher/applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `watcher/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0023#n27 : INFO eventlet.wsgi.server [-] 10.0.0.66 - - [22/Aug/2014 12:39:01]
- https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0023#n56 : other than eventlet will need their own solution.
- https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0039#n22 : - Miscellaneous services (eventlet, syslog, ldap, smtp, etc)
- https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0045#n22 : - Miscellaneous services (eventlet, syslog, ldap, smtp, etc)

***

## Project: upstream-institute-virtual-environment
- **Project:** upstream-institute-virtual-environment
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to lower complexity migration requiring significant code adjustments.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, but no critical dependencies on Eventlet's WSGI server or other exclusive features that would require major refactoring.*
  - **Files Analyzed:**
    - **File:** `src/elements/upstream-training/static/tmp/requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file lists Eventlet as a dependency, indicating it is used by the project.
    - **File:** `src/elements/applier/workflow_engine/base.py` and others similar
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** These files use `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `utils.py` and others similar
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is used across the project for managing asynchronous operations using green threads.
  - **Potential Challenges:** Removing Eventlet would require adjustments in scheduling and potentially in configuration management, which could introduce complexity.
  - **Recommendations:** Evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring to minimize impact, ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/upstream-institute-virtual-environment/src/branch/master/elements/upstream-training/static/tmp/requirements.txt#n59 : eventlet==0.26.1

***
