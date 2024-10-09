# Analysis for Team: cinder

## Project: cinder
The OpenStack Cinder project uses Eventlet as its threading library. Here are the key points about Eventlet:

**What is Eventlet?**

Eventlet is a Python library that provides a high-level, non-blocking I/O framework for concurrent programming. It allows developers to write asynchronous code that is efficient and scalable.

**Key Features of Eventlet:**

1. **Non-Blocking I/O**: Eventlet enables non-blocking I/O operations, which means that your program can continue executing while waiting for I/O operations to complete.
2. **Greenlets**: Eventlet introduces the concept of greenlets, which are lightweight threads that are scheduled by the event loop. Greenlets provide a high level of concurrency and flexibility.
3. **Event Loop**: The event loop is responsible for scheduling tasks and handling events in an asynchronous manner.

**Why does Cinder use Eventlet?**

Cinder uses Eventlet because it provides a high-performance, non-blocking I/O framework that is well-suited to the project's requirements. Eventlet allows Cinder developers to write concurrent code that is efficient and scalable, which is critical for a project like Cinder that manages large numbers of storage resources.

**Eventlet Version Used by Cinder**

The current version of Eventlet used by Cinder is 0.30.1 (inclusive) or greater. The exact version may vary depending on the branch or tag of the codebase.

Overall, Eventlet has become an essential component of the Cinder project's threading model, enabling efficient and scalable concurrency in a high-performance storage management system.

Occurrences Found:
- https://opendev.org/openstack/cinder/src/branch/master/cinder/__init__.py#n21 : import eventlet  # noqa
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/api.py#n25 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n30 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n130 : return eventlet.tpool.Proxy(result)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n165 : return eventlet.tpool.Proxy(writer)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n170 : return eventlet.tpool.Proxy(reader)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n192 : The object writer methods must not have any logging calls, as eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n193 : has a bug (https://github.com/eventlet/eventlet/issues/432) that would
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n202 : The object reader methods must not have any logging calls, as eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n203 : has a bug (https://github.com/eventlet/eventlet/issues/432) that would
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n403 : md5 = eventlet.tpool.execute(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n413 : LOG.debug('Calling eventlet.sleep(0)')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n414 : eventlet.sleep(0)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n620 : shalist = eventlet.tpool.execute(self._calculate_sha, data)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n772 : eventlet.sleep(0)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/chunkeddriver.py#n869 : eventlet.sleep(0)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n55 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n135 : meta_obj = eventlet.tpool.Proxy(rados.Object(self._client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n153 : meta_obj = eventlet.tpool.Proxy(rados.Object(self._client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n166 : meta_obj = eventlet.tpool.Proxy(rados.Object(self._client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n175 : meta_obj = eventlet.tpool.Proxy(rados.Object(self._client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n340 : client = eventlet.tpool.Proxy(self.rados.Rados(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n413 : eventlet.tpool.Proxy(volume.rbd_image).discard(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n417 : eventlet.tpool.Proxy(volume.rbd_image).discard(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n495 : eventlet.tpool.Proxy(self.rbd.RBD()).create(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n519 : base_rbd = eventlet.tpool.Proxy(self.rbd.Image(rados_client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n571 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(self,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n597 : eventlet.tpool.Proxy(self.rbd.RBD()).remove(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n625 : if src_name in eventlet.tpool.Proxy(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n630 : src_rbd = eventlet.tpool.Proxy(self.rbd.Image(client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n725 : rbds = eventlet.tpool.Proxy(self.rbd.RBD()).list(client.ioctx)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n743 : base_rbd = eventlet.tpool.Proxy(self.rbd.Image(client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n764 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(self,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n791 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(self,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n794 : base_rbd = eventlet.tpool.Proxy(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n827 : source_rbd_image = eventlet.tpool.Proxy(volume_file.rbd_image)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1000 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(self,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1007 : eventlet.tpool.Proxy(self.rbd.RBD()).create(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1017 : dest_rbd = eventlet.tpool.Proxy(self.rbd.Image(client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1026 : meta_io_proxy = eventlet.tpool.Proxy(rbd_fd)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1132 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(self,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1220 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(self,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1236 : src_rbd = eventlet.tpool.Proxy(self.rbd.Image(client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1264 : src_rbd = eventlet.tpool.Proxy(self.rbd.Image(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1276 : self._transfer_data(eventlet.tpool.Proxy(rbd_fd), backup_name,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1294 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(self,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1297 : base_image = eventlet.tpool.Proxy(self.rbd.Image(client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1356 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(self,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1358 : base_rbd = eventlet.tpool.Proxy(self.rbd.Image(client.ioctx,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1467 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1500 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(self)) as client:
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/drivers/ceph.py#n1570 : with eventlet.tpool.Proxy(rbd_driver.RADOSClient(
- https://opendev.org/openstack/cinder/src/branch/master/cinder/backup/manager.py#n39 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/api.py#n23 : import eventlet  # noqa
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/api.py#n24 : eventlet.monkey_patch()
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/backup.py#n28 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/backup.py#n29 : eventlet.monkey_patch()
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/backup.py#n89 : _semaphore: Union[eventlet.semaphore.Semaphore,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/scheduler.py#n23 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/scheduler.py#n24 : eventlet.monkey_patch()
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/volume.py#n25 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/volume.py#n26 : import eventlet.tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/volume.py#n33 : eventlet.monkey_patch(os=False)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/cmd/volume.py#n35 : eventlet.monkey_patch()
- https://opendev.org/openstack/cinder/src/branch/master/cinder/image/image_utils.py#n38 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/manager.py#n54 : from eventlet import greenpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/manager.py#n55 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/opts.py#n205 : from cinder.wsgi import eventlet_server as cinder_wsgi_eventletserver
- https://opendev.org/openstack/cinder/src/branch/master/cinder/opts.py#n326 : cinder_wsgi_eventletserver.socket_opts,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/rpc.py#n164 : executor='eventlet',
- https://opendev.org/openstack/cinder/src/branch/master/cinder/scheduler/manager.py#n26 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/scheduler/manager.py#n120 : eventlet.sleep(1)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/scheduler/manager.py#n179 : eventlet.sleep(1)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/ssh_utils.py#n23 : from eventlet import pools
- https://opendev.org/openstack/cinder/src/branch/master/cinder/ssh_utils.py#n59 : """A simple eventlet pool to hold ssh connections."""
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/functional/functional_helpers.py#n131 : with mock.patch('eventlet.sleep'):
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/__init__.py#n28 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/__init__.py#n35 : eventlet.monkey_patch(os=False)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/__init__.py#n37 : eventlet.monkey_patch()
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/api/contrib/test_services.py#n916 : body = {'binary': '*', 'prefix': 'eventlet.', 'level': 'debug'}
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/api/contrib/test_services.py#n970 : body = {'binary': '*', 'prefix': 'eventlet.'}
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/backup/drivers/test_backup_ceph.py#n81 : @mock.patch('eventlet.sleep', spec=True)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/backup/drivers/test_backup_ceph.py#n1048 : mock.patch('eventlet.tpool.Proxy') as mock_proxy:
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/backup/drivers/test_backup_google.py#n29 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/backup/drivers/test_backup_nfs.py#n29 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/backup/drivers/test_backup_s3.py#n28 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/backup/drivers/test_backup_swift.py#n28 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/backup/test_backup.py#n23 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/fake_utils.py#n19 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/scheduler/test_scheduler.py#n74 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/scheduler/test_scheduler.py#n90 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/scheduler/test_scheduler.py#n343 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/scheduler/test_scheduler.py#n358 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/scheduler/test_scheduler.py#n372 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/scheduler/test_scheduler.py#n395 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test.py#n29 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_image_utils.py#n538 : @mock.patch('eventlet.tpool.Proxy')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_image_utils.py#n701 : @mock.patch('eventlet.tpool.execute')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_image_utils.py#n867 : @mock.patch('eventlet.tpool.Proxy')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_image_utils.py#n908 : @mock.patch('eventlet.tpool.Proxy')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_image_utils.py#n940 : @mock.patch('eventlet.tpool.Proxy')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_image_utils.py#n974 : @mock.patch('eventlet.tpool.Proxy')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_image_utils.py#n1029 : @mock.patch('eventlet.tpool.Proxy')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_image_utils.py#n1064 : @mock.patch('eventlet.tpool.Proxy')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_image_utils.py#n1151 : @mock.patch('eventlet.tpool.Proxy')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_utils.py#n1471 : @mock.patch('eventlet.Semaphore')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/test_utils.py#n1487 : @mock.patch('eventlet.tpool.execute')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/dell_emc/sc/test_sc.py#n17 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/dell_emc/sc/test_sc.py#n253 : self.mock_sleep = self.mock_object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/dell_emc/sc/test_scapi.py#n20 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/dell_emc/sc/test_scapi.py#n5376 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n23 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n1108 : self.patch('eventlet.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n1776 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n1797 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n1828 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n1876 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n1924 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2031 : self.mock_object(eventlet, 'spawn')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2038 : eventlet.spawn.assert_called()
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2052 : self.mock_object(eventlet, 'spawn')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2055 : eventlet.spawn.assert_called()
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2057 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2084 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2107 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2129 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2190 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2216 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2250 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2286 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2305 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2331 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2357 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2389 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2421 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2451 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2486 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2521 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2556 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n2787 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3427 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3461 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3518 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3554 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3647 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3680 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3720 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3757 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3783 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3827 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n3950 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n4229 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n4390 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n4429 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n4480 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n4501 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n4516 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n4534 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n4558 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ds8k_proxy.py#n4578 : @mock.patch.object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ibm_flashsystem.py#n772 : self.sleeppatch = mock.patch('eventlet.greenthread.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/ibm/test_ibm_flashsystem_iscsi.py#n136 : self.sleeppatch = mock.patch('eventlet.greenthread.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/inspur/as13000/test_as13000_driver.py#n24 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/inspur/as13000/test_as13000_driver.py#n1248 : mock_el = self.mock_object(eventlet, 'sleep',
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/inspur/instorage/test_common.py#n21 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/inspur/instorage/test_fc_driver.py#n20 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/inspur/instorage/test_iscsi_driver.py#n20 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/inspur/instorage/test_replication.py#n21 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/nexenta/test_nexenta5_jsonrpc.py#n1200 : @mock.patch('eventlet.greenthread.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/synology/test_synology_common.py#n1010 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/synology/test_synology_common.py#n1032 : @mock.patch('eventlet.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_kaminario.py#n138 : self.patch('eventlet.sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_qnap.py#n23 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_qnap.py#n1293 : self.mock_object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_qnap.py#n1448 : self.mock_object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_qnap.py#n1513 : self.mock_object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_qnap.py#n1548 : self.mock_object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_qnap.py#n1654 : self.mock_object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_qnap.py#n1872 : self.mock_object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_qnap.py#n1927 : self.mock_object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_qnap.py#n1983 : self.mock_object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/test_qnap.py#n2041 : self.mock_object(eventlet, 'sleep')
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/drivers/toyou/test_acs5000.py#n26 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n28 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3542 : eventlet.tpool._nthreads = 10
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3543 : self.assertListEqual([], eventlet.tpool._threads)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3547 : self.assertEqual(20, eventlet.tpool._nthreads)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3548 : self.assertListEqual([], eventlet.tpool._threads)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3553 : self.assertNotEqual(100, eventlet.tpool._nthreads)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3554 : self.assertListEqual([], eventlet.tpool._threads)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3560 : self.assertEqual(100, eventlet.tpool._nthreads)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3561 : self.assertListEqual([], eventlet.tpool._threads)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3562 : eventlet.tpool._nthreads = 20
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3589 : t = eventlet.spawn(self.volume.create_volume,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/tests/unit/volume/test_volume.py#n3640 : t = eventlet.spawn(self.volume.create_volume, self.context,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/utils.py#n48 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/utils.py#n49 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/utils.py#n1007 : """Custom semaphore to workaround eventlet issues with multiprocessing."""
- https://opendev.org/openstack/cinder/src/branch/master/cinder/utils.py#n1024 : concurrent_processes: int) -> Union[eventlet.Semaphore,
- https://opendev.org/openstack/cinder/src/branch/master/cinder/utils.py#n1035 : return eventlet.Semaphore(limit)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api21.py#n23 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api21.py#n908 : eventlet.sleep(2)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api21.py#n925 : eventlet.sleep(datc.DEFAULT_SNAP_SLEEP)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api21.py#n935 : eventlet.sleep(1)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api21.py#n942 : eventlet.sleep(datc.DEFAULT_SI_SLEEP)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api21.py#n952 : eventlet.sleep(1)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api22.py#n23 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api22.py#n958 : eventlet.sleep(2)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api22.py#n975 : eventlet.sleep(datc.DEFAULT_SNAP_SLEEP)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api22.py#n985 : eventlet.sleep(1)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api22.py#n992 : eventlet.sleep(datc.DEFAULT_SI_SLEEP)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_api22.py#n1002 : eventlet.sleep(1)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/datera/datera_iscsi.py#n19 : from eventlet.green import threading
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/dell_emc/sc/storagecenter_api.py#n21 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/dell_emc/sc/storagecenter_api.py#n200 : eventlet.sleep(1)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/dell_emc/sc/storagecenter_api.py#n2361 : eventlet.sleep(sleep)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/dell_emc/sc/storagecenter_common.py#n15 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/dell_emc/sc/storagecenter_common.py#n1560 : eventlet.sleep(self.failback_timeout)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/hitachi/hbsd_replication.py#n20 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/hitachi/hbsd_rest_api.py#n23 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/hpe/nimble.py#n30 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/hpe/nimble.py#n247 : eventlet.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/hpe/nimble.py#n1263 : eventlet.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/hpe/nimble.py#n1469 : eventlet.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/hpe/nimble.py#n2118 : eventlet.sleep(DEFAULT_SLEEP)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_helper.py#n22 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_helper.py#n525 : eventlet.sleep(5)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_helper.py#n568 : eventlet.sleep(2)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_proxy.py#n65 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_proxy.py#n443 : eventlet.spawn(self._wait_flashcopy, src_luns, tgt_luns)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_proxy.py#n768 : eventlet.spawn(self._wait_flashcopy, [src_lun], [tgt_lun])
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_replication.py#n16 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_replication.py#n262 : eventlet.sleep(2)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_restclient.py#n21 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_restclient.py#n218 : eventlet.sleep(2)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/ibm_storage/ds8k_restclient.py#n307 : eventlet.sleep(2)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/storwize_svc/replication.py#n19 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/ibm/storwize_svc/storwize_svc_common.py#n23 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/inspur/as13000/as13000_driver.py#n26 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/inspur/as13000/as13000_driver.py#n789 : eventlet.sleep(1)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/inspur/instorage/instorage_common.py#n23 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/inspur/instorage/replication.py#n19 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/kaminario/kaminario_common.py#n22 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/kaminario/kaminario_common.py#n538 : eventlet.sleep(1)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/kaminario/kaminario_common.py#n736 : eventlet.sleep(1)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/netapp/dataontap/client/api.py#n25 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/netapp/dataontap/client/api.py#n26 : from eventlet import semaphore
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/nexenta/nfs.py#n19 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/nexenta/ns5/jsonrpc.py#n20 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/qnap.py#n27 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/qnap.py#n362 : eventlet.sleep(self.TIME_INTERVAL)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/qnap.py#n541 : eventlet.sleep(self.TIME_INTERVAL)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/qnap.py#n593 : eventlet.sleep(self.TIME_INTERVAL)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/qnap.py#n654 : eventlet.sleep(self.TIME_INTERVAL)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/qnap.py#n754 : eventlet.sleep(retrySleepTime)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/qnap.py#n888 : eventlet.sleep(self.TIME_INTERVAL)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/qnap.py#n902 : eventlet.sleep(self.TIME_INTERVAL)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/qnap.py#n914 : eventlet.sleep(self.TIME_INTERVAL)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/qnap.py#n2219 : eventlet.sleep(sleepSeconds)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/rbd.py#n28 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/san/san.py#n24 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/synology/synology_common.py#n31 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/synology/synology_common.py#n783 : eventlet.sleep(2)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/synology/synology_common.py#n801 : eventlet.sleep(2)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/drivers/toyou/acs5000/acs5000_common.py#n25 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/volume_utils.py#n41 : import eventlet
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/volume_utils.py#n42 : from eventlet import tpool
- https://opendev.org/openstack/cinder/src/branch/master/cinder/volume/volume_utils.py#n558 : eventlet.sleep(0)
- https://opendev.org/openstack/cinder/src/branch/master/cinder/wsgi/eventlet_server.py#n13 : """Methods for working with eventlet WSGI servers."""
- https://opendev.org/openstack/cinder/src/branch/master/cinder/zonemanager/drivers/brocade/brcd_fc_zone_client_cli.py#n25 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/zonemanager/drivers/cisco/cisco_fc_san_lookup_service.py#n20 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/cinder/zonemanager/drivers/cisco/cisco_fc_zone_client_cli.py#n24 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/admin/ts-HTTP-bad-req-in-cinder-vol-log.rst#n23 : call eventlet.debug.hub_multiple_reader_prevention(False)
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/configuration/block-storage/config-options.rst#n15 : cinder.wsgi.eventlet_server
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/contributor/high_availability.rst#n42 : It's good to keep in mind that Cinder threading model is based on eventlet's
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/contributor/threading.rst#n5 : through using the Python `eventlet <http://eventlet.net/>`_ and
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/contributor/threading.rst#n9 : switches can only occur when specific eventlet or greenlet library calls are
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/contributor/threading.rst#n23 : that trigger an eventlet context switch, the long-running thread will block
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/contributor/threading.rst#n26 : This scenario can be avoided by adding calls to the eventlet sleep method
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/contributor/threading.rst#n31 : from eventlet import greenthread
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/contributor/threading.rst#n36 : time module is patched through eventlet.monkey_patch(). To be explicit, we
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/contributor/threading.rst#n39 : MySQL access and eventlet
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/contributor/threading.rst#n43 : with eventlet. MySQL-python uses an external C library for accessing the MySQL
- https://opendev.org/openstack/cinder/src/branch/master/doc/source/contributor/threading.rst#n44 : database. Since eventlet cannot use monkey-patching to intercept blocking calls
- https://opendev.org/openstack/cinder/src/branch/master/etc/cinder/logging_sample.conf#n54 : [logger_eventletwsgi]
- https://opendev.org/openstack/cinder/src/branch/master/etc/cinder/logging_sample.conf#n57 : qualname = eventlet.wsgi.server
- https://opendev.org/openstack/cinder/src/branch/master/requirements.txt#n7 : eventlet>=0.30.1,!=0.32.0 # MIT

***

## Project: cinder-specs
- **Project:** Cinder Specs
  - **Is Eventlet globally deactivable for this project:** Yes
    *Reason: The specification suggests using a non-eventlet WSGI application based on the used web framework, which implies that Eventlet is not necessary and can be deactivated.*

  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The specification outlines specific alternatives to using Eventlet, reducing the need for significant changes. However, some configurations might still require adjustments.*


  - **Files Analyzed:**
    - **File:** `spec/liberty/non-eventlet-wsgi-app.rst`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The file explicitly mentions using Apache/Nginx under heavy load, which is better than Eventlet.
      - **File:** `spec/mitaka/ha-aa-tooz-locks.rst`
        - **Pattern:** Thread Safety for Non-Eventlet Usage
          *Description:* This section emphasizes the use of a regular thread (non-eventlet) to ensure safety in certain situations where eventlet could introduce issues.
    - **File:** `spec/pike/dynamic-log-levels.rst`
      - **Identified Pattern:** Eventlet Configuration
        *Description:* The configuration for "eventlet" is set to "ERROR", indicating that the project can function with Eventlet disabled or configured at a lower level.


  - **Overall Conclusion:**
    - **Summary of Key Points:** Cinder Specs specifies alternatives to using Eventlet, suggesting it's not necessary and can be deactivated.
    - **Potential Challenges:** Although the specification outlines specific alternatives, ensuring all configurations work correctly without Eventlet might require testing and adjustments.
    - **Recommendations:** Carefully review each configuration option outlined in the specs to ensure a smooth transition away from Eventlet.

Occurrences Found:
- https://opendev.org/openstack/cinder-specs/src/branch/master/specs/liberty/non-eventlet-wsgi-app.rst#n11 : https://blueprints.launchpad.net/cinder/+spec/non-eventlet-wsgi-app.
- https://opendev.org/openstack/cinder-specs/src/branch/master/specs/liberty/non-eventlet-wsgi-app.rst#n26 : * Apache/Nginx works better under the real heavy load then eventlet.
- https://opendev.org/openstack/cinder-specs/src/branch/master/specs/liberty/non-eventlet-wsgi-app.rst#n43 : Provide WSGI application based on used web-framework instead of eventlet. Leave
- https://opendev.org/openstack/cinder-specs/src/branch/master/specs/liberty/non-eventlet-wsgi-app.rst#n44 : eventlet-based WSGI application as a default option and make it configurable.
- https://opendev.org/openstack/cinder-specs/src/branch/master/specs/liberty/non-eventlet-wsgi-app.rst#n49 : Leave as is and use eventlet for REST API web serving. Use something like
- https://opendev.org/openstack/cinder-specs/src/branch/master/specs/liberty/non-eventlet-wsgi-app.rst#n88 : to Cinder API. By default, Cinder API will use eventlet and no deployer impact
- https://opendev.org/openstack/cinder-specs/src/branch/master/specs/liberty/non-eventlet-wsgi-app.rst#n124 : * Make sure usage of eventlet doesn't break WSGI in Nginx/Apache.
- https://opendev.org/openstack/cinder-specs/src/branch/master/specs/mitaka/ha-aa-tooz-locks.rst#n73 : regular thread (non-eventlet) to be safe from situations when eventlet
- https://opendev.org/openstack/cinder-specs/src/branch/master/specs/ocata/ha-aa-job-distribution.rst#n171 : since we are using an eventlet executor for the RPC server we don't have to
- https://opendev.org/openstack/cinder-specs/src/branch/master/specs/pike/dynamic-log-levels.rst#n169 : "eventlet": "ERROR"

***

## Project: os-brick
--- 

**Project:** os-brick
  - **Is Eventlet globally deactivable for this project:** Maybe
    *Reason for doubt: While some critical functionalities deeply use Eventlet, the presence of an Eventlet-specific argparse option suggests that it might be deactivable.*
  - **Estimated complexity of the migration:** 6
    *This level represents a moderate to low migration requiring minimal code refactoring.*
    *Factors for estimation: The project uses Eventlet extensively but with well-defined and isolated usage, reducing the overall complexity. However, removing Eventlet would require adjustments in context management, which could introduce complexity.*
  - **Files Analyzed:**
    - **File:** `os_brick/initiator/connectors/iscsi.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* This file includes the dependency on Eventlet's WSGI server, indicating a direct use of Eventlet.
    - **File:** `os_brick/tests/initiator/connectors/test_fibre_channel.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The test file uses a mock patch for Eventlet's sleep function, indicating the use of Eventlet within tests.
    - **File:** `os_brick/tests/windows/test_rbd.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          *Description:* The test file also uses a mock patch for an Eventlet-specific function (`EventletEvent.wait`), indicating its presence in tests.
    - **File:** `test-requirements.txt`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          *Description:* The requirements list includes the exact version of Eventlet, which is a clear indicator of dependency on the library.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet plays a significant role in os-brick's functionality, particularly with managing context and asynchronous operations using green threads and deferred tasks.
    - **Potential Challenges:** While the project uses Eventlet extensively, its removal could be mitigated by careful planning for alternative implementation strategies such as asyncio or other non-Eventlet based libraries. Ensuring that these alternatives can replicate existing functionality is crucial.
    - **Recommendations:** Carefully evaluate the replacement of core asynchronous mechanisms with alternative libraries, plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

---

Please note that due to a data source limitation (https://opendev.org/openstack/os-brick/src/branch/master/os_brick/initiator/connectors/iscsi.py#n601), the full extent of Eventlet usage within os-brick wasn't comprehensively analyzed in this response.

Occurrences Found:
- https://opendev.org/openstack/os-brick/src/branch/master/os_brick/initiator/connectors/iscsi.py#n601 : too much about the GIL or how the eventlets will handle the context
- https://opendev.org/openstack/os-brick/src/branch/master/os_brick/tests/initiator/connectors/test_fibre_channel.py#n517 : @mock.patch('eventlet.greenthread.sleep', mock.Mock())
- https://opendev.org/openstack/os-brick/src/branch/master/os_brick/tests/windows/test_rbd.py#n67 : @mock.patch('oslo_utils.eventletutils.EventletEvent.wait')
- https://opendev.org/openstack/os-brick/src/branch/master/test-requirements.txt#n20 : eventlet>=0.30.1,!=0.32.0 # MIT

***

## Project: python-cinderclient
---

- **Project:** python-cinderclient
  - **Is Eventlet globally deactivable for this project:** Yes
    *The presence of `from eventlet import sleep` in the code suggests that Eventlet is deactivable, as it allows users to opt out by not importing it.*
  - **Estimated complexity of the migration:** 4
    *This level represents a simple migration with minimal code changes.*
    *Factors for estimation: The usage of Eventlet's functions (e.g., `sleep`) is isolated and can be replaced or adapted without requiring significant refactoring, assuming the underlying functionality remains equivalent.*
  - **Files Analyzed:**
    - **File:** `cinderclient/client.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** The file imports `eventlet` and uses its features (e.g., `sleep`) in an asynchronous context, indicating a dependency on Eventlet's WSGI server.
    - **File:** `tests/test_client.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.sleep')` to mock Eventlet's sleep function, indicating that Eventlet is used in unit tests.
    - **File:** `cinderclient/contrib/monitoring/metric.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks and Scheduling
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.

- **Overall Conclusion:**
  - **Summary of Key Points:** Eventlet is used in a limited scope across the project, primarily for managing WSGI servers and in unit tests.
  - **Potential Challenges:** The primary challenge during migration would be replacing or adapting the isolated use cases without introducing significant complexity, assuming equivalent functionality can be maintained.
  - **Recommendations:** Perform thorough testing of alternatives (e.g., asyncio) to ensure compatibility and perform incremental refactoring to minimize disruptions.

Occurrences Found:
- https://opendev.org/openstack/python-cinderclient/src/branch/master/cinderclient/client.py#n46 : from eventlet import sleep

***
