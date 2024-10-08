# Analysis for Team: oslo Project: etcd3gw
---

- **Project:** etcd3gw
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `utils.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
        - **Pattern:** Use in Tests with `mock`
          *Description:* Uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Utilizes Eventlet features for scheduling deferred tasks, impacting how background operations are handled.
    - **File:** `applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* The file uses `eventlet.spawn` to manage green threads, essential for the asynchronous operation of the workflow engine.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is widely used in etcd3gw, primarily for managing concurrency with green threads and deferred tasks. Removing Eventlet would require significant refactoring and adjustments to configuration management.
  - **Potential Challenges:** The project's reliance on Eventlet poses challenges during migration due to the complexity introduced by its features. Careful evaluation of alternative libraries is required, along with thorough testing at each stage to maintain system stability.
  - **Recommendations:** Gradually refactor code to use compatible alternatives, prioritize unit tests for each change, and consider a staged rollout to minimize disruptions during migration.

Occurrences Found:
https://opendev.org/openstack/etcd3gw/src/branch/master/etcd3gw/utils.py#n78 : _eventlet = _try_import('eventlet')
https://opendev.org/openstack/etcd3gw/src/branch/master/etcd3gw/utils.py#n79 : _patcher = _try_import('eventlet.patcher')
https://opendev.org/openstack/etcd3gw/src/branch/master/etcd3gw/utils.py#n83 : if all((_eventlet, _patcher)) and _patcher.is_monkey_patched('thread'):

Project: futurist
The code uses the Eventlet library, which is a green threading implementation for Python. The `eventlet` module is imported in various places, and specific modules from Eventlet are also imported.

Here's a summary of the imports:

* `eventlet`: The main Eventlet module, which provides an alternative to the GIL (Global Interpreter Lock) that allows for true parallelism.
* `patcher as greenpatcher` and `queue as greenqueue`: These imports are related to patching and queuing functionality in Eventlet.
* `green threading as greenthreading`: This import is used in specific tests to ensure that the correct threading implementation is being used.

The requirements.txt file specifies the minimum version of Eventlet required, which is 0.18.2.

It's worth noting that Eventlet is a green threading library, meaning it uses cooperative scheduling instead of preemptive scheduling like traditional threads. This allows for more efficient and lightweight concurrency in Python.

Occurrences Found:
https://opendev.org/openstack/futurist/src/branch/master/README.rst#n25 : executed. This library currently adds statistics gathering, an eventlet
https://opendev.org/openstack/futurist/src/branch/master/doc/source/user/examples.rst#n15 : import eventlet
https://opendev.org/openstack/futurist/src/branch/master/doc/source/user/examples.rst#n19 : eventlet.sleep(3)
https://opendev.org/openstack/futurist/src/branch/master/doc/source/user/examples.rst#n25 : eventlet.sleep(1)
https://opendev.org/openstack/futurist/src/branch/master/doc/source/user/examples.rst#n27 : eventlet.sleep(1)
https://opendev.org/openstack/futurist/src/branch/master/doc/source/user/examples.rst#n49 : import eventlet
https://opendev.org/openstack/futurist/src/branch/master/doc/source/user/examples.rst#n53 : eventlet.sleep(3)
https://opendev.org/openstack/futurist/src/branch/master/doc/source/user/examples.rst#n59 : eventlet.sleep(1)
https://opendev.org/openstack/futurist/src/branch/master/doc/source/user/examples.rst#n61 : eventlet.sleep(1)
https://opendev.org/openstack/futurist/src/branch/master/doc/source/user/features.rst#n8 : * A :py:class:`.futurist.GreenThreadPoolExecutor` using `eventlet`_ green
https://opendev.org/openstack/futurist/src/branch/master/doc/source/user/features.rst#n33 : .. _eventlet: http://eventlet.net/
https://opendev.org/openstack/futurist/src/branch/master/futurist/_futures.py#n244 : object operate correctly under eventlet)
https://opendev.org/openstack/futurist/src/branch/master/futurist/_futures.py#n322 : and http://eventlet.net/doc/modules/greenpool.html for information on
https://opendev.org/openstack/futurist/src/branch/master/futurist/_green.py#n19 : from eventlet import greenpool
https://opendev.org/openstack/futurist/src/branch/master/futurist/_green.py#n20 : from eventlet import patcher as greenpatcher
https://opendev.org/openstack/futurist/src/branch/master/futurist/_green.py#n21 : from eventlet import queue as greenqueue
https://opendev.org/openstack/futurist/src/branch/master/futurist/_green.py#n23 : from eventlet.green import threading as greenthreading
https://opendev.org/openstack/futurist/src/branch/master/futurist/_utils.py#n26 : import eventlet as _eventlet  # noqa
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_executors.py#n17 : from eventlet.green import threading as green_threading
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_periodics.py#n19 : import eventlet
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_periodics.py#n20 : from eventlet.green import threading as green_threading
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_periodics.py#n51 : t = eventlet.spawn(run_what, *args, **kwargs)
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_periodics.py#n81 : 'sleep': eventlet.sleep,
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_periodics.py#n583 : eventlet.sleep(2.0)
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n15 : import eventlet
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n27 : def mini_delay(use_eventlet_sleep=False):
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n28 : if use_eventlet_sleep:
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n29 : eventlet.sleep(0.1)
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n38 : 'executor_kwargs': {}, 'use_eventlet_sleep': False}),
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n41 : 'use_eventlet_sleep': True}),
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n43 : 'executor_kwargs': {}, 'use_eventlet_sleep': True}),
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n45 : 'executor_kwargs': {}, 'use_eventlet_sleep': False}),
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n47 : 'executor_kwargs': {}, 'use_eventlet_sleep': False}),
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n63 : mini_delay, use_eventlet_sleep=self.use_eventlet_sleep))
https://opendev.org/openstack/futurist/src/branch/master/futurist/tests/test_waiters.py#n76 : mini_delay, use_eventlet_sleep=self.use_eventlet_sleep))
https://opendev.org/openstack/futurist/src/branch/master/futurist/waiters.py#n31 : from eventlet.green import threading as greenthreading
https://opendev.org/openstack/futurist/src/branch/master/futurist/waiters.py#n58 : def _ensure_eventlet(func):
https://opendev.org/openstack/futurist/src/branch/master/futurist/waiters.py#n59 : """Decorator that verifies we have the needed eventlet components."""
https://opendev.org/openstack/futurist/src/branch/master/futurist/waiters.py#n174 : @_ensure_eventlet
https://opendev.org/openstack/futurist/src/branch/master/futurist/waiters.py#n196 : @_ensure_eventlet
https://opendev.org/openstack/futurist/src/branch/master/test-requirements.txt#n2 : eventlet>=0.18.2 # MIT

Project: oslo-specs
It appears that this is a collection of OpenStack specification pages from the Oslo project, which is responsible for providing utilities and tools for building OpenStack applications.

These specifications cover various aspects of the Oslo project, including:

1. `make-enginefacade-a-facade.rst`: This page discusses the need to remove thread locals in eventlet environments.
2. `adopt-futurist.rst`: This page introduces a new type of executor called `GreenThreadPoolExecutor` that uses an `eventlet` green pool and greenthreads, making it easier to move from eventlet to another style of execution.
3. `graduate-oslo-service.rst`, `windows-oslo-service-workers.rst`, and other related pages: These pages discuss the integration of Eventlet with various OpenStack services, such as RabbitMQ, Redis, and logging.

Some key points to take away from these specifications include:

* The need to remove thread locals in eventlet environments.
* The introduction of a new type of executor called `GreenThreadPoolExecutor` that uses an `eventlet` green pool and greenthreads.
* Eventlet fix: https://github.com/eventlet/eventlet/pull/309

It's worth noting that these specifications are part of the OpenStack project, which is a popular open-source software framework for building cloud computing platforms.

Occurrences Found:
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/bobcat/http-driver.rst#n121 : * WSGI server: eventlet
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/bobcat/http-driver.rst#n273 : eventlet which is used by oslo.service.
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n7 : During Icehouse release cycle in order to drop dependency on eventlet we
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n8 : removed eventlet ``tpool.Proxy`` helper and the corresponding config option
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n13 : optional integration with eventlet ``tpool.Proxy`` as a separate module within
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n22 : eventlet green threads: being a C-extension it hangs the process on blocking
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n23 : DB queries (as eventlet can't monkey patch it to force a green thread switch
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n26 : eventlet provides a work around for this problem: ``tpool.Proxy`` helper class
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n29 : real OS thread, which knows how to deal with eventlet thread pool, on return
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n36 : option one had to use a patched version of eventlet, as neither PyPI releases
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n40 : common DB code. eventlet was removed as one of the dependencies we thought were
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n43 : process concurrent requests, whether they use eventlet green threads, real
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n46 : eventlet ``tpool.Proxy``, so the people wouldn't need to do this in their
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n54 : eventlet ``tpool.Proxy`` call proxying we need:
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n63 : 4. Even though we'll use eventlet ``tpool.Proxy`` class at runtime, we are not
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n64 : going to add it to oslo.db ``requirements.txt`` (we don't want eventlet to
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n65 : be an install time dependency; if you use eventlet, it will be up to you to
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n79 : 2. Document, that if someone wanted to use oslo.db with eventlet
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n83 : The advantage of this solution is that we don't need to re-add eventlet stuff
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n88 : clear to users why they need to use another library, if they use eventlet and
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n108 : lot, when eventlet and ``MySQL-Python`` are used. Note: you will still need
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n109 : to use unreleased eventlet code.
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n124 : OpenStack projects that want to use oslo.db with eventlet ``tpool.Proxy`` call
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n184 : that patched version of eventlet is needed to use it.
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n190 : To work properly, this depends on the unreleased version of eventlet. Though,
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n192 : version of eventlet in production. We are going to leave it up to deployers to
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n211 : eventlet tpool docs:
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n213 : http://eventlet.net/doc/threading.html#module-eventlet.tpool
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n215 : The patch fixing the eventlet issue:
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/juno/add-tpool-proxy-wrapper.rst#n217 : https://bitbucket.org/eventlet/eventlet/pull-request/29/
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/kilo/make-enginefacade-a-facade.rst#n403 : at the eventlet level and 2. thread locals are seen as "action at a distance",
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/kilo/make-enginefacade-a-facade.rst#n605 : 1. The need for thread locals or any issues with eventlet is removed.
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/adopt-futurist.rst#n21 : - An eventlet executor (aka a ``GreenThreadPoolExecutor``) that uses a
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/adopt-futurist.rst#n22 : `eventlet`_ greenpool and greenthreads (making the same futures/executor
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/adopt-futurist.rst#n23 : interface work when running in an eventlet environment). This kind of
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/adopt-futurist.rst#n24 : addition makes it eas(ier) to move from eventlet to another style of
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/adopt-futurist.rst#n125 : and http://eventlet.net/doc/modules/greenpool.html for information on
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/adopt-futurist.rst#n214 : * `eventlet`_ (**optionally** required for green futures/executors)
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/adopt-futurist.rst#n231 : .. _eventlet: http://eventlet.net/
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/graduate-oslo-service.rst#n17 : * openstack/common/eventlet_backdoor.py
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/graduate-oslo-service.rst#n18 : * tests/unit/test_eventlet_backdoor.py
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/graduate-oslo-service.rst#n25 : * tests/unit/eventlet_service.py
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/windows-oslo-service-workers.rst#n18 : * eventlet.greenio.GreenPipe, which it cannot be used, as it tries to set the
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/liberty/zmq-req-rep.rst#n189 : .. [3] https://blueprints.launchpad.net/oslo.messaging/+spec/zmq-work-without-eventlet
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/mitaka/rabbitmq-pika-driver.rst#n61 : It does not have special adapter for eventlet. But It is possible to use
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/mitaka/rabbitmq-pika-driver.rst#n62 : 'BlockingConnection' adapter and eventlet monkey patching. It works pretty
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/mitaka/rabbitmq-pika-driver.rst#n64 : not patched by current eventlet implementation. So I added code which removes
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/mitaka/rabbitmq-pika-driver.rst#n65 : 'pull' and 'epull' attributes from 'select' module if eventlet is patched.
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/mitaka/rabbitmq-pika-driver.rst#n66 : In this case Pika uses standard select api which is patched by eventlet
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/newton/reconfigurable-oslo-logging.rst#n127 : The 'logging' module was creating a Lock before eventlet could monkey-patch
https://opendev.org/openstack/oslo-specs/src/branch/master/specs/newton/reconfigurable-oslo-logging.rst#n135 : * Eventlet fix: https://github.com/eventlet/eventlet/pull/309

Project: oslo.cache
- **Project:** oslo.cache
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an Eventlet-specific argparse option (`--disable-eventlet`) suggests that Eventlet is deactivable.*
  - **Estimated complexity of the migration:** 5
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: Eventlet's deactivation does not require extensive refactoring, and the project uses other asynchronous mechanisms in some cases.*
  - **Files Analyzed:**
    - **File:** `_bmemcache_pool.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* Eventlet is imported and used as a dependency.
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `_opts.py`
      - **Identified Patterns:**
        - **Pattern:** Help Messages
          *Description:* The file provides help messages that include references to Eventlet.
    - **File:** `tests/unit/test_connection_pool.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `tests/unit/test_connection_pool.py` (continued)
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* The file uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in oslo.cache primarily for dependency management and testing. Its deactivation does not require extensive changes to the codebase.
    - **Potential Challenges:** Removing Eventlet might introduce minor issues with testing and configuration management, but overall, the migration should be straightforward.
    - **Recommendations:** Carefully evaluate alternatives to Eventlet's WSGI server for dependency management and consider maintaining support for testing scenarios using `mock.patch`.

Occurrences Found:
https://opendev.org/openstack/oslo.cache/src/branch/master/oslo_cache/_bmemcache_pool.py#n18 : import eventlet
https://opendev.org/openstack/oslo.cache/src/branch/master/oslo_cache/_bmemcache_pool.py#n20 : eventlet = None
https://opendev.org/openstack/oslo.cache/src/branch/master/oslo_cache/_bmemcache_pool.py#n40 : if eventlet and eventlet.patcher.is_monkey_patched('thread'):
https://opendev.org/openstack/oslo.cache/src/branch/master/oslo_cache/_memcache_pool.py#n26 : import eventlet
https://opendev.org/openstack/oslo.cache/src/branch/master/oslo_cache/_memcache_pool.py#n28 : eventlet = None
https://opendev.org/openstack/oslo.cache/src/branch/master/oslo_cache/_memcache_pool.py#n51 : if eventlet and eventlet.patcher.is_monkey_patched('thread'):
https://opendev.org/openstack/oslo.cache/src/branch/master/oslo_cache/_opts.py#n51 : help='Cache backend module. For eventlet-based or '
https://opendev.org/openstack/oslo.cache/src/branch/master/oslo_cache/tests/unit/test_connection_pool.py#n155 : import eventlet
https://opendev.org/openstack/oslo.cache/src/branch/master/oslo_cache/tests/unit/test_connection_pool.py#n156 : eventlet.monkey_patch()

Project: oslo.concurrency
The `eventlet` library is used in the OpenStack project, specifically in the `oslo_concurrency` module. Here's a summary of the changes made to import and patch `subprocess` and `threading` with `eventlet`:

1. The `original` function from `eventlet.patcher` is used to restore the original behavior of `subprocess` and `threading`.
2. The `subprocess` module is imported from `eventlet.green`, which suggests that Python 3.x is being used.
3. The `tpool` module is imported from `eventlet`, which is likely used for concurrent execution.

The patching process involves:

1. Importing the `eventlet` library.
2. Using the `monkey_patch()` function to modify the behavior of `subprocess` and `threading`.
3. Setting `os.name == 'nt'` as a condition to apply the patches only on Windows platforms (`NT` is short for "New Technology" in MS-DOS).

The tests use the patched versions of `subprocess` and `threading` to verify their behavior, especially when executing processes and threads concurrently.

To reproduce this code, you would need to:

1. Install `eventlet` using pip: `pip install eventlet`
2. Import the necessary modules from `eventlet`.
3. Use the `monkey_patch()` function to modify the behavior of `subprocess` and `threading`, if needed.
4. Write tests that use the patched versions of `subprocess` and `threading`.

Occurrences Found:
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/lockutils.py#n37 : import eventlet
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/lockutils.py#n38 : from eventlet import patcher as eventlet_patcher
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/lockutils.py#n40 : eventlet = None
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/lockutils.py#n41 : eventlet_patcher = None
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/lockutils.py#n99 : if eventlet is not None and eventlet_patcher is not None:
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/lockutils.py#n100 : if eventlet_patcher.is_monkey_patched('thread'):
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/lockutils.py#n101 : self._current_thread = eventlet.getcurrent
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/lockutils.py#n258 : eventlet.monkey_patch(), else `semaphore.Semaphore`) unless external is
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/processutils.py#n49 : eventlet = importutils.try_import('eventlet')
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/processutils.py#n50 : eventlet_patched = eventlet and eventlet.patcher.is_monkey_patched(time)
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/processutils.py#n51 : if eventlet_patched:
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/processutils.py#n58 : subprocess = eventlet.patcher.original('subprocess')
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/processutils.py#n59 : subprocess.threading = eventlet.patcher.original('threading')
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/processutils.py#n61 : from eventlet.green import subprocess
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/processutils.py#n63 : from eventlet import tpool
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/processutils.py#n418 : if eventlet_patched and os.name == 'nt':
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/__init__.py#n18 : import eventlet
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/__init__.py#n19 : eventlet.monkey_patch()
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n18 : import eventlet
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n19 : from eventlet import greenpool
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n43 : wait1 = eventlet.event.Event()
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n44 : wait2 = eventlet.event.Event()
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n49 : eventlet.sleep(0)
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n58 : self.other_started = eventlet.event.Event()
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n59 : self.other_finished = eventlet.event.Event()
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n73 : eventlet.sleep(0)
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n88 : fair=False, spawn=eventlet.spawn
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n93 : fair=False, spawn=eventlet.spawn_n
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n98 : fair=True, spawn=eventlet.spawn
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_lockutils_eventlet.py#n103 : fair=True, spawn=eventlet.spawn_n
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_processutils.py#n124 : use_eventlet=False):
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_processutils.py#n133 : with mock.patch.object(processutils, 'eventlet_patched',
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_processutils.py#n134 : use_eventlet):
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_processutils.py#n147 : if use_eventlet:
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_processutils.py#n153 : def test_windows_execute_without_eventlet(self):
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_processutils.py#n156 : def test_windows_execute_using_eventlet(self):
https://opendev.org/openstack/oslo.concurrency/src/branch/master/oslo_concurrency/tests/unit/test_processutils.py#n157 : self._test_windows_execute(use_eventlet=True)
https://opendev.org/openstack/oslo.concurrency/src/branch/master/test-requirements.txt#n5 : eventlet>=0.19.0 # MIT

Project: oslo.config
---

- **Project:** oslo.config
  - **Is Eventlet globally deactivable for this project:** No
    *The presence of `eventlet_server` as a valid replacement group suggests that Eventlet is used in a way that cannot be easily deactivated or replaced with another server.*
  - **Estimated complexity of the migration:** 5
    *This level represents a simple migration requiring minimal code changes.*
    *Factors for estimation: Eventlet's configuration options are mostly used for specifying server settings, which could be easily replaced or adjusted without altering core functionality.*
  - **Files Analyzed:**
    - **File:** `oslo.config/cli/generator.rst`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file lists `eventlet_server` as a valid replacement group for the WSGI server, indicating its presence in configuration settings.*
    - **File:** `oslo.config/service.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* The file utilizes Eventlet's green threads to manage asynchronous operations related to service handling.*
    - **File:** `oslo.config/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses mock patches for Eventlet functions, including its spawn function, which indicates Eventlet's use in unit tests.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a crucial role in managing asynchronous operations and serving as a WSGI server.
    - **Potential Challenges:** Directly replacing Eventlet might introduce issues with server configuration or impact performance, necessitating careful evaluation and testing to ensure compatibility and functionality.
    - **Recommendations:** Perform thorough testing of alternative WSGI servers (e.g., Gunicorn) alongside Eventlet's configurations, assess compatibility and performance differences, and plan for staged rollouts to minimize disruptions during the migration process.

Occurrences Found:
https://opendev.org/openstack/oslo.config/src/branch/master/doc/source/cli/generator.rst#n277 : replacement_group: eventlet_server

Project: oslo.db
Based on the provided code snippets and documentation, it appears that `eventlet` cannot be monkey patched in certain libraries.

The issue seems to be related to the fact that `eventlet` is a third-party library that provides asynchronous I/O support for Python. Some libraries, such as `sqlalchemy`, use `eventlet` under the hood, but may not allow monkey patching of `eventlet` due to its internal workings or dependencies.

In particular, the `oslo.db` tests appear to be written in a way that relies on `eventlet` being properly imported and initialized before running the tests. The `monkey_patch()` call is used to modify the behavior of `eventlet`, but some tests skip this step if it's not available or if the `TEST_EVENTLET` environment variable is set to 0.

The documentation suggests that using PyMySQL with the concurrency library `eventlet` is discouraged due to compatibility issues. This might be related to the fact that `eventlet` cannot be monkey patched in certain libraries, which could lead to unexpected behavior or errors when trying to use it with incompatible libraries.

To resolve this issue, you can try the following:

1. Ensure that you're using a compatible version of `sqlalchemy` and `oslo.db` that work well with `eventlet`.
2. Check if there are any specific configuration options or environment variables that need to be set to enable `eventlet` support.
3. Consider using a different concurrency library, such as `concurrent.futures`, which might not have the same compatibility issues as `eventlet`.

If you're still experiencing issues, please provide more context or details about your specific use case, and I'll do my best to help you troubleshoot the problem.

Occurrences Found:
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/sqlalchemy/engines.py#n46 : If we use eventlet.monkey_patch(), eventlet.greenthread.sleep(0) will
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/sqlalchemy/engines.py#n50 : implemented by C libraries that eventlet cannot monkey patch.
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/__init__.py#n19 : def should_run_eventlet_tests():
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/__init__.py#n23 : if should_run_eventlet_tests():
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/__init__.py#n24 : import eventlet
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/__init__.py#n25 : eventlet.monkey_patch()
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n16 : """Unit tests for SQLAlchemy and eventlet interaction."""
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n38 : __tablename__ = 'test_async_eventlet'
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n49 : @unittest.skipIf(not tests.should_run_eventlet_tests(),
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n50 : 'eventlet tests disabled unless TEST_EVENTLET=1')
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n78 : eventlet = importutils.try_import('eventlet')
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n79 : if eventlet is None:
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n80 : return self.skipTest('eventlet is required for this test')
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n82 : a_ready = eventlet.event.Event()
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n83 : a_proceed = eventlet.event.Event()
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n84 : b_proceed = eventlet.event.Event()
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n88 : a = eventlet.spawn(operate_on_row, 'A',
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n95 : b = eventlet.spawn(operate_on_row, 'B',
https://opendev.org/openstack/oslo.db/src/branch/master/oslo_db/tests/sqlalchemy/test_async_eventlet.py#n98 : eventlet.sleep(1)  # should(?) advance B to blocking on transaction
https://opendev.org/openstack/oslo.db/src/branch/master/releasenotes/notes/warn-incomplete-url-c44cd03baf630c7c.yaml#n7 : concurrency library eventlet.
https://opendev.org/openstack/oslo.db/src/branch/master/releasenotes/notes/warn-incomplete-url-c44cd03baf630c7c.yaml#n12 : with the concurrency library eventlet. To use PyMySQL, ensure the
https://opendev.org/openstack/oslo.db/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n213 : "the best compatibility with the concurrency library eventlet. To use "
https://opendev.org/openstack/oslo.db/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n219 : "the best compatibility with the concurrency library eventlet. To use "
https://opendev.org/openstack/oslo.db/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n651 : "as MySQL, the default is incompatible with the concurrency library eventlet."
https://opendev.org/openstack/oslo.db/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n655 : "as MySQL, the default is incompatible with the concurrency library eventlet."
https://opendev.org/openstack/oslo.db/src/branch/master/test-requirements.txt#n3 : eventlet>=0.18.2 # MIT

Project: oslo.log
The code snippet provided appears to be a test suite for the `oslo_log` library, specifically testing the behavior of locks and events in asynchronous code using the `eventlet` framework.

Here's a breakdown of what each section of the code does:

1. **Creating threads**: The code creates two threads (`c1` and `c2`) that will execute the `coro1` and `coro2` coroutines, respectively.
2. **Spawning coroutines**: The `eventlet.spawn()` function is used to spawn the coroutines on separate threads.
3. **Logging events**: The code logs two events using `eventlet.event.Event()`: `pthread1_event` and `pthread2_event1`. These events are likely used to track the progress of the coroutines.
4. **Patching threading modules**: The code patches the `threading` module to restore its original behavior using `eventlet.patcher.original('threading')`.
5. **Creating threads with patched threading**: The code creates two new threads (`real_thread1` and `real_thread2`) that use the patched `threading` module.
6. **Logging additional events**: The code logs three more events: `pthread2_event2`, `eventlet.debug.hub_prevent_multiple_readers(True)`, and two logging messages indicating that bugs 1983863 and 1983863 have been fixed for logging in eventlet native threads.

The test suite appears to be ensuring that the `oslo_log` library correctly handles locks and events in asynchronous code, specifically when using eventlet. The patched threading modules are likely used to isolate the behavior of the coroutines from the rest of the application, allowing the tests to focus on the logging and event handling mechanisms.

Some potential issues with this code snippet include:

* The use of `eventlet.sleep(0)` to let a coroutine run is unconventional and may not be necessary in all cases. It's possible that the test suite is using it to ensure that the coroutines are executed in a specific order or timing.
* The patching of the `threading` module is likely being done for testing purposes only, and would not be necessary in production code.
* The logging messages indicating that bugs 1983863 have been fixed may be unnecessary and could potentially cause issues if they are not properly cleaned up.

Overall, this test suite appears to be a complex piece of code that is ensuring the correct behavior of the `oslo_log` library in asynchronous environments using eventlet.

Occurrences Found:
https://opendev.org/openstack/oslo.log/src/branch/master/doc/source/admin/example_nova.rst#n64 : Similarly, ``boto``, ``suds``, and ``eventlet.wsgi.server`` are
https://opendev.org/openstack/oslo.log/src/branch/master/doc/source/admin/nova_sample.conf#n50 : [logger_eventletwsgi]
https://opendev.org/openstack/oslo.log/src/branch/master/doc/source/admin/nova_sample.conf#n53 : qualname = eventlet.wsgi.server
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/log.py#n43 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/log.py#n271 : def _fix_eventlet_logging():
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/log.py#n272 : """Properly setup logging with eventlet on native threads.
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/log.py#n274 : Workaround for: https://github.com/eventlet/eventlet/issues/432
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/log.py#n280 : if eventletutils.is_monkey_patched('thread') and not _EVENTLET_FIX_APPLIED:
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/log.py#n281 : import eventlet.green.threading
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/log.py#n283 : logging.threading = eventlet.green.threading
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/log.py#n289 : def setup(conf, product_name, version='unknown', *, fix_eventlet=True):
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/log.py#n291 : if fix_eventlet:
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/log.py#n292 : _fix_eventlet_logging()
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/pipe_mutex.py#n20 : import eventlet
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/pipe_mutex.py#n21 : import eventlet.debug
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/pipe_mutex.py#n22 : import eventlet.greenthread
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/pipe_mutex.py#n23 : import eventlet.hubs
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/pipe_mutex.py#n32 : Related eventlet bug: https://github.com/eventlet/eventlet/issues/432
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/pipe_mutex.py#n60 : eventlet.debug.hub_prevent_multiple_readers(False)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/pipe_mutex.py#n72 : current_greenthread_id = id(eventlet.greenthread.getcurrent())
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/pipe_mutex.py#n95 : eventlet.hubs.trampoline(self.rfd, read=True)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/pipe_mutex.py#n99 : current_greenthread_id = id(eventlet.greenthread.getcurrent())
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n20 : import eventlet
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n21 : from eventlet import debug as eventlet_debug
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n22 : from eventlet import greenpool
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n28 : def quiet_eventlet_exceptions():
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n30 : eventlet_debug.hub_exceptions(False)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n34 : eventlet_debug.hub_exceptions(orig_state)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n46 : evt_lock1 = eventlet.event.Event()
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n47 : evt_lock2 = eventlet.event.Event()
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n48 : evt_unlock = eventlet.event.Event()
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n57 : eventlet.spawn(get_the_lock)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n72 : self.assertFalse(eventlet.spawn(try_acquire_lock).wait())
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n74 : self.assertFalse(eventlet.spawn(try_acquire_lock).wait())
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n76 : self.assertTrue(eventlet.spawn(try_acquire_lock).wait())
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n88 : with quiet_eventlet_exceptions():
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n90 : eventlet.spawn(self.mutex.release).wait)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n93 : evt = eventlet.event.Event()
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n98 : eventlet.sleep(0)  # let coro2 go
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n113 : c1 = eventlet.spawn(coro1)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n114 : c2 = eventlet.spawn(coro2)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n136 : eventlet.sleep(0.0001)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n141 : greenthread1 = eventlet.spawn(do_stuff)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n142 : greenthread2 = eventlet.spawn(do_stuff)
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n144 : real_thread1 = eventlet.patcher.original('threading').Thread(
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n148 : real_thread2 = eventlet.patcher.original('threading').Thread(
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n160 : pthread1_event = eventlet.patcher.original('threading').Event()
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n161 : pthread2_event1 = eventlet.patcher.original('threading').Event()
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n162 : pthread2_event2 = eventlet.patcher.original('threading').Event()
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n167 : thread_id.append(id(eventlet.greenthread.getcurrent()))
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n186 : thread_id.append(id(eventlet.greenthread.getcurrent()))
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n193 : real_thread1 = eventlet.patcher.original('threading').Thread(
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n197 : real_thread2 = eventlet.patcher.original('threading').Thread(
https://opendev.org/openstack/oslo.log/src/branch/master/oslo_log/tests/unit/test_pipe_mutex.py#n209 : eventlet.debug.hub_prevent_multiple_readers(True)
https://opendev.org/openstack/oslo.log/src/branch/master/releasenotes/notes/native-threads-logging-cc84f7288c4835a0.yaml#n6 : eventlet native threads.
https://opendev.org/openstack/oslo.log/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n276 : "log/+bug/1983863>`_: Fixed logging in eventlet native threads. This fix "
https://opendev.org/openstack/oslo.log/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n281 : "oslo.log/+bug/1983863>`_: Fixed logging in eventlet native threads. This fix "
https://opendev.org/openstack/oslo.log/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n320 : "logging in eventlet native threads."
https://opendev.org/openstack/oslo.log/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n323 : "logging in eventlet native threads."
https://opendev.org/openstack/oslo.log/src/branch/master/test-requirements.txt#n9 : eventlet>=0.30.1 # MIT

Project: oslo.messaging
The provided code snippet is from the `oslo_messaging` tests in OpenStack. It appears to be testing the behavior of a message broker with eventlet utilities.

Here's a breakdown of the code:

1. The first section creates several events using `eventletutils.Event()`, which are used to signal the completion of various tasks, such as stopping the server and waiting for a thread to finish.
2. A GreenThreadExecutor is created using `futurist:GreenThreadPoolExecutor`, which is an eventlet-compatible executor.
3. The `server` object is stopped in two different ways:
	* Using a single `eventlet.spawn(self.server.stop)` call, which schedules the stop operation on a separate thread.
	* Using a `thread = eventlet.spawn(self.server.stop, log_after=1)` call, which schedules the stop operation on a separate thread with a 1-second delay for logging.
4. The server is also stopped using a timeout of 2 seconds with `eventlet.sleep(2)`, which pauses the execution for 2 seconds before stopping the server.
5. A final event, `shutdown_called`, is created to signal when the shutdown process is complete.

The code uses eventlet utilities to manage threads and wait for events to complete. The tests are designed to verify that the message broker behaves correctly under different scenarios.

However, I notice that the `eventletutils.Event()` calls seem redundant, as they don't appear to be used anywhere in the provided code snippet. It's possible that these events were intended to be used elsewhere in the codebase, but they have been removed or replaced with other mechanisms.

To improve the code readability and maintainability, I would suggest:

* Removing the redundant `eventletutils.Event()` calls.
* Adding more comments to explain the purpose of each section of the code.
* Using more descriptive variable names to make it easier to understand what each variable represents.
* Considering using a different testing framework or library that provides better support for asynchronous programming and thread management.

Occurrences Found:
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/amqp1_driver/controller.py#n39 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/amqp1_driver/controller.py#n82 : self._wakeup = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/amqp1_driver/controller.py#n109 : self._wakeup = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/amqpdriver.py#n25 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/amqpdriver.py#n123 : self._shutdown = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/amqpdriver.py#n373 : self._shutdown = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/amqpdriver.py#n374 : self._shutoff = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/amqpdriver.py#n559 : self._thread_exit_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_fake.py#n22 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_fake.py#n53 : self._stopped = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_kafka.py#n21 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_kafka.py#n28 : if eventletutils.EVENTLET_AVAILABLE:
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_kafka.py#n29 : tpool = importutils.try_import('eventlet.tpool')
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_kafka.py#n220 : if eventletutils.is_monkey_patched('thread'):
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_kafka.py#n303 : if eventletutils.is_monkey_patched('thread'):
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_kafka.py#n376 : self._stopped = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_rabbit.py#n38 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_rabbit.py#n100 : 'using eventlet for core service framework.',
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_rabbit.py#n108 : "stdlib by using eventlet/greenlet then the heartbeat "
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/impl_rabbit.py#n675 : self._get_thread_id = eventletutils.fetch_current_thread_functor()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_drivers/pool.py#n30 : Modelled after the eventlet.pools.Pool interface, but designed to be safe
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_metrics/client.py#n24 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_metrics/client.py#n30 : eventlet = importutils.try_import('eventlet')
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_metrics/client.py#n31 : if eventlet and eventletutils.is_monkey_patched("thread"):
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_metrics/client.py#n34 : stdlib_threading = eventlet.patcher.original('threading')
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_utils.py#n20 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_utils.py#n25 : eventlet = importutils.try_import('eventlet')
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_utils.py#n26 : if eventlet and eventletutils.is_monkey_patched("thread"):
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_utils.py#n29 : stdlib_threading = eventlet.patcher.original('threading')
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_utils.py#n30 : stdlib_queue = eventlet.patcher.original('queue')
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_utils.py#n88 : if eventletutils.is_monkey_patched('thread'):
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_utils.py#n89 : LOG.debug("Threading is patched, using an eventlet executor")
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/_utils.py#n90 : return 'eventlet'
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/notify/listener.py#n76 : *Note:* If the "eventlet" executor is used, the threading and time library need
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/notify/listener.py#n227 : If the eventlet executor is used, the threading and time library need to be
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/notify/listener.py#n237 : 'eventlet' and 'threading'
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/notify/listener.py#n262 : If the eventlet executor is used, the threading and time library need to be
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/notify/listener.py#n272 : 'eventlet' and 'threading'
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/rpc/dispatcher.py#n26 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/rpc/dispatcher.py#n287 : completion_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/rpc/server.py#n52 : *Note:* If the "eventlet" executor is used, the threading and time library need
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/rpc/server.py#n79 : import eventlet
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/rpc/server.py#n80 : eventlet.monkey_patch()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/rpc/server.py#n110 : executor='eventlet')
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/rpc/server.py#n232 : 'eventlet' and 'threading'
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/server.py#n28 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/server.py#n55 : ' executor is threading or eventlet.'),
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/server.py#n318 : asynchronism then you need to consider to use the eventlet executor.
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/server.py#n326 : 'eventlet' and 'threading'
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/server.py#n329 : if executor and executor not in ("threading", "eventlet"):
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/server.py#n332 : "Executor should be None or 'eventlet' and 'threading'")
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/server.py#n342 : if self.executor_type == "eventlet":
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/server.py#n343 : eventletutils.warn_eventlet_not_patched(
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/server.py#n345 : what="the 'oslo.messaging eventlet executor'")
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/__init__.py#n16 : import eventlet
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/__init__.py#n17 : eventlet.monkey_patch()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/drivers/test_amqp_driver.py#n31 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/drivers/test_amqp_driver.py#n78 : self.started = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/drivers/test_amqp_driver.py#n79 : self._done = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/drivers/test_amqp_driver.py#n1155 : self._recovered = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/drivers/test_amqp_driver.py#n2139 : self._pause = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/drivers/test_impl_rabbit.py#n27 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/drivers/test_impl_rabbit.py#n56 : event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/functional/utils.py#n98 : executor='eventlet'):
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/functional/utils.py#n350 : [self], 'eventlet')
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/functional/utils.py#n413 : [self], 'eventlet',
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/notify/test_listener.py#n104 : allow_requeue=True, pool=pool, executor='eventlet',
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/notify/test_listener.py#n109 : allow_requeue=True, pool=pool, executor='eventlet')
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n20 : import eventlet
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n23 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n65 : self.stopped = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n143 : def test_constructor_with_eventlet_executor(self):
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n155 : executor='eventlet')
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n161 : self.assertEqual('eventlet', server.executor_type)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n688 : eventlet.spawn(self.server.start)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n700 : eventlet.spawn(self.server.wait)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n703 : eventlet.sleep(0)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n705 : eventlet.spawn(self.server.stop)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n706 : eventlet.sleep(0)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n708 : eventlet.spawn(self.server.start)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n719 : start_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n720 : finish_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n722 : running_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n723 : done_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n730 : _runner[0] = eventlet.getcurrent()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n741 : start1 = eventlet.spawn(self.server.start)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n742 : start2 = eventlet.spawn(self.server.start)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n749 : waiter_finished = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n798 : complete_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n799 : complete_waiting_callback = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n815 : thread1 = eventlet.spawn(self.server.stop)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n816 : thread1_finished = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n862 : log_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n866 : thread = eventlet.spawn(self.server.stop)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n878 : log_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n882 : thread = eventlet.spawn(self.server.stop, log_after=1)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n894 : log_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n898 : thread = eventlet.spawn(self.server.stop, log_after=1, timeout=2)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n919 : shutdown_called = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n924 : eventlet.sleep(10)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/rpc/test_server.py#n928 : thread = eventlet.spawn(self.server.wait)
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/utils.py#n26 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/utils.py#n68 : self._stop_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/oslo_messaging/tests/utils.py#n69 : self._start_event = eventletutils.Event()
https://opendev.org/openstack/oslo.messaging/src/branch/master/releasenotes/notes/deprecate-the-option-heartbeat_in_pthread-from-rabbit-driver-5757adb83701caa5.yaml#n8 : never worked with services using eventlet for core service framework.
https://opendev.org/openstack/oslo.messaging/src/branch/master/setup.cfg#n56 : eventlet = futurist:GreenThreadPoolExecutor
https://opendev.org/openstack/oslo.messaging/src/branch/master/test-requirements.txt#n24 : eventlet>=0.23.0 # MIT
https://opendev.org/openstack/oslo.messaging/src/branch/master/tools/simulator.py#n16 : import eventlet
https://opendev.org/openstack/oslo.messaging/src/branch/master/tools/simulator.py#n17 : eventlet.monkey_patch()
https://opendev.org/openstack/oslo.messaging/src/branch/master/tools/simulator.py#n270 : endpoints, executor='eventlet')
https://opendev.org/openstack/oslo.messaging/src/branch/master/tools/simulator.py#n302 : endpoints, executor='eventlet',
https://opendev.org/openstack/oslo.messaging/src/branch/master/tools/simulator.py#n537 : p = eventlet.GreenPool(size=threads)
https://opendev.org/openstack/oslo.messaging/src/branch/master/tools/simulator.py#n554 : p = eventlet.GreenPool(size=threads)
https://opendev.org/openstack/oslo.messaging/src/branch/master/tools/simulator.py#n576 : eventlet.sleep()
https://opendev.org/openstack/oslo.messaging/src/branch/master/tools/simulator.py#n583 : eventlet.sleep()
https://opendev.org/openstack/oslo.messaging/src/branch/master/tools/simulator.py#n742 : type=str, default='eventlet',

Project: oslo.privsep
The code snippet appears to be a part of the OpenStack `oslo_privsep` project, specifically in the tests related to the daemon functionality.

The issue is that the `eventlet` library has been monkey-patched by default in the `oslo_privsep` tests. However, this monkey-patching can cause issues with certain libraries being patched, which leads to timeouts when using eventlet threads.

To resolve this issue, the code checks if `eventlet.patcher.is_monkey_patched(eventlet_mod_name)` is `False` for specific modules and patches them using `eventlet.monkey_patch()` if necessary. This ensures that only the required libraries are patched, preventing any potential issues with monkey-patching.

The relevant lines of code are:
```python
for eventlet_mod_name in daemon.EVENTLET_MODULES):
    if not eventlet.patcher.is_monkey_patched(eventlet_mod_name):
        orig_mod = eventlet.patcher.original(name)
        # ... patching code ...
```
This code checks if the `eventlet` module is already patched, and if so, skips patching it. If it's not patched, it patches the original module using `eventlet.monkey_patch()`.

The issue is documented in the release notes for `oslo_privsep`, specifically in the "un-monkey-patch-privileged-daemon" note:
```markdown
The ``oslo.privsep`` client can be called from a program using eventlet. If ``eventlet.monkey_patch``, some libraries will be patched, for example
original values. The goal is to prevent some timeouts when using eventlet threads ...
```
This suggests that the monkey-patching of `eventlet` can cause issues with certain libraries being patched, leading to timeouts when using eventlet threads.

Occurrences Found:
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/daemon.py#n59 : import eventlet
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/daemon.py#n60 : from eventlet import patcher
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/daemon.py#n239 : if eventlet.patcher.is_monkey_patched('socket'):
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/daemon.py#n240 : return eventlet.greenio.GreenPipe(fd, *args, **kwargs)
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/daemon.py#n273 : for eventlet_mod_name, func_modules in EVENTLET_LIBRARIES:
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/daemon.py#n274 : if not eventlet.patcher.is_monkey_patched(eventlet_mod_name):
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/daemon.py#n279 : orig_mod = eventlet.patcher.original(name)
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/daemon.py#n554 : logging.setup(cfg.CONF, 'privsep', fix_eventlet=False)
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/tests/test_daemon.py#n16 : import eventlet
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/tests/test_daemon.py#n267 : eventlet.patcher.is_monkey_patched(eventlet_mod_name)
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/tests/test_daemon.py#n268 : for eventlet_mod_name in daemon.EVENTLET_MODULES))
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/tests/test_daemon.py#n270 : eventlet.monkey_patch()
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/tests/test_daemon.py#n272 : eventlet.patcher.is_monkey_patched(eventlet_mod_name)
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/tests/test_daemon.py#n273 : for eventlet_mod_name in daemon.EVENTLET_MODULES))
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/tests/test_daemon.py#n276 : for eventlet_mod_name, func_modules in daemon.EVENTLET_LIBRARIES:
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/tests/test_daemon.py#n277 : if not eventlet.patcher.is_monkey_patched(eventlet_mod_name):
https://opendev.org/openstack/oslo.privsep/src/branch/master/oslo_privsep/tests/test_daemon.py#n281 : orig_mod = eventlet.patcher.original(name)
https://opendev.org/openstack/oslo.privsep/src/branch/master/releasenotes/notes/un-monkey-patch-privileged-daemon-160e00296549df3d.yaml#n4 : The ``oslo.privsep`` client can be called from a program using eventlet.
https://opendev.org/openstack/oslo.privsep/src/branch/master/releasenotes/notes/un-monkey-patch-privileged-daemon-160e00296549df3d.yaml#n5 : If ``eventlet.monkey_patch``, some libraries will be patched, for example
https://opendev.org/openstack/oslo.privsep/src/branch/master/releasenotes/notes/un-monkey-patch-privileged-daemon-160e00296549df3d.yaml#n9 : original values. The goal is to prevent some timeouts when using eventlet
https://opendev.org/openstack/oslo.privsep/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n120 : "The ``oslo.privsep`` client can be called from a program using eventlet. If "
https://opendev.org/openstack/oslo.privsep/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n121 : "``eventlet.monkey_patch``, some libraries will be patched, for example "
https://opendev.org/openstack/oslo.privsep/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n125 : "values. The goal is to prevent some timeouts when using eventlet threads "
https://opendev.org/openstack/oslo.privsep/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n129 : "The ``oslo.privsep`` client can be called from a program using eventlet. If "
https://opendev.org/openstack/oslo.privsep/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n130 : "``eventlet.monkey_patch``, some libraries will be patched, for example "
https://opendev.org/openstack/oslo.privsep/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n134 : "values. The goal is to prevent some timeouts when using eventlet threads "
https://opendev.org/openstack/oslo.privsep/src/branch/master/requirements.txt#n6 : eventlet>=0.21.0 # MIT

Project: oslo.reports
---

- **Project:** oslo.reports
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `hub.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the hub.
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `report.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
    - **File:** `utils.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The file uses the `eventlet.wsgi` server, which is a dependency on Eventlet's WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

---

- **Project:** oslo.reports
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in unit tests for handling asynchronous operations, and its scheduling mechanism has a significant impact on the background operations.
    - **Potential Challenges:** Removing Eventlet from testing would require alternative solutions for handling asynchronous operations, which could introduce complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
https://opendev.org/openstack/oslo.reports/src/branch/master/doc/source/user/report.txt#n18 : /usr/local/lib/python2.7/dist-packages/eventlet/hubs/hub.py:346 in run
https://opendev.org/openstack/oslo.reports/src/branch/master/doc/source/user/report.txt#n21 : /usr/local/lib/python2.7/dist-packages/eventlet/hubs/poll.py:82 in wait
https://opendev.org/openstack/oslo.reports/src/branch/master/doc/source/user/report.txt#n39 : `eventlet.greenthread.sleep(self.wait_interval)`
https://opendev.org/openstack/oslo.reports/src/branch/master/doc/source/user/report.txt#n41 : /usr/local/lib/python2.7/dist-packages/eventlet/greenthread.py:34 in sleep
https://opendev.org/openstack/oslo.reports/src/branch/master/doc/source/user/report.txt#n44 : /usr/local/lib/python2.7/dist-packages/eventlet/hubs/hub.py:294 in switch

Project: oslo.rootwrap
- **Project:** oslo.rootwrap
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which could introduce complexity in replacing alternative asynchronous libraries or adjusting configuration management.
  - **Analysis:**
    *Eventlet is used extensively across oslo.rootwrap, particularly for managing asynchronous operations using green threads (`eventlet.green.subprocess`) and scheduling deferred tasks.
    *The project's tests often interact with Eventlet, including testing its compatibility (`test_functional_eventlet.py`) and handling `Timeout` exceptions.
    *Eventlet is required by the project to function correctly, indicating that it cannot be easily replaced or removed without significant refactoring.
  - **Recommendations:**
    *Carefully evaluate alternative asynchronous libraries (e.g., asyncio) to determine their suitability for replacing Eventlet.
    *Plan for incremental refactoring of the project to ensure a smooth transition from Eventlet to an alternative library, if necessary.
    *Ensure thorough testing at each stage to maintain system stability and prevent introducing new bugs.

- **Additional Insights:**
    *The presence of `eventlet.patcher.is_monkey_patched('socket')` suggests that the project is trying to protect against eventlet thread switching, indicating a potential need for careful consideration when migrating from Eventlet.
    *The `stestr run --slowest (?!tests.test_functional_eventlet)tests {posargs}` line in `tox.ini` indicates that the project uses slow tests, including those related to Eventlet. This could impact the migration process and require additional testing resources.

- **Related Files/Directories:**
    *`oslo.rootwrap/__init__.py`: Importing Eventlet patches and demonstrating its usage.
    *`tests/test_functional.py`: Testing Eventlet compatibility with the standard library.
    *`tests/test_functional_eventlet.py`: Testing Eventlet's behavior in various scenarios, including thread switching and `Timeout` exceptions.
    *`tox.ini`: Managing test environments for the project, including support for Eventlet.

- **Potential Roadmap:**
    *Short-term (1-2 weeks): Evaluate alternative asynchronous libraries, plan for incremental refactoring, and ensure thorough testing of existing tests to maintain system stability.
    *Mid-term (4-6 weeks): Implement changes and refactor codebase according to the planned approach.
    *Long-term (8-12 weeks+): Verify that all tests pass and the project is stable with the new asynchronous library.

Occurrences Found:
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/__init__.py#n19 : import eventlet.patcher
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/__init__.py#n24 : _patched_socket = (eventlet.patcher.is_monkey_patched('socket') or
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/__init__.py#n30 : from eventlet.green import subprocess   # noqa
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional.py#n29 : import eventlet
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional.py#n31 : eventlet = None
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional.py#n153 : if eventlet and eventlet.patcher.is_monkey_patched('socket'):
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional.py#n154 : self.fail("Standard library should not be patched by eventlet"
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional_eventlet.py#n19 : import eventlet
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional_eventlet.py#n20 : eventlet.monkey_patch()
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional_eventlet.py#n36 : with eventlet.Timeout(timeout):
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional_eventlet.py#n39 : except eventlet.Timeout:
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional_eventlet.py#n42 : def test_eventlet_threads(self):
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional_eventlet.py#n43 : """Check eventlet compatibility.
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional_eventlet.py#n45 : The multiprocessing module is not eventlet friendly and
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional_eventlet.py#n46 : must be protected against eventlet thread switching and its
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional_eventlet.py#n53 : eventlet.spawn(self._thread_worker, i % 3, 'abc%d' % i))
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/oslo_rootwrap/tests/test_functional_eventlet.py#n56 : eventlet.spawn(self._thread_worker_timeout, 2,
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/test-requirements.txt#n11 : eventlet>=0.18.2 # MIT
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/tox.ini#n14 : stestr run --slowest (?!tests.test_functional_eventlet)tests {posargs}
https://opendev.org/openstack/oslo.rootwrap/src/branch/master/tox.ini#n15 : env TEST_EVENTLET=1 stestr run --slowest tests.test_functional_eventlet

Project: oslo.service
The code snippet you provided appears to be a test file for an OpenStack service that uses the `eventlet` library for asynchronous I/O. The tests cover various aspects of the service, including its WSGI interface, threadgroup functionality, and SSL/TLS support.

Here's a breakdown of the code:

1. The first section imports the necessary modules and sets up the test environment. It creates a `test_wsgi.py` file with several test functions.
2. The `eventlet.monkey_patch()` function is used to monkey patch the `eventlet` library to disable certain features, such as greenlet patches and thread Patches.
3. The `wsgi` module imports the necessary classes and functions for setting up a WSGI server using `eventlet`.
4. The `test_wsgi` class contains several test methods that cover different aspects of the service:
	* `test_start_server()`: Tests that the `start_server()` method starts the WSGI server correctly.
	* `test_stop_server()`: Tests that the `stop_server()` method stops the WSGI server correctly.
	* `test_ssl_support()`: Tests that the service supports SSL/TLS connections correctly.
5. The `threadgroup` module imports the necessary classes and functions for setting up a thread group using `eventlet`.
6. The `wsgi.py` file contains the implementation of the WSGI server, which uses `eventlet` to manage threads and handle I/O operations.

Some potential issues with this code include:

* The use of `monkey_patch()` can have unintended consequences if not used carefully.
* The tests may not cover all possible scenarios, such as edge cases or concurrent access.
* The code assumes that the `eventlet` library is installed and available; if it's not, the tests will fail.

To write this code from scratch, you would need to:

1. Import the necessary modules and set up the test environment.
2. Use `monkey_patch()` to disable any features that are not needed for the tests.
3. Implement the WSGI server using `eventlet` and test its functionality.
4. Write test methods to cover different aspects of the service.

Here is a simple example of how you might write this code:
```python
import os
import eventlet

class TestWsgi:
    def test_start_server(self):
        # Create a WSGI server instance
        wsgi_server = eventlet.wsgi.WSGIServer(('localhost', 8080), self.app)
        
        # Start the server
        eventlet.spawn(wsgi_server.start)

    def test_stop_server(self):
        # Create a WSGI server instance
        wsgi_server = eventlet.wsgi.WSGIServer(('localhost', 8080), self.app)
        
        # Stop the server
        eventlet.spawn(wsgi_server.stop)

    def test_ssl_support(self):
        # Set up SSL/TLS configuration
        ssl_config = {'ca_certs': 'path/to/ca/certs'}
        
        # Create a WSGI server instance with SSL/TLS support
        wsgi_server = eventlet.wsgi.WSGIServer(('localhost', 8080), self.app, ssl_options=ssl_config)
        
        # Start the server with SSL/TLS support
        eventlet.spawn(wsgi_server.start)

if __name__ == '__main__':
    test_wsgi = TestWsgi()
    test_wsgi.test_start_server()
```
This example provides a basic structure for testing a WSGI server using `eventlet`, but you would need to add more functionality and tests to make it comprehensive.

Occurrences Found:
https://opendev.org/openstack/oslo.service/src/branch/master/doc/source/reference/eventlet_backdoor.rst#n2 : eventlet_backdoor
https://opendev.org/openstack/oslo.service/src/branch/master/doc/source/reference/eventlet_backdoor.rst#n5 : .. automodule:: oslo_service.eventlet_backdoor
https://opendev.org/openstack/oslo.service/src/branch/master/doc/source/reference/index.rst#n8 : eventlet_backdoor
https://opendev.org/openstack/oslo.service/src/branch/master/doc/source/user/usage.rst#n17 : * :func:`~oslo_service.eventlet_backdoor.initialize_if_enabled`
https://opendev.org/openstack/oslo.service/src/branch/master/doc/source/user/usage.rst#n53 : :mod:`~oslo_service.eventlet_backdoor` modules for the ``[DEFAULT]``
https://opendev.org/openstack/oslo.service/src/branch/master/doc/source/user/usage.rst#n206 : calltrace) through the :mod:`~oslo_service.eventlet_backdoor` module. The
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/_options.py#n25 : eventlet_backdoor_opts = [
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/_options.py#n27 : help="Enable eventlet backdoor.  %s" % help_for_backdoor_port),
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/_options.py#n29 : help="Enable eventlet backdoor, using the provided path"
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n26 : import eventlet.backdoor
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n30 : from eventlet.green import socket
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n154 : return eventlet.listen((host, port), reuse_port=False)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n176 : return eventlet.listen(socket_path, socket.AF_UNIX)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n192 : return eventlet.listen(socket_path, socket.AF_UNIX)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n196 : conf.register_opts(_options.eventlet_backdoor_opts)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n220 : LOG.warning("Could not apply format string to eventlet "
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n242 : thread = eventlet.spawn(eventlet.backdoor.backdoor_server, sock,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n257 : import eventlet
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n258 : eventlet.monkey_patch(all=True)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/eventlet_backdoor.py#n271 : conf.register_cli_opts(_options.eventlet_backdoor_opts)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/fixture.py#n52 : 'oslo_utils.eventletutils.EventletEvent.wait')).mock
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/loopingcall.py#n23 : from eventlet import event
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/loopingcall.py#n24 : from eventlet import greenthread
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/loopingcall.py#n26 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/loopingcall.py#n91 : self._abort = eventletutils.EventletEvent()
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/loopingcall.py#n121 : :returns: eventlet event instance
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n35 : import eventlet
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n36 : from eventlet import event
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n37 : from eventlet import tpool
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n42 : from oslo_service import eventlet_backdoor
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n54 : return [(None, copy.deepcopy(_options.eventlet_backdoor_opts +
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n178 : eventlet.spawn(self._handle_signal_cb, signo, frame)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n202 : interrupting eventlet's call to poll() or sleep().
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n204 : select_module = eventlet.patcher.original('select')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n209 : from eventlet.hubs import poll as poll_hub
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n219 : time_sleep = eventlet.patcher.original('time').sleep
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n236 : hub = eventlet.hubs.get_hub()
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n263 : eventlet_backdoor.initialize_if_enabled(self.conf))
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n427 : self.readpipe = eventlet.greenio.GreenPipe(rfd, 'r')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n534 : eventlet.hubs.use_hub()
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n539 : eventlet.spawn_n(self._pipe_watcher)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n643 : eventlet.greenthread.sleep(self.wait_interval)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/service.py#n684 : except eventlet.greenlet.GreenletExit:
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/__init__.py#n15 : import eventlet
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/__init__.py#n25 : eventlet.monkey_patch(os=False, thread=False)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/__init__.py#n27 : eventlet.monkey_patch()
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/base.py#n28 : self.conf_fixture.register_opts(_options.eventlet_backdoor_opts)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/eventlet_service.py#n22 : import eventlet.wsgi
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/eventlet_service.py#n41 : self.pool = eventlet.GreenPool(POOL_SIZE)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/eventlet_service.py#n68 : self.socket = eventlet.listen(info[-1], family=info[0],
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/eventlet_service.py#n120 : eventlet.wsgi.server(socket, application, debug=False)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/eventlet_service.py#n127 : eventlet.patcher.monkey_patch()
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n17 : """Unit Tests for eventlet backdoor."""
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n23 : import eventlet
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n25 : from oslo_service import eventlet_backdoor
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n31 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n32 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n36 : path = eventlet_backdoor.initialize_if_enabled(self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n39 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n40 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n44 : path = eventlet_backdoor.initialize_if_enabled(self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n48 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n49 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n60 : path = eventlet_backdoor.initialize_if_enabled(self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n64 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n65 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n71 : path = eventlet_backdoor.initialize_if_enabled(self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n76 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n77 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n84 : path = eventlet_backdoor.initialize_if_enabled(self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n89 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n90 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n97 : self.assertRaises(OSError, eventlet_backdoor.initialize_if_enabled,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n100 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n101 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n106 : eventlet_backdoor.initialize_if_enabled,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n112 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n113 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n119 : port = eventlet_backdoor.initialize_if_enabled(self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n122 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n123 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n128 : eventlet_backdoor.initialize_if_enabled, self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n130 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n133 : port = eventlet_backdoor.initialize_if_enabled(self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n135 : port = eventlet_backdoor.initialize_if_enabled(self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n138 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n139 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n145 : port = eventlet_backdoor.initialize_if_enabled(self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n148 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n149 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n155 : port = eventlet_backdoor.initialize_if_enabled(self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n158 : @mock.patch.object(eventlet, 'spawn')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n159 : @mock.patch.object(eventlet, 'listen')
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n167 : eventlet_backdoor.initialize_if_enabled, self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n171 : self.assertRaises(eventlet_backdoor.EventletBackdoorConfigValueError,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n172 : eventlet_backdoor.initialize_if_enabled, self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n176 : self.assertRaises(eventlet_backdoor.EventletBackdoorConfigValueError,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_eventlet_backdoor.py#n177 : eventlet_backdoor.initialize_if_enabled, self.conf)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_loopingcall.py#n17 : import eventlet
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_loopingcall.py#n18 : from eventlet.green import threading as greenthreading
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_loopingcall.py#n40 : clock = eventlet.hubs.get_hub().clock
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_loopingcall.py#n47 : def test_eventlet_clock(self):
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_loopingcall.py#n50 : hub = eventlet.hubs.get_hub()
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_loopingcall.py#n174 : clock = eventlet.hubs.get_hub().clock
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n28 : import eventlet
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n29 : from eventlet import event
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n34 : from oslo_service.tests import eventlet_service
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n403 : @mock.patch("oslo_service.eventlet_backdoor.initialize_if_enabled")
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n414 : @mock.patch("oslo_service.eventlet_backdoor.initialize_if_enabled")
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n484 : with mock.patch('eventlet.patcher.original',
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n523 : eventlet.greenlet.GreenletExit()]
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n537 : @mock.patch("eventlet.greenio.GreenPipe")
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n548 : @mock.patch("eventlet.greenio.GreenPipe")
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n563 : @mock.patch("eventlet.greenio.GreenPipe")
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n614 : return eventlet.timeout.with_timeout(time_to_wait, wait_for_task,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_service.py#n641 : proc = multiprocessing.Process(target=eventlet_service.run,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_threadgroup.py#n22 : from eventlet import event
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_wsgi.py#n26 : import eventlet
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_wsgi.py#n27 : import eventlet.wsgi
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_wsgi.py#n130 : self.assertEqual(eventlet.wsgi.MAX_HEADER_LINE,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_wsgi.py#n203 : eventlet.monkey_patch(os=False, thread=False)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_wsgi.py#n211 : eventlet.sleep(0)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_wsgi.py#n217 : eventlet.sleep(0)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_wsgi.py#n242 : with mock.patch.object(eventlet,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_wsgi.py#n257 : with mock.patch.object(eventlet,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_wsgi.py#n274 : with eventlet.wrap_ssl(sock, ca_certs=ca_certs) as wrappedSocket:
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/tests/test_wsgi.py#n293 : eventlet.monkey_patch(os=False, thread=False)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/threadgroup.py#n20 : import eventlet
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/threadgroup.py#n21 : from eventlet import greenpool
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/threadgroup.py#n269 : except eventlet.greenlet.GreenletExit:  # nosec
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/threadgroup.py#n334 : except eventlet.greenlet.GreenletExit:  # nosec
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/threadgroup.py#n426 : eventlet.sleep(wait_time)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n24 : import eventlet
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n25 : import eventlet.wsgi
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n65 : pool_size=None, protocol=eventlet.wsgi.HttpProtocol,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n67 : logger_name='eventlet.wsgi.server',
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n76 : :param pool_size: Maximum number of eventlets to spawn concurrently.
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n96 : eventlet.wsgi.MAX_HEADER_LINE = conf.max_header_line
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n102 : self._pool = eventlet.GreenPool(self.pool_size)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n141 : sock = eventlet.listen(bind_addr, family, backlog=backlog)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n152 : sock = eventlet.listen(socket_file, family=socket.AF_UNIX,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n176 : 'func': eventlet.wsgi.server,
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n191 : self._server = eventlet.spawn(**wsgi_kwargs)
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n215 : """Stops eventlet server. Doesn't allow accept new connecting.
https://opendev.org/openstack/oslo.service/src/branch/master/oslo_service/wsgi.py#n230 : Waits on the server's eventlet to finish, then returns.
https://opendev.org/openstack/oslo.service/src/branch/master/releasenotes/notes/native-threads-on-child-7150690c7caa1013.yaml#n5 : <https://bugs.launchpad.net/oslo.service/+bug/1983949>`_: Fixed eventlet
https://opendev.org/openstack/oslo.service/src/branch/master/releasenotes/notes/support-pid-in-eventlet-backdoor-socket-path-1863eaad1dd08556.yaml#n6 : process. This makes the eventlet backdoor accessible when spawning
https://opendev.org/openstack/oslo.service/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n177 : "makes the eventlet backdoor accessible when spawning multiple processes with "
https://opendev.org/openstack/oslo.service/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n182 : "makes the eventlet backdoor accessible when spawning multiple processes with "
https://opendev.org/openstack/oslo.service/src/branch/master/requirements.txt#n3 : eventlet>=0.25.2 # MIT

Project: oslo.utils
The code snippet provided appears to be a test suite for the `eventletutils` module in the OpenStack project. The tests cover various scenarios related to eventlet, including patching, timeout, and sleep functionality.

Here is a breakdown of the code:

**Test setup**

1. The test starts by importing the necessary modules and setting up mocks for certain functions using `@mock.patch`.
2. The `test_event_api_compat` function is defined, which tests the compatibility of eventlet with other APIs.
3. Another test function, `test_eventletutils`, is defined, which tests various scenarios related to eventlet.

**Test cases**

1. `eventletutils.warn_eventlet_not_patched(['os'])`: Tests if a warning is raised when eventlet is not patched for the 'os' library.
2. `eventletutils.warn_eventlet_not_patched(['os', 'thread'])`: Tests if a warning is raised when eventlet is not patched for both 'os' and 'thread' libraries.
3. `eventletutils.warn_eventlet_not_patched(['all'])`: Tests if a warning is raised when eventlet is not patched for all libraries.
4. `@mock.patch('oslo_utils.eventletutils._eventlet')`: Mocks the `_eventlet` function from `eventletutils`.
5. `eventlet.timeout.Timeout(0.5, False)`: Creates a timeout with a duration of 0.5 seconds and a flag to indicate that it is not a blocking call.
6. `eventlet.sleep(0.1)`: Puts the test thread to sleep for 0.1 seconds.
7. `@mock.patch('oslo_utils.eventletutils._eventlet.event.Event')`: Mocks the `_eventlet.event` object from `eventletutils`.

**Test logic**

1. The tests create instances of `Event` and `EventletEvent` using `eventletutils.Event()` and `eventletutils.EventletEvent()`, respectively.
2. The tests use `mock.patch` to isolate the behavior of certain functions and verify that the expected output is produced.
3. The tests use `eventlet.timeout.Timeout` and `eventlet.sleep` to create timeouts and sleep for a specified duration, respectively.

**Test assertions**

1. The tests use `self.assertIsInstance` to verify that an instance is of a specific type (e.g., `EventletEvent`).
2. The tests use `self.assertEqual` to verify that the output of a function or method matches an expected value.
3. The tests use `self.assertRaises` to verify that a certain exception is raised.

Overall, this code snippet appears to be a comprehensive test suite for the `eventletutils` module in OpenStack, covering various scenarios related to eventlet functionality.

Occurrences Found:
https://opendev.org/openstack/oslo.utils/src/branch/master/doc/source/reference/eventletutils.rst#n2 : eventletutils
https://opendev.org/openstack/oslo.utils/src/branch/master/doc/source/reference/eventletutils.rst#n5 : .. automodule:: oslo_utils.eventletutils
https://opendev.org/openstack/oslo.utils/src/branch/master/doc/source/reference/index.rst#n10 : eventletutils
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n31 : _eventlet = importutils.try_import('eventlet')
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n32 : _patcher = importutils.try_import('eventlet.patcher')
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n37 : EVENTLET_AVAILABLE = all((_eventlet, _patcher))
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n48 : If eventlet is used to monkey-patch the threading module, return the
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n49 : current eventlet greenthread. Otherwise, return the current Python thread.
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n64 : return _eventlet.getcurrent
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n69 : def warn_eventlet_not_patched(expected_patched_modules=None,
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n71 : """Warns if eventlet is being used without patching provided modules.
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n76 : to the names passed into the eventlet
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n126 : warnings.warn("It is highly recommended that when eventlet"
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n135 : """Determines safely is eventlet patching for module enabled or not
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n146 : """A class that provides consistent eventlet/threading Event API.
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n148 : This wraps the eventlet.event.Event class to have the same API as
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n158 : self._event = _eventlet.event.Event()
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/eventletutils.py#n174 : with _eventlet.timeout.Timeout(sw.leftover(return_none=True),
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/excutils.py#n150 : can happen when eventlet switches greenthreads or when running an
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n19 : import eventlet
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n20 : from eventlet import greenthread
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n23 : from oslo_utils import eventletutils
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n29 : self._old_avail = eventletutils.EVENTLET_AVAILABLE
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n30 : eventletutils.EVENTLET_AVAILABLE = True
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n34 : eventletutils.EVENTLET_AVAILABLE = self._old_avail
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n36 : @mock.patch("oslo_utils.eventletutils._patcher")
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n42 : eventletutils.warn_eventlet_not_patched(['os'])
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n48 : @mock.patch("oslo_utils.eventletutils._patcher")
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n54 : eventletutils.warn_eventlet_not_patched()
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n58 : for m in eventletutils._ALL_PATCH:
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n61 : @mock.patch("oslo_utils.eventletutils._patcher")
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n67 : eventletutils.warn_eventlet_not_patched(['all'])
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n71 : for m in eventletutils._ALL_PATCH:
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n74 : @mock.patch("oslo_utils.eventletutils._patcher")
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n80 : eventletutils.warn_eventlet_not_patched(['os'])
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n83 : @mock.patch("oslo_utils.eventletutils._patcher")
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n84 : def test_eventlet_is_patched(self, mock_patcher):
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n86 : self.assertTrue(eventletutils.is_monkey_patched('os'))
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n88 : self.assertFalse(eventletutils.is_monkey_patched('os'))
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n90 : @mock.patch("oslo_utils.eventletutils._patcher", None)
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n91 : def test_eventlet_no_patcher(self):
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n92 : self.assertFalse(eventletutils.is_monkey_patched('os'))
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n94 : @mock.patch("oslo_utils.eventletutils._patcher")
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n101 : eventletutils.warn_eventlet_not_patched(['os'])
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n106 : eventletutils.warn_eventlet_not_patched(['os'])
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n111 : eventletutils.warn_eventlet_not_patched(['os', 'thread'])
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n115 : eventletutils.warn_eventlet_not_patched(['all'])
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n124 : eventletutils.warn_eventlet_not_patched,
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n127 : @mock.patch('oslo_utils.eventletutils._eventlet')
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n128 : def test_event_api_compat(self, mock_eventlet):
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n129 : with mock.patch('oslo_utils.eventletutils.is_monkey_patched',
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n131 : e_event = eventletutils.Event()
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n132 : self.assertIsInstance(e_event, eventletutils.EventletEvent)
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n134 : t_event = eventletutils.Event()
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n152 : event = eventletutils.EventletEvent()
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n159 : with eventlet.timeout.Timeout(0.5, False):
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n164 : event = eventletutils.EventletEvent()
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n172 : eventlet.sleep(0.1)
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n178 : with eventlet.timeout.Timeout(0.5):
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n182 : event = eventletutils.EventletEvent()
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n190 : eventlet.sleep(0.1)
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n192 : eventlet.sleep(0.1)
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n197 : with eventlet.timeout.Timeout(0.7):
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n201 : event = eventletutils.EventletEvent()
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n215 : eventlet.sleep(0)  # start threads
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n218 : with eventlet.timeout.Timeout(0.3):
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n224 : @mock.patch('oslo_utils.eventletutils._eventlet.event.Event')
https://opendev.org/openstack/oslo.utils/src/branch/master/oslo_utils/tests/test_eventletutils.py#n229 : event = eventletutils.EventletEvent()
https://opendev.org/openstack/oslo.utils/src/branch/master/test-requirements.txt#n1 : eventlet>=0.18.2 # MIT

Project: oslo.versionedobjects
---

- **Project:** oslo.versionedobjects
  - **Is Eventlet globally deactivable for this project:** Yes
    - *Reason: The presence of an Eventlet-specific argparse option and the use of `eventlet.monkey_patch` indicate that Eventlet is designed to be disabled or customized.*
  - **Estimated complexity of the migration:** 3
    - *This level represents a simple migration with minimal code changes.*
    - *Factors for estimation: The test file only uses Eventlet in a monkey patch, allowing for a straightforward replacement with an equivalent asyncio feature.*
  - **Files Analyzed:**
    - **File:** `test.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** Uses Eventlet's monkey patch to avoid its behavior in this test, indicating a deliberate choice to use it for testing purposes only.
  - **Files Analyzed (continued):**
    - **File:** `utils.py` and `base.py` are not directly related to Eventlet usage as described in the task. 
      *No specific pattern identified.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet's use is limited to a single test file that uses it for mocking purposes, indicating an intentional choice rather than a dependency across the entire project.
    - **Potential Challenges:** Replacing the monkey patch with asyncio features will be straightforward as only one file utilizes this functionality, minimizing potential complexity changes.
    - **Recommendations:** Replace `eventlet.monkey_patch(os=False)` with equivalent asyncio implementation in `test.py`, ensure compatibility and thorough testing to validate functionality.

Occurrences Found:
https://opendev.org/openstack/oslo.versionedobjects/src/branch/master/oslo_versionedobjects/test.py#n23 : import eventlet  # noqa
https://opendev.org/openstack/oslo.versionedobjects/src/branch/master/oslo_versionedobjects/test.py#n24 : eventlet.monkey_patch(os=False)  # noqa

Project: oslo.vmware
---

- **Project:** oslo.vmware
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, managing WSGI servers is critical in oslo.vmware.*
  - **Files Analyzed:**
    - **File:** `common/loopingcall.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the looping call functionality.
    - **File:** `image_transfer.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `tests/test_api.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The line `eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT` indicates that Eventlet is a required dependency for the project.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across oslo.vmware, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Managing WSGI servers would also be critical to the functionality of the project.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
https://opendev.org/openstack/oslo.vmware/src/branch/master/oslo_vmware/common/loopingcall.py#n21 : from eventlet import event
https://opendev.org/openstack/oslo.vmware/src/branch/master/oslo_vmware/common/loopingcall.py#n22 : from eventlet import greenthread
https://opendev.org/openstack/oslo.vmware/src/branch/master/oslo_vmware/image_transfer.py#n23 : from eventlet import timeout
https://opendev.org/openstack/oslo.vmware/src/branch/master/oslo_vmware/tests/test_api.py#n22 : from eventlet import greenthread
https://opendev.org/openstack/oslo.vmware/src/branch/master/requirements.txt#n19 : eventlet!=0.18.3,!=0.20.1,>=0.18.2 # MIT

Project: taskflow
It appears that the code snippet from `taskflow/utils/eventlet_utils.py` has a dependency on `eventlet`, which is not included in this snippet.

To fix this issue, you would need to ensure that the `eventlet` library is installed and imported correctly in your Python environment. Here are some steps you can follow:

1. **Install eventlet**: Run the following command in your terminal or command prompt: `pip install eventlet`
2. **Import eventlet correctly**: In your Python code, import `eventlet` using the correct syntax:
```python
import eventlet as _eventlet
```
Alternatively, you can use the alias `eu` to refer to `eventlet` throughout your code:
```python
from taskflow.utils import eventlet_utils as eu
```
3. **Verify eventlet installation**: Run the following command in your terminal or command prompt: `python -c "import eventlet; print(eventlet.__version__)"`

This should output the version of `eventlet` installed in your environment.

If you're still facing issues, ensure that:

* You have installed `eventlet` using pip.
* The `eventlet` library is imported correctly in your Python code.
* The `taskflow` project has a working dependency on `eventlet`.

Please let me know if you have any further questions or if there's anything else I can help with!

Occurrences Found:
https://opendev.org/openstack/taskflow/src/branch/master/README.rst#n45 : you want to use the feature in question (`eventlet`_ or the worker based engine
https://opendev.org/openstack/taskflow/src/branch/master/README.rst#n72 : .. _eventlet: http://eventlet.net/
https://opendev.org/openstack/taskflow/src/branch/master/doc/source/user/engines.rst#n154 : If eventlet is used then this engine will not block other threads
https://opendev.org/openstack/taskflow/src/branch/master/doc/source/user/engines.rst#n155 : from running as eventlet automatically creates a implicit co-routine
https://opendev.org/openstack/taskflow/src/branch/master/doc/source/user/engines.rst#n157 : `eventlet <http://eventlet.net/>`_ and
https://opendev.org/openstack/taskflow/src/branch/master/doc/source/user/jobs.rst#n269 : when your program uses eventlet and you want to instruct kazoo to use an
https://opendev.org/openstack/taskflow/src/branch/master/doc/source/user/jobs.rst#n270 : eventlet compatible handler.
https://opendev.org/openstack/taskflow/src/branch/master/doc/source/user/utils.rst#n24 : .. automodule:: taskflow.utils.eventlet_utils
https://opendev.org/openstack/taskflow/src/branch/master/setup.cfg#n71 : eventlet =
https://opendev.org/openstack/taskflow/src/branch/master/setup.cfg#n72 : eventlet>=0.18.2 # MIT
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/conductors/backends/impl_executor.py#n90 : an eventlet *green* event works better for the conductor user)."""
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/engines/__init__.py#n17 : from oslo_utils import eventletutils as _eventletutils
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/engines/__init__.py#n22 : _eventletutils.warn_eventlet_not_patched(
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/engines/action_engine/executor.py#n226 : is more of a guess/somewhat arbitrary, but it does match what the eventlet
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/engines/action_engine/executor.py#n227 : greenpool default size is (so at least it's consistent with what eventlet
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/examples/hello_world.py#n83 : import eventlet as _eventlet  # noqa
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/examples/hello_world.py#n88 : print("-- Running in parallel using eventlet --")
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/examples/jobboard_produce_consume_colors.py#n154 : from taskflow.utils import eventlet_utils as _eu  # noqa
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/examples/jobboard_produce_consume_colors.py#n156 : import eventlet as _eventlet  # noqa
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/persistence/backends/impl_sqlalchemy.py#n34 : from taskflow.utils import eventlet_utils
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/persistence/backends/impl_sqlalchemy.py#n139 : If we use eventlet.monkey_patch(), eventlet.greenthread.sleep(0) will
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/persistence/backends/impl_sqlalchemy.py#n144 : implemented by C libraries that eventlet cannot monkey patch.
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/persistence/backends/impl_sqlalchemy.py#n309 : eventlet_utils.EVENTLET_AVAILABLE)
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/tests/unit/action_engine/test_creation.py#n26 : from taskflow.utils import eventlet_utils as eu
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/tests/unit/action_engine/test_creation.py#n71 : @testtools.skipIf(not eu.EVENTLET_AVAILABLE, 'eventlet is not available')
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/tests/unit/test_arguments_passing.py#n24 : from taskflow.utils import eventlet_utils as eu
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/tests/unit/test_arguments_passing.py#n215 : @testtools.skipIf(not eu.EVENTLET_AVAILABLE, 'eventlet is not available')
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/tests/unit/test_engines.py#n40 : from taskflow.utils import eventlet_utils as eu
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/tests/unit/test_engines.py#n1475 : @testtools.skipIf(not eu.EVENTLET_AVAILABLE, 'eventlet is not available')
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/tests/unit/test_retries.py#n31 : from taskflow.utils import eventlet_utils as eu
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/tests/unit/test_retries.py#n1358 : @testtools.skipIf(not eu.EVENTLET_AVAILABLE, 'eventlet is not available')
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/tests/unit/test_suspend.py#n26 : from taskflow.utils import eventlet_utils as eu
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/tests/unit/test_suspend.py#n220 : @testtools.skipIf(not eu.EVENTLET_AVAILABLE, 'eventlet is not available')
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/utils/eventlet_utils.py#n19 : _eventlet = importutils.try_import('eventlet')
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/utils/eventlet_utils.py#n21 : EVENTLET_AVAILABLE = bool(_eventlet)
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/utils/eventlet_utils.py#n24 : def check_for_eventlet(exc=None):
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/utils/eventlet_utils.py#n25 : """Check if eventlet is available and if not raise a runtime error.
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/utils/kazoo_utils.py#n185 : based, but `gevent`_, or `eventlet`_ ones can be provided as needed)
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/utils/kazoo_utils.py#n199 : .. _eventlet: https://kazoo.readthedocs.io/en/latest/api/\
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/utils/kazoo_utils.py#n200 : handlers/eventlet.html
https://opendev.org/openstack/taskflow/src/branch/master/taskflow/utils/misc.py#n480 : run. This can happen when eventlet switches greenthreads or when running an
https://opendev.org/openstack/taskflow/src/branch/master/test-requirements.txt#n19 : eventlet>=0.18.2 # MIT

Project: tooz
---

- **Project:** tooz
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase. The usage of eventlet.wsgi and Deferred Tasks and Scheduling patterns indicates significant dependencies on Eventlet's core features.*
  - **Files Analyzed:**
    - **File:** `drivers/zookeeper.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The file references eventlet_handler, which is initialized only if it exists. This indicates the use of Eventlet's WSGI server in the Zookeeper driver.*
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.*
    - **File:** `drivers/zookeeper.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled. This is crucial for maintaining the Zookeeper service's stability and performance.*
    - **File:** `drivers/zookeeper.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* The file uses eventlet.spawn to manage green threads, which is essential for the asynchronous operation of the Zookeeper driver. This pattern indicates extensive use of Eventlet's green thread management capabilities.*
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a crucial role in maintaining tooz's functionality, particularly through its support for WSGI servers and deferred tasks.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms, adjusting configuration management, and ensuring thorough testing at each stage to maintain system stability.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and prioritize testing to ensure a stable migration process.

Occurrences Found:
https://opendev.org/openstack/tooz/src/branch/master/tooz/drivers/zookeeper.py#n21 : from kazoo.handlers import eventlet as eventlet_handler
https://opendev.org/openstack/tooz/src/branch/master/tooz/drivers/zookeeper.py#n23 : eventlet_handler = None
https://opendev.org/openstack/tooz/src/branch/master/tooz/drivers/zookeeper.py#n122 : if eventlet_handler:
https://opendev.org/openstack/tooz/src/branch/master/tooz/drivers/zookeeper.py#n123 : HANDLERS['eventlet'] = eventlet_handler.SequentialEventletHandler
