---
layout: dashboard
title: Migrating to AsyncIO
permalink: /guide/migrating-to-asyncio/
id: asyncio
description: A comprehensive guide on how to migrate your Python application from Eventlet to AsyncIO. Learn about code refactoring patterns, the Sync-Async Adapter Pattern, and common pitfalls to avoid during migration.
keywords: asyncio migration, eventlet to asyncio, async await, python asynchronous programming, sync-async adapter, refactoring eventlet
og_type: article
og_title: Complete Guide for Migrating from Eventlet to AsyncIO
og_description: Detailed step-by-step instructions for refactoring Eventlet code to use modern AsyncIO patterns with practical examples and best practices.
---
<section>
    <h1 class="text-4xl font-bold mb-6">Migrating to AsyncIO</h1>
    <p class="mt-10 text-xl">This chapter aims to guide you through the process of migrating your codebase from Eventlet to AsyncIO. We will cover the necessary steps, best practices, and <a href="#common-pitfalls" class="text-cyan-400">common pitfalls</a> to avoid during the migration.</p>
    
    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#overview-of-asyncio" class="text-cyan-400 hover:underline">Overview of AsyncIO</a></li>
                <li><a href="#steps-to-migrate" class="text-cyan-400 hover:underline">Steps to Migrate</a></li>
                <li><a href="#replacing-eventlet-with-asyncio" class="text-cyan-400 hover:underline">Replacing Eventlet with AsyncIO</a></li>
                <li><a href="#advanced-scenarios" class="text-cyan-400 hover:underline">Advanced Migration Scenarios</a></li>
                <li><a href="#sync-async-adapter" class="text-cyan-400 hover:underline">Sync-Async Adapter Pattern</a></li>
                <li><a href="#asyncio-libraries" class="text-cyan-400 hover:underline">AsyncIO Libraries</a></li>
                <li><a href="#common-pitfalls" class="text-cyan-400 hover:underline">Common Pitfalls</a></li>
            </ul>
        </div>
    </div>
</section>
<section>
    <h2 id="overview-of-asyncio" class="mt-10 text-3xl font-bold mb-6">Overview of AsyncIO</h2>
    <p class="mt-10 text-xl">AsyncIO is a library to write concurrent code using the async/await syntax. It is part of the Python standard library and provides a foundation for asynchronous programming in Python.</p>
    <p class="mt-10 text-xl">AsyncIO allows you to write programs that can handle many tasks at once, such as network requests, file I/O, and other I/O-bound operations, without blocking the main thread. This is achieved through the use of coroutines, which are special functions that can pause and resume their execution, allowing other coroutines to run in the meantime.</p>
    <p class="mt-10 text-xl">The core components of AsyncIO include:</p>
    <ul class="mt-10 text-xl list-disc list-inside">
        <li><strong>Event Loop:</strong> The central component that manages the execution of asynchronous tasks. It runs in a single thread and schedules the execution of coroutines, callbacks, and other I/O operations.</li>
        <li><strong>Coroutines:</strong> Functions defined with the <code class="language-python">async def</code> syntax. They use the <code class="language-python">await</code> keyword to pause their execution and yield control back to the event loop.</li>
        <li><strong>Tasks:</strong> Wrappers around coroutines that allow them to be scheduled and managed by the event loop. They can be created using the <code class="language-python">asyncio.create_task()</code> function.</li>
        <li><strong>Futures:</strong> Objects that represent the result of an asynchronous operation. They can be awaited to retrieve the result once the operation is complete.</li>
    </ul>
    <p class="mt-10 text-xl">AsyncIO also provides various utilities for managing asynchronous tasks, such as synchronization primitives (e.g., locks, semaphores), queues, and streams for handling network I/O.</p>
    <p class="mt-10 text-xl">For more details, you can consult the official AsyncIO documentation <a href="https://docs.python.org/3.13/library/asyncio.html" class="text-cyan-400" target="_blank">here</a>.</p>
</section>
<section>
    <h2 id="steps-to-migrate" class="mt-10 text-3xl font-bold mb-6">Steps to Migrate</h2>
    <p class="mt-10 text-xl">The following steps outline the process of migrating from Eventlet to AsyncIO:</p>
    <ol class="mt-10 text-xl list-decimal list-inside">
        <li><strong>Activate the AsyncIO hub in Eventlet:</strong> Begin by switching to Eventlet's AsyncIO hub to enable compatibility with AsyncIO code.
            <pre class="line-numbers mt-4 ml-6"><code class="language-python">import eventlet
eventlet.use_hub('asyncio')
# If you use monkey patching, place it after hub activation
eventlet.monkey_patch()</code></pre>
            <p class="mt-2 ml-6">This critical first step allows you to gradually introduce AsyncIO code while maintaining compatibility with existing Eventlet code. For more details, refer to the <a href="https://eventlet.readthedocs.io/en/latest/asyncio/migration.html#step-1-switch-to-the-asyncio-hub" class="text-cyan-400" target="_blank">official Eventlet documentation</a>.</p>
        </li>
        <li><a href="{{ site.baseurl }}{% link guide/preparing.md %}#locate-eventlet-usages" class="text-cyan-400">Identify Eventlet usages</a> in your codebase.</li>
        <li>Replace Eventlet-specific code with <a href="#replacing-eventlet-with-asyncio" class="text-cyan-400">AsyncIO equivalents</a>.</li>
        <li>Refactor synchronous code to use async/await syntax.</li>
        <li>Test the migrated code thoroughly to ensure functionality.</li>
        <li>Optimize performance and handle edge cases.</li>
    </ol>
</section>
<section>
    <h2 id="replacing-eventlet-with-asyncio" class="mt-10 text-3xl font-bold mb-6">Replacing Eventlet with AsyncIO</h2>
    <p class="mt-10 text-xl">This section provides examples of how to replace common Eventlet-specific code with AsyncIO equivalents. For each common Eventlet usage, we'll show how to refactor the code using AsyncIO.</p>
    
    <div class="mt-10 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-10">
        <!-- WSGI Server -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">WSGI Server</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">from eventlet import wsgi, listen

def simple_app(environ, start_response):
    start_response('200 OK', [('Content-Type', 'text/plain')])
    return [b'Hello, World!\n']

wsgi.server(listen(('', 8080)), simple_app)</code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">from aiohttp import web

async def simple_app(request):
    return web.Response(text="Hello, World!")

app = web.Application()
app.add_routes([web.get('/', simple_app)])

if __name__ == '__main__':
    web.run_app(app, port=8080)</code></pre>
            <p class="mt-2">For WSGI servers, use <a href="https://docs.aiohttp.org/en/stable/" class="text-cyan-400" target="_blank">aiohttp</a> or <a href="https://fastapi.tiangolo.com/" class="text-cyan-400" target="_blank">FastAPI</a> with <a href="https://www.uvicorn.org/" class="text-cyan-400" target="_blank">uvicorn</a> as an ASGI server.</p>
        </div>
        
        <!-- Asynchronous Network Calls -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Asynchronous Network Calls</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">import eventlet
from eventlet.green import urllib2

def fetch_url(url):
    return urllib2.urlopen(url).read()

urls = ['http://example.com', 'http://example.org']
pool = eventlet.GreenPool()

for body in pool.imap(fetch_url, urls):
    print(body)</code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import aiohttp
import asyncio

async def fetch_url(session, url):
    async with session.get(url) as response:
        return await response.text()

async def main():
    urls = ['http://example.com', 'http://example.org']
    async with aiohttp.ClientSession() as session:
        tasks = [fetch_url(session, url) for url in urls]
        for body in await asyncio.gather(*tasks):
            print(body)

asyncio.run(main())</code></pre>
            <p class="mt-2">For HTTP requests, use <a href="https://docs.aiohttp.org/en/stable/client.html" class="text-cyan-400" target="_blank">aiohttp.ClientSession</a> with asyncio.gather to run multiple requests concurrently.</p>
        </div>
        
        <!-- Background Execution of Long Tasks -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Background Execution of Long Tasks</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">import eventlet

def long_running_task():
    # Simulate a long-running task
    eventlet.sleep(10)
    print("Task completed")

eventlet.spawn(long_running_task)
print("Main thread continues to run")</code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import asyncio

async def long_running_task():
    # Simulate a long-running task
    await asyncio.sleep(10)
    print("Task completed")

async def main():
    # Create a task that runs in the background
    task = asyncio.create_task(long_running_task())
    print("Main thread continues to run")
    
    # Wait for the task to complete
    await task

asyncio.run(main())</code></pre>
            <p class="mt-2">Use <code class="language-python">asyncio.create_task()</code> to run coroutines concurrently in the background.</p>
        </div>
        
        <!-- Deferred Task Management -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Deferred Task Management</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">from eventlet.event import Event

def deferred_task(evt):
    print("Waiting for event")
    evt.wait()
    print("Event occurred, executing task")

evt = Event()
eventlet.spawn_after(5, deferred_task, evt)
eventlet.sleep(5)
evt.send()</code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import asyncio

async def deferred_task(event):
    print("Waiting for event")
    await event.wait()
    print("Event occurred, executing task")

async def main():
    event = asyncio.Event()
    
    # Schedule the task
    task = asyncio.create_task(deferred_task(event))
    
    # Wait a bit then set the event
    await asyncio.sleep(5)
    event.set()
    
    await task

asyncio.run(main())</code></pre>
            <p class="mt-2">Use <code class="language-python">asyncio.Event()</code> for synchronization between coroutines.</p>
        </div>
        
        <!-- Green Thread Management -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Green Thread Management</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
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
eventlet.spawn(task2)
eventlet.sleep(5)  # Let tasks run for a while</code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import asyncio

async def task1():
    while True:
        print("Task 1 running")
        await asyncio.sleep(1)

async def task2():
    while True:
        print("Task 2 running")
        await asyncio.sleep(1)

async def main():
    # Create tasks
    task_1 = asyncio.create_task(task1())
    task_2 = asyncio.create_task(task2())
    
    # Run for a limited time
    await asyncio.sleep(5)
    
    # Cancel tasks
    task_1.cancel()
    task_2.cancel()
    
    try:
        await asyncio.gather(task_1, task_2)
    except asyncio.CancelledError:
        pass

asyncio.run(main())</code></pre>
            <p class="mt-2">Use <code class="language-python">asyncio.create_task()</code> to run multiple coroutines concurrently within a single thread.</p>
        </div>
        
        <!-- Socket Compatibility -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Socket Compatibility</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
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

echo_server(6000)</code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import asyncio

async def handle_client(reader, writer):
    while True:
        data = await reader.read(1024)
        if not data:
            break
        writer.write(data)
        await writer.drain()
    writer.close()
    await writer.wait_closed()

async def main():
    server = await asyncio.start_server(
        handle_client, '0.0.0.0', 6000
    )
    
    async with server:
        await server.serve_forever()

asyncio.run(main())</code></pre>
            <p class="mt-2">Use <code class="language-python">asyncio.start_server()</code> for TCP servers and <code class="language-python">asyncio.open_connection()</code> for TCP clients.</p>
        </div>
        
        <!-- WebSocket Support -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">WebSocket Support</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">import eventlet
from eventlet import websocket

@websocket.WebSocketWSGI
def echo(ws):
    while True:
        msg = ws.wait()
        if msg is None:
            break
        ws.send(msg)

server = eventlet.listen(('0.0.0.0', 7000))
eventlet.wsgi.server(server, echo)</code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import asyncio
import websockets

async def echo(websocket, path):
    async for message in websocket:
        await websocket.send(message)

async def main():
    async with websockets.serve(echo, "0.0.0.0", 7000):
        await asyncio.Future()  # Run forever

asyncio.run(main())</code></pre>
            <p class="mt-2">Use <a href="https://websockets.readthedocs.io/en/stable/" class="text-cyan-400" target="_blank">websockets</a> library for WebSocket communication.</p>
        </div>
        
        <!-- Integration with Other Libraries -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Integration with Other Libraries</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
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
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import asyncio

async def worker(name, queue):
    while True:
        item = await queue.get()
        if item is None:
            queue.task_done()
            break
        print(f"Worker {name} processing {item}")
        await asyncio.sleep(0.1)  # Simulate processing time
        queue.task_done()

async def main():
    queue = asyncio.Queue()
    
    # Create worker tasks
    workers = [
        asyncio.create_task(worker(f"worker-{i}", queue))
        for i in range(3)
    ]
    
    # Add items to the queue
    for item in range(10):
        await queue.put(item)
    
    # Wait until all items are processed
    await queue.join()
    
    # Send shutdown signal to workers
    for _ in workers:
        await queue.put(None)
    
    # Wait for workers to finish
    await asyncio.gather(*workers)

asyncio.run(main())</code></pre>
            <p class="mt-2">Use <code class="language-python">asyncio.Queue</code> for task distribution among coroutines.</p>
        </div>
    </div>
    
    <h3 class="mt-10 text-2xl font-bold mb-3">Tips for Refactoring</h3>
    <ul class="mt-4 text-xl list-disc list-inside">
        <li>Start by identifying pure I/O operations that can be made asynchronous</li>
        <li>Refactor functions into coroutines by adding <code class="language-python">async</code> to function definitions and <code class="language-python">await</code> to calls that need to wait</li>
        <li>Use <code class="language-python">asyncio.gather()</code> to run multiple coroutines concurrently</li>
        <li>Be careful with CPU-bound tasks - consider using <code class="language-python">concurrent.futures.ProcessPoolExecutor</code> with <code class="language-python">asyncio.to_thread()</code> for them</li>
        <li>Keep synchronization in mind - use <code class="language-python">asyncio.Lock()</code>, <code class="language-python">asyncio.Semaphore()</code>, and other primitives</li>
    </ul>
</section>
<section>
    <h2 id="advanced-scenarios" class="mt-10 text-3xl font-bold mb-6">Advanced Migration Scenarios <a href="#advanced-scenarios" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">Beyond the basic patterns shown above, here are some additional migration scenarios you might encounter:</p>
    
    <div class="mt-10 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-10">
        <!-- Handling Monkey Patching -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Handling Monkey Patching</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">import eventlet
eventlet.monkey_patch()

import socket
import time

def blocking_operation():
    # This socket operation is patched to be non-blocking
    sock = socket.socket()
    sock.connect(('example.com', 80))
    # This sleep is patched to yield to other green threads
    time.sleep(1)  </code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import asyncio
import socket

async def non_blocking_operation():
    # Use AsyncIO's explicit non-blocking operations
    reader, writer = await asyncio.open_connection('example.com', 80)
    # Explicitly yield control with await
    await asyncio.sleep(1)
    
    # When dealing with libraries that don't support AsyncIO:
    # Use executor for blocking operations
    result = await asyncio.to_thread(some_blocking_function)</code></pre>
            <p class="mt-2">Replace implicit monkey patching with explicit async calls. Use <code class="language-python">asyncio.to_thread()</code> for unavoidable blocking operations.</p>
        </div>
        
        <!-- Timeouts and Exception Handling -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Timeouts and Exception Handling</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">import eventlet

def with_timeout():
    try:
        with eventlet.Timeout(5):
            # Operation that should timeout
            eventlet.sleep(10)
        print("This won't be reached")
    except eventlet.Timeout:
        print("Operation timed out")</code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import asyncio

async def with_timeout():
    try:
        # Operation that should timeout
        await asyncio.wait_for(asyncio.sleep(10), timeout=5)
        print("This won't be reached")
    except asyncio.TimeoutError:
        print("Operation timed out")
        
    # Alternatively, use shield to protect from cancellation
    try:
        result = await asyncio.wait_for(
            asyncio.shield(important_operation()), 
            timeout=5
        )
    except asyncio.TimeoutError:
        print("Operation timed out but continues running")</code></pre>
            <p class="mt-2">Use <code class="language-python">asyncio.wait_for()</code> for timeouts and <code class="language-python">asyncio.shield()</code> to protect critical operations from cancellation.</p>
        </div>
        
        <!-- Testing Async Code -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Testing Async Code</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">import unittest
import eventlet

def test_function():
    eventlet.sleep(0.1)
    return "done"

class TestEventlet(unittest.TestCase):
    def test_async_function(self):
        result = test_function()
        self.assertEqual(result, "done")</code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import unittest
import asyncio

async def test_function():
    await asyncio.sleep(0.1)
    return "done"

class TestAsyncIO(unittest.TestCase):
    def test_async_function(self):
        result = asyncio.run(test_function())
        self.assertEqual(result, "done")
        
    # With pytest you can use:
    # @pytest.mark.asyncio
    # async def test_async_function(self):
    #     result = await test_function()
    #     assert result == "done"</code></pre>
            <p class="mt-2">Use <code class="language-python">asyncio.run()</code> in standard unittest or pytest's <code class="language-python">@pytest.mark.asyncio</code> decorator for async tests.</p>
        </div>
        
        <!-- Resource Pooling -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Resource Pooling</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">import eventlet
from eventlet.db_pool import ConnectionPool

def db_connect():
    # Create a database connection
    return db_connection

# Create a pool of database connections
pool = ConnectionPool(db_connect, max_size=10)

def use_db():
    conn = pool.get()
    try:
        # Use the connection
        conn.execute("SELECT * FROM table")
    finally:
        pool.put(conn)</code></pre>
            <p class="mb-2"><strong>AsyncIO:</strong></p>
            <pre class="line-numbers"><code class="language-python">import asyncio
import asyncpg

async def use_db():
    # Using connection pools with AsyncIO
    pool = await asyncpg.create_pool(dsn="postgres://user:pass@host/db", 
                                     min_size=5,
                                     max_size=10)
    async with pool.acquire() as conn:
        # Use the connection
        result = await conn.fetch("SELECT * FROM table")
    
    await pool.close()  # Close the pool when done</code></pre>
            <p class="mt-2">Use async database libraries like <code class="language-python">asyncpg</code>, <code class="language-python">aiomysql</code>, or <code class="language-python">aiosqlite</code> that provide built-in connection pooling.</p>
        </div>
        
        <!-- Web Framework Integration -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Web Framework Integration</h3>
            <p class="mb-2"><strong>Eventlet with Flask:</strong></p>
            <pre class="line-numbers"><code class="language-python">import eventlet
eventlet.monkey_patch()

from flask import Flask
app = Flask(__name__)

@app.route('/')
def index():
    return "Hello, World!"

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8000)</code></pre>
            <p class="mb-2"><strong>AsyncIO-based Alternative:</strong></p>
            <pre class="line-numbers"><code class="language-python"># Option 1: Use an async framework like FastAPI
from fastapi import FastAPI
import uvicorn

app = FastAPI()

@app.get('/')
async def index():
    return {"message": "Hello, World!"}

if __name__ == '__main__':
    uvicorn.run(app, host='0.0.0.0', port=8000)

# Option 2: Use ASGI adapters for existing frameworks
# For Flask:
from flask import Flask
import asyncio
from asgiref.wsgi import WsgiToAsgi
import uvicorn

app = Flask(__name__)

@app.route('/')
def index():
    return "Hello, World!"

asgi_app = WsgiToAsgi(app)

if __name__ == '__main__':
    uvicorn.run(asgi_app, host='0.0.0.0', port=8000)</code></pre>
            <p class="mt-2">Either migrate to native async frameworks like FastAPI, or use ASGI adapters to run traditional WSGI frameworks with async servers.</p>
        </div>
    </div>
</section>
<section>
    <h2 id="sync-async-adapter" class="mt-10 text-3xl font-bold mb-6">Sync-Async Adapter Pattern <a href="#sync-async-adapter" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">The <strong>Sync-Async Adapter Pattern</strong> enables a library to support both <strong>synchronous and asynchronous execution modes</strong>, while internally using a unified implementation based on <code class="language-python">asyncio</code>.</p>
    
    <p class="mt-6 text-xl">This pattern is particularly suitable for refactoring existing libraries originally based on <strong>Eventlet</strong>, allowing them to remove their dependency on <code class="language-python">eventlet.monkey_patch()</code>. Thus, a single library can seamlessly operate both in modern <code class="language-python">asyncio</code>-based applications and traditional synchronous environments.</p>

    <h3 class="mt-10 text-2xl font-bold mb-3">Initial Use Case (with Eventlet)</h3>
    <p class="mt-4 text-xl">Consider a hypothetical library named <code class="language-python">hypotheticlib</code> that exposes a synchronous HTTP client:</p>

    <pre class="line-numbers mt-6"><code class="language-python"># hypotheticlib.py
import requests

class Client:
    headers = {
        'User-Agent': 'sync-client'
    }

    def request(self, url):
        response = requests.get(url)
        return response.json()</code></pre>

    <p class="mt-6 text-xl">Currently, some applications utilize Eventlet to make this library <strong>implicitly non-blocking</strong>:</p>

    <pre class="line-numbers mt-6"><code class="language-python">import eventlet
eventlet.monkey_patch()

from hypotheticlib import Client

client = Client()
result = client.request("https://api.example.com/data")  # Non-blocking due to Eventlet</code></pre>

    <p class="mt-6 text-xl">This approach has several drawbacks:</p>
    <ul class="mt-4 text-xl list-disc list-inside">
        <li>Direct dependency on Eventlet and global monkey-patching.</li>
        <li>Maintenance difficulties due to monkey-patching side-effects.</li>
        <li>No native compatibility with <code class="language-python">asyncio</code>.</li>
    </ul>

    <h3 class="mt-10 text-2xl font-bold mb-3">Refactoring with the Sync-Async Adapter Pattern</h3>
    <p class="mt-4 text-xl">The refactoring goal is to completely replace Eventlet with a native <code class="language-python">asyncio</code>-based implementation while preserving an interface usable in both synchronous and asynchronous contexts.</p>

    <p class="mt-4 text-xl">Here's an example of how the refactored implementation would look:</p>

    <pre class="line-numbers mt-6"><code class="language-python">import asyncio
import aiohttp

class AsyncClient:
    """Native asyncio-based implementation."""
    headers = {
        'User-Agent': 'async-client'
    }

    async def request(self, url):
        async with aiohttp.ClientSession(headers=self.headers) as session:
            async with session.get(url) as response:
                return await response.json()

class Client(AsyncClient):
    """Adapter enabling synchronous usage of AsyncClient."""

    def _run_coroutine(self, coro):
        try:
            loop = asyncio.get_running_loop()
            if loop.is_running():
                raise RuntimeError(
                    "Cannot run synchronous call when event loop is already running."
                )
        except RuntimeError:
            loop = asyncio.new_event_loop()
            asyncio.set_event_loop(loop)
            return loop.run_until_complete(coro)

    def request(self, url):
        return self._run_coroutine(super().request(url))</code></pre>

    <h3 class="mt-10 text-2xl font-bold mb-3">Managing asyncio Event Loops</h3>
    <p class="mt-4 text-xl">The <code class="language-python">_run_coroutine()</code> method handles the event loop as follows:</p>
    <ul class="mt-4 text-xl list-disc list-inside">
        <li>It first checks if an event loop (<code class="language-python">asyncio.get_running_loop()</code>) is currently active.</li>
        <li>If no loop is active, it creates and starts a new loop using <code class="language-python">asyncio.new_event_loop()</code> to synchronously execute the coroutine with <code class="language-python">run_until_complete()</code>.</li>
        <li>If an active loop is detected, it raises an explicit error, as synchronous calls cannot be executed in this context without additional handling.</li>
    </ul>

    <p class="mt-4 text-xl">This approach ensures robust management of asyncio event loops.</p>

    <h3 class="mt-10 text-2xl font-bold mb-3">Examples of Usage after Refactoring</h3>
    
    <p class="mt-4 text-xl"><strong>Explicit asynchronous usage (<code class="language-python">asyncio</code>):</strong></p>
    <pre class="line-numbers mt-6"><code class="language-python">import asyncio
from hypotheticlib import AsyncClient

async def main():
    client = AsyncClient()
    result = await client.request("https://api.example.com/data")
    print(result)

asyncio.run(main())</code></pre>

    <p class="mt-6 text-xl"><strong>Traditional synchronous usage:</strong></p>
    <pre class="line-numbers mt-6"><code class="language-python">from hypotheticlib import Client

client = Client()
result = client.request("https://api.example.com/data")
print(result)</code></pre>

    <h3 class="mt-10 text-2xl font-bold mb-3">Why is this pattern effective for replacing Eventlet?</h3>
    
    <div class="mt-6 grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Complete removal of Eventlet monkey-patching</h4>
            <p>No more global dynamic patching, significantly reducing side-effects.</p>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Transparent compatibility with asyncio</h4>
            <p>Enables direct integration into modern applications that follow the async/await paradigm.</p>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Interoperability with existing code</h4>
            <p>Allows the same library to be equally usable in traditional synchronous contexts and in modern async scenarios.</p>
        </div>
    </div>

    <h3 class="mt-10 text-2xl font-bold mb-3">Limitations and considerations</h3>
    <ul class="mt-4 text-xl list-disc list-inside">
        <li>The adapter cannot execute synchronous calls if an asyncio event loop is already running in the calling thread. In such scenarios, explicit asynchronous usage is required.</li>
        <li>Long-running CPU-bound operations are not recommended within this pattern, as they can block the event loop.</li>
    </ul>

    <h3 class="mt-10 text-2xl font-bold mb-3">Last words about this pattern</h3>
    <p class="mt-4 text-xl">This pattern is inspired by <a href="https://bugs.python.org/issue22239" class="text-cyan-400" target="_blank">this Python bug discussion</a> and is inspired from the <a href="https://github.com/geldata/gel-python" class="text-cyan-400" target="_blank">edgedb-python (now named gel-python)</a> library. Speaking examples of edgedb-python <a href="https://docs.geldata.com/learn/clients" class="text-cyan-400" target="_blank">usages are available here</a>.</p>
    <p class="mt-4 text-xl">The <strong>Sync-Async Adapter Pattern</strong> simplifies refactoring Eventlet-dependent libraries towards a native <code class="language-python">asyncio</code> architecture while maintaining backward compatibility with traditional synchronous use-cases. This approach enables smooth migration towards modern async environments, simplifies maintenance, and increases overall application robustness.</p>
</section>
<section>
    <h2 id="asyncio-libraries" class="mt-10 text-3xl font-bold mb-6">AsyncIO Libraries <a href="#asyncio-libraries" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-4 text-xl">There is plenty of AsyncIO based libraries. We invite to <a href="https://github.com/timofurrer/awesome-asyncio" class="text-cyan-400" target="_blank">have a look to this curated list</a> of AsyncIO libraries.</p>
</section>
<section>
    <h2 id="common-pitfalls" class="mt-10 text-3xl font-bold mb-6">Common Pitfalls <a href="#common-pitfalls" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">Be aware of the following common pitfalls during the migration:</p>
    <div class="mt-10 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-10">
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-4">Blocking calls within async functions</h3>
            <p class="text-xl">Blocking calls can halt the execution of your async code, leading to performance issues. Ensure that all I/O operations are non-blocking and use appropriate AsyncIO methods.</p>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-4">Improper use of async/await syntax</h3>
            <p class="text-xl">Incorrect usage of async/await can lead to unexpected behavior and bugs. Familiarize yourself with the correct patterns and practices for using async/await in Python.</p>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-4">Concurrency issues and race conditions</h3>
            <p class="text-xl">Concurrency issues and race conditions can arise when multiple tasks access shared resources. Use synchronization primitives like locks, semaphores, and queues to manage access to shared resources.</p>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-4">Performance bottlenecks due to improper use of AsyncIO features</h3>
            <p class="text-xl">Improper use of AsyncIO features can lead to performance bottlenecks. Profile your code to identify slow parts and optimize them by using efficient AsyncIO patterns and techniques.</p>
        </div>
    </div>
</section>
<section>
    <h2 id="conclusion" class="mt-10 text-3xl font-bold mb-6">Conclusion</h2>
    <p class="mt-10 text-xl">Migrating to AsyncIO can significantly improve the performance and scalability of your application. Follow the steps outlined in this guide and be mindful of common pitfalls to ensure a smooth migration process.</p>
</section>
