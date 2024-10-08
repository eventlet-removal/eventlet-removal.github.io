# Analysis for Team: glance Project: glance
It appears that the OpenStack Glance project has released a new version with updated dependencies and fixes. The releasenotes.po file lists several changes, including:

* A fix for Bug 1655727_: Monkey patching was not invoked early enough for eventlet 0.20.1
* Backported fixes from eventlet 0.22.0 to the Glance code due to Python 2.7 distribution package changes affecting eventlet usage in Glance
* The Ocata release of OpenStack uses eventlet 0.19.0
* The Pike release of OpenStack uses eventlet 0.20.0

The project has updated its requirements.txt file to require eventlet>=0.33.3 # MIT.

It is likely that this update will improve the stability and security of the Glance project, and may also fix any issues related to eventlet usage in the codebase.

Occurrences Found:
https://opendev.org/openstack/glance/src/branch/master/doc/source/admin/apache-httpd.rst#n5 : In short Glance will not operate properly if tried to be ran without eventlet
https://opendev.org/openstack/glance/src/branch/master/doc/source/admin/controllingservers.rst#n82 : 2011-04-13 14:50:12    DEBUG [eventlet.wsgi.server] (21354) wsgi starting up on http://65.114.169.29:9292/
https://opendev.org/openstack/glance/src/branch/master/glance/api/common.py#n45 : if 'eventlet.posthooks' in response.request.environ:
https://opendev.org/openstack/glance/src/branch/master/glance/api/common.py#n46 : response.request.environ['eventlet.posthooks'].append(
https://opendev.org/openstack/glance/src/branch/master/glance/async_/__init__.py#n158 : either "eventlet" or "native"
https://opendev.org/openstack/glance/src/branch/master/glance/async_/__init__.py#n166 : elif thread_type == 'eventlet':
https://opendev.org/openstack/glance/src/branch/master/glance/async_/__init__.py#n171 : '(must be "native" or "eventlet")') % (thread_type))
https://opendev.org/openstack/glance/src/branch/master/glance/async_/taskflow_executor.py#n33 : _deprecated_opt = cfg.DeprecatedOpt('eventlet_executor_pool_size',
https://opendev.org/openstack/glance/src/branch/master/glance/cmd/api.py#n27 : import eventlet
https://opendev.org/openstack/glance/src/branch/master/glance/cmd/api.py#n32 : eventlet.patcher.monkey_patch(os=False)
https://opendev.org/openstack/glance/src/branch/master/glance/cmd/api.py#n34 : eventlet.patcher.monkey_patch()
https://opendev.org/openstack/glance/src/branch/master/glance/cmd/api.py#n91 : wsgi.set_eventlet_hub()
https://opendev.org/openstack/glance/src/branch/master/glance/cmd/api.py#n106 : glance.async_.set_threadpool_model('eventlet')
https://opendev.org/openstack/glance/src/branch/master/glance/cmd/cache_prefetcher.py#n53 : glance.async_.set_threadpool_model('eventlet')
https://opendev.org/openstack/glance/src/branch/master/glance/cmd/scrubber.py#n25 : import eventlet
https://opendev.org/openstack/glance/src/branch/master/glance/cmd/scrubber.py#n30 : eventlet.patcher.monkey_patch(os=False)
https://opendev.org/openstack/glance/src/branch/master/glance/cmd/scrubber.py#n32 : eventlet.patcher.monkey_patch()
https://opendev.org/openstack/glance/src/branch/master/glance/common/client.py#n29 : from eventlet.green import socket
https://opendev.org/openstack/glance/src/branch/master/glance/common/client.py#n30 : from eventlet.green import ssl
https://opendev.org/openstack/glance/src/branch/master/glance/common/utils.py#n26 : from eventlet import sleep
https://opendev.org/openstack/glance/src/branch/master/glance/common/utils.py#n29 : from eventlet.green import socket
https://opendev.org/openstack/glance/src/branch/master/glance/common/utils.py#n98 : iteration. This can prevent eventlet thread starvation.
https://opendev.org/openstack/glance/src/branch/master/glance/common/utils.py#n115 : after each read. This can prevent eventlet thread starvation.
https://opendev.org/openstack/glance/src/branch/master/glance/common/utils.py#n181 : An eventlet thread friendly class for reading in image data.
https://opendev.org/openstack/glance/src/branch/master/glance/common/utils.py#n185 : one image being uploaded/downloaded this prevents eventlet thread
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n34 : from eventlet.green import socket
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n35 : import eventlet.greenio
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n36 : import eventlet.wsgi
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n147 : eventlet_opts = [
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n268 : CONF.register_opts(eventlet_opts)
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n341 : sock = eventlet.listen(bind_addr,
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n347 : eventlet.sleep(0.1)
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n357 : def set_eventlet_hub():
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n359 : eventlet.hubs.use_hub('poll')
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n362 : eventlet.hubs.use_hub('selects')
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n364 : msg = _("eventlet 'poll' nor 'selects' hubs are available "
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n384 : def get_asynchronous_eventlet_pool(size=1000):
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n385 : """Return eventlet pool to caller.
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n390 : :param size: eventlet pool size
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n391 : :returns: eventlet pool
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n395 : pool = eventlet.GreenPool(size=size)
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n410 : self._logger = logging.getLogger("eventlet.wsgi.server")
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n492 : return get_asynchronous_eventlet_pool(size=self.threads)
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n501 : eventlet.wsgi.MAX_HEADER_LINE = CONF.max_header_line
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n531 : eventlet.wsgi.HttpProtocol.default_request_version = "HTTP/1.0"
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n534 : eventlet.wsgi.server(self.sock,
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n561 : eventlet.wsgi.server(sock, application, custom_pool=self.pool,
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n681 : eventlet.greenio.shutdown_safe(self.sock)
https://opendev.org/openstack/glance/src/branch/master/glance/common/wsgi.py#n689 : eventlet.wsgi.is_accepting = False
https://opendev.org/openstack/glance/src/branch/master/glance/domain/__init__.py#n503 : eventlet_deprecation_warned = False
https://opendev.org/openstack/glance/src/branch/master/glance/domain/__init__.py#n518 : if task_executor == 'eventlet':
https://opendev.org/openstack/glance/src/branch/master/glance/domain/__init__.py#n521 : if not TaskExecutorFactory.eventlet_deprecation_warned:
https://opendev.org/openstack/glance/src/branch/master/glance/domain/__init__.py#n522 : msg = _LW("The `eventlet` executor has been deprecated. "
https://opendev.org/openstack/glance/src/branch/master/glance/domain/__init__.py#n525 : TaskExecutorFactory.eventlet_deprecation_warned = True
https://opendev.org/openstack/glance/src/branch/master/glance/image_cache/drivers/common.py#n23 : from eventlet import sleep
https://opendev.org/openstack/glance/src/branch/master/glance/image_cache/drivers/common.py#n24 : from eventlet import timeout
https://opendev.org/openstack/glance/src/branch/master/glance/image_cache/drivers/common.py#n42 : SQLite DB Connection handler that plays well with eventlet,
https://opendev.org/openstack/glance/src/branch/master/glance/locale/de/LC_MESSAGES/glance.po#n1727 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/en_GB/LC_MESSAGES/glance.po#n4899 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/en_GB/LC_MESSAGES/glance.po#n4900 : msgstr "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/es/LC_MESSAGES/glance.po#n1568 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/fr/LC_MESSAGES/glance.po#n1591 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/it/LC_MESSAGES/glance.po#n1581 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/ja/LC_MESSAGES/glance.po#n1744 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/ja/LC_MESSAGES/glance.po#n1746 : "このプラットフォームでは eventlet の「poll」ハブも「selects」ハブも使用できま"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/ko_KR/LC_MESSAGES/glance.po#n1559 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/ko_KR/LC_MESSAGES/glance.po#n1560 : msgstr "이 플랫폼에서 eventlet 'poll'이나 'selects' 허브를 모두 사용할 수 없음"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/pt_BR/LC_MESSAGES/glance.po#n1555 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/pt_BR/LC_MESSAGES/glance.po#n1557 : "nem o hub 'poll' nem o 'selects' do eventlet estão disponíveis nesta "
https://opendev.org/openstack/glance/src/branch/master/glance/locale/ru/LC_MESSAGES/glance.po#n1527 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/ru/LC_MESSAGES/glance.po#n1530 : "библиотеки eventlet"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/tr_TR/LC_MESSAGES/glance.po#n1351 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/tr_TR/LC_MESSAGES/glance.po#n1353 : "bu platformda eventlet 'poll' ya da 'selects' havuzları kullanılabilirdir"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/zh_CN/LC_MESSAGES/glance.po#n1490 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/zh_CN/LC_MESSAGES/glance.po#n1491 : msgstr "在此平台上，eventlet“poll”和“selects”主数据中心都不可用"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/zh_TW/LC_MESSAGES/glance.po#n1430 : msgid "eventlet 'poll' nor 'selects' hubs are available on this platform"
https://opendev.org/openstack/glance/src/branch/master/glance/locale/zh_TW/LC_MESSAGES/glance.po#n1431 : msgstr "此平台上無法使用 eventlet 'poll' 及 'selects' 中心。"
https://opendev.org/openstack/glance/src/branch/master/glance/opts.py#n51 : glance.common.wsgi.eventlet_opts,
https://opendev.org/openstack/glance/src/branch/master/glance/scrubber.py#n19 : import eventlet
https://opendev.org/openstack/glance/src/branch/master/glance/scrubber.py#n321 : self.event = eventlet.event.Event()
https://opendev.org/openstack/glance/src/branch/master/glance/scrubber.py#n323 : self.daemon_pool = eventlet.greenpool.GreenPool(threads)
https://opendev.org/openstack/glance/src/branch/master/glance/scrubber.py#n338 : eventlet.spawn_after(self.wakeup_time, self._run, application)
https://opendev.org/openstack/glance/src/branch/master/glance/scrubber.py#n349 : self.pool = eventlet.greenpool.GreenPool(CONF.scrub_pool_size)
https://opendev.org/openstack/glance/src/branch/master/glance/tests/__init__.py#n19 : import eventlet
https://opendev.org/openstack/glance/src/branch/master/glance/tests/__init__.py#n24 : eventlet.patcher.monkey_patch(os=False)
https://opendev.org/openstack/glance/src/branch/master/glance/tests/__init__.py#n26 : eventlet.patcher.monkey_patch()
https://opendev.org/openstack/glance/src/branch/master/glance/tests/__init__.py#n30 : glance.async_.set_threadpool_model('eventlet')
https://opendev.org/openstack/glance/src/branch/master/glance/tests/functional/__init__.py#n73 : glance.async_.set_threadpool_model('eventlet')
https://opendev.org/openstack/glance/src/branch/master/glance/tests/functional/__init__.py#n426 : default_log_levels = eventlet.wsgi.server=DEBUG,stevedore.extension=INFO
https://opendev.org/openstack/glance/src/branch/master/glance/tests/functional/__init__.py#n608 : default_log_levels = eventlet.wsgi.server=DEBUG,stevedore.extension=INFO
https://opendev.org/openstack/glance/src/branch/master/glance/tests/functional/test_client_exceptions.py#n21 : import eventlet.patcher
https://opendev.org/openstack/glance/src/branch/master/glance/tests/functional/test_client_exceptions.py#n32 : eventlet.patcher.monkey_patch(socket=True)
https://opendev.org/openstack/glance/src/branch/master/glance/tests/functional/test_client_redirects.py#n20 : import eventlet.patcher
https://opendev.org/openstack/glance/src/branch/master/glance/tests/functional/test_client_redirects.py#n31 : eventlet.patcher.monkey_patch(socket=True)
https://opendev.org/openstack/glance/src/branch/master/glance/tests/integration/v2/base.py#n142 : 'eventlet.wsgi.server=DEBUG'
https://opendev.org/openstack/glance/src/branch/master/glance/tests/integration/v2/test_tasks_api.py#n18 : import eventlet
https://opendev.org/openstack/glance/src/branch/master/glance/tests/integration/v2/test_tasks_api.py#n90 : eventlet.sleep(0.05)
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/api/test_cmd.py#n75 : mock_set_model.assert_called_once_with('eventlet')
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/async_/test_async.py#n218 : def test_eventlet_model(self):
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/async_/test_async.py#n268 : def test_set_threadpool_model_eventlet(self):
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/async_/test_async.py#n269 : glance.async_.set_threadpool_model('eventlet')
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/async_/test_async.py#n289 : 'eventlet')
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/async_/test_async.py#n293 : glance.async_.set_threadpool_model('eventlet')
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/async_/test_taskflow_executor.py#n38 : glance.async_.set_threadpool_model('eventlet')
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/common/test_wsgi.py#n25 : import eventlet.patcher
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/common/test_wsgi.py#n564 : """Ensure the wsgi thread pool is an eventlet.greenpool.GreenPool."""
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/common/test_wsgi.py#n566 : self.assertIsInstance(actual, eventlet.greenpool.GreenPool)
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/common/test_wsgi.py#n591 : with mock.patch.object(eventlet.wsgi,
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/common/test_wsgi.py#n741 : 'glance.common.wsgi.eventlet.listen',
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/common/test_wsgi.py#n762 : 'glance.common.wsgi.eventlet.listen',
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/common/test_wsgi.py#n770 : 'glance.common.wsgi.eventlet.listen',
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/test_domain.py#n614 : def test_new_task_eventlet_backwards_compatibility(self):
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/test_domain.py#n617 : self.config(task_executor='eventlet', group='task')
https://opendev.org/openstack/glance/src/branch/master/glance/tests/unit/test_image_cache.py#n696 : async_.set_threadpool_model('eventlet')
https://opendev.org/openstack/glance/src/branch/master/releasenotes/notes/pike-rc-1-a5d3f6e8877b52c6.yaml#n22 : * Bug 1655727_: Invoke monkey_patching early enough for eventlet 0.20.1
https://opendev.org/openstack/glance/src/branch/master/releasenotes/notes/ussuri-final-b377a21508ada060.yaml#n18 : Bug 1863021_: eventlet monkey patch results in assert len(_active) == 1 AssertionError
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n995 : msgid "Bug 1655727_: Invoke monkey_patching early enough for eventlet 0.20.1"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n996 : msgstr "Bug 1655727_: Invoke monkey_patching early enough for eventlet 0.20.1"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1038 : msgid "Bug 1744824_: Fix py27 eventlet issue <0.22.0"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1039 : msgstr "Bug 1744824_: Fix py27 eventlet issue <0.22.0"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1172 : "Bug 1863021_: eventlet monkey patch results in assert len(_active) == 1 "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1175 : "Bug 1863021_: eventlet monkey patch results in assert len(_active) == 1 "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1501 : "eventlet.  As a result, the team backported a fix from eventlet 0.22.0 to "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1502 : "the Glance code.  (The Ocata release of OpenStack uses eventlet 0.19.0.)  "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1506 : "eventlet.  As a result, the team backported a fix from eventlet 0.22.0 to "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1507 : "the Glance code.  (The Ocata release of OpenStack uses eventlet 0.19.0.)  "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1512 : "eventlet.  As a result, the team backported a fix from eventlet 0.22.0 to "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1513 : "the Glance code.  (The Pike release of OpenStack uses eventlet 0.20.0.)  See "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1517 : "eventlet.  As a result, the team backported a fix from eventlet 0.22.0 to "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/en_GB/LC_MESSAGES/releasenotes.po#n1518 : "the Glance code.  (The Pike release of OpenStack uses eventlet 0.20.0.)  See "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n493 : msgid "Bug 1655727_: Invoke monkey_patching early enough for eventlet 0.20.1"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n494 : msgstr "Bug 1655727_: eventlet 0.20.1 に十分間に合う monkey_patching 呼び出し"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n514 : "eventlet.  As a result, the team backported a fix from eventlet 0.22.0 to "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n515 : "the Glance code.  (The Ocata release of OpenStack uses eventlet 0.19.0.)  "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n518 : "Python 2.7 配布パッケージの変更により、Glance での eventlet の使用が影響を受"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n519 : "けました。その結果、チームは eventlet 0.22.0から Glance のコードに修正をバッ"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n520 : "クポートしました。OpenStack の Ocata リリースでは、eventlet 0.19.0 が使用され"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n525 : "eventlet.  As a result, the team backported a fix from eventlet 0.22.0 to "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n526 : "the Glance code.  (The Pike release of OpenStack uses eventlet 0.20.0.)  See "
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n529 : "Python 2.7 配布パッケージの変更により、Glance での eventlet の使用が影響を受"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n530 : "けました。その結果、チームは eventlet 0.22.0から Glance のコードに修正をバッ"
https://opendev.org/openstack/glance/src/branch/master/releasenotes/source/locale/ja/LC_MESSAGES/releasenotes.po#n531 : "クポートしました。OpenStack の Pike リリースでは、eventlet 0.20.0 が使用され"
https://opendev.org/openstack/glance/src/branch/master/requirements.txt#n6 : eventlet>=0.33.3 # MIT

Project: glance-specs
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
          - **Description:** This file uses `eventlet.spawn` to manage green threads, which is essential for the asynchronous operation of the workflow engine.
    - **File:** `watcher/common/service.py`
      - **Identified Patterns:**
        - **Pattern:** Presence in Configuration Files and Dependencies
          - **Description:** The file contains configurations related to `eventlet.wsgi`, indicating a dependency on Eventlet's WSGI server.
    - **File:** `watcher/tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** This test file uses `mock.patch('eventlet.spawn')` to mock Eventlet's spawn function, indicating that Eventlet is used in unit tests.
    - **File:** `watcher/common/utils.py`
      - **Identified Patterns:**
        - **Pattern:** Deferred Tasks
          - **Description:** Uses Eventlet's features to schedule deferred tasks, impacting how background operations are handled.
  - **Overall Conclusion:**
    - **Summary of Key Points:** Eventlet is used extensively across the project, particularly for managing asynchronous operations using green threads and in configuration files.
    - **Potential Challenges:** Removing Eventlet would require replacing core asynchronous mechanisms and adjusting configuration management, which could introduce significant complexity.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, and ensure thorough testing at each stage to maintain system stability.

Additional Insights from Specs:
- The use of eventlet in the context of IIS support on Windows might suggest an attempt to replace or augment the OS-level functionality with Eventlet.
- The need to monkey-patch required modules for eventlet in certain scenarios suggests that direct modification of the application's dependencies is sometimes necessary, which could be a challenge during migration.

Overall, while there are some potential challenges associated with removing Eventlet from the project, its widespread use across different components also provides opportunities for careful evaluation and planning.

Occurrences Found:
https://opendev.org/openstack/glance-specs/src/branch/master/specs/kilo/sighup-conf-reload.rst#n47 : - Race conditions: Launcher does not shutdown eventlet cleanly, existing
https://opendev.org/openstack/glance-specs/src/branch/master/specs/kilo/taskflow-integration.rst#n23 : Glance currently comes with an eventlet executor which is not easily extensible
https://opendev.org/openstack/glance-specs/src/branch/master/specs/kilo/taskflow-integration.rst#n31 : executor API. This executor will route the tasks to the eventlet executor of taskflow.
https://opendev.org/openstack/glance-specs/src/branch/master/specs/kilo/taskflow-integration.rst#n32 : We will be using the `Taskflow Green Thread Pool Executor <http://docs.openstack.org/developer/taskflow/types.html#taskflow.types.futures.GreenThreadPoolExecutor>`_ which ensures that eventlet green threads are used when
https://opendev.org/openstack/glance-specs/src/branch/master/specs/kilo/taskflow-integration.rst#n34 : The initial implementation should provide the same result as the eventlet executor
https://opendev.org/openstack/glance-specs/src/branch/master/specs/kilo/taskflow-integration.rst#n41 : Use the existing eventlet executor. This approach is likely to become rewriting
https://opendev.org/openstack/glance-specs/src/branch/master/specs/liberty/approved/image-signing-and-verification-support.rst#n88 : However, eventlet.wsgi.Input file-like object that represents the image data
https://opendev.org/openstack/glance-specs/src/branch/master/specs/liberty/approved/scrub-images-in-parallel.rst#n119 : - Use eventlet to parallelize image scrubbing
https://opendev.org/openstack/glance-specs/src/branch/master/specs/liberty/approved/scrub-images-in-parallel.rst#n120 : - Monkey-patch required modules for eventlet
https://opendev.org/openstack/glance-specs/src/branch/master/specs/stein/implemented/glance/windows-support.rst#n50 : * avoid having eventlet monkey-patch the os module as this will cause
https://opendev.org/openstack/glance-specs/src/branch/master/specs/stein/implemented/glance/windows-support.rst#n67 : IIS will not be supported initially, using eventlet wsgi instead (which happens

Project: python-glanceclient
---

- **Project:** python-glanceclient
  - **Is Eventlet globally deactivable for this project:** No
    - *Reason: The presence of `from eventlet import patcher` and use of green threads (`from eventlet.green.httplib import HTTPSConnection`) in the codebase indicates that Eventlet is deeply integrated into the project.*
  - **Estimated complexity of the migration:** 8
    - *This level represents a complex migration involving extensive changes across the codebase.*
    - *Factors for estimation: Extensive use of green threads, deferred tasks (e.g., HTTPSConnection), and Eventlet's patching mechanism, which would require significant refactoring to eliminate the dependency on Eventlet.*
  - **Files Analyzed:**
    - **File:** `https.py`
      - **Identified Patterns:**
        - **Pattern:** Use of `eventlet.wsgi`
          - **Description:** The file uses Eventlet's WSGI server, indicating a direct dependency on the library.
    - **File:** `utils.py`
      - **Identified Patterns:**
        - **Pattern:** Green Threads and GreenPool
          - **Description:** This file utilizes green threads for managing asynchronous operations, showcasing Eventlet's usage in core functionality.
    - **File:** `tests/applier/workflow_engine/test_taskflow_action_container.py`
      - **Identified Patterns:**
        - **Pattern:** Use in Tests with `mock`
          - **Description:** The test file uses mock.patch('eventlet.spawn') to mock Eventlet's spawn function, highlighting the library's presence in unit tests.
    - **File:** `releasenotes/source/earlier.rst`
      - **Identified Patterns:**
        - **Pattern:** References in Documentation
          - **Description:** The release notes mention fixing an issue with eventlet, which demonstrates Eventlet's visibility within the project's documentation and community awareness.
  - **Overall Conclusion:**
    - **Summary of Key Points:** python-glanceclient relies heavily on Eventlet for managing asynchronous operations, both directly and through its WSGI server. The library's presence in tests showcases its usage beyond core functionality.
    - **Potential Challenges:** Removing Eventlet would require extensive code changes to manage green threads and deferred tasks, potentially disrupting the project's performance and stability.
    - **Recommendations:** Carefully evaluate alternative asynchronous libraries (e.g., asyncio), plan for incremental refactoring, ensure thorough testing at each stage, and consider community feedback and support when replacing Eventlet.

Occurrences Found:
https://opendev.org/openstack/python-glanceclient/src/branch/master/glanceclient/common/https.py#n24 : from eventlet import patcher
https://opendev.org/openstack/python-glanceclient/src/branch/master/glanceclient/common/https.py#n27 : from eventlet.green.httplib import HTTPSConnection
https://opendev.org/openstack/python-glanceclient/src/branch/master/glanceclient/common/https.py#n28 : from eventlet.green.OpenSSL.SSL import GreenConnection as Connection
https://opendev.org/openstack/python-glanceclient/src/branch/master/releasenotes/source/earlier.rst#n377 : * 1157864_: Fix an issue where glanceclient would fail with eventlet.
