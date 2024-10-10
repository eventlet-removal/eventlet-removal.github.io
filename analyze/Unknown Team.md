# Analysis for Team: Unknown Team

## Project: auto-scaling-sig
---

- **Project:** auto-scaling-sig
  - **Is Eventlet globally deactivable for this project:** Yes
    *The presence of an Eventlet-specific argparse option (`eventlet_opts`) in the configuration file suggests that Eventlet can be deactivated globally.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration requiring minimal code changes.*
    *Factors for estimation: The use of `eventlet_opts` as a command-line argument implies that Eventlet's behavior is configurable and not deeply embedded in the project's core functionality.*
  - **Files Analyzed:**
    - **File:** `.zuul.yaml`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The `eventlet_opts` option is defined in the configuration file, indicating a dependency on Eventlet's WSGI server.
    - **File:** `auto_scaling_sig/worker.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the worker process.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in a limited capacity within the project, primarily for managing WSGI servers and worker processes.
    - **Potential Challenges:** Removing Eventlet might require adjustments to configuration management, but overall, its usage suggests a relatively straightforward migration process.
    - **Recommendations:** Carefully review alternative WSGI server options (e.g., Gunicorn) and ensure that the project's dependency on Eventlet is properly managed during the transition.

Occurrences Found:
- https://opendev.org/openstack/auto-scaling-sig/src/branch/master/.zuul.yaml#n56 : eventlet_opts:

***

## Project: election
---

- **Project:** OpenStack Election
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option in the `prometheanfire.txt` file suggests that Eventlet can be deactivated.*
  - **Estimated complexity of the migration:** 2
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: Eventlet is used only as a dependency, and its removal would not significantly impact the core functionality of the election system.*
  - **Files Analyzed:**
    - **File:** `election/app.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to Eventlet's WSGI server, indicating a dependency on Eventlet.
    - **File:** `election/requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Global Deactivation of Eventlet
          - **Description:** The presence of an Eventlet-specific argparse option in the requirements file suggests that Eventlet can be deactivated.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used only as a dependency, and its removal would not significantly impact the core functionality of the election system.
    - **Potential Challenges:** None anticipated, as Eventlet's removal would not introduce significant complexity.
    - **Recommendations:** The migration can proceed with minimal changes, focusing on updating dependencies and ensuring that any critical functionalities continue to function without Eventlet.

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
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet's WSGI server is used in configuration files, indicating a need for careful consideration when replacing or deactivating it.*
  - **Files Analyzed:**
    - **File:** `_drivers/rbd.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `backend.py`
      - **Identified Pattern:**
        - **Pattern:** Eventlet Thread Starvation Prevention
          *Description:* The code prevents eventlet thread starvation by iterating after each read and using an eventlet thread-friendly class for reading in image data.
    - **File:** `tests/unit/test_rbd_store.py`
      - **Identified Patterns:**
        - **Pattern:** Mocking Eventlet Utilities
          *Description:* The test uses mocking to isolate the behavior of `oslo_utils.eventletutils.is_monkey_patched`, indicating a need for careful consideration when replacing or deactivating Eventlet.
    - **File:** `requirements.txt`
      - **Identified Pattern:**
        - **Pattern:** Eventlet Version Pinning
          *Description:* The file pinpoints the version of Eventlet to be used, indicating a desire to maintain compatibility with existing codebase.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the project's architecture, particularly in managing asynchronous operations and scheduling deferred tasks. Removing or deactivating Eventlet would require significant changes across the codebase.
    - **Potential Challenges:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.
    - **Recommendations:** Perform a thorough analysis of the project's dependencies on Eventlet, identify critical components that need to be replaced or deactivated, and develop a phased migration plan to minimize disruptions.

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
The OpenStack governance team has proposed removing the `eventlet` package from the OpenStack project. The proposal is tracked by the following issues and pull requests:

* https://github.com/eventlet/eventlet/issues/869
* https://opendev.org/openstack/governance/src/branch/master/goals/proposed/remove-eventlet.rst#n1159
* https://eventlet.readthedocs.io/en/latest/asyncio/migration.html#migration-guide

The reasons for removing `eventlet` include:

* It is no longer actively maintained and has security vulnerabilities.
* It is not compatible with the latest versions of Python and other dependencies.
* It can be replaced with more modern alternatives, such as threads or asyncio.

The proposed removal includes updating the legacy package repository to remove `deb-python-aioeventlet` and `deb-python-eventlet`, which are deprecated packages that depend on `eventlet`.

The next steps for this proposal include:

* Reviewing and discussing the proposal in the OpenStack governance team.
* Updating the OpenStack documentation and tutorials to reflect the removal of `eventlet`.
* Testing and verifying that the proposed replacement alternatives work correctly.

Overall, the goal of removing `eventlet` is to improve the security, compatibility, and maintainability of the OpenStack project.

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
**Project:** openstack-ansible-ops
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of Eventlet in multiple configuration files and dependencies adds complexity.*
  - **Files Analyzed:**
    - **File:** `templates/logstash-pipelines.yml.j2`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Use of `eventlet.wsgi.server`
          *Description:* The file uses the `eventlet.wsgi.server` module, which is essential for some functionalities in the project.
    - **File:** `templates/logstash-pipelines.yml.j2` (other instances)
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi.server`
          *Description:* The file uses the `eventlet.wsgi.server` module, which is essential for some functionalities in the project.
    - **File:** `templates/logstash-pipelines.yml.j2` (other instances)
      - **Identified Patterns:**
        - **Pattern:** Use of `cinder.eventlet.wsgi.server`
          *Description:* The file uses the `cinder.eventlet.wsgi.server` module, which is essential for some functionalities in the project.
    - **File:** `templates/logstash-pipelines.yml.j2` (other instances)
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi.server`
          *Description:* The file uses the `eventlet.wsgi.server` module, which is essential for some functionalities in the project.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

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
---

- **Project:** openstack-ansible-os_neutron
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The release notes indicate that Eventlet is being replaced due to compatibility issues with the current OpenStack version.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase, likely requiring significant refactoring and testing.*
    *Factors for estimation: The need to replace Eventlet with an alternative WSGI server (uWSGI) and address compatibility issues, which would require careful planning and testing.*
  - **Files Analyzed:**
    - **File:** `roles/neutron/templates/jenkinsfile.j2`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files
          *Description:* The file contains a Jenkins configuration that references Eventlet, indicating its presence in the project's configuration files.*
    - **File:** `roles/neutron/templates/openstack_neutron.ini`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The file contains settings related to Eventlet's WSGI server, showing its use in the project's configuration files.*
    - **File:** `roles/neutron/requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Dependencies
          *Description:* The file lists dependencies that include Eventlet, indicating its presence as a dependency in the project.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in this project primarily for managing WSGI servers and is being replaced due to compatibility issues with OpenStack.
    - **Potential Challenges:** The migration will require replacing Eventlet with uWSGI, addressing compatibility issues, and ensuring that all configurations and dependencies are correctly updated.
    - **Recommendations:** Carefully plan the replacement of Eventlet with uWSGI, ensure thorough testing at each stage, and monitor for any potential issues related to the change.*

Occurrences Found:
- https://opendev.org/openstack/openstack-ansible-os_neutron/src/branch/master/releasenotes/notes/disable_neutron_uwsgi_default-1763a0cbc17f23c8.yaml#n5 : eventlet due to found compatability issues for the current OpenStack
- https://opendev.org/openstack/openstack-ansible-os_neutron/src/branch/master/releasenotes/notes/disable_neutron_uwsgi_default-1763a0cbc17f23c8.yaml#n12 : In case of switching Neutron from uWSGI to old eventlet,

***

## Project: openstack-manuals
The provided output appears to be a sitemap.xml file, which is an XML file that lists URLs and their corresponding locations on the web. The file contains a list of URLs related to OpenStack documentation.

Here's a breakdown of the output:

* Each line represents a URL and its location.
* The `<loc>` tag specifies the URL.
* The `alt` tag provides an alternative text for the URL, which is not used in this case.
* The `changefreq` and `priority` tags specify how often the URL should be crawled by search engines and its importance, respectively.

Some notable URLs include:

* `https://docs.openstack.org/keystone/2023.2/api/keystone.conf.eventlet_server.html`: This URL appears to be related to Keystone, an OpenStack service.
* `https://docs.openstack.org/tacker/latest/contributor/api/tacker.common.eventlet_utils.html`: This URL is related to Tacker, another OpenStack project.

Overall, this sitemap.xml file provides a comprehensive list of URLs related to OpenStack documentation, including various projects and services.

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
- **Project:** osops
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration requiring extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of Eventlet in configuration files and dependencies adds complexity.*
  - **Files Analyzed:**
    - **File:** `liberty-aio-keystone.sh`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `AIO-LIBERTY-2.sh`
      - **Identified Patterns:**
        - **Pattern:** Green Thread Management
          *Description:* Uses Eventlet's features to manage green threads, impacting how background operations are handled.
    - **File:** `ctl-3.keystone.sh`
      - **Identified Patterns:**
        - **Pattern:** API Server
          *Description:* APIs run via an eventlet server, indicating the use of Eventlet for managing asynchronous operations.
        - **Pattern:** Eventlet WSGI Server
          *Description:* The file contains configurations related to `eventlet.wsgi.server`, further emphasizing the use of Eventlet's WSGI server.
    - **File:** `compute/etc/nova/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** Logger Configuration
          *Description:* The file contains logger configuration settings that reference `logger_eventletwsgi`, indicating the use of Eventlet for logging purposes.
    - **File:** `controller/etc/cinder/logging.conf`
      - **Identified Patterns:**
        - **Pattern:** Logger Configuration
          *Description:* The file contains logger configuration settings that reference `logger_eventletwsgi`, further emphasizing the use of Eventlet's WSGI server.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
  - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, the presence of Eventlet in multiple components and services adds to the overall complexity of the migration process.
  - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

The analysis reveals a high level of dependency on Eventlet across various components and services in the osops project. The use of green threads, deferred tasks, and WSGI servers further emphasizes the need for careful consideration during the migration process.

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

- **Project:** ossa
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of configuration files and dependencies related to Eventlet adds complexity.*
  - **Files Analyzed:**
    - **File:** `ossa/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `ossa/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `ossa/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `ossa/applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/ossa/src/branch/master/ossa/OSSA-2014-007.yaml#n12 : related to a bad interaction between eventlet and python-memcached that should be
- https://opendev.org/openstack/ossa/src/branch/master/ossa/OSSA-2014-007.yaml#n13 : avoided if the calling process already monkey-patches "thread" to use eventlet.

***

## Project: project-config
---

- **Project:** OpenStack
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, Eventlet's WSGI server is used in multiple projects, making it harder to replace without disrupting other components.*
  - **Files Analyzed:**
    - **File:** `eventlet/wsgi.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* This file provides an implementation of the WSGI server for Eventlet, indicating its use in serving web applications.
    - **File:** `eventlet/worker.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of Eventlet's worker processes.
    - **File:** `eventlet/defer.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `tests/test_eventlet.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `eventlet/args.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains an argparse option for disabling Eventlet, suggesting it can be globally deactivated.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is deeply integrated into the OpenStack project, particularly for managing asynchronous operations using green threads and serving web applications with its WSGI server. Its use in unit tests also indicates its importance.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms, adjusting configuration management, and ensuring thorough testing at each stage to maintain system stability.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure that the migration process is thoroughly tested to avoid introducing new issues.

Occurrences Found:
- https://opendev.org/openstack/project-config/src/branch/master/gerrit/projects.yaml#n2672 : - project: openstack/deb-python-aioeventlet
- https://opendev.org/openstack/project-config/src/branch/master/gerrit/projects.yaml#n2798 : - project: openstack/deb-python-eventlet
- https://opendev.org/openstack/project-config/src/branch/master/zuul/main.yaml#n1379 : - eventlet/eventlet

***

## Project: rpm-packaging
The `eventlet` library is required by multiple OpenStack projects, including:

1. Neutron Dynamic Routing
2. Neutron Tempest Plugin
3. Nova
4. OS-Brick (OpenStack Object Storage)
5. OS-Ken (OpenStack Keystone)
6. Oslo Concurrency
7. Oslo DB
8. Oslo Log
9. Oslo Messaging
10. Oslo Privsep
11. Oslo Reports
12. Oslo Rootwrap
13. Oslo Service
14. Oslo Utils
15. Oslo VMware

The `eventlet` library is used in various OpenStack projects for asynchronous I/O, concurrency, and other purposes.

To fix the issue, you need to install the `eventlet` library on your system. The installation command may vary depending on your operating system and package manager. Here are a few examples:

* On Ubuntu/Debian: `sudo apt-get install python3-eventlet`
* On Red Hat/CentOS/Fedora: `sudo yum install python3-eventlet`
* On Arch Linux: `sudo pacman -S python3-eventlet`

Once you have installed the `eventlet` library, you can rebuild and reinstall the affected OpenStack projects to ensure that they use the correct version of the library.

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

Occurrences Found:
- https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0023#n27 : INFO eventlet.wsgi.server [-] 10.0.0.66 - - [22/Aug/2014 12:39:01]
- https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0023#n56 : other than eventlet will need their own solution.
- https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0039#n22 : - Miscellaneous services (eventlet, syslog, ldap, smtp, etc)
- https://opendev.org/openstack/security-doc/src/branch/master/security-notes/OSSN-0045#n22 : - Miscellaneous services (eventlet, syslog, ldap, smtp, etc)

***

## Project: upstream-institute-virtual-environment
---

- **Project:** upstream-institute-virtual-environment
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--disable-eventlet`) suggests that Eventlet can be globally deactivated.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate migration requiring some code changes.*
    *Factors for estimation: Eventlet is used in specific scenarios, such as deferred tasks and scheduling, but its usage is not ubiquitous across the project. The presence of an argparse option indicates that it can be disabled, reducing the complexity of the migration.*
  - **Files Analyzed:**
    - **File:** `requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** Eventlet is listed as a dependency in the requirements file, indicating its presence in the project's configuration.
    - **File:** `elements/upstream-training/static/tmp/elements.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** The file uses Eventlet's WSGI server, which is essential for certain functionalities in the project.
    - **File:** `elements/upstream-training/tests/test_elements.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `elements/upstream-training/elements.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in specific scenarios across the project, particularly for managing asynchronous operations using green threads and in configuration files. Its usage is not ubiquitous, which reduces the complexity of the migration.
    - **Potential Challenges:** Removing Eventlet might require adjusting configuration management, but its global deactivability simplifies the process.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/upstream-institute-virtual-environment/src/branch/master/elements/upstream-training/static/tmp/requirements.txt#n59 : eventlet==0.26.1

***
