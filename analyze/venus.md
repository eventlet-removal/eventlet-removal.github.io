# Analysis for Team: venus

## Project: venus
---

- **Project:** Venus
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The presence of an `eventlet.wsgi` reference in the logging configuration suggests that Eventlet can be deactivated or replaced with an alternative WSGI server.*
  - **Key Features of Eventlet Usage:**
    + Green threads for asynchronous operations
    + Event-driven I/O using `eventlet.sleep(0.1)`
    + WSGI server setup with `eventlet.spawn` and `eventlet.listen`
    + Integration with the `venus/wsgi/eventlet_server.py` module

- **Configuration and Requirements:**
  * The project uses Eventlet version 0.26.0 or later.
  * The `requirements.txt` file lists `eventlet>=0.26.0` as a dependency.
  * The logging configuration references `eventlet.wsgi.server`.
  * The WSGI server setup uses `eventlet.spawn` and `eventlet.listen`.

- **Alternative Asynchronous Libraries:**
  * The use of asyncio or other alternative asynchronous libraries could be considered as alternatives to Eventlet.

- **Impact on Removal:**
  * Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management.
  * This could introduce significant complexity, particularly for the WSGI server setup.

**Recommendation:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Occurrences Found:
- https://opendev.org/openstack/venus/src/branch/master/etc/venus/logging_sample.conf#n54 : [logger_eventletwsgi]
- https://opendev.org/openstack/venus/src/branch/master/etc/venus/logging_sample.conf#n57 : qualname = eventlet.wsgi.server
- https://opendev.org/openstack/venus/src/branch/master/openstack_venus.egg-info/SOURCES.txt#n217 : venus/wsgi/eventlet_server.py
- https://opendev.org/openstack/venus/src/branch/master/openstack_venus.egg-info/SOURCES.txt#n306 : venus/wsgi/eventlet_server.py
- https://opendev.org/openstack/venus/src/branch/master/openstack_venus.egg-info/requires.txt#n4 : eventlet>=0.26.0
- https://opendev.org/openstack/venus/src/branch/master/requirements.txt#n8 : eventlet>=0.26.0 # MIT
- https://opendev.org/openstack/venus/src/branch/master/venus/rpc.py#n157 : executor='eventlet',
- https://opendev.org/openstack/venus/src/branch/master/venus/service.py#n41 : from venus.wsgi import eventlet_server as wsgi
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n15 : """Methods for working with eventlet WSGI servers."""
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n25 : import eventlet
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n26 : import eventlet.wsgi
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n46 : protocol=eventlet.wsgi.HttpProtocol, backlog=128):
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n53 : :param pool_size: Maximum number of eventlets to spawn concurrently.
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n58 : eventlet.wsgi.MAX_HEADER_LINE = CONF.max_header_line
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n68 : self._pool = eventlet.GreenPool(self.pool_size)
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n69 : self._logger = logging.getLogger("eventlet.wsgi.server")
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n114 : self._socket = eventlet.listen(bind_addr, backlog=backlog,
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n119 : eventlet.sleep(0.1)
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n178 : 'func': eventlet.wsgi.server,
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n188 : self._server = eventlet.spawn(**wsgi_kwargs)
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n202 : server is stopped is by killing its eventlet.
- https://opendev.org/openstack/venus/src/branch/master/venus/wsgi/eventlet_server.py#n216 : Waits on the server's eventlet to finish, then returns.

***
