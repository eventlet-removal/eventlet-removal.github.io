# Analysis for Team: nova

## Project: nova
It appears that the OpenStack Nova project has deprecated the use of `eventlet` for running the Nova API, due to compatibility issues with recent releases.

The relevant sections from the release notes are:

* `request_log-e7680b3276910743.yaml`: "Nova API is not running under eventlet.wsgi. Because this is an api-paste."
* `zookeeper-servicegroup-driver-removed-c3bcaa6f9fe976ed.yaml`: "`incompatible with recent eventlet releases`_.
* `mitaka.rst`: `incompatible with recent eventlet releases`_

These notes indicate that the Nova API should no longer be run under `eventlet`, and that any existing configurations using `eventlet` may need to be updated or replaced.

Additionally, the `requirements.txt` file lists `eventlet>=0.30.1` as a required dependency, which suggests that the project is now recommending the use of a newer version of `eventlet`.

Overall, it appears that the OpenStack Nova project has deprecated the use of `eventlet` for running the Nova API, and recommends using a different approach or updating existing configurations to be compatible with recent releases.

Occurrences Found:
- https://opendev.org/openstack/nova/src/branch/master/.coveragerc#n5 : concurrency = eventlet
- https://opendev.org/openstack/nova/src/branch/master/HACKING.rst#n45 : - [N340] Check nova.utils.spawn() is used instead of greenthread.spawn() and eventlet.spawn()
- https://opendev.org/openstack/nova/src/branch/master/HACKING.rst#n76 : with eventlet patched code. Use nova.utils.ReaderWriterLock() instead.
- https://opendev.org/openstack/nova/src/branch/master/doc/source/admin/index.rst#n110 : WSGI container instead of the baked-in eventlet web server.
- https://opendev.org/openstack/nova/src/branch/master/doc/source/cli/opts/debugger.rst#n6 : uses the eventlet library to support async IO. This could result in
- https://opendev.org/openstack/nova/src/branch/master/doc/source/cli/opts/debugger.rst#n14 : how Nova uses the eventlet library to support async IO. This could result
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/index.rst#n121 : * :doc:`/contributor/testing/eventlet-profiling`
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/index.rst#n133 : testing/eventlet-profiling
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/testing/eventlet-profiling.rst#n10 : Because most Nova services use eventlet_, the standard profiling tool provided
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/testing/eventlet-profiling.rst#n12 : changing tasks. Thankfully eventlet comes with
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/testing/eventlet-profiling.rst#n13 : ``eventlet.green.profile.Profile``, a mostly undocumented class that provides a
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/testing/eventlet-profiling.rst#n17 : .. note:: The eventlet Profile outputs the ``prof`` format produced by
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/testing/eventlet-profiling.rst#n20 : so the options for analyzing eventlet profiling are not always
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/testing/eventlet-profiling.rst#n59 : .. note:: ``stop()`` and ``start()`` are two of the ways in which the eventlet
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/testing/eventlet-profiling.rst#n117 : +        from eventlet.green import profile
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/testing/eventlet-profiling.rst#n198 : Real world use indicates that the eventlet profiler is not perfect. There are
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/testing/eventlet-profiling.rst#n202 : to these issues; profiling eventlet services is more an art than science.
- https://opendev.org/openstack/nova/src/branch/master/doc/source/contributor/testing/eventlet-profiling.rst#n268 : .. _eventlet: https://eventlet.net/
- https://opendev.org/openstack/nova/src/branch/master/doc/source/index.rst#n161 : WSGI container instead of the baked-in eventlet web server.
- https://opendev.org/openstack/nova/src/branch/master/doc/source/reference/index.rst#n26 : based on eventlet, and may not be familiar to everyone.
- https://opendev.org/openstack/nova/src/branch/master/doc/source/reference/threading.rst#n5 : through using the Python `eventlet <http://eventlet.net/>`_ and
- https://opendev.org/openstack/nova/src/branch/master/doc/source/reference/threading.rst#n9 : switches can only occur when specific eventlet or greenlet library calls are
- https://opendev.org/openstack/nova/src/branch/master/doc/source/reference/threading.rst#n23 : that trigger an eventlet context switch, the long-running thread will block
- https://opendev.org/openstack/nova/src/branch/master/doc/source/reference/threading.rst#n26 : This scenario can be avoided by adding calls to the eventlet sleep method
- https://opendev.org/openstack/nova/src/branch/master/doc/source/reference/threading.rst#n31 : from eventlet import greenthread
- https://opendev.org/openstack/nova/src/branch/master/doc/source/reference/threading.rst#n36 : time module is patched through eventlet.monkey_patch(). To be explicit, we recommend
- https://opendev.org/openstack/nova/src/branch/master/doc/source/reference/threading.rst#n39 : MySQL access and eventlet
- https://opendev.org/openstack/nova/src/branch/master/doc/source/reference/threading.rst#n43 : eventlet. MySQL-python uses an external C library for accessing the MySQL database.
- https://opendev.org/openstack/nova/src/branch/master/doc/source/reference/threading.rst#n44 : Since eventlet cannot use monkey-patching to intercept blocking calls in a C library,
- https://opendev.org/openstack/nova/src/branch/master/doc/source/user/wsgi.rst#n5 : provide eventlet-based HTTP servers, it is generally considered more performant
- https://opendev.org/openstack/nova/src/branch/master/doc/source/user/wsgi.rst#n28 : eventlet-based scripts).
- https://opendev.org/openstack/nova/src/branch/master/etc/nova/logging_sample.conf#n50 : [logger_eventletwsgi]
- https://opendev.org/openstack/nova/src/branch/master/etc/nova/logging_sample.conf#n53 : qualname = eventlet.wsgi.server
- https://opendev.org/openstack/nova/src/branch/master/nova/api/openstack/requestlog.py#n55 : If we detect we are running under eventlet wsgi processing, we
- https://opendev.org/openstack/nova/src/branch/master/nova/api/openstack/requestlog.py#n59 : if req.environ.get('eventlet.input', None) is not None:
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n42 : import eventlet.event
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n43 : from eventlet import greenthread
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n44 : import eventlet.semaphore
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n45 : import eventlet.timeout
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n243 : _InstanceEvents = ty.Dict[ty.Tuple[str, str], eventlet.event.Event]
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n259 : ) -> eventlet.event.Event:
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n264 : action will trigger the event. The resulting eventlet.event.Event
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n282 : eventlet.event.Event())
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n296 : :returns: the eventlet.event.Event object on which the waiters
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n346 : :returns: a dictionary of {event_name: eventlet.event.Event}
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n370 : for (name, tag), eventlet_event in events.items():
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n380 : eventlet_event.send(event)
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n416 : def __init__(self, name: str, event: eventlet.event.Event) -> None:
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n437 : except eventlet.timeout.Timeout:
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n490 : Note that this uses an eventlet.timeout.Timeout to bound the
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n495 : eventlet.timeout.Timeout is raised.
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n558 : with eventlet.timeout.Timeout(deadline):
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n561 : except eventlet.timeout.Timeout:
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n644 : self._sync_power_pool = eventlet.GreenPool(
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n650 : self._build_semaphore = eventlet.semaphore.Semaphore(
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n655 : self._snapshot_semaphore = eventlet.semaphore.Semaphore(
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n1625 : eventlet.semaphore.BoundedSemaphore(
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n2817 : except (Exception, eventlet.timeout.Timeout) as exc:
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n3675 : except (Exception, eventlet.timeout.Timeout) as exc:
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n7157 : except (Exception, eventlet.timeout.Timeout) as exc:
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/manager.py#n8916 : except eventlet.timeout.Timeout:
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/multi_cell_list.py#n17 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/compute/multi_cell_list.py#n98 : with eventlet.timeout.Timeout(context.CELL_TIMEOUT, exception.CellTimeout):
- https://opendev.org/openstack/nova/src/branch/master/nova/conductor/manager.py#n20 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/conductor/manager.py#n2075 : fetch_pool = eventlet.GreenPool(size=threads)
- https://opendev.org/openstack/nova/src/branch/master/nova/conf/remote_debug.py#n30 : Note that using the remote debug option changes how nova uses the eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/conf/remote_debug.py#n49 : Note that using the remote debug option changes how nova uses the eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/conf/wsgi.py#n51 : This option only works when running nova-api under eventlet, and
- https://opendev.org/openstack/nova/src/branch/master/nova/conf/wsgi.py#n52 : encodes very eventlet specific pieces of information. Starting in Pike
- https://opendev.org/openstack/nova/src/branch/master/nova/conf/wsgi.py#n62 : nova-api under eventlet. If used under uwsgi or apache, this option
- https://opendev.org/openstack/nova/src/branch/master/nova/console/websocketproxy.py#n171 : from eventlet import hubs
- https://opendev.org/openstack/nova/src/branch/master/nova/context.py#n23 : import eventlet.queue
- https://opendev.org/openstack/nova/src/branch/master/nova/context.py#n24 : import eventlet.timeout
- https://opendev.org/openstack/nova/src/branch/master/nova/context.py#n417 : queue = eventlet.queue.LightQueue()
- https://opendev.org/openstack/nova/src/branch/master/nova/context.py#n437 : with eventlet.timeout.Timeout(timeout, exception.CellTimeout):
- https://opendev.org/openstack/nova/src/branch/master/nova/debugger.py#n58 : 'Nova uses the eventlet library to support async IO. This '
- https://opendev.org/openstack/nova/src/branch/master/nova/hacking/checks.py#n91 : r".*(eventlet|greenthread)\.(?P<spawn_part>spawn(_n)?)\(.*\)")
- https://opendev.org/openstack/nova/src/branch/master/nova/hacking/checks.py#n572 : eventlet.spawn(), and eventlet.spawn_n()
- https://opendev.org/openstack/nova/src/branch/master/nova/hacking/checks.py#n577 : "greenthread.%(spawn)s() and eventlet.%(spawn)s()")
- https://opendev.org/openstack/nova/src/branch/master/nova/hacking/checks.py#n1019 : and identify lock holders and waiters. The eventlet implementation of
- https://opendev.org/openstack/nova/src/branch/master/nova/hacking/checks.py#n1023 : See https://github.com/eventlet/eventlet/issues/731 for details.
- https://opendev.org/openstack/nova/src/branch/master/nova/hacking/checks.py#n1028 : "function correctly with eventlet patched code. "
- https://opendev.org/openstack/nova/src/branch/master/nova/monkey_patch.py#n19 : """Enable eventlet monkey patching."""
- https://opendev.org/openstack/nova/src/branch/master/nova/monkey_patch.py#n30 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/monkey_patch.py#n46 : eventlet.monkey_patch(thread=False)
- https://opendev.org/openstack/nova/src/branch/master/nova/monkey_patch.py#n50 : eventlet.monkey_patch(os=False)
- https://opendev.org/openstack/nova/src/branch/master/nova/monkey_patch.py#n52 : eventlet.monkey_patch()
- https://opendev.org/openstack/nova/src/branch/master/nova/monkey_patch.py#n76 : LOG.warning("Modules with known eventlet monkey patching issues were "
- https://opendev.org/openstack/nova/src/branch/master/nova/monkey_patch.py#n77 : "imported prior to eventlet monkey patching: %s. This "
- https://opendev.org/openstack/nova/src/branch/master/nova/rpc.py#n223 : executor='eventlet',
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n27 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n28 : import eventlet.wsgi
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n336 : protocol=eventlet.wsgi.HttpProtocol, backlog=128,
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n344 : :param pool_size: Maximum number of eventlets to spawn concurrently.
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n351 : eventlet.wsgi.MAX_HEADER_LINE = CONF.wsgi.max_header_line
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n357 : self._pool = eventlet.GreenPool(self.pool_size)
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n382 : self._socket = eventlet.listen(bind_addr, family, backlog=backlog)
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n449 : dup_socket = eventlet.wrap_ssl(dup_socket,
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n460 : 'func': eventlet.wsgi.server,
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n489 : server is stopped is by killing its eventlet.
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n504 : Waits on the server's eventlet to finish, then returns.
- https://opendev.org/openstack/nova/src/branch/master/nova/service.py#n533 : LOG.warning('Running %s using eventlet is deprecated. Deploy with '
- https://opendev.org/openstack/nova/src/branch/master/nova/storage/rbd_utils.py#n19 : from eventlet import tpool
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/notifications.py#n17 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/notifications.py#n154 : current = eventlet.getcurrent()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/notifications.py#n172 : 'probably leaked a running eventlet that emitted '
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/notifications.py#n174 : 'eventlet is terminated by raising this exception.' %
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n31 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n563 : executor='eventlet',
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1150 : self.greenpool = eventlet.greenpool.GreenPool()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1169 : if isinstance(gt, eventlet.greenthread.GreenThread):
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1172 : utils.DEFAULT_GREEN_POOL = eventlet.greenpool.GreenPool()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1174 : isinstance(gt, eventlet.greenthread.GreenThread)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1762 : eventlet. This information then later used by the NotificationFixture to
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1763 : detect if a notification was emitted by an eventlet that was spawned by a
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1777 : c = eventlet.getcurrent()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1785 : caller = eventlet.getcurrent()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1797 : current = eventlet.getcurrent()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1816 : caller = eventlet.getcurrent()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1828 : current = eventlet.getcurrent()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1843 : """Wrap oslo.concurrency lockutils.ReaderWriterLock to support eventlet.
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1845 : As of fasteners >= 0.15, the workaround code to use eventlet.getcurrent()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1846 : if eventlet patching is detected has been removed and
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1849 : type GreenThread. A GreenThread is created by calling eventlet.spawn() and
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1851 : eventlet patched threading.current_thread() method falls back to the
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1856 : eventlet.getcurrent() during creation of the lock object, if we detect we
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1857 : are eventlet patched. If we are not eventlet patched, we use a no-op
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1863 : See https://github.com/eventlet/eventlet/issues/731 for details.
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1865 : [1] https://github.com/eventlet/eventlet/blob/v0.32.0/eventlet/green/threading.py#L128  # noqa
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1869 : eventlet_patched = eventlet.patcher.is_monkey_patched('thread')
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1871 : 'threading.current_thread', eventlet.getcurrent)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n1872 : with mpatch if eventlet_patched else contextlib.ExitStack():
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n2007 : created by eventlet.spawn_n(). Bare greenlets cannot be killed the same
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/fixtures/nova.py#n2008 : way as GreenThreads created by eventlet.spawn().
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/api/openstack/test_requestlog.py#n26 : other than eventlet. While Nova grew up on eventlet, and it's wsgi
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/api/openstack/test_requestlog.py#n28 : of what Nova was emitting, and what eventlet.wsgi was emitting on our
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/api/openstack/test_requestlog.py#n127 : def test_no_log_under_eventlet(self, emit):
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/api/openstack/test_requestlog.py#n128 : """Ensure that logs don't end up under eventlet.
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/api/openstack/test_requestlog.py#n131 : the situation where eventlet is removed from tests and this
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/api/openstack/test_requestlog.py#n134 : NOTE(sdague): this test can be deleted when eventlet is no
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/eventlet_utils.py#n15 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/eventlet_utils.py#n18 : class SyncPool(eventlet.GreenPool):
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute.py#n74 : from nova.tests.unit.compute import eventlet_utils
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute.py#n162 : self.compute._sync_power_pool = eventlet_utils.SyncPool()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute.py#n390 : self.stub_out('eventlet.greenthread.sleep',
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n25 : from eventlet import event as eventlet_event
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n26 : from eventlet import timeout as eventlet_timeout
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n3865 : event = eventlet_event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n3881 : event = eventlet_event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n3916 : event = eventlet_event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n4393 : fake_eventlet_event = mock.MagicMock()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n4396 : ('network-vif-plugged', uuids.portid): fake_eventlet_event,
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n4402 : self.assertTrue(fake_eventlet_event.send.called)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n4403 : event = fake_eventlet_event.send.call_args_list[0][0][0]
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n6948 : mock_wait_inst_ev.side_effect = eventlet_timeout.Timeout
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n6950 : self.assertRaises(eventlet_timeout.Timeout,
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n10385 : eventlet Timeout exception and we're configured such that vif plugging
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n10403 : eventlet_timeout.Timeout())
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n10423 : eventlet Timeout exception and we're configured such that vif plugging
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_compute_mgr.py#n10441 : eventlet_timeout.Timeout())
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_shelve.py#n15 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_shelve.py#n451 : mock_get_arqs.side_effect = eventlet.timeout.Timeout()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_shelve.py#n453 : self.assertRaises(eventlet.timeout.Timeout,
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_virtapi.py#n18 : import eventlet.timeout
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_virtapi.py#n202 : side_effect=eventlet.timeout.Timeout())
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_virtapi.py#n208 : self.assertRaises(eventlet.timeout.Timeout, do_test)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_virtapi.py#n241 : raise eventlet.timeout.Timeout()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_virtapi.py#n251 : self.assertRaises(eventlet.timeout.Timeout, do_test)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_virtapi.py#n285 : raise eventlet.timeout.Timeout()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/compute/test_virtapi.py#n317 : self.assertRaises(eventlet.timeout.Timeout, do_test)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/fake_processutils.py#n19 : from eventlet import greenthread
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/storage/test_rbd.py#n15 : from eventlet import tpool
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_context.py#n365 : @mock.patch('eventlet.timeout.Timeout')
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_context.py#n366 : @mock.patch('eventlet.queue.LightQueue.get')
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_hacking.py#n438 : code = "eventlet.greenthread.spawn(func, arg1, kwarg1=kwarg1)"
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_hacking.py#n442 : code = "eventlet.spawn(func, arg1, kwarg1=kwarg1)"
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_hacking.py#n446 : code = "eventlet.spawn_n(func, arg1, kwarg1=kwarg1)"
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_rpc.py#n249 : executor='eventlet', serializer=ser,
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_rpc.py#n291 : executor='eventlet', serializer=ser,
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_service.py#n25 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_service.py#n26 : import eventlet.wsgi
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_service.py#n337 : eventlet.wsgi.MAX_HEADER_LINE)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_service.py#n398 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/test_service.py#n404 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/disk/mount/test_nbd.py#n22 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/disk/mount/test_nbd.py#n328 : thread1 = eventlet.spawn(get_a_device)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/disk/mount/test_nbd.py#n329 : thread2 = eventlet.spawn(get_a_device)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/disk/vfs/test_guestfs.py#n346 : with mock.patch('eventlet.tpool.Proxy', return_value=m) as tpool_mock:
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/disk/vfs/test_guestfs.py#n357 : """Asserts that we do not use an eventlet thread pool when guestfs
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/disk/vfs/test_guestfs.py#n365 : with mock.patch('eventlet.tpool.Proxy',
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n38 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n39 : from eventlet import greenthread
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n561 : wait1 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n562 : done1 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n563 : sig1 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n564 : thr1 = eventlet.spawn(backend.by_name(self._fake_instance(uuid),
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n568 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n572 : wait2 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n573 : done2 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n574 : sig2 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n575 : thr2 = eventlet.spawn(backend.by_name(self._fake_instance(uuid),
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n581 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n587 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n599 : wait1 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n600 : done1 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n601 : sig1 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n602 : thr1 = eventlet.spawn(backend.by_name(self._fake_instance(uuid),
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n606 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n610 : wait2 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n611 : done2 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n612 : sig2 = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n613 : thr2 = eventlet.spawn(backend.by_name(self._fake_instance(uuid),
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n617 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n624 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n630 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n13956 : side_effect=lambda x: eventlet.sleep(0))
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n13989 : finish_event = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n14672 : return type(other) is eventlet.event.Event
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n15455 : @mock.patch.object(eventlet.greenthread, 'sleep',
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n15456 : side_effect=eventlet.sleep(0))
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n15483 : @mock.patch.object(eventlet.greenthread, 'sleep',
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n15484 : side_effect=eventlet.sleep(0))
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n20652 : raise eventlet.timeout.Timeout()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_driver.py#n28977 : @mock.patch.object(eventlet.tpool, 'execute')
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_host.py#n21 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_host.py#n22 : from eventlet import greenthread
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_host.py#n23 : from eventlet import tpool
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_host.py#n353 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_host.py#n381 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_host.py#n396 : thr1 = eventlet.spawn(get_conn_currency, self.host)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_host.py#n397 : thr2 = eventlet.spawn(get_conn_currency, self.host)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_host.py#n400 : eventlet.sleep(0)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/test_host.py#n460 : event = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/volume/test_mount.py#n20 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/volume/test_mount.py#n85 : self.start_event = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/volume/test_mount.py#n100 : self.thread = eventlet.greenthread.spawn(deferred_start)
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/volume/test_mount.py#n105 : return cls.all_threads.get(eventlet.greenthread.getcurrent())
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/libvirt/volume/test_mount.py#n659 : host_up = eventlet.greenthread.spawn(self.m.host_up,
- https://opendev.org/openstack/nova/src/branch/master/nova/tests/unit/virt/vmwareapi/test_driver_api.py#n26 : from eventlet import greenthread
- https://opendev.org/openstack/nova/src/branch/master/nova/utils.py#n31 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/utils.py#n32 : from eventlet import tpool
- https://opendev.org/openstack/nova/src/branch/master/nova/utils.py#n89 : DEFAULT_GREEN_POOL = eventlet.greenpool.GreenPool(
- https://opendev.org/openstack/nova/src/branch/master/nova/utils.py#n670 : """Passthrough method for eventlet.spawn.
- https://opendev.org/openstack/nova/src/branch/master/nova/utils.py#n684 : """Passthrough method for eventlet.greenpool.spawn_n.
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/disk/vfs/guestfs.py#n17 : from eventlet import tpool
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/driver.py#n51 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/driver.py#n52 : from eventlet import greenthread
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/driver.py#n53 : from eventlet import tpool
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/driver.py#n196 : """eventlet.tpool.Proxy doesn't work with old-style class in __str__()
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/driver.py#n8135 : except eventlet.timeout.Timeout:
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/driver.py#n11203 : finish_event = eventlet.event.Event()
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/host.py#n42 : from eventlet import greenio
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/host.py#n43 : from eventlet import greenthread
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/host.py#n44 : from eventlet import patcher
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/host.py#n45 : from eventlet import tpool
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/host.py#n207 : eventlet's greenthread integration
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/host.py#n249 : an eventlet coroutine. It can only invoke other libvirt
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/host.py#n262 : an eventlet coroutine. It can only invoke other libvirt
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/host.py#n275 : an eventlet coroutine. It can only invoke other libvirt
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/libvirt/host.py#n462 : This code is taken from the eventlet tpool module, under terms
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/zvm/driver.py#n15 : import eventlet
- https://opendev.org/openstack/nova/src/branch/master/nova/virt/zvm/driver.py#n290 : except eventlet.timeout.Timeout:
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/deprecate-api-eventlet-1a0279f1f2333082.yaml#n4 : Running API services (nova-osapi_compute or nova-metadata) with eventlet
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/deprecate_wsgi_log_format-43a10b7a608ea8f3.yaml#n5 : applies when running nova-api under eventlet, which is no longer
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/dnspython-2.0.0-not-supported-by-eventlet-b8adf73ed9e14817.yaml#n4 : The dnspython 2.0.0 package is incompatible with even the latest eventlet
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/dnspython-2.0.0-not-supported-by-eventlet-b8adf73ed9e14817.yaml#n6 : of the dnspython package is equal or greater than 2.0.0. See `eventlet
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/dnspython-2.0.0-not-supported-by-eventlet-b8adf73ed9e14817.yaml#n9 : .. _eventlet issue 619: https://github.com/eventlet/eventlet/issues/619
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/greendns-34df7f9fba952bcd.yaml#n4 : During the havana cycle it was discovered that eventlet
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/greendns-34df7f9fba952bcd.yaml#n7 : Since then nova has been disabling eventlet monkey patching
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/request_log-e7680b3276910743.yaml#n5 : even if Nova API is not running under eventlet.wsgi. Because this
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/request_log-e7680b3276910743.yaml#n9 : detected that nova-api is not running under eventlet, and will
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/wsgi-applications-8017c3192d2b143e.yaml#n7 : or gunicorn). The eventlet-based servers are still available, but the WSGI
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/zookeeper-servicegroup-driver-removed-c3bcaa6f9fe976ed.yaml#n9 : `incompatible with recent eventlet releases`_.
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/notes/zookeeper-servicegroup-driver-removed-c3bcaa6f9fe976ed.yaml#n14 : .. _`incompatible with recent eventlet releases`: https://bugs.launchpad.net/nova/+bug/1443910
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n523 : "Nova API is not running under eventlet.wsgi. Because this is an api-paste."
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n526 : "emit when it is detected that nova-api is not running under eventlet, and "
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n530 : "新しい request_log ミドルウェアが、たとえ Nova API が eventlet.wsgi 下で動作"
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n533 : "があります。新しいリクエストログは nova-api が eventlet 下で動作していないこ"
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1290 : "when running nova-api under eventlet, which is no longer the preferred "
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n1293 : "``wsgi_log_format`` 設定オプションは非推奨となりました。これは eventlet 配下"
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n3521 : "`incompatible with recent eventlet releases`_."
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n3526 : "`incompatible with recent eventlet releases`_ です。"
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/source/mitaka.rst#n644 : `incompatible with recent eventlet releases`_.
- https://opendev.org/openstack/nova/src/branch/master/releasenotes/source/mitaka.rst#n649 : .. _`incompatible with recent eventlet releases`: https://bugs.launchpad.net/nova/+bug/1443910
- https://opendev.org/openstack/nova/src/branch/master/requirements.txt#n8 : eventlet>=0.30.1 # MIT

***

## Project: nova-specs
---

- **Project:** nova-specs
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 8
    *This level represents a complex migration involving extensive changes across the codebase.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `specs/abandoned/parallel-scheduler.rst`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          *Description:* The C based DB driver and eventlet means the scheduler performs best when the eventlet thread pool is very small, ideally less than 5.
    - **File:** `specs/liberty/approved/service-group-using-tooz.rst`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* Maintained and doesn't work with eventlet >= 0.17.1.
    - **File:** `specs/rocky/implemented/abort-live-migration-in-queued-status.rst`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          *Description:* The current ``eventlet.spawn_n()`` + python ``Semaphore`` implementation.
    - **File:** `specs/rocky/implemented/abort-live-migration-in-queued-status.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          *Description:* Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads in configuration files and in scheduling deferred tasks.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/nova-specs/src/branch/master/specs/abandoned/parallel-scheduler.rst#n48 : * The C based DB driver and eventlet means the scheduler performs best
- https://opendev.org/openstack/nova-specs/src/branch/master/specs/abandoned/parallel-scheduler.rst#n49 : when the eventlet thread pool is very small, ideally less than 5.
- https://opendev.org/openstack/nova-specs/src/branch/master/specs/liberty/approved/service-group-using-tooz.rst#n27 : maintained and doesn't work with eventlet >= 0.17.1.
- https://opendev.org/openstack/nova-specs/src/branch/master/specs/rocky/implemented/abort-live-migration-in-queued-status.rst#n67 : the current ``eventlet.spawn_n()`` + python ``Semaphore`` implementation.

***

## Project: os-vif
---

- **Project:** os-vif
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to simple migration requiring some code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. However, the specific functionality of Eventlet in os-vif is relatively isolated compared to other OpenStack components.*
  - **Files Analyzed:**
    - **File:** `tests/functional/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.timeout` and `eventlet.sleep` to manage green threads, which is essential for the asynchronous operation of the test suite.
    - **File:** `tests/functional/base.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This file uses `mock.patch('eventlet.timeout')` to mock Eventlet's timeout function, indicating that Eventlet is used in unit tests.
    - **File:** `tests/functional/base.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads in tests and configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce some complexity. However, given the isolated nature of os-vif's functionality, this might be more manageable than initially thought.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

---

- **Project:** OpenStack Watcher
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 7
    *This level represents a moderate to complex migration requiring significant code refactoring.*
    *Factors for estimation: Extensive use of green threads and deferred tasks, which would require significant code refactoring to eliminate the dependency on Eventlet. Additionally, the presence of Eventlet-specific configurations indicates that it is deeply integrated into the project's architecture.*
  - **Files Analyzed:**
    - **File:** `applier/workflow_engine/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity. Additionally, given the critical nature of some functionalities that rely on Eventlet, this might be more challenging than initially thought.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

---

- **Project:** os-vif
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: Given the isolated nature of os-vif's functionality and its relatively simple asynchronous operations, removing Eventlet might not introduce significant complexity.*
  - **Estimated complexity of the migration:** 2
    *This level represents a very low to no migration complexity.*
    *Factors for estimation: The specific functionality of Eventlet in os-vif is relatively isolated compared to other OpenStack components. Removing Eventlet would likely only require minor adjustments to configuration management and testing.*
  - **Files Analyzed:**
    - **File:** `tests/functional/base.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file uses `eventlet.timeout` and `eventlet.sleep` to manage green threads, which is essential for the asynchronous operation of the test suite.
    - **File:** `tests/functional/base.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This file uses `mock.patch('eventlet.timeout')` to mock Eventlet's timeout function, indicating that Eventlet is used in unit tests.
    - **File:** `tests/functional/base.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads in tests and configuration files. However, its removal would likely not introduce significant complexity due to its isolated nature.*
    - **Potential Challenges:** None
    *Recommendations:* Given the relatively simple nature of os-vif's functionality, removing Eventlet might be a straightforward process that requires minimal adjustments.

Occurrences Found:
- https://opendev.org/openstack/os-vif/src/branch/master/os_vif/tests/functional/base.py#n22 : import eventlet.timeout
- https://opendev.org/openstack/os-vif/src/branch/master/os_vif/tests/functional/base.py#n54 : with eventlet.Timeout(timeout):
- https://opendev.org/openstack/os-vif/src/branch/master/os_vif/tests/functional/base.py#n56 : eventlet.sleep(sleep)
- https://opendev.org/openstack/os-vif/src/branch/master/os_vif/tests/functional/base.py#n57 : except eventlet.Timeout:
- https://opendev.org/openstack/os-vif/src/branch/master/os_vif/tests/functional/base.py#n78 : except eventlet.Timeout as e:

***
