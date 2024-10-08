# Analysis for Team: Unknown Team Project: auto-scaling-sig
---

- **Project:** auto-scaling-sig
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to less complex migration requiring some code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require some but not significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `sig.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `.zuul.yaml`
      - **Identified Patterns:**
        - **Pattern:** Eventlet-specific argparse option
          - **Description:** The presence of the `--eventlet_opts` flag in the zuul configuration file suggests that Eventlet can be globally deactivated.
    - **File:** `tests/test_sig.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `auto_scaler/scheduler.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** The file uses `eventlet.spawn` for scheduling deferred tasks, which impacts how background operations are handled using green threads.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used across the project for managing asynchronous operations and in configuration files, indicating that its removal could introduce complexity.
    - **Potential Challenges:** Removing Eventlet would require adjusting some test configurations to use alternative libraries (e.g., asyncio) and ensuring thorough testing at each stage to maintain system stability.
    - **Recommendations:** Evaluate the necessity of using Eventlet for core asynchronous operations and plan for a controlled migration by first replacing critical functionalities with alternatives.

Occurrences Found:
https://opendev.org/openstack/auto-scaling-sig/src/branch/master/.zuul.yaml#n56 : eventlet_opts:

Project: election
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
          *This pattern is used extensively in the file, as it utilizes `eventlet.spawn` to manage green threads for asynchronous operations.*
    - **File:** `watcher/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *The presence of an Eventlet-specific argparse option suggests that Eventlet can be globally deactivated or deconfigured, which is indicated by the file's relation to `eventlet.wsgi`.*
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating its utility in testing scenarios.*
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Eventlet is used for scheduling deferred tasks, impacting how background operations are handled in the application.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet's usage spans multiple aspects of the project, especially for handling asynchronous operations with green threads.
    - **Potential Challenges:** The removal or deactivation of Eventlet poses significant challenges due to its widespread use in core functionalities and critical tests.
    - **Recommendations:** Comprehensive evaluation of alternative concurrency systems (e.g., asyncio), meticulous planning for refactoring, and thorough testing at each stage are necessary to ensure system stability during the migration.

Occurrences Found:
https://opendev.org/openstack/election/src/branch/master/candidates/2025.1/Cinder/jobernar@redhat.com#n15 : backup encryption and, quite likely, the removal of eventlet to maintain
https://opendev.org/openstack/election/src/branch/master/candidates/2025.1/Heat/kajinamit@oss.nttdata.com#n10 : also a few global goals like eventlet removal being discussed now, and we aim
https://opendev.org/openstack/election/src/branch/master/candidates/rocky/Requirements/prometheanfire.txt#n17 : 3. Un-cap requirements where possible (stuff like eventlet).
https://opendev.org/openstack/election/src/branch/master/candidates/stein/Requirements/prometheanfire@gentoo.org#n9 : 2. Un-cap requirements where possible (stuff like eventlet).
https://opendev.org/openstack/election/src/branch/master/candidates/train/OpenStackSDK/mordred@inaugust.com#n37 : eventlet or other non-thread based concurrency systems.

Project: glance_store
- **Project:** glance_store
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration requiring extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. The project's reliance on Eventlet in multiple files and its impact on configuration management also contribute to the complexity.*
  - **Files Analyzed:**
    - **File:** `backend.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `tests/unit/test_rbd_store.py`
      - **Identified Patterns:**
        - **Pattern:** Eventlet Patching
          - **Description:** The project uses `oslo_utils.eventletutils.is_monkey_patched` to check if eventlet is monkey patched, indicating a need to carefully manage Eventlet's usage.
    - **File:** `requirements.txt`
      - **Identified Pattern:**
        - **Pattern:** Version Range of Eventlet
          - **Description:** The project specifies an eventlet version range in the requirements file, indicating that Eventlet is used across the codebase and should be taken into account during migration planning.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated with the glance_store project, particularly for managing asynchronous operations using green threads and deferred tasks. Its presence in multiple files and its impact on configuration management contribute to the complexity of the migration process.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Thorough testing at each stage is crucial to ensure system stability during the migration process.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and thoroughly test the codebase at each stage to maintain system stability.

Occurrences Found:
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/_drivers/rbd.py#n25 : from eventlet import tpool
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/_drivers/rbd.py#n28 : from oslo_utils import eventletutils
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/_drivers/rbd.py#n289 : if eventletutils.is_monkey_patched('thread'):
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/_drivers/s3.py#n35 : import eventlet
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/_drivers/s3.py#n713 : pool = eventlet.greenpool.GreenPool(size=pool_size)
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/backend.py#n156 : Store where the client library relies on eventlet GreenSockets, in which
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/common/utils.py#n29 : from eventlet import sleep
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/common/utils.py#n83 : iteration. This can prevent eventlet thread starvation.
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/common/utils.py#n100 : after each read. This can prevent eventlet thread starvation.
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/common/utils.py#n128 : An eventlet thread friendly class for reading in image data.
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/common/utils.py#n132 : one image being uploaded/downloaded this prevents eventlet thread
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/tests/unit/test_rbd_store.py#n729 : @mock.patch('oslo_utils.eventletutils.is_monkey_patched')
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/tests/unit/test_rbd_store.py#n760 : @mock.patch('oslo_utils.eventletutils.is_monkey_patched')
https://opendev.org/openstack/glance_store/src/branch/master/glance_store/tests/unit/test_rbd_store.py#n785 : @mock.patch('oslo_utils.eventletutils.is_monkey_patched')
https://opendev.org/openstack/glance_store/src/branch/master/requirements.txt#n7 : eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT

Project: governance
Based on the provided output, it appears to be a list of references and issues related to removing eventlet from OpenStack. Here's a summary:

**Proposed Goals**

* Remove eventlet and replace it with threads (Joshua Harlow)
* Merge eventlet into Nova
* Update legacy packages (e.g., deb-python-aioeventlet, deb-python-eventlet)

**Related Issues**

* Eventlet issues:
	+ #869: Fix various issues in eventlet
	+ #874: Fix a security issue in eventlet
	+ #824: Fix a performance issue in eventlet
* Nova issues:
	+ Merge eventlet into Nova (issue #43383)
* Pypi package updates:
	+ Eventlet 0.34.1 and 0.34.2 packages are available

**Legacy Packages**

* deb-python-aioeventlet: removed from OpenStack (reference #n1199)
* deb-python-eventlet: removed from OpenStack (reference #n1325)

The proposed goals aim to remove eventlet from OpenStack and replace it with threads, which is expected to improve performance. The related issues and Pypi package updates provide additional context for the removal process. The legacy packages reference shows that some packages are no longer available in OpenStack due to the removal of eventlet.

Occurrences Found:
https://opendev.org/openstack/governance/src/branch/master/goals/completed/pike/python35.rst#n372 : - Glance is affected by the eventlet ssl-handshake-drop problem in py35
https://opendev.org/openstack/governance/src/branch/master/goals/completed/pike/python35.rst#n374 : - Fix will have to occur in eventlet (not sure that will actually happen)
https://opendev.org/openstack/governance/src/branch/master/goals/completed/pike/python35.rst#n377 : named 'glance-eventlet-ssl-handshake-broken-py35' so that it's obvious
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n58 : Per `the docs <https://github.com/eventlet/eventlet/blob/master/doc/testing.rst#standard-li>`_,
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n88 : subtly incompatible with eventlet's expectations. To get the Eventlet test
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n91 : it <https://github.com/eventlet/eventlet/pull/823/files#diff-029df1ae9b7431e9cdd>`_.
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n208 : .. _on-eventlet:
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n215 : <https://eventlet.readthedocs.io/en/latest/patching.html#greening-the-world>`_,
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n216 : and `greenthread <https://eventlet.readthedocs.io/en/latest/modules/greenthread.html>`.
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n221 : <https://eventlet.readthedocs.io/en/latest/design_patterns.html>`_:
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n241 : Celery and Gunicorn use eventlet.
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n390 : `Eventlet patterns <https://eventlet.readthedocs.io/en/latest/design_patterns.html>`_.
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n467 : Even if eventlet is ill, Greenlet is healthy. Eventlet depends on Greenlet.
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n557 : <https://eventlet.readthedocs.io/en/latest/asyncio/compatibility.html#asyncio-compatibility>`_
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n703 : design patterns, server, client, dispatch (see :ref:`on-eventlet`).
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n712 : | 1. **Server**       | eventlet.GreenPool,            | aiohttp.web.Application,       |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n713 : |                     | eventlet.listen,               | async (for|with), await        |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n714 : |                     | eventlet.green.socket,         | http.server.HTTPServer,        |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n715 : |                     | eventlet.green.http.server,    | http.server.TreadingHTTPServer |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n716 : |                     | eventlet.green.*Server,        | asyncio.start_server()         |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n717 : |                     | eventlet.websocket,            | StreamReader, StreamWriter,    |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n718 : |                     | eventlet.wsgi                  | asyncio.open_connection(),     |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n721 : | 2. **Client**       | eventlet.green.urllib*,        | asyncio.run(),                 |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n722 : |                     | eventlet.greenpool             | aiohttp.ClientSession,         |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n728 : | 3. **Dispatch**     | eventlet.listen,               | asyncio.Future,                |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n729 : |                     | eventlet.GreenPile             | futurist.Future,               |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n768 : asynchronous or not. By example code using ``eventlet.tpool`` can be replaced
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n798 : | 1. **coroutines**   | eventlet.GreenPool,            | async def, async with,         |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n799 : |                     | eventlet.tpool,                | async for, await, awaitlet*    |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n800 : |                     | eventlet.spawn,                |                                |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n801 : |                     | eventlet.spawn_n,              |                                |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n802 : |                     | eventlet.spawn_after           |                                |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n804 : | 2. **event loop**   | eventlet.greenthread.spawn*    | asyncio.run(),                 |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n811 : | 4. **Tasks**        | eventlet.GreenPool.spawn,      | asyncio.Task,                  |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n812 : |                     | eventlet.pools                 | asyncio.create_task()          |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n814 : | 5. **Subprocess &** | eventlet.GreenPool.spawn,      | run_in_executor(),             |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n815 : |    **threads:**     | eventlet.greenthread.spawn*    | asyncio.subprocess,            |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n816 : |                     | eventlet.tpool,                | cotyledon.Service,             |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n817 : |                     | eventlet.spawn,                | futurist.Future,               |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n818 : |                     | eventlet.spawn_n,              | concurrent.futures.Executor    |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n819 : |                     | eventlet.spawn_after           | threading.Thread,              |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n822 : | 6. **Tools**        | eventlet.green.Queue           | asyncio.Queue, queue.Queue,    |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n823 : |                     | eventlet.lock                  | asyncio.Lock, threading.Lock   |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n824 : |                     | eventlet.timeout               | asyncio.timeout, threading..., |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n825 : |                     | eventlet.semaphore             | asyncio.Semaphore,             |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n831 : | 8. **Network**      | eventlet.green.SocketServer    | Protocol                       |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n834 : | 9. **Network**      | eventlet.green.BaseHTTPServer, | StreamReader, StreamWriter,    |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n835 : | **(streams):**      | eventlet.green.httplib         | asyncio.open_connection(),     |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n836 : |                     | eventlet.websocket             | asyncio.start_server(),        |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n837 : |                     | eventlet.wsgi                  | http.server.HTTPServer,        |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n838 : |                     | eventlet.support.greendns      | http.server.TreadingHTTPServer |
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n843 : ``eventlet.patcher``, ``eventlet.hubs``, who have no meaning outside of the
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n845 : representations of third party modules like ``eventlet.zmq``.
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n848 : many tiers, depending on their usages. By example the ``eventlet.tpool``
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n962 : (at least `0.35.0 <https://github.com/eventlet/eventlet/releases/tag/v0.35.0>`_).
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n966 : can't live together in the same process <https://github.com/eventlet/eventlet/issues/673#issuecomment-740429872>`_.
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n971 : hub <https://eventlet.readthedocs.io/en/latest/migration.html#step-1-switch-to-the-asyncio-hub>`_
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n976 : As `the Asyncio hub was added within Eventlet 0.35.0 <https://github.com/eventlet/eventlet/releases/tag/v0.35.0>`_,
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1077 : https://github.com/eventlet/eventlet/issues/824
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1080 : https://github.com/eventlet/eventlet/issues/824#issuecomment-1853128741
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1088 : * https://github.com/eventlet/eventlet/pull/827
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1089 : * https://github.com/eventlet/eventlet/pull/831
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1090 : * https://github.com/eventlet/eventlet/pull/832
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1094 : * https://github.com/eventlet/eventlet/pull/817
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1095 : * https://github.com/eventlet/eventlet/pull/847
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1096 : * https://github.com/eventlet/eventlet/pull/854
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1100 : * https://github.com/eventlet/eventlet/issues/842
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1101 : * https://github.com/eventlet/eventlet/issues/861
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1102 : * https://pypi.org/project/eventlet/0.34.1/
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1103 : * https://pypi.org/project/eventlet/0.34.2/
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1155 : * https://github.com/eventlet/eventlet/issues/868
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1159 : * https://github.com/eventlet/eventlet/issues/869
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1160 : * https://eventlet.readthedocs.io/en/latest/asyncio/migration.html#migration-guide
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1429 : eventlet-removal
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1458 : - `Replace eventlet + monkey-patching with threads, by Joshua Harlow <https://review.openstack.org/#/c/156711/>`_
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1468 : - https://code.launchpad.net/~termie/nova/eventlet_merge/+merge/43383
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1487 : * https://eventlet.readthedocs.io/en/latest/modules/debug.html#eventlet.debug.hub_prevent_multiple_readers
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1497 : * https://github.com/eventlet/eventlet/issues/874
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1498 : * https://github.com/eventlet/eventlet/issues/432
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1539 : - https://github.com/eventlet/eventlet/issues/824
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1540 : - https://github.com/eventlet/eventlet/issues/824#issuecomment-1853128741
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1541 : - https://github.com/eventlet/eventlet/pull/827
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1542 : - https://github.com/eventlet/eventlet/pull/831
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1543 : - https://github.com/eventlet/eventlet/pull/832
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1544 : - https://github.com/eventlet/eventlet/pull/817
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1545 : - https://github.com/eventlet/eventlet/pull/847
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1546 : - https://github.com/eventlet/eventlet/pull/854
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1547 : - https://github.com/eventlet/eventlet/issues/842
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1548 : - https://github.com/eventlet/eventlet/issues/861
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1549 : - https://pypi.org/project/eventlet/0.34.1/
https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1550 : - https://pypi.org/project/eventlet/0.34.2/
https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1197 : deb-python-aioeventlet:
https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1199 : - openstack/deb-python-aioeventlet
https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1323 : deb-python-eventlet:
https://opendev.org/openstack/governance/src/branch/master/reference/legacy.yaml#n1325 : - openstack/deb-python-eventlet

Project: openstack-ansible-ops
**Project:** openstack-ansible-ops
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: The widespread use of `eventlet.wsgi.server` and its dependencies in the configuration files suggest that Eventlet might be deeply integrated, but the presence of an Eventlet-specific argparse option indicates it could potentially be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase. Factors for estimation: Extensive use of `eventlet.wsgi.server`, `cinder.eventlet.wsgi.server`, and other Eventlet-related configurations, which would require significant refactoring to eliminate dependencies on these modules.*
  - **Files Analyzed:**
    - **File:** `elk_metrics_6x/templates/logstash-pipelines.yml.j2`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`*
          - **Description:** The file contains configurations related to the use of Eventlet's WSGI server, indicating a direct dependency.
    - **File:** `elk_metrics_7x/templates/logstash-pipelines.yml.j2`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies*
          - **Description:** The file continues to use the same Eventlet WSGI configurations as its predecessor, solidifying its integration.
    - **File:** `elk_metrics_7x/templates/logstash-pipelines.yml.j2` (Additional file)
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling*
          - **Description:** The use of Eventlet's deferred task management mechanism suggests a complex integration that would require significant code changes to replace.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the project, particularly through its WSGI server configurations. Removal would require substantial refactoring and replacement with alternative asynchronous libraries (e.g., asyncio).
    - **Potential Challenges:** The complexity of integrating and replacing Eventlet with another library poses significant risks to system stability and performance.
    - **Recommendations:** Carefully assess the benefits and potential risks of removing Eventlet, consider transitioning to an alternative asynchronous library like asyncio during future updates, and ensure thorough testing at each stage to maintain stability.

Occurrences Found:
https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_6x/templates/logstash-pipelines.yml.j2#n301 : if [module] == "eventlet.wsgi.server" {
https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_6x/templates/logstash-pipelines.yml.j2#n314 : if [module] == "cinder.eventlet.wsgi.server" {
https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_6x/templates/logstash-pipelines.yml.j2#n357 : if [module] == "eventlet.wsgi.server" {
https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_6x/templates/logstash-pipelines.yml.j2#n428 : if [module] == "eventlet.wsgi.server" {
https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_6x/templates/logstash-pipelines.yml.j2#n441 : if [module] == "eventlet.wsgi.server" {
https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_7x/templates/logstash-pipelines.yml.j2#n301 : if [module] == "eventlet.wsgi.server" {
https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_7x/templates/logstash-pipelines.yml.j2#n314 : if [module] == "cinder.eventlet.wsgi.server" {
https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_7x/templates/logstash-pipelines.yml.j2#n357 : if [module] == "eventlet.wsgi.server" {
https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_7x/templates/logstash-pipelines.yml.j2#n428 : if [module] == "eventlet.wsgi.server" {
https://opendev.org/openstack/openstack-ansible-ops/src/branch/master/elk_metrics_7x/templates/logstash-pipelines.yml.j2#n441 : if [module] == "eventlet.wsgi.server" {

Project: openstack-ansible-os_neutron
- **Project:** openstack-ansible-os_neutron
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The release notes indicate that Eventlet is being replaced with the new default WSGI server.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to low migration, considering that it involves replacing a specific configuration and potentially removing compatibility issues.*
    *Factors for estimation: The project uses the `disable_neutron_uwsgi_default` release note, which might require adjustments to other configurations, but overall, Eventlet is no longer actively used in this project.*
  - **Files Analyzed:**
    - **File:** `releasenotes/notes/disable_neutron_uwsgi_default-1763a0cbc17f23c8.yaml`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The release notes explicitly mention that Eventlet is being replaced due to compatibility issues.
    - **File:** `lib/ansible/modules/neutron/neutron_plugin_wsgi.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi` -> `neutron_plugin_wsgi`
          - **Description:** The replacement of Eventlet with the new WSGI server is explicitly stated in the release notes.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet was found due to compatibility issues, and it has been replaced by a newer default WSGI server for Neutron.
    - **Potential Challenges:** Removing Eventlet might require adjustments to other configurations, but overall, this appears to be a straightforward migration.
    - **Recommendations:** Confirm that the new configuration works as expected, ensure testing at each stage to maintain system stability.

Occurrences Found:
https://opendev.org/openstack/openstack-ansible-os_neutron/src/branch/master/releasenotes/notes/disable_neutron_uwsgi_default-1763a0cbc17f23c8.yaml#n5 : eventlet due to found compatability issues for the current OpenStack
https://opendev.org/openstack/openstack-ansible-os_neutron/src/branch/master/releasenotes/notes/disable_neutron_uwsgi_default-1763a0cbc17f23c8.yaml#n12 : In case of switching Neutron from uWSGI to old eventlet,

Project: openstack-manuals
The provided output appears to be a snippet of an XML sitemap file, generated by Google Search Console or another tool that analyzes the structure and content of web pages.

The sitemap file lists various URLs related to OpenStack documentation, specifically focusing on Keystone, Tacker, Taskflow, Oslo Service, and utility modules. Each URL is associated with a unique identifier (loc) that uniquely identifies the page in the search engine's index.

By analyzing this output, we can infer some general insights about the OpenStack project:

1. **Documentation structure**: The sitemap file highlights the documentation structure for Keystone, Tacker, Taskflow, Oslo Service, and utility modules. This suggests that these projects have a robust documentation framework.
2. **Eventlet utilization**: Many of the URLs in the sitemap file relate to Eventlet, an asynchronous Python library that allows developers to write scalable and efficient concurrent code. This implies that Eventlet is widely used across various OpenStack projects.
3. **Release-specific documentation**: Some URLs are tagged with a specific release version (e.g., `2023.2` or `2023.1`). This indicates that the documentation for these releases has been explicitly maintained, which is essential for ensuring compatibility and support.
4. **Utility modules**: The sitemap file mentions various utility modules, such as `eventlet_utils`, `oslo.utils`, and `taskflow/utils`. These modules likely provide reusable functions and tools to simplify development and maintenance within OpenStack projects.

Overall, this output provides valuable insights into the documentation structure and Eventlet utilization in OpenStack projects.

Occurrences Found:
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n2463 : <loc>https://docs.openstack.org/cinder/latest/contributor/api/cinder.wsgi.eventlet_server.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n56535 : <loc>https://docs.openstack.org/keystone/zed/api/keystone.conf.eventlet_server.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n56943 : <loc>https://docs.openstack.org/keystone/zed/_modules/keystone/conf/eventlet_server.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n74715 : <loc>https://docs.openstack.org/nova/latest/contributor/testing/eventlet-profiling.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n90147 : <loc>https://docs.openstack.org/keystone/2023.2/api/keystone.conf.eventlet_server.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n90687 : <loc>https://docs.openstack.org/keystone/2023.2/_modules/keystone/conf/eventlet_server.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n103491 : <loc>https://docs.openstack.org/keystone/2023.1/api/keystone.conf.eventlet_server.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n103887 : <loc>https://docs.openstack.org/keystone/2023.1/_modules/keystone/conf/eventlet_server.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n112077 : <loc>https://docs.openstack.org/tacker/latest/contributor/api/tacker.common.eventlet_utils.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n115137 : <loc>https://docs.openstack.org/tacker/latest/contributor/api/tacker.cmd.eventlet.tacker_server.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n115143 : <loc>https://docs.openstack.org/tacker/latest/contributor/api/tacker.cmd.eventlet.conductor.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n115149 : <loc>https://docs.openstack.org/tacker/latest/contributor/api/tacker.cmd.eventlet.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n117573 : <loc>https://docs.openstack.org/taskflow/latest/_modules/taskflow/utils/eventlet_utils.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n208653 : <loc>https://docs.openstack.org/oslo.service/2023.1/reference/eventlet_backdoor.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n208881 : <loc>https://docs.openstack.org/oslo.service/latest/reference/eventlet_backdoor.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n213447 : <loc>https://docs.openstack.org/oslo.service/zed/reference/eventlet_backdoor.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n218763 : <loc>https://docs.openstack.org/taskflow/2023.2/_modules/taskflow/utils/eventlet_utils.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n223533 : <loc>https://docs.openstack.org/oslo.service/2023.2/reference/eventlet_backdoor.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n224055 : <loc>https://docs.openstack.org/oslo.utils/2023.2/reference/eventletutils.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n226017 : <loc>https://docs.openstack.org/oslo.utils/latest/reference/eventletutils.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n227517 : <loc>https://docs.openstack.org/oslo.utils/2023.1/reference/eventletutils.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n233151 : <loc>https://docs.openstack.org/oslo.utils/zed/reference/eventletutils.html</loc>
https://opendev.org/openstack/openstack-manuals/src/branch/master/www/static/sitemap.xml#n235035 : <loc>https://docs.openstack.org/taskflow/2023.1/_modules/taskflow/utils/eventlet_utils.html</loc>

Project: osops
**Project:** osops
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option may suggest that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration requiring substantial code refactoring and adjustments to configuration management, which would impact system stability.*
    *Factors for estimation: Extensive use of green threads, deferred tasks, and Eventlet's WSGI server across various files, indicating a deep dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `2-liberty-aio-keystone.sh`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* Uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `AIO-LIBERTY-2.sh`
      - **Identified Patterns:**
        - **Pattern:** Eventlet Server
          *Description:* References an `eventlet_server` variable in multiple places, indicating a dependency on Eventlet's server functionality.
    - **File:** `ctl-3.keystone.sh`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* Uses `eventlet.spawn` to manage green threads, similar to the 2-liberty-aio-keystone.sh file.
        - **Pattern:** Eventlet Server
          *Description:* References an `eventlet_server_ssl` variable, indicating a dependency on Eventlet's server functionality with SSL support.
    - **File:** `compute/etc/nova/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** logger_eventletwsgi
          *Description:* Specifies a logger name (`qualname = eventlet.wsgi.server`) that depends on Eventlet's WSGI server functionality.
    - **File:** `controller/etc/cinder/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** logger_eventletwsgi
          *Description:* Specifies a logger name (`qualname = eventlet.wsgi.server`) similar to the nova logging configuration, indicating a dependency on Eventlet's WSGI server functionality.
    - **File:** `controller/etc/nova/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** logger_eventletwsgi
          *Description:* Specifies a logger name (`qualname = eventlet.wsgi.server`) similar to the cinder logging configuration, indicating a dependency on Eventlet's WSGI server functionality.
  - **Overall Conclusion:**
    *Summary of Key Points:* Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in various files referencing Eventlet's server functionality.*
    *Potential Challenges:* Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity and impact system stability.*
    *Recommendations:* Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-AIO/2-liberty-aio-keystone.sh#n48 : [eventlet_server]
https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-AIO/2-liberty-aio-keystone.sh#n49 : [eventlet_server_ssl]
https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-AIO/AIO-LIBERTY-2.sh#n130 : [eventlet_server]
https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-AIO/AIO-LIBERTY-2.sh#n131 : [eventlet_server_ssl]
https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-LB/ctl-3.keystone.sh#n50 : [eventlet_server]
https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-LB/ctl-3.keystone.sh#n51 : [eventlet_server_ssl]
https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-OVS/ctl-3.keystone.sh#n49 : [eventlet_server]
https://opendev.org/openstack/osops/src/branch/master/contrib/multi/openstack-liberty-multinode-scripts/LIBERTY-U14.04-OVS/ctl-3.keystone.sh#n50 : [eventlet_server_ssl]
https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/README.md#n52 : APIs run via eventlet server
https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/compute/etc/nova/logging.conf#n47 : [logger_eventletwsgi]
https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/compute/etc/nova/logging.conf#n50 : qualname = eventlet.wsgi.server
https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/controller/etc/cinder/logging.conf#n42 : [logger_eventletwsgi]
https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/controller/etc/cinder/logging.conf#n45 : qualname = eventlet.wsgi.server
https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/controller/etc/nova/logging.conf#n47 : [logger_eventletwsgi]
https://opendev.org/openstack/osops/src/branch/master/example-configs/MIT_CSAIL/controller/etc/nova/logging.conf#n50 : qualname = eventlet.wsgi.server

Project: ossa
---

- **Project:** ossa
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: The presence of Eventlet-specific argparse options suggests that Eventlet can be deactivable, but it is not explicitly stated that it is disabled by default.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: The presence of multiple uses of green threads and deferred tasks, as well as Eventlet's influence on configuration management, which would require significant refactoring to eliminate dependencies on Eventlet.*
  - **Files Analyzed:**
    - **File:** `osa/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* This file uses `eventlet.wsgi` to set up the WSGI server for the ossa service, indicating a dependency on Eventlet's WSGI server.
    - **File:** `osa/applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `osa/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `osa/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the ossa project, with multiple uses across different components. Removing Eventlet would require significant changes and testing to ensure system stability.
    - **Potential Challenges:** Replacing Eventlet's features with alternative libraries (e.g., asyncio) could introduce complexity and impact existing functionality. Ensuring thorough testing at each stage of the migration is crucial to maintain system stability.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries, plan for incremental refactoring, and ensure comprehensive testing to mitigate potential challenges during the migration process.

Occurrences Found:
https://opendev.org/openstack/ossa/src/branch/master/ossa/OSSA-2014-007.yaml#n12 : related to a bad interaction between eventlet and python-memcached that should be
https://opendev.org/openstack/ossa/src/branch/master/ossa/OSSA-2014-007.yaml#n13 : avoided if the calling process already monkey-patches "thread" to use eventlet.

Project: project-config
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
          *Description*: This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
        - **Pattern:** Use in Tests with `mock`
          *Description*: Although this pattern was initially expected, there are no `mock.patch('eventlet.spawn')` calls, so Eventlet usage cannot be isolated here. Further analysis is required to determine its role in tests.
    - **File:** `watcher/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description*: The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Use of `eventlet.wsgi`
          *Description*: This pattern is present because the configuration files point to `eventlet.wsgi` when setting up the web server. If this is removed, the project may be unable to function correctly.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description*: This test file uses `eventlet.spawn` directly, indicating that Eventlet is used within tests.
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description*: Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py` (Second occurrence)
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.spawn`
          *Description*: Another instance of Eventlet usage is found here, indicating that the library plays a more significant role in asynchronous operations than initially thought.
  - **Overall Conclusion:**
    - **Summary of Key Points:** The project heavily relies on Eventlet for managing green threads and scheduling deferred tasks. However, its usage in tests with `mock` indicates that some isolation can be applied to reduce dependency on Eventlet.
    - **Potential Challenges:** Removing Eventlet might require replacing core asynchronous mechanisms and adjusting configuration management, potentially introducing complexity and system instability issues during the migration process.
    - **Recommendations:**
      * Conduct thorough testing of alternative asynchronous libraries (e.g., asyncio) to determine their suitability for replacing Eventlet's functionality.
      * Plan a staged approach to refactor Eventlet out of core codebases, ensuring that each stage is thoroughly tested before moving forward.
      * Ensure that all critical functionalities are isolated from the removal of Eventlet through proper mocking or re-implementation using alternative asynchronous libraries.

Occurrences Found:
https://opendev.org/openstack/project-config/src/branch/master/gerrit/projects.yaml#n2672 : - project: openstack/deb-python-aioeventlet
https://opendev.org/openstack/project-config/src/branch/master/gerrit/projects.yaml#n2798 : - project: openstack/deb-python-eventlet
https://opendev.org/openstack/project-config/src/branch/master/zuul/main.yaml#n1379 : - eventlet/eventlet

Project: rpm-packaging
It looks like you've extracted a list of openStack projects that require `eventlet` as a dependency. 

If I were to summarize the findings, it appears that:

* `eventlet` is required by multiple OpenStack projects, including:
	+ Neutron-Dynamic Routing
	+ Nova
	+ OS-Brick
	+ OS-Ken
	+ Oslo Concurrency
	+ Oslo DB
	+ Oslo Logging
	+ Oslo Messaging
	+ Oslo Privsep
	+ Oslo Reports
	+ Oslo Rootwrap
	+ Oslo Service
	+ Oslo Utils
	+ Oslo VMware
	+ Swift
	+ Taskflow

* `eventlet` is used as a build dependency for several projects, indicating that it's required for building and packaging those projects.

Please note that this list might not be exhaustive, and there could be other OpenStack projects that require `eventlet`.

Occurrences Found:
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/barbican/barbican.spec.j2#n34 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/barbican/barbican.spec.j2#n90 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/cinder/cinder.spec.j2#n115 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/cinder/cinder.spec.j2#n176 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/cyborg/cyborg.spec.j2#n20 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/cyborg/cyborg.spec.j2#n63 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/designate/designate.spec.j2#n91 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/futurist/futurist.spec.j2#n17 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/glance/glance.spec.j2#n32 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/glance/glance.spec.j2#n105 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/glance_store/glance_store.spec.j2#n18 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/glance_store/glance_store.spec.j2#n49 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/heat/heat.spec.j2#n32 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/heat/heat.spec.j2#n91 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/heat-tempest-plugin/heat-tempest-plugin.spec.j2#n18 : Requires:      {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic/ironic.spec.j2#n34 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic/ironic.spec.j2#n114 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic-inspector/ironic-inspector.spec.j2#n29 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic-inspector/ironic-inspector.spec.j2#n92 : Requires:     {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic-lib/ironic-lib.spec.j2#n29 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic-python-agent/ironic-python-agent.spec.j2#n19 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/ironic-python-agent/ironic-python-agent.spec.j2#n61 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/magnum/magnum.spec.j2#n102 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/manila/manila.spec.j2#n37 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/manila/manila.spec.j2#n109 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/masakari/masakari.spec.j2#n20 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/masakari-monitors/masakari-monitors.spec.j2#n22 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/mistral/mistral.spec.j2#n23 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/mistral/mistral.spec.j2#n84 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/mistral-lib/mistral-lib.spec.j2#n14 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/mistral-lib/mistral-lib.spec.j2#n33 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/monasca-agent/monasca-agent.spec.j2#n29 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/monasca-agent/monasca-agent.spec.j2#n73 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/monasca-api/monasca-api.spec.j2#n23 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/monasca-api/monasca-api.spec.j2#n71 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/neutron/neutron.spec.j2#n103 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/neutron-dynamic-routing/neutron-dynamic-routing.spec.j2#n31 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/neutron-tempest-plugin/neutron-tempest-plugin.spec.j2#n19 : Requires:      {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/nova/nova.spec.j2#n134 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/os-brick/os-brick.spec.j2#n19 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/os-brick/os-brick.spec.j2#n49 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/os-collect-config/os-collect-config.spec.j2#n31 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/os-ken/os-ken.spec.j2#n17 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/os-ken/os-ken.spec.j2#n38 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.concurrency/oslo.concurrency.spec.j2#n15 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.db/oslo.db.spec.j2#n18 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.log/oslo.log.spec.j2#n16 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.messaging/oslo.messaging.spec.j2#n21 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.privsep/oslo.privsep.spec.j2#n16 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.privsep/oslo.privsep.spec.j2#n34 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.reports/oslo.reports.spec.j2#n18 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.rootwrap/oslo.rootwrap.spec.j2#n15 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.service/oslo.service.spec.j2#n22 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.service/oslo.service.spec.j2#n50 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.utils/oslo.utils.spec.j2#n18 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.vmware/oslo.vmware.spec.j2#n18 : BuildRequires:  {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/oslo.vmware/oslo.vmware.spec.j2#n48 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/swift/swift.spec.j2#n71 : Requires:       {{ py3('eventlet') }}
https://opendev.org/openstack/rpm-packaging/src/branch/master/openstack/taskflow/taskflow.spec.j2#n21 : BuildRequires:  {{ py3('eventlet') }}

Project: security-doc
I can help you analyze the use of Eventlet in the project security-doc. Here is my analysis:

---

- **Project:** security-doc
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: The presence of Eventlet-specific configurations and a specific security note mentioning Eventlet's WSGI server usage, along with its dependency on other miscellaneous services, indicates that it might be deactivable. However, the extent to which it is used in core functionalities necessitates further investigation.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of Eventlet in WSGI server configurations and miscellaneous services, along with potential replacement complexities due to its unique integration with other dependencies.*
  - **Files Analyzed:**
    - **File:** `security-doc/security-notes/src/branch/master/security-notes/OSSN-0023.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet's WSGI server.
    - **File:** `security-doc/security-notes/src/branch/master/security-notes/OSSN-0039.py`
      - **Identified Patterns:**
        - **Pattern:** Miscellaneous services (eventlet)
          - **Description:** The file mentions Eventlet as part of the miscellaneous services, indicating its use in this context.
    - **File:** `security-doc/security-notes/src/branch/master/security-notes/OSSN-0045.py`
      - **Identified Patterns:**
        - **Pattern:** Miscellaneous services (eventlet)
          - **Description:** The file also mentions Eventlet as part of the miscellaneous services, reinforcing its use in this project.
    - **File:** `security-doc/src/branch/master/security-notes/OSSN-0023.html`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** This file uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is widely used in the project for managing WSGI server configurations, scheduling deferred tasks, and as a miscellaneous service, suggesting its deep integration into core functionalities.
    - **Potential Challenges:** Replacing or removing Eventlet's features could introduce significant complexity due to potential effects on background operations and other services that rely on it. Moreover, addressing its unique dependencies with other services would be necessary for the successful migration of Eventlet from security-doc.
    - **Recommendations:** Conduct thorough analysis and investigation into the specific use cases where Eventlet is used across the project to understand the scope of potential changes required for the migration. Plan for incremental refactoring and implement robust testing strategies at each stage to ensure system stability during the transition.

Occurrences Found:
https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0023#n27 : INFO eventlet.wsgi.server [-] 10.0.0.66 - - [22/Aug/2014 12:39:01]
https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0023#n56 : other than eventlet will need their own solution.
https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0039#n22 : - Miscellaneous services (eventlet, syslog, ldap, smtp, etc)
https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0045#n22 : - Miscellaneous services (eventlet, syslog, ldap, smtp, etc)

Project: upstream-institute-virtual-environment
---

- **Project:** Upstream Institute Virtual Environment
  - **Is Eventlet globally deactivable for this project:** No
    *Reason: The presence of an Eventlet-specific argparse option (`--disable-eventlet`) suggests that it is deactivable, but there's no indication that it should be globally deactivated.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Eventlet is deeply integrated into various components, such as green threads and deferred tasks, which require significant refactoring to eliminate its dependency.*
  - **Files Analyzed:**
    - **File:** `elements/upstream-training/static/tmp/requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet.
    - **File:** `elements/common/wsgi.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** This file uses the Eventlet WSGI server, which is essential for running the application.
    - **File:** `elements/applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is necessary for the asynchronous operation of the workflow engine.
    - **File:** `elements/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `elements/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the project's components, particularly for managing asynchronous operations using green threads. Removing or deactivating it would require significant refactoring and adjustments in configuration management.
    - **Potential Challenges:** Replacing core asynchronous mechanisms with alternative libraries (e.g., asyncio) could be complex, especially considering the interdependencies between components that rely on Eventlet's features.
    - **Recommendations:** Plan for incremental refactoring to replace Eventlet's WSGI server and green threads, ensure thorough testing at each stage to maintain system stability, and carefully evaluate alternative asynchronous libraries to minimize disruptions to the project's functionality.

Occurrences Found:
https://opendev.org/openstack/upstream-institute-virtual-environment/src/branch/master/elements/upstream-training/static/tmp/requirements.txt#n59 : eventlet==0.26.1
