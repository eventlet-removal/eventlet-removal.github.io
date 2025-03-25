---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: dashboard
title: Preparing for Migration
permalink: /guide/preparing-for-migration/
description: Learn how to prepare your codebase for migration away from Eventlet. This guide covers auditing your code, identifying Eventlet usage patterns, and creating a strategic migration plan for your specific application.
keywords: eventlet code audit, migration planning, identifying eventlet usage, eventlet patterns, code analysis, migration strategy, code preparation
og_type: article
og_title: Preparing Your Codebase for Eventlet Migration - Comprehensive Planning Guide
og_description: Step-by-step instructions for auditing your codebase, identifying Eventlet usage patterns, and creating a tailored migration strategy.
---
<section>
    <h1 class="text-4xl font-bold">Preparing for Migration</h1>
    <p class="mt-10 text-xl">Before​ starting your migration it is important to take inventory of your use cases with Eventlet and so of your needs. This chapter help you to make this inventory.</p>
    
    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#locate-eventlet-usages" class="text-cyan-400 hover:underline">Locate your Eventlet Usages</a></li>
                <li><a href="#audit-your-code" class="text-cyan-400 hover:underline">Audit your Code</a></li>
                <li><a href="#is-eventlet-disablable" class="text-cyan-400 hover:underline">Is Eventlet Disablable?</a></li>
                <li><a href="#choosing-an-alternative" class="text-cyan-400 hover:underline">Choosing an Alternative</a></li>
                <li><a href="#split-your-works" class="text-cyan-400 hover:underline">Split your Works</a></li>
                <li><a href="#migrating-applications-with-internal-private-apis" class="text-cyan-400 hover:underline">Migrating Applications with Internal Private APIs</a></li>
                <li><a href="#implementation-example" class="text-cyan-400 hover:underline">Implementation Example of Migrating Applications with Internal Private APIs</a></li>
            </ul>
        </div>
    </div>
    
</section>

<!-- locate -->
<section>
    <h2 id="locate-eventlet-usages" class="mt-10 text-3xl font-bold mb-6">Locate your Eventlet Usages <a href="#locate-eventlet-usages" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">The first thing to do to prepare your migration is to locate all the Eventlet usages in your code base. That's pretty easy to realize by using a shell command. The snippet below should help you to collect 99 percents of your occurences.</p>
    <pre class="line-numbers"><code class="language-bash">#!/bin/bash
echo "Searching Eventlet Instances..."
grep -rnw 'import eventlet' . > eventlet_instances.txt
grep -rnw 'from eventlet' . >> eventlet_instances.txt
grep -rnw 'eventlet.spawn' . >> eventlet_instances.txt
grep -rnw 'eventlet.monkey_patch' . >> eventlet_instances.txt
echo "Results saved at eventlet_instances.txt"</code></pre>
</section>

<!-- code audit -->
<section>
    <h2 id="audit-your-code" class="text-3xl font-bold mb-6 mt-10">Audit your Code <a href="#audit-your-code" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="text-xl mt-4">In accordance with <a href="{{ site.baseurl }}{% link guide/eventlet.md %}#common-usages" class="text-cyan-400">the common scenario of Eventlet previously identified</a>, the elements below will help you to triagge the occurences previously found. The scenario below will guide you to audit your code and will help you to identify which kind of usage is present in your code base.</p>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-10 mt-10">
        <!-- WSGI Server -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">WSGI Server</h3>
            <p class="mb-2"><strong>Objective:</strong> Serve Python web applications asynchronously.</p>
            <p class="mb-2"><strong>Usage:</strong> Eventlet is used to create a WSGI server capable of handling a large number of simultaneous connections with low latency.</p>
            <pre class="line-numbers"><code class="language-python">from eventlet import wsgi, listen

def simple_app(environ, start_response):
    start_response('200 OK', [('Content-Type', 'text/plain')])
    return [b'Hello, World!\n']

wsgi.server(listen(('', 8080)), simple_app)</code></pre>
            <p class="mb-2"><strong>Role:</strong> Efficiently manages incoming HTTP requests without blocking the main thread.</p>
            <p><a href="https://eventlet.readthedocs.io/en/latest/modules/wsgi.html" class="text-cyan-400 underline">Documentation</a></p>
        </div>
        <!-- Asynchronous Network Calls -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Asynchronous Network Calls</h3>
            <p class="mb-2"><strong>Objective:</strong> Make network calls without blocking the main execution.</p>
            <p class="mb-2"><strong>Usage:</strong> Used to make HTTP requests to REST APIs without waiting for the response.</p>
            <pre class="line-numbers"><code class="language-python">import eventlet
from eventlet.green import urllib2

def fetch_url(url):
    return urllib2.urlopen(url).read()

urls = ['http://example.com', 'http://example.org']
pool = eventlet.GreenPool()

for body in pool.imap(fetch_url, urls):
    print(body)</code></pre>
            <p class="mb-2"><strong>Role:</strong> Improves performance by allowing the application to continue running during network calls.</p>
            <p><a href="https://eventlet.readthedocs.io/en/latest/reference/api/eventlet.green.urllib.html" class="text-cyan-400 underline">Documentation</a></p>
        </div>
        <!-- Background Execution of Long Tasks -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Background Execution of Long Tasks</h3>
            <p class="mb-2"><strong>Objective:</strong> Execute long tasks in the background.</p>
            <p class="mb-2"><strong>Usage:</strong> Used for operations like processing large files or performing intensive calculations.</p>
            <pre class="line-numbers"><code class="language-python">import eventlet

def long_running_task():
    # Simulate a long-running task
    eventlet.sleep(10)
    print("Task completed")

eventlet.spawn(long_running_task)
print("Main thread continues to run")</code></pre>
            <p class="mb-2"><strong>Role:</strong> Maintains application responsiveness by executing long tasks without blocking the main thread.</p>
            <p><a href="https://eventlet.readthedocs.io/en/latest/basic_usage.html#eventlet.spawn" class="text-cyan-400 underline">Documentation</a></p>
        </div>
        <!-- Deferred Task Management -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Deferred Task Management</h3>
            <p class="mb-2"><strong>Objective:</strong> Manage deferred tasks that can be executed later.</p>
            <p class="mb-2"><strong>Usage:</strong> Used for complex workflows where some tasks depend on the completion of others.</p>
            <pre class="line-numbers"><code class="language-python">from eventlet.event import Event

def deferred_task(evt):
    print("Waiting for event")
    evt.wait()
    print("Event occurred, executing task")

evt = Event()
eventlet.spawn_after(5, deferred_task, evt)
eventlet.sleep(5)
evt.send()</code></pre>
            <p class="mb-2"><strong>Role:</strong> Facilitates the management of task dependencies in complex workflows.</p>
            <p><a href="https://eventlet.readthedocs.io/en/latest/basic_usage.html#eventlet.spawn_after" class="text-cyan-400 underline">Documentation</a></p>
        </div>
        <!-- Green Thread Management -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Green Thread Management</h3>
            <p class="mb-2"><strong>Objective:</strong> Manage concurrency with green threads.</p>
            <p class="mb-2"><strong>Usage:</strong> Used to switch between different tasks without creating new system threads.</p>
            <pre class="line-numbers"><code class="language-python">import eventlet

def task1():
    while True:
        print("Task 1 running")
        eventlet.sleep(1)

def task2():
    while True:
        print("Task 2 running")
        eventlet.sleep(1)

eventlet.spawn(task1)
eventlet.spawn(task2)</code></pre>
            <p class="mb-2"><strong>Role:</strong> Provides efficient resource management by avoiding the creation of many system threads.</p>
            <p><a href="https://eventlet.readthedocs.io/en/latest/basic_usage.html#greenthread-spawn" class="text-cyan-400 underline">Documentation</a></p>
        </div>
        <!-- Socket Compatibility -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Socket Compatibility</h3>
            <p class="mb-2"><strong>Objective:</strong> Provide an API compatible with Python's standard sockets.</p>
            <p class="mb-2"><strong>Usage:</strong> Facilitates the migration of existing applications to an asynchronous model.</p>
            <pre class="line-numbers"><code class="language-python">import eventlet
from eventlet.green import socket

def echo_server(port):
    server = socket.socket()
    server.bind(('0.0.0.0', port))
    server.listen(5)
    while True:
        client_socket, addr = server.accept()
        eventlet.spawn(handle_client, client_socket)

def handle_client(client_socket):
    while True:
        data = client_socket.recv(1024)
        if not data:
            break
        client_socket.sendall(data)
    client_socket.close()

echo_server(6000) </code></pre>
            <p class="mb-2"><strong>Role:</strong> Simplifies the integration of sockets in an asynchronous environment.</p>
            <p><a href="https://eventlet.readthedocs.io/en/latest/reference/api/eventlet.green.html#module-eventlet.green.socket" class="text-cyan-400 underline">Documentation</a></p>
        </div>
        <!-- WebSocket Support -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">WebSocket Support</h3>
            <p class="mb-2"><strong>Objective:</strong> Manage WebSocket connections for real-time, bidirectional communication.</p>
            <p class="mb-2"><strong>Usage:</strong> Used for applications requiring real-time updates between the server and the client.</p>
            <pre class="line-numbers"><code class="language-python">import eventlet
from eventlet.websocket import WebSocket, websocket

@websocket('/echo')
def echo(ws):
    while True:
        msg = ws.wait()
        if msg is None:
            break
        ws.send(msg)

listener = eventlet.listen(('0.0.0.0', 7000))
ws = WebSocket(listener)
eventlet.spawn(ws.accept)</code></pre>
            <p class="mb-2"><strong>Role:</strong> Enables interactive, real-time communication between the server and clients.</p>
            <p><a href="https://eventlet.readthedocs.io/en/latest/reference/api/eventlet.html#module-eventlet.websocket" class="text-cyan-400 underline">Documentation</a></p>
        </div>
        <!-- Integration with Other Libraries -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Integration with Other Libraries</h3>
            <p class="mb-2"><strong>Objective:</strong> Integrate Eventlet with other libraries to provide asynchronous functionality.</p>
            <p class="mb-2"><strong>Usage:</strong> Used for queue systems or databases.</p>
            <pre class="line-numbers"><code class="language-python">import eventlet
from eventlet.green import threading
import queue

def worker():
    while True:
        item = q.get()
        if item is None:
            break
        print(f"Processing {item}")
        q.task_done()

q = queue.Queue()
threads = []
for i in range(3):
    t = threading.Thread(target=worker)
    t.start()
    threads.append(t)

for item in range(10):
    q.put(item)

q.join()

for i in range(3):
    q.put(None)
for t in threads:
    t.join()</code></pre>
            <p class="mb-2"><strong>Role:</strong> Integrates asynchronous functionality into systems requiring efficient resource management.</p>
            <p><a href="https://eventlet.readthedocs.io/en/latest/patching.html#monkeypatching-the-standard-library" class="text-cyan-400 underline">Documentation</a></p>
        </div>
    </div>
</section>

<!-- is eventlet disablable -->
<section>
    <h2 id="is-eventlet-disablable" class="mt-10 text-3xl font-bold mb-6">Is Eventlet Disablable? <a href="#is-eventlet-disablable" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">Depending on your project kind, time to time, your application may support running with or without Eventlet. As said in the <a href="{{ site.baseurl }}{% link guide/eventlet.md %}">Eventlet section</a> of this guide, Eventlet is often used to make existing code asynchronous, meaning, that your application could potentially run without Eventlet but in a blocking way. That mean that some Eventlet usages can be disabled.</p>
    <p class="mt-10 text-xl">This type of scenario can ease your migration. You just, if possible, of toggle monkey patching off to switch in synchronous mode. Starting from that point, you would just to transform your calls with the proposed alternatives to make them async again, without having to carre about Eventlet and its side effects in the middle.</p>
</section>


<!-- choose an alternative -->
<section>
    <h2 id="choosing-an-alternative" class="mt-10 text-3xl font-bold mb-6">Choosing an Alternative <a href="#choosing-an-alternative" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">This guide made the choice of AsyncIO and Threading as alternatives. We won't cover the other solutions like <a href="https://curio.readthedocs.io/en/latest/" class="text-cyan-400" target="_blank">Curio</a>, <a href="http://www.gevent.org/" class="text-cyan-400" target="_blank">Gevent</a>, <a href="https://www.tornadoweb.org/en/stable/" class="text-cyan-400" target="_blank">Tornado</a>, <a href="https://twisted.org/" class="text-cyan-400" target="_blank">Twisted</a>, however, if you volunteer to document them, then, feel free to propose <a href="{{ site.github_repo }}" class="text-cyan-400">a pull request</a> to update that guide.</p>
    <p class="mt-4 text-xl">This section aim to help you to determine when using AsyncIO or Threading.</p>
    <p class="mt-4 text-xl">AsyncIO is great but is explicit nature can force you to rewrite entirely your application. Indeed, when you use AsyncIO, all your stack have to be scattered with the <code class="language-python">await</code> and <code class="language-python">async</code> keywords.</p>
    <p class="mt-4 text-xl">If your application is just a few lines of code and a couple of sub-modules that cannot be an issue. But, if your implicit async call made with Eventlet are deeply rooted in your call stack then refactoring your application with AsyncIO could become really painful.</p>
    <p class="mt-4 text-xl">In parallel, AsyncIO is tailored for asynchronous network I/O, making your application more efficient and less resource consuming.</p>
    <p class="mt-4 text-xl">Threads, on their side, can consume resources even if your application do nothing and is waiting for a network I/O. But threads won't force you to rewrite all your application due to a deeply rooted Eventlet async call.</p>
    <p class="mt-4 text-xl">So what is the solution? If your application is not composed of tons of sub-modules with deeply rooted Eventlet async calls, and, if these calls are mostly used to realize non-blocking network calls, then go straight to AsyncIO, else, prefer Threads.</p>
    <p class="mt-4 text-xl">Ideally it shouldn't be a all or nothing decision. If you the time and the resources to rewrite your use cases with the right solution, then, that would not be a problem to mix the solutions together. By example, if you are using the WSGI feature from Eventlet to implement an asynchronous server, then, you could consider migrate to the AsyncIO ecosystem and by example choosing to use <a href="https://docs.aiohttp.org/en/stable/" class="text-cyan-400">aiohttp</a> and <a href="https://www.uvicorn.org/" class="text-cyan-400">uvicorn</a> to refactor your WSGI scenario, and, in parallel you could consider to use threading to manage your background tasks that are CPU bound.</p>
    <p class="mt-4 text-xl">There is one last option that we need to speak about in this section. This option is named <a href="https://awaitlet.sqlalchemy.org/en/latest/" class="text-cyan-400">awaitlet</a>. Awaitlet can allow you to mitigate the depth of your AsyncIO refactor. Awaitlet can be used as a glue between default synchronous code and part of your code that need async mechanisms and AsyncIO. Awaitlet would limit the impact of an AsyncIO refactor, and would allow you a more incremental migration scenario. We will describe <a href="{{ site.baseurl }}{% link guide/depth.md %}" class="text-cyan-400">how to manage the depth</a> of your refactor later in this guide.</p>
    <p class="mt-4 text-xl">The graph below summarize your decision tree based on the solutions retained in this guide:</p>
    <div class="mt-10 mermaid">
        %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#312e81', 'primaryTextColor': '#fff', 'primaryBorderColor': '#433e99', 'lineColor': '#ffffff', 'secondaryColor': '#312e81', 'tertiaryColor': '#312e81' }}}%%
        flowchart TD
            A[Choosing between available alternatives] --> B{Application composed of<br>many sub-modules?}
            B -->|Yes| C{Time and resources<br>for implementation?}
            B -->|No| D[Consider AsyncIO]
            D --> Z
            
            C -->|Yes| D
            C -->|No| F{Eventlet calls are deeply<br>rooted in your application?}
            
            F -->|Yes| H{Need to limit<br>refactoring impact?}
            F -->|No| D[Consider AsyncIO]
            
            H -->|Yes| I{Incremental migration<br>to AsyncIO is something<br>that is possible?}
            H -->|No| D
            
            I --> |Yes| K[Consider using<br>Awaitlet to migrate<br>to AsyncIO incrementally]
            I --> |No| X[Consider Threading]

            K --> Z
            X --> Z

            Z[Choice made]
    </div>
</section>

<!-- split your work -->
<section>
    <h2 id="split-your-works" class="mt-10 text-3xl font-bold mb-6">Split your Works <a href="#split-your-works" class="text-cyan-400 text-xl">🔗</a></h2>
    <h3 class="mt-10 text-2xl font-bold mb-6">Services First</h3>
    <p class="mt-4 text-xl">Time to time your application can be composed of one or many services and of one or many libraries. All these deliverables represent your application.</p>
    <p class="mt-4 text-xl">The majority of the time that's your services that monkey patch your environment, meaning that the occurences of Eventlet in your libraries is often just the result of a monkey patching at an higher level in your stack. If you migrate your libraries first you will surely broke something in your service or you will surely face unexpected side effects.</p>
    <p  class="mt-4 text-xl">The right way to migrate this kind of scenario is to first completelly migrate your services, and once done, to drop the Eventlet occurences in your libraries.As we explained <a href=" {{ site.baseurl }}{% link guide/eventlet.md %}#promises" class="text-cyan-400">ealier in this guide the magic of Eventlet do not keep</a> its promises and surely had side effects on all your stack. It is not rare to see libraries fixed with patches just to run with Eventlet (to avoid race conditions or side effects of monkey patching).</p>
    <p  class="mt-4 text-xl text-yellow-300">Migrate your services first, and, then, tackle your libraries.</p>
    <h3 class="mt-10 text-2xl font-bold mb-6">Isolate your Application into Functional Parts</h3>
    <p class="mt-4 text-xl">Sometimes a single codebase hosts different services or different parts of a service, endpoints, and entrypoints, which are not necessarly executed together in the same process. Identifying these functional parts can help you plan your migration in isolated, functionally autonomous sub-parts that are therefore easier to migrate.</p>
    <p class="mt-4 text-xl">Identifying these parts will allow you to incrementally migrating your deliverables without impacting everything in your app at the same time. It will give you more control and will allow you a more fine grained migration.</p>
    <p class="mt-4 text-xl">For more detailed guidance on handling applications with interconnected components, be sure to review the <a href="#migrating-shared-private-apis" class="text-cyan-400">Migrating Applications with Internal Private APIs</a> section later in this guide.</p>
    <p class="mt-4 text-xl text-yellow-300">Divide and conquer.</p>
    <h3 class="mt-10 text-2xl font-bold mb-6">Deprecate First</h3>
    <p class="mt-4 text-xl">If your deliverables rely on Eventlet, there is chances that it also own parameters or config options related to Eventlet. We have to care about our end users and we have to avoid abrupt removal of these things. Removing them without advising firt could abruply broke their configurations or their environments, for this reason, we have to respect a deprecation period first.</p>
    <p class="mt-4 text-xl">This deprecation period will make your end users less nervous and frustrated. This deprecation period will show that you care about about your end users.</p>
    <p  class="mt-4 text-xl text-yellow-300">Explicit is better than implicit.</p>
    <h3 class="mt-10 text-2xl font-bold mb-6">Secondly Executors</h3>
    <p class="mt-4 text-xl">In general Greenthread are easy to identify and so easy to flush out. Adressing them first will drastically reduce the scope of Eventlet in your application, leaving you more brain resources to handle the complexe scenarios introduced by monkey patching and implicit async.</p>
    <p  class="mt-4 text-xl text-yellow-300">Migrate greenthread first, and, then, tackle wild monkey patching.</p>
    <p class="mt-4 text-xl">To conclude this section, this incremental approach is particularly suited for applications using internal private APIs, as it allows you to progressively migrate these APIs without disrupting other parts of the system.</p>
</section>

<!-- Migrating applications with internal private APIs-->
<section>
    <h2 id="migrating-shared-private-apis" class="mt-10 text-3xl font-bold mb-6">
        Migrating Applications with Internal Private APIs
        <a href="#migrating-applications-with-internal-private-apis" class="text-cyan-400 text-xl">🔗</a>
    </h2>
    <p class="mt-4 text-xl">
        You may encounter situations where isolating parts of your application into separate processes is challenging because these parts rely on a shared internal private API using Eventlet. The goal here is to break this dependency without rewriting your entire codebase at once.
    </p>

    <p class="mt-4 text-xl">Here's a clear and incremental strategy you can follow:</p>

    <ol class="list-decimal pl-8 mt-6 text-xl space-y-4">
        <li>
            <strong>Identify your API boundaries clearly.</strong>
            <p class="mt-2">Start by precisely defining the boundary between your isolated sub-parts and the shared internal Eventlet-based API. Understand exactly which parts of your code depend on Eventlet functionalities.</p>
            <pre class="line-numbers mt-4"><code class="language-python"># Current Eventlet-based API
def async_fetch_data():
    eventlet.spawn(...)

def async_send_request():
    eventlet.spawn(...)</code></pre>
        </li>

        <li>
            <strong>Create a neutral interface.</strong>
            <p class="mt-2">Design an intermediate interface that doesn't expose Eventlet directly. This new interface can be synchronous (blocking) or use standard communication methods such as REST or RPC, abstracting away the specifics of Eventlet from the rest of your application.</p>
            <pre class="line-numbers mt-4"><code class="language-python"># New neutral interface without direct Eventlet references
def fetch_data():
    pass

def send_request():
    pass</code></pre>
        </li>

        <li>
            <strong>Initially run your neutral interface alongside Eventlet.</strong>
            <p class="mt-2">Initially, your new neutral interface might internally call the existing Eventlet-based implementation, ensuring compatibility and stability while allowing incremental migration.</p>
            <pre class="line-numbers mt-4"><code class="language-python"># Neutral API initially calling Eventlet-based implementation
def fetch_data():
    return async_fetch_data()</code></pre>
        </li>

        <li>
            <strong>Migrate each isolated part incrementally.</strong>
            <p class="mt-2">Move your application parts one at a time to the neutral interface. Once a part uses this interface, you can safely run it in an isolated process since it no longer directly depends on Eventlet.</p>
        </li>

        <li>
            <strong>Remove Eventlet completely at the end.</strong>
            <p class="mt-2">After migrating all the isolated parts to your neutral interface, replace the underlying Eventlet implementations incrementally with alternatives such as AsyncIO or Threading without impacting the isolated parts.</p>
            <pre class="line-numbers mt-4"><code class="language-python"># Final neutral API implemented with AsyncIO
def fetch_data():
    asyncio.run(async_fetch())</code></pre>
        </li>
    </ol>


    <h3 id="shared-api-sequence" class="mt-10 text-2xl font-bold mb-6">Sequence Overview</h3>

    <div class="mt-10 mermaid">
        %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#312e81', 'primaryTextColor': '#fff', 'primaryBorderColor': '#433e99', 'lineColor': '#ffffff', 'secondaryColor': '#312e81', 'tertiaryColor': '#312e81' }}}%%
        flowchart TD
            A[Shared API based on Eventlet] --> B[Create Neutral Interface]
            B --> C[Neutral Interface calls Eventlet internally]
            C --> D[Migrate isolated sub-parts incrementally]
            D --> E[All sub-parts using Neutral Interface]
            E --> F[Replace Eventlet with AsyncIO or Threading internally]
            F --> G[Eventlet completely removed]
    </div>

    <p class="mt-4 text-xl text-yellow-300">
        Decouple, abstract, and incrementally migrate to minimize risk and maximize control.
    </p>
</section>

<section>
    <h2 id="implementation-example" class="mt-10 text-3xl font-bold mb-6">Implementation Example of Migrating Applications with Internal Private APIs <a href="#implementation-example" class="text-cyan-400 text-xl">🔗</a></h2>

    <h3 class="mt-10 text-2xl font-bold mb-6">1. Context and Objectives</h3>
    <p class="mt-4 text-xl">
        Your application is built on a single codebase with several entry points that run dedicated business processes (for example, API Server, Compute Server, and XYZ Server). Up to now, these processes rely on Eventlet for concurrency management (e.g., green threads, monkey-patching, etc.). However, as your requirements evolve and for reasons such as performance, maintainability, or integration with other Python libraries, you have decided to migrate away from Eventlet to an alternative concurrency solution—whether that be asyncio, threading, or a combination of both.
    </p>

    <p class="mt-4 text-xl"><strong>Objectives:</strong></p>
    <ul class="list-disc pl-8 mt-4 text-xl space-y-2">
        <li><strong>Decouple from Eventlet:</strong> Replace the Eventlet-dependent concurrency model with a more modern or suitable alternative.</li>
        <li><strong>Ensure a smooth transition:</strong> Migrate the business processes incrementally to minimize risks and maintain service continuity.</li>
        <li><strong>Maintain functionality:</strong> Guarantee that during and after the migration, the application’s business processes continue to interact with core services (database connections, RPC communications, etc.) correctly.</li>
    </ul>

    <hr class="my-10">

    <h3 class="mt-10 text-2xl font-bold mb-6">2. Step 1: Initial State (Before Migration)</h3>

    <h4 class="mt-6 text-xl font-bold mb-4">Description</h4>
    <p class="mt-4 text-xl">
        At this stage, all business processes (API Server, Compute Server, XYZ Server) are tightly coupled with Eventlet for concurrency. The Eventlet layer underpins how these processes manage asynchronous tasks, network I/O, and cooperative multitasking.
    </p>

    <h4 class="mt-6 text-xl font-bold mb-4">Diagram</h4>
    <div class="mt-10 mermaid">
        %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#312e81', 'primaryTextColor': '#fff', 'primaryBorderColor': '#433e99', 'lineColor': '#ffffff', 'secondaryColor': '#312e81', 'tertiaryColor': '#312e81' }}}%%
        flowchart TD
        %% Business Processes
        subgraph Business_Processes [Business Processes]
            A[API Server]
            B[Compute Server]
            C[XYZ Server]
        end

        %% Legacy Concurrency Model (Eventlet)
        subgraph Legacy [Eventlet Concurrency - Legacy]
            D[Database Connection]
            E[RPC Communications]
            F[Security Management]
        end

        %% Direct Connections using Eventlet
        A --> D
        A --> E
        A --> F

        B --> D
        B --> E
        B --> F

        C --> D
        C --> E
        C --> F
    </div>

    <h4 class="mt-6 text-xl font-bold mb-4">Explanation</h4>
    <p class="mt-4 text-xl">
        <strong>Vertical Silos:</strong> Represent the different business processes (API Server, Compute Server, XYZ Server) that execute concurrently.
    </p>
    <p class="mt-4 text-xl">
        <strong>Horizontal Silo:</strong> The Eventlet-based concurrency layer (Legacy) handles core functionalities like database connections, RPC, and security.
    </p>
    <p class="mt-4 text-xl">
        <strong>Tight Coupling:</strong> Every process directly depends on Eventlet, which makes it difficult to evolve or integrate with alternative concurrency frameworks.
    </p>

    <hr class="my-10">

    <h3 class="mt-10 text-2xl font-bold mb-6">3. Step 2: Progressive Migration (During Migration with One Process Migrated)</h3>

    <h4 class="mt-6 text-xl font-bold mb-4">Description</h4>
    <p class="mt-4 text-xl">
        In this phase, you begin the migration by refactoring one of the business processes (e.g., the API Server) to use a newly developed concurrency adapter. This adapter abstracts the underlying concurrency mechanism and redirects calls to the chosen alternative (asyncio, threading, or a hybrid approach). Meanwhile, the remaining processes continue to use Eventlet.
    </p>

    <h4 class="mt-6 text-xl font-bold mb-4">Diagram</h4>
    <div class="mt-10 mermaid">
        %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#312e81', 'primaryTextColor': '#fff', 'primaryBorderColor': '#433e99', 'lineColor': '#ffffff', 'secondaryColor': '#312e81', 'tertiaryColor': '#312e81' }}}%%
        flowchart TD
        %% Business Processes
        subgraph Business_Processes [Business Processes]
            A[API Server - Migrated]
            B[Compute Server]
            C[XYZ Server]
        end

        %% Legacy Concurrency Model for non-migrated processes
        subgraph Legacy [Eventlet Concurrency - Legacy]
            D[Database Connection - Legacy]
            E[RPC Communications - Legacy]
            F[Security Management - Legacy]
        end

        %% Migrated Concurrency Model accessible via the adapter
        subgraph Migrated [New Concurrency - Migrated]
            G[Database Connection - Migrated]
            H[RPC Communications - Migrated]
            I[Security Management - Migrated]
        end

        %% Concurrency Adapter for Migration
        subgraph Adapter [Concurrency Adapter]
            J[Migration Interface]
        end

        %% The migrated process uses the adapter that points to the new concurrency model
        A ---|Uses Adapter| J
        J --> G
        J --> H
        J --> I

        %% Other processes still use the legacy Eventlet layer
        B --> D
        B --> E
        B --> F

        C --> D
        C --> E
        C --> F
    </div>

    <h4 class="mt-6 text-xl font-bold mb-4">Explanation</h4>
    <p class="mt-4 text-xl">
        <strong>Partial Migration:</strong> Only the API Server has been refactored to use the concurrency adapter. This adapter bridges the existing functionality with the new concurrency mechanism.
    </p>
    <p class="mt-4 text-xl">
        <strong>Adapter Role:</strong> The adapter abstracts away the differences between Eventlet and the new concurrency approach, allowing the migrated process to operate with the new model without affecting the others.
    </p>
    <p class="mt-4 text-xl">
        <strong>Coexistence:</strong> The legacy (Eventlet) and the migrated concurrency models run in parallel, ensuring uninterrupted service for non-migrated processes.
    </p>

    <hr class="my-10">

    <h3 class="mt-10 text-2xl font-bold mb-6">4. Step 3: Complete Migration (After Migration, All Processes Migrated)</h3>

    <h4 class="mt-6 text-xl font-bold mb-4">Description</h4>
    <p class="mt-4 text-xl">
        After thorough testing and validation, all business processes are refactored to use the concurrency adapter, which now directs them to the new concurrency model (whether that's asyncio, threading, or a combination thereof). This marks the end of dependency on Eventlet.
    </p>

    <h4 class="mt-6 text-xl font-bold mb-4">Diagram</h4>
    <div class="mt-10 mermaid">
        %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#312e81', 'primaryTextColor': '#fff', 'primaryBorderColor': '#433e99', 'lineColor': '#ffffff', 'secondaryColor': '#312e81', 'tertiaryColor': '#312e81' }}}%%
        flowchart TD
        %% Business Processes
        subgraph Business_Processes [Business Processes]
            A[API Server]
            B[Compute Server]
            C[XYZ Server]
        end

        %% Concurrency Adapter to access the new concurrency model
        subgraph Adapter [Concurrency Adapter]
            J[Migration Interface]
        end

        %% New Concurrency Model used by all processes
        subgraph Migrated [New Concurrency - Migrated]
            G[Database Connection - Migrated]
            H[RPC Communications - Migrated]
            I[Security Management - Migrated]
        end

        %% All processes use the adapter
        A ---|Uses Adapter| J
        B ---|Uses Adapter| J
        C ---|Uses Adapter| J

        %% The adapter directs to the new concurrency model
        J --> G
        J --> H
        J --> I
    </div>

    <h4 class="mt-6 text-xl font-bold mb-4">Explanation</h4>
    <p class="mt-4 text-xl">
        <strong>Unified Abstraction:</strong> All business processes now leverage the concurrency adapter, ensuring consistent access to the new concurrency model.
    </p>
    <p class="mt-4 text-xl">
        <strong>Centralized Control:</strong> The adapter centralizes concurrency management, simplifying maintenance and future improvements.
    </p>
    <p class="mt-4 text-xl">
        <strong>Smooth Transition:</strong> This stage confirms that all components interact consistently with the new model, paving the way for complete deprecation of Eventlet.
    </p>

    <hr class="my-10">

    <h3 class="mt-10 text-2xl font-bold mb-6">5. Step 4: Adapter Removal (Simplified Final Architecture)</h3>

    <h4 class="mt-6 text-xl font-bold mb-4">Description</h4>
    <p class="mt-4 text-xl">
        Once the migration is fully validated and the new concurrency model proves stable across all business processes, the adapter can be removed. The business processes are then directly integrated with the new concurrency framework, further simplifying the architecture.
    </p>

    <h4 class="mt-6 text-xl font-bold mb-4">Diagram</h4>
    <div class="mt-10 mermaid">
        %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#312e81', 'primaryTextColor': '#fff', 'primaryBorderColor': '#433e99', 'lineColor': '#ffffff', 'secondaryColor': '#312e81', 'tertiaryColor': '#312e81' }}}%%
        flowchart TD
        %% Business Processes
        subgraph Business_Processes [Business Processes]
            A[API Server]
            B[Compute Server]
            C[XYZ Server]
        end

        %% New Concurrency Model used by all processes
        subgraph Migrated [New Concurrency - Migrated]
            G[Database Connection - Migrated]
            H[RPC Communications - Migrated]
            I[Security Management - Migrated]
        end

        %% Direct Connections (Adapter removed)
        A --> G
        A --> H
        A --> I

        B --> G
        B --> H
        B --> I

        C --> G
        C --> H
        C --> I
    </div>

    <h4 class="mt-6 text-xl font-bold mb-4">Explanation</h4>
    <p class="mt-4 text-xl">
        <strong>Removal of the Adapter:</strong> With the new concurrency model fully in place, the intermediary adapter becomes redundant and is removed.
    </p>
    <p class="mt-4 text-xl">
        <strong>Simplified Architecture:</strong> Direct integration with the new concurrency model reduces complexity and may improve performance.
    </p>
    <p class="mt-4 text-xl">
        <strong>Final State:</strong> This diagram represents the final state of your application, where all business processes are fully independent of Eventlet and operate under the chosen concurrency paradigm.
    </p>

    <hr class="my-10">

    <h3 class="mt-10 text-2xl font-bold mb-6">6. Conclusion</h3>
    <p class="mt-4 text-xl">
        This report outlines a structured approach for migrating your application from an Eventlet-based concurrency model to an alternative solution (asyncio, threading, or a hybrid). The migration strategy comprises:
    </p>
    <ul class="list-disc pl-8 mt-4 text-xl space-y-2">
        <li><strong>Initial State:</strong> All business processes depend on Eventlet.</li>
        <li><strong>Partial Migration:</strong> Begin by refactoring one process (API Server) to use a concurrency adapter while the others continue using Eventlet.</li>
        <li><strong>Complete Migration:</strong> Transition all business processes to use the adapter, ensuring a unified migration to the new concurrency model.</li>
        <li><strong>Final Simplification:</strong> Remove the adapter once the new concurrency model is fully stable, resulting in a simplified and modernized architecture.</li>
    </ul>
    <p class="mt-4 text-xl">
        By following this plan, you can minimize risks and ensure a smooth, controlled transition from Eventlet to your chosen alternative, ultimately enhancing the scalability and maintainability of your application.
    </p>
    <p class="mt-4 text-xl">
        Feel free to adjust this approach according to your specific requirements and add further details such as testing strategies, performance benchmarks, and error handling mechanisms to ensure a successful migration.
    </p>
</section>