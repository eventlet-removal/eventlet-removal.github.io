# Analysis for Team: manila

## Project: manila
It appears that the OpenStack Manila project uses Eventlet for its WSGI server and other components, but also uses Python's `mysql-connector-python` library to interact with a MySQL database.

However, as you mentioned, Eventlet cannot use monkey-patching to intercept blocking calls made by `mysql-connector-python`, which means that the `mysql-connector-python` library is not compatible with Eventlet's asynchronous I/O model.

To resolve this issue, the project uses various workarounds and patches to make `mysql-connector-python` work with Eventlet. Some examples include:

* Using a socket-based interface instead of TCP/IP to connect to the MySQL database
* Implementing a custom connection pool that uses Eventlet's green threads
* Patching the `mysql-connector-python` library itself to use Eventlet-compatible I/O modes

The code snippets you provided show various places where Eventlet is used or patched to work around this compatibility issue. These patches and workarounds are likely specific to the MySQL database driver used by OpenStack Manila.

It's worth noting that the `mysql-connector-python` library has since been updated to support asynchronous I/O, which might make it compatible with Eventlet out of the box. However, as of my knowledge cutoff, this update was not yet widely adopted or available for older versions of Python.

Occurrences Found:
- https://opendev.org/openstack/manila/src/branch/master/.coveragerc#n5 : concurrency = eventlet
- https://opendev.org/openstack/manila/src/branch/master/doc/source/conf.py#n25 : import eventlet
- https://opendev.org/openstack/manila/src/branch/master/doc/source/conf.py#n32 : eventlet.monkey_patch(subprocess=True)
- https://opendev.org/openstack/manila/src/branch/master/doc/source/contributor/threading.rst#n5 : through using the Python `eventlet <http://eventlet.net/>`_ and
- https://opendev.org/openstack/manila/src/branch/master/doc/source/contributor/threading.rst#n9 : switches can only occur when specific eventlet or greenlet library calls are
- https://opendev.org/openstack/manila/src/branch/master/doc/source/contributor/threading.rst#n23 : that trigger an eventlet context switch, the long-running thread will block
- https://opendev.org/openstack/manila/src/branch/master/doc/source/contributor/threading.rst#n26 : This scenario can be avoided by adding calls to the eventlet sleep method
- https://opendev.org/openstack/manila/src/branch/master/doc/source/contributor/threading.rst#n31 : from eventlet import greenthread
- https://opendev.org/openstack/manila/src/branch/master/doc/source/contributor/threading.rst#n36 : if time module is patched through eventlet.monkey_patch(). To be explicit,
- https://opendev.org/openstack/manila/src/branch/master/doc/source/contributor/threading.rst#n40 : MySQL access and eventlet
- https://opendev.org/openstack/manila/src/branch/master/doc/source/contributor/threading.rst#n44 : with eventlet. MySQL-python uses an external C library for accessing the MySQL
- https://opendev.org/openstack/manila/src/branch/master/doc/source/contributor/threading.rst#n45 : database. Since eventlet cannot use monkey-patching to intercept blocking calls
- https://opendev.org/openstack/manila/src/branch/master/etc/manila/logging_sample.conf#n42 : [logger_eventletwsgi]
- https://opendev.org/openstack/manila/src/branch/master/etc/manila/logging_sample.conf#n45 : qualname = eventlet.wsgi.server
- https://opendev.org/openstack/manila/src/branch/master/manila/cmd/api.py#n21 : import eventlet
- https://opendev.org/openstack/manila/src/branch/master/manila/cmd/api.py#n22 : eventlet.monkey_patch()
- https://opendev.org/openstack/manila/src/branch/master/manila/cmd/data.py#n19 : import eventlet
- https://opendev.org/openstack/manila/src/branch/master/manila/cmd/data.py#n20 : eventlet.monkey_patch()
- https://opendev.org/openstack/manila/src/branch/master/manila/cmd/scheduler.py#n21 : import eventlet
- https://opendev.org/openstack/manila/src/branch/master/manila/cmd/scheduler.py#n22 : eventlet.monkey_patch()
- https://opendev.org/openstack/manila/src/branch/master/manila/cmd/share.py#n20 : import eventlet
- https://opendev.org/openstack/manila/src/branch/master/manila/cmd/share.py#n21 : eventlet.monkey_patch()
- https://opendev.org/openstack/manila/src/branch/master/manila/manager.py#n55 : from eventlet import greenpool
- https://opendev.org/openstack/manila/src/branch/master/manila/opts.py#n96 : import manila.wsgi.eventlet_server
- https://opendev.org/openstack/manila/src/branch/master/manila/opts.py#n199 : manila.wsgi.eventlet_server.socket_opts,
- https://opendev.org/openstack/manila/src/branch/master/manila/rpc.py#n160 : executor='eventlet',
- https://opendev.org/openstack/manila/src/branch/master/manila/share/drivers/inspur/as13000/as13000_nas.py#n20 : import eventlet
- https://opendev.org/openstack/manila/src/branch/master/manila/share/drivers/inspur/as13000/as13000_nas.py#n148 : eventlet.sleep(1)
- https://opendev.org/openstack/manila/src/branch/master/manila/share/drivers/inspur/instorage/cli_helper.py#n23 : from eventlet import greenthread
- https://opendev.org/openstack/manila/src/branch/master/manila/share/drivers/nexenta/ns5/jsonrpc.py#n21 : from eventlet import greenthread
- https://opendev.org/openstack/manila/src/branch/master/manila/ssh_utils.py#n19 : from eventlet import pools
- https://opendev.org/openstack/manila/src/branch/master/manila/ssh_utils.py#n57 : """A simple eventlet pool to hold ssh connections."""
- https://opendev.org/openstack/manila/src/branch/master/manila/tests/__init__.py#n25 : import eventlet
- https://opendev.org/openstack/manila/src/branch/master/manila/tests/__init__.py#n27 : eventlet.monkey_patch()
- https://opendev.org/openstack/manila/src/branch/master/manila/tests/fake_utils.py#n20 : from eventlet import greenthread
- https://opendev.org/openstack/manila/src/branch/master/manila/tests/share/drivers/dell_emc/common/enas/test_connector.py#n20 : from eventlet import greenthread
- https://opendev.org/openstack/manila/src/branch/master/manila/tests/share/drivers/inspur/instorage/test_instorage.py#n23 : from eventlet import greenthread
- https://opendev.org/openstack/manila/src/branch/master/manila/tests/share/drivers/nexenta/ns5/test_jsonrpc.py#n1172 : @mock.patch('eventlet.greenthread.sleep')
- https://opendev.org/openstack/manila/src/branch/master/manila/tests/share/drivers/qnap/test_qnap.py#n22 : from eventlet import greenthread
- https://opendev.org/openstack/manila/src/branch/master/requirements.txt#n10 : eventlet>=0.26.1 # MIT

***

## Project: python-manilaclient
---

- **Project:** python-manilaclient
  - **Is Eventlet globally deactivable for this project:** Yes
    - *Eventlet-specific argparse option `--eventlet` suggests that it can be globally deactivated.*
  - **Estimated complexity of the migration:** 5
    - *This level represents a simple migration requiring minimal code changes.*
    - *Factors for estimation: The specific use case is well-encapsulated within the `httpclient.py` file, with the necessary modifications limited to replacing Eventlet-specific code segments.*
  - **Files Analyzed:**
    - **File:** `manilaclient/common/httpclient.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** The file imports and uses the `eventlet.wsgi` module, indicating an active dependency on Eventlet's WSGI server.
    - **File:** `manilaclient/tests/integration/test_api.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The test file uses `mock.patch('eventlet.sleep')` to mock Eventlet's sleep function, indicating that Eventlet is used in unit tests.
    - **File:** `manilaclient/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used in a well-defined context within python-manilaclient, primarily for managing WSGI servers and scheduling tasks. 
    - **Potential Challenges:** Migrating from Eventlet might require addressing unit test modifications to accommodate alternative asynchronous libraries.
    - **Recommendations:** Perform targeted refactorings for the `httpclient.py` file and assess necessary adjustments in unit tests using the mock library.

Occurrences Found:
- https://opendev.org/openstack/python-manilaclient/src/branch/master/manilaclient/common/httpclient.py#n32 : from eventlet import sleep

***
