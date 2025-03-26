---
layout: dashboard
title: Migrating to threading
permalink: /guide/migrating-to-threading/
description: A practical guide for migrating from Eventlet to Python's native threading module. Learn code patterns, best practices, and specific techniques for replacing green threads with standard threads in your applications.
keywords: eventlet to threading, thread migration, green threads, python threading module, native threads, thread executors, thread pools
og_type: article
og_title: Migrating from Eventlet to Python's Native Threading Module
og_description: Comprehensive code patterns and best practices for replacing Eventlet's green threads with Python's standard threading in your applications.
---


<h1 class="text-4xl font-bold mb-8">Migrating to Python Native Threading</h1>

<p class="mt-10 text-xl">This chapter aims to guide you through the process of migrating your codebase from Eventlet to Python native threading. We will cover the necessary steps, best practices, and common pitfalls to avoid during the migration.</p>

<div class="mt-6 p-4 rounded-lg futuristic-section">
    <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
        <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
        <ul class="space-y-2">
            <li><a href="#overview-of-threading" class="text-cyan-400 hover:underline">Overview of Threading</a></li>
            <li><a href="#replacing-eventlet-with-threading" class="text-cyan-400 hover:underline">Replacing Eventlet with Threading</a></li>
            <li><a href="#key-migration-patterns" class="text-cyan-400 hover:underline">Key Migration Patterns</a></li>
            <li><a href="#threading-libraries" class="text-cyan-400 hover:underline">Threading Libraries</a></li>
            <li><a href="#common-pitfalls" class="text-cyan-400 hover:underline">Common Pitfalls</a></li>
            <li><a href="#best-practices-and-considerations" class="text-cyan-400 hover:underline">Best Practices and Considerations</a></li>
            <li><a href="#conclusion" class="text-cyan-400 hover:underline">Conclusion</a></li>
        </ul>
    </div>
</div>

<section>
    <h2 id="overview-of-threading" class="mt-10 text-3xl font-bold mb-6">Overview of Threading <a href="#overview-of-threading" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">Threading is a built-in Python module that allows you to run multiple operations concurrently in the same process space. It is part of the Python standard library and provides a foundation for concurrent programming in Python without the need for monkey patching.</p>
    
    <p class="mt-10 text-xl">Threading enables your applications to perform multiple operations in parallel, especially for I/O-bound tasks like network requests, file operations, and database queries. Unlike Eventlet's green threads, Python's threading module uses actual system threads, allowing for true parallelism on multi-core systems, though still subject to the Global Interpreter Lock (GIL) for CPU-bound tasks.</p>
    
    <p class="mt-10 text-xl">The core components of the threading module include:</p>
    <ul class="mt-10 text-xl list-disc list-inside">
        <li><strong>Thread:</strong> The fundamental unit of execution. You can create a thread by instantiating the <code class="language-python">Thread</code> class and passing a function to execute.</li>
        <li><strong>Lock:</strong> A synchronization primitive that allows only one thread to execute a particular section of code at a time, preventing race conditions.</li>
        <li><strong>RLock:</strong> A reentrant lock that can be acquired multiple times by the same thread before releasing.</li>
        <li><strong>Semaphore:</strong> A counter that controls access to a shared resource by multiple threads.</li>
        <li><strong>Event:</strong> A simple mechanism for communication between threads, allowing one thread to signal an event and other threads to wait for it.</li>
        <li><strong>Condition:</strong> A more complex synchronization primitive that combines a Lock with notification.</li>
    </ul>
    
    <p class="mt-10 text-xl">The threading module also integrates well with higher-level abstractions in the <code class="language-python">concurrent.futures</code> module, which provides thread pools and executors for more efficient thread management.</p>
    
    <p class="mt-10 text-xl">For more details, you can consult the official threading documentation <a href="https://docs.python.org/3/library/threading.html" class="text-cyan-400" target="_blank">here</a> and the concurrent.futures documentation <a href="https://docs.python.org/3/library/concurrent.futures.html" class="text-cyan-400" target="_blank">here</a>.</p>
</section>


<section>
    <h2 id="replacing-eventlet-with-threading" class="mt-10 text-3xl font-bold mb-6">Replacing Eventlet with Threading <a href="#replacing-eventlet-with-threading" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">This section provides examples of how to replace common Eventlet-specific code with Threading equivalents. For each common Eventlet usage, we'll show how to refactor the code using Python's native threading module.</p>
    
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
            <p class="mb-2"><strong>Threading:</strong></p>
            <pre class="line-numbers"><code class="language-python">from wsgiref.simple_server import make_server
import threading

def simple_app(environ, start_response):
    start_response('200 OK', [('Content-Type', 'text/plain')])
    return [b'Hello, World!\n']

def run_server():
    httpd = make_server('', 8080, simple_app)
    httpd.serve_forever()

# Start in a separate thread
server_thread = threading.Thread(target=run_server)
server_thread.daemon = True  # Allow app to exit if this is the only thread
server_thread.start()</code></pre>
            <p class="mt-2">For WSGI servers with threading, use <a href="https://docs.python.org/3/library/wsgiref.html" class="text-cyan-400" target="_blank">wsgiref</a> for development or <a href="https://gunicorn.org/" class="text-cyan-400" target="_blank">Gunicorn</a> with thread workers for production.</p>
        </div>
        
        <!-- Asynchronous Network Calls -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Concurrent Network Calls</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">import eventlet
from eventlet.green import urllib2

def fetch_url(url):
    return urllib2.urlopen(url).read()

urls = ['http://example.com', 'http://example.org']
pool = eventlet.GreenPool()

for body in pool.imap(fetch_url, urls):
    print(body)</code></pre>
            <p class="mb-2"><strong>Threading:</strong></p>
            <pre class="line-numbers"><code class="language-python">import requests
from concurrent.futures import ThreadPoolExecutor

def fetch_url(url):
    response = requests.get(url)
    return response.text

urls = ['http://example.com', 'http://example.org']
with ThreadPoolExecutor(max_workers=10) as executor:
    future_to_url = {executor.submit(fetch_url, url): url for url in urls}
    for future in future_to_url:
        try:
            body = future.result()
            print(body)
        except Exception as exc:
            print(f'{future_to_url[future]} generated an exception: {exc}')</code></pre>
            <p class="mt-2">Use <a href="https://docs.python.org/3/library/concurrent.futures.html" class="text-cyan-400" target="_blank">ThreadPoolExecutor</a> to run multiple requests concurrently with thread pools.</p>
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
            <p class="mb-2"><strong>Threading:</strong></p>
            <pre class="line-numbers"><code class="language-python">import threading
import time

def long_running_task():
    # Simulate a long-running task
    time.sleep(10)
    print("Task completed")

# Create and start the thread
thread = threading.Thread(target=long_running_task)
thread.daemon = True  # Optional: make thread exit when main thread exits
thread.start()
print("Main thread continues to run")

# Optional: wait for thread completion before exiting
# thread.join()</code></pre>
            <p class="mt-2">Use <code class="language-python">threading.Thread</code> to run tasks in background threads.</p>
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
            <p class="mb-2"><strong>Threading:</strong></p>
            <pre class="line-numbers"><code class="language-python">import threading
import time

def deferred_task(event):
    print("Waiting for event")
    event.wait()
    print("Event occurred, executing task")

def delayed_start(delay, target, args):
    time.sleep(delay)
    thread = threading.Thread(target=target, args=args)
    thread.start()
    return thread

event = threading.Event()

# Start thread after delay
task_thread = delayed_start(5, deferred_task, (event,))

# Wait before setting event
time.sleep(5)
event.set()</code></pre>
            <p class="mt-2">Use <code class="language-python">threading.Event</code> for synchronization between threads, and <code class="language-python">threading.Timer</code> for delayed execution.</p>
        </div>
        
        <!-- Green Thread Management -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Thread Management</h3>
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
            <p class="mb-2"><strong>Threading:</strong></p>
            <pre class="line-numbers"><code class="language-python">import threading
import time
import signal

def task1(stop_event):
    while not stop_event.is_set():
        print("Task 1 running")
        time.sleep(1)

def task2(stop_event):
    while not stop_event.is_set():
        print("Task 2 running")
        time.sleep(1)

stop_event = threading.Event()

thread1 = threading.Thread(target=task1, args=(stop_event,))
thread2 = threading.Thread(target=task2, args=(stop_event,))

thread1.start()
thread2.start()

# Let tasks run for a while
time.sleep(5)

# Signal threads to stop
stop_event.set()

# Wait for threads to finish
thread1.join()
thread2.join()</code></pre>
            <p class="mt-2">Use a shared <code class="language-python">threading.Event</code> to coordinate stopping multiple threads.</p>
        </div>
        
        <!-- Socket Compatibility -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Socket Operations</h3>
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
            <p class="mb-2"><strong>Threading:</strong></p>
            <pre class="line-numbers"><code class="language-python">import socket
import threading

def handle_client(client_socket):
    try:
        while True:
            data = client_socket.recv(1024)
            if not data:
                break
            client_socket.sendall(data)
    finally:
        client_socket.close()

def echo_server(port):
    server = socket.socket()
    server.bind(('0.0.0.0', port))
    server.listen(5)
    
    try:
        while True:
            client_socket, addr = server.accept()
            client_thread = threading.Thread(
                target=handle_client, 
                args=(client_socket,)
            )
            client_thread.daemon = True
            client_thread.start()
    finally:
        server.close()

# Run the server in a dedicated thread
server_thread = threading.Thread(target=echo_server, args=(6000,))
server_thread.daemon = True
server_thread.start()

# Keep main thread alive
server_thread.join()</code></pre>
            <p class="mt-2">Create a new thread for each client connection, or use a thread pool for better resource management.</p>
        </div>
        
        <!-- Pool Management -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Worker Pool Management</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">import eventlet

def worker(item):
    print(f"Processing {item}")
    eventlet.sleep(1)  # Simulating work
    return item * 2

# Create a limited pool
pool = eventlet.GreenPool(size=4)

items = range(10)
for result in pool.imap(worker, items):
    print(f"Result: {result}")</code></pre>
            <p class="mb-2"><strong>Threading:</strong></p>
            <pre class="line-numbers"><code class="language-python">from concurrent.futures import ThreadPoolExecutor
import time

def worker(item):
    print(f"Processing {item}")
    time.sleep(1)  # Simulating work
    return item * 2

# Create a limited thread pool
with ThreadPoolExecutor(max_workers=4) as executor:
    items = range(10)
    for item, result in zip(items, executor.map(worker, items)):
        print(f"Result for {item}: {result}")
        
    # Alternative approach:
    # futures = [executor.submit(worker, item) for item in items]
    # for future in as_completed(futures):
    #     try:
    #         result = future.result()
    #         print(f"Result: {result}")
    #     except Exception as exc:
    #         print(f"Generated an exception: {exc}")</code></pre>
            <p class="mt-2">Use <code class="language-python">ThreadPoolExecutor</code> from the <code class="language-python">concurrent.futures</code> module to manage worker thread pools efficiently.</p>
        </div>
        
        <!-- Integration with Other Libraries -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Integration with Other Libraries</h3>
            <p class="mb-2"><strong>Eventlet:</strong></p>
            <pre class="line-numbers"><code class="language-python">import eventlet
import eventlet.queue as queue

def producer(q):
    for i in range(10):
        q.put(i)
        eventlet.sleep(0.1)

def consumer(q):
    while True:
        item = q.get()
        print(f"Processed: {item}")
        q.task_done()

q = queue.Queue()
eventlet.spawn(producer, q)
eventlet.spawn(consumer, q)
q.join()</code></pre>
            <p class="mb-2"><strong>Threading:</strong></p>
            <pre class="line-numbers"><code class="language-python">import threading
import queue
import time

def producer(q, stop_event):
    for i in range(10):
        q.put(i)
        time.sleep(0.1)
    stop_event.set()  # Signal that production is complete

def consumer(q, stop_event):
    while not stop_event.is_set() or not q.empty():
        try:
            item = q.get(timeout=0.5)
            print(f"Processed: {item}")
            q.task_done()
        except queue.Empty:
            continue

q = queue.Queue()
stop_event = threading.Event()

producer_thread = threading.Thread(target=producer, args=(q, stop_event))
consumer_thread = threading.Thread(target=consumer, args=(q, stop_event))

producer_thread.start()
consumer_thread.start()

# Wait for all tasks to be processed
q.join()

# Wait for threads to finish
producer_thread.join()
consumer_thread.join()</code></pre>
            <p class="mt-2">Use Python's standard <code class="language-python">queue</code> module with threading for thread-safe task distribution.</p>
        </div>
    </div>
    
    <h3 class="mt-10 text-2xl font-bold mb-3">Tips for Refactoring</h3>
    <ul class="mt-4 text-xl list-disc list-inside">
        <li>Use <code class="language-python">threading.Thread</code> for basic thread creation</li>
        <li>Implement proper thread synchronization using locks, events, and semaphores</li>
        <li>Consider <code class="language-python">ThreadPoolExecutor</code> for managing thread pools efficiently</li>
        <li>Always handle thread join and cleanup to avoid resource leaks</li>
        <li>Be aware of the Global Interpreter Lock (GIL) which can limit CPU-bound parallel performance</li>
        <li>Use <code class="language-python">threading.local()</code> for thread-local storage to avoid race conditions</li>
        <li>Consider signal handling for timeouts and graceful shutdown</li>
    </ul>
</section>

<section>
    <h2 id="key-migration-patterns" class="mt-10 text-3xl font-bold mb-6">Key Migration Patterns <a href="#key-migration-patterns" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="text-xl">This section is a curated list of migration patterns that you might meet during your journey.</p>

    <div class="mt-8">
        <h3 class="text-2xl font-bold mb-4">Removing Monkey Patching</h3>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">Before (Eventlet):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import eventlet
eventlet.monkey_patch()</code></pre>
        </div>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">After (Native Threading):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python"># Explicit imports without altering the behavior of standard modules
import socket
import threading</code></pre>
        </div>
    </div>

    <div class="mt-8">
        <h3 class="text-2xl font-bold mb-4">Replacing Green Threads and Spawn Functions</h3>
        
        <p class="mt-4 text-xl">Using spawn / spawn_after</p>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">Before (Eventlet):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import eventlet

# Asynchronous launch
eventlet.spawn(process_data, data)

# Launch with delay
eventlet.spawn_after(delay, function)</code></pre>
        </div>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">After (Threading):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import threading

# Launch a thread
threading.Thread(target=process_data, args=(data,)).start()

# Launch with a delay using Timer
threading.Timer(delay, function).start()</code></pre>
        </div>
    </div>

    <div class="mt-8">
        <h3 class="text-2xl font-bold mb-4">Refactoring Concurrency Pools and Executors</h3>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">Before (Eventlet):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import eventlet

pool = eventlet.GreenPool(100)
for item in items:
    pool.spawn_n(process_item, item)
pool.waitall()</code></pre>
        </div>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">After Using ThreadPoolExecutor (Static Size):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">from concurrent.futures import ThreadPoolExecutor

with ThreadPoolExecutor(max_workers=100) as executor:
    futures = [executor.submit(process_item, item) for item in items]
    for future in futures:
        future.result()  # Optionally, handle exceptions</code></pre>
        </div>

        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">After Using ResizableThreadPool (A Dynamically Sized Thread Pool):</p>
            <p  class="text-xl">To replace GreenPool with a dynamically resizable thread pool, you can use a custom implementation of a thread pool that allows resizing. Below is an example of how you can achieve this using Python's threading module:</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import threading
from queue import Queue
import time

class ResizableThreadPool:
    def __init__(self, initial_size):
        self.tasks = Queue()
        self.lock = threading.Lock()
        self.threads = []
        self.resize(initial_size)

    def worker(self):
        while True:
            task, args, kwargs = self.tasks.get()
            if task is None:
                break
            try:
                task(*args, **kwargs)
            finally:
                self.tasks.task_done()

    def resize(self, new_size):
        with self.lock:
            current_size = len(self.threads)
            if new_size > current_size:
                for _ in range(new_size - current_size):
                    thread = threading.Thread(target=self.worker)
                    thread.daemon = True
                    thread.start()
                    self.threads.append(thread)
            elif new_size < current_size:
                for _ in range(current_size - new_size):
                    self.tasks.put((None, None, None))
                self.threads = self.threads[:new_size]

    def spawn(self, task, *args, **kwargs):
        self.tasks.put((task, args, kwargs))

    def join(self):
        self.tasks.join()

# Example usage
def example_task(duration):
    print(f"Task started, sleeping for {duration} seconds")
    time.sleep(duration)
    print("Task completed")

# Initialize the pool with 2 threads
pool = ResizableThreadPool(initial_size=2)

# Spawn tasks
for i in range(5):
    pool.spawn(example_task, i)

# Resize the pool to 4 threads
pool.resize(4)

# Wait for all tasks to complete
pool.join()</code></pre>
        <p class="text-xl">In this example, ResizableThreadPool allows you to dynamically resize the pool by adding or removing worker threads. You can use the spawn method to add tasks to the pool and the resize method to change the number of threads.</p>
        <p class="text-xl">Please find out <a href="https://review.opendev.org/c/openstack/neutron/+/945566" class="text-cyan-400" target="_blank">this living example</a> of the implementation of this pattern.</p>
        </div>
    </div>

    <div class="mt-8">
        <h3 class="text-2xl font-bold mb-4">Timeout Management</h3>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">Before (Eventlet):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import eventlet

try:
    with eventlet.Timeout(5):
        do_something()
except eventlet.Timeout:
    handle_timeout()</code></pre>
        </div>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">After (Using signal or join with timeout):</p>
            <p class="mt-2 text-xl italic">Example using the signal module:</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import signal

def handler(signum, frame):
    raise TimeoutError("Operation timed out")

signal.signal(signal.SIGALRM, handler)
signal.alarm(5)  # Set a timeout of 5 seconds

try:
    do_something()
except TimeoutError:
    handle_timeout()
finally:
    signal.alarm(0)  # Cancel the timeout</code></pre>

            <p class="mt-4 text-xl italic">Example using a thread with join:</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">thread = threading.Thread(target=function)
thread.start()
thread.join(timeout=timeout_value)

if thread.is_alive():
    LOG.warning("Action timed out, waiting for the thread to finish...")
    thread.join()  # Extended waiting if necessary</code></pre>
        </div>
    </div>

    <div class="mt-8">
        <h3 class="text-2xl font-bold mb-4">Adjusting Sleep Calls</h3>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">Before (Eventlet):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import eventlet
eventlet.sleep(0.5)</code></pre>
        </div>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">After (Using time.sleep):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import time
time.sleep(0.5)</code></pre>
        </div>
    </div>

    <div class="mt-8">
        <h3 class="text-2xl font-bold mb-4">Replacing Semaphores and Events</h3>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">Before (Eventlet):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">from eventlet import semaphore, event

lock = semaphore.Semaphore()
flag = event.Event()</code></pre>
        </div>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">After (Threading):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import threading

lock = threading.Semaphore()
flag = threading.Event()</code></pre>
        </div>
    </div>

    <div class="mt-8">
        <h3 class="text-2xl font-bold mb-4">Cleaning Up Imports and Reducing Eventlet Dependencies</h3>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">Before (Eventlet):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import eventlet
import eventlet.wsgi</code></pre>
        </div>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">After (Using a standard WSGI server):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">import socket
from wsgiref.simple_server import make_server</code></pre>
        </div>
    </div>

    <div class="mt-8">
        <h3 class="text-2xl font-bold mb-4">Refactoring Asynchronous I/O Wrappers</h3>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">Before (Eventlet):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">from eventlet.green import socket

s = socket.socket()
s.connect(('example.com', 80))</code></pre>
        </div>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">After (Using a server or standard I/O approach):</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">from wsgiref.simple_server import make_server

server = make_server('0.0.0.0', 8080, app)
server.serve_forever()</code></pre>
        </div>
    </div>

    <div class="mt-8">
        <h3 class="text-2xl font-bold mb-4">Explicit Thread Termination</h3>
        
        <p class="mt-4 text-xl">Unlike Eventlet—which provides methods like <code class="language-python">kill()</code> or <code class="language-python">wait()</code>—with threading it is recommended to use <code class="language-python">join()</code> to wait for a thread to finish.</p>
        
        <div class="mt-4 mb-6">
            <p class="text-xl font-semibold">Example:</p>
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto line-numbers"><code class="language-python">thread = threading.Thread(target=function)
thread.start()
thread.join()  # Wait for the thread to finish</code></pre>
        </div>
    </div>
</section>

<section>
    <h2 id="threading-libraries" class="mt-10 text-3xl font-bold mb-6">Threading Libraries <a href="#threading-libraries" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">Beyond Python's standard library threading modules, several third-party libraries can enhance your threading capabilities. These libraries provide higher-level abstractions, additional functionalities, and improved performance for specific use cases.</p>
    
    <div class="mt-10 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-10">
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">joblib</h3>
            <p class="text-xl">A lightweight library providing simple tools for parallel execution. It's particularly useful for CPU-bound operations that need to bypass the GIL limitations.</p>
            <p class="mt-4"><a href="https://joblib.readthedocs.io/" class="text-cyan-400" target="_blank">Documentation</a></p>
            <p class="mt-2 text-lg italic">Example:</p>
            <pre class="line-numbers mt-2"><code class="language-python">from joblib import Parallel, delayed

def process_item(item):
    return item * 2

results = Parallel(n_jobs=4)(
    delayed(process_item)(i) for i in range(10)
)</code></pre>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Twisted</h3>
            <p class="text-xl">An event-driven networking engine that can work alongside threading code for network applications. It's particularly useful for servers handling many concurrent connections.</p>
            <p class="mt-4"><a href="https://twisted.org/" class="text-cyan-400" target="_blank">Documentation</a></p>
            <p class="mt-2 text-lg italic">Key features:</p>
            <ul class="list-disc list-inside">
                <li>Event-driven architecture</li>
                <li>Support for many network protocols</li>
                <li>Integration with threading for mixed concurrency models</li>
            </ul>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">SQLAlchemy</h3>
            <p class="text-xl">While primarily an ORM, SQLAlchemy provides excellent thread-safety features for database operations, including connection pooling designed for threaded applications.</p>
            <p class="mt-4"><a href="https://www.sqlalchemy.org/" class="text-cyan-400" target="_blank">Documentation</a></p>
            <p class="mt-2 text-lg italic">Thread-safe features:</p>
            <ul class="list-disc list-inside">
                <li>Thread-local session management</li>
                <li>Connection pooling optimized for threading</li>
                <li>Thread-safe query execution</li>
            </ul>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">RQ (Redis Queue)</h3>
            <p class="text-xl">A simple Python library for queueing jobs and processing them in the background with workers. Great for distributing work across threads or processes.</p>
            <p class="mt-4"><a href="https://python-rq.org/" class="text-cyan-400" target="_blank">Documentation</a></p>
            <p class="mt-2 text-lg italic">Example:</p>
            <pre class="line-numbers mt-2"><code class="language-python">from redis import Redis
from rq import Queue

q = Queue(connection=Redis())
job = q.enqueue(long_running_function, arg1, arg2)
# Job runs in a worker thread or process</code></pre>
        </div>
    </div>
    
    <p class="mt-10 text-xl">When selecting a threading library, consider your specific requirements, such as the nature of your workloads (I/O-bound vs. CPU-bound), the level of concurrency needed, and any specific features like timeout handling or thread-safe data structures that your application requires.</p>
    
    <p class="mt-6 text-xl">Remember that for CPU-bound tasks, Python's <code class="language-python">multiprocessing</code> module might be more appropriate due to GIL limitations. For mixed workloads, combining threading with multiprocessing can provide optimal performance.</p>
</section>

<section>
    <h2 id="common-pitfalls" class="mt-10 text-3xl font-bold mb-6">Common Pitfalls <a href="#common-pitfalls" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">Be aware of the following common pitfalls during the migration from Eventlet to native threading:</p>
    <div class="mt-10 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-10">
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-4">Race conditions and thread safety</h3>
            <p class="text-xl">Unlike Eventlet's cooperative multitasking where tasks yield control explicitly, native threads can be preempted at any time. Always use proper synchronization mechanisms (locks, semaphores, events) when accessing shared resources to prevent race conditions.</p>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-4">Deadlocks and thread starvation</h3>
            <p class="text-xl">Improper use of locks can lead to deadlocks where threads wait indefinitely for resources held by other threads. Always acquire locks in a consistent order and use timeouts when appropriate to prevent deadlocks. Also consider using <code class="language-python">threading.RLock</code> for recursive lock acquisition in the same thread.</p>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-4">Thread lifecycle management</h3>
            <p class="text-xl">Unlike Eventlet green threads, native threads cannot be easily terminated from outside. Design your threads with proper termination signals (using <code class="language-python">threading.Event</code> objects) and ensure all threads are joined before application exit to prevent resource leaks and orphaned threads.</p>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-4">Global Interpreter Lock (GIL) limitations</h3>
            <p class="text-xl">The Python GIL prevents multiple threads from executing Python bytecodes simultaneously, limiting performance gains for CPU-bound tasks. Threading works best for I/O-bound operations. For CPU-intensive tasks, consider using <code class="language-python">multiprocessing</code> to achieve true parallelism.</p>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-4">Debugging difficulties</h3>
            <p class="text-xl">Multithreaded code can be notoriously difficult to debug due to its non-deterministic nature. Use thread names, comprehensive logging, and thread-local storage to make debugging easier. Consider specialized tools like <code class="language-python">faulthandler</code> for tracking thread crashes.</p>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-4">Overuse of thread synchronization</h3>
            <p class="text-xl">Excessive use of locks and other synchronization primitives can lead to contention and performance degradation. Design your code to minimize shared state, and consider using thread-safe collections from <code class="language-python">queue</code> or <code class="language-python">concurrent.futures</code> modules instead of manually synchronizing access to standard collections.</p>
        </div>
    </div>
</section>

<section>
    <h2 id="best-practices-and-considerations" class="mt-10 text-3xl font-bold mb-6">Best Practices and Considerations <a href="#best-practices-and-considerations" class="text-cyan-400 text-xl">🔗</a></h2>

    <ul class="mt-6 text-xl space-y-6">
        <li>
            <strong>Active Thread Monitoring:</strong><br>
            Implement proactive logging to detect and manage threads that exceed expected runtimes.
            <pre class="bg-gray-800 text-white p-4 rounded-lg overflow-x-auto mt-2 line-numbers"><code class="language-python">if thread.is_alive():
    LOG.warning("The action thread has exceeded the expected runtime.")</code></pre>
        </li>
        
        <li>
            <strong>Explicit Executor Management:</strong><br>
            When using <code class="language-python">ThreadPoolExecutor</code>, ensure you handle exceptions appropriately and monitor active thread statuses.
        </li>
        
        <li>
            <strong>Anticipate Non-Termination of Threads:</strong><br>
            Native threads cannot be arbitrarily terminated; thus, design your application logic to gracefully handle thread shutdowns.
        </li>
    </ul>
</section>

<section>
    <h2 id="conclusion" class="mt-10 text-3xl font-bold mb-6">Conclusion <a href="#conclusion" class="text-cyan-400 text-xl">🔗</a></h2>

    <p class="mt-6 text-xl">Migrating from Eventlet to native threading offers several key benefits:</p>
    
    <ul class="mt-4 text-xl list-disc list-inside">
        <li>
            <strong>Clarity and Transparency:</strong><br>
            Eliminating monkey patching ensures predictable behavior of standard modules.
        </li>
        <li>
            <strong>Compatibility and Maintainability:</strong><br>
            Leveraging Python's standard tools (like <code class="language-python">threading</code> and <code class="language-python">concurrent.futures</code>) simplifies integration with other libraries and supports long-term maintenance.
        </li>
        <li>
            <strong>Explicit Concurrency Control:</strong><br>
            Shifting to explicit threads allows for better tracking and fine-grained management of asynchronous tasks.
        </li>
    </ul>
    
    <p class="mt-6 text-xl">Whether applied in the specific context of OpenStack or in broader applications, adopting the practices presented in this chapter ensures a smooth transition to modern, robust threading mechanisms.</p>
</section>