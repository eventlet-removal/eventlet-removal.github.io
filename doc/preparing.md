---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: dashboard
title: Preparing for Migration
---
<section>
    <h1 class="text-4xl font-bold">Preparing for Migration</h1>
    <p class="mt-10 text-xl">Before​ starting your migration it is important to take inventory of your use cases with Eventlet and so of your needs. This chapter help you to make this inventory.</p>
</section>
<section>
    <h2 class="mt-10 text-3xl font-bold">Evaluation of Needs</h2>

    <h3 class="mt-10 text-2xl font-bold">Identify Key Functionalities</h3>
    <p class="mt-10 text-xl">List all the features of your application that use Eventlet. Determine if these features are essential or if they can be replaced with alternatives.​<br>Among common Eventlet usages you can find:</p>
    <ul class="mt-10">
        <li class="mt-4 text-xl"><i class="fas fa-gear text-teal-300 mr-2"></i><strong>WSGI Server:</strong> Eventlet can be used to create a WSGI server, allowing Python web applications to be served asynchronously. This is particularly useful for handling a large number of simultaneous connections with low latency.</li>
        <li class="mt-4 text-xl"><i class="fas fa-gear text-teal-300 mr-2"></i><strong>Asynchronous Network Calls:</strong> Eventlet enables asynchronous network calls, such as HTTP requests to REST APIs. This prevents the main program execution from being blocked while waiting for network responses.</li>
        <li class="mt-4 text-xl"><i class="fas fa-gear text-teal-300 mr-2"></i><strong>Background Execution of Long Tasks:</strong> You can use Eventlet to run long tasks in the background without blocking the main thread. This is useful for operations like processing large files or performing intensive calculations.</li>
        <li class="mt-4 text-xl"><i class="fas fa-gear text-teal-300 mr-2"></i><strong>Deferred Task Management:</strong> Eventlet allows the creation of deferred tasks that can be executed later or in response to certain events. This is useful for managing complex workflows where some tasks depend on the completion of others.</li>
        <li class="mt-4 text-xl"><i class="fas fa-gear text-teal-300 mr-2"></i><strong>Green Thread Management:</strong> Eventlet uses green threads (or coroutines) to manage concurrency. This allows switching between different tasks without the need to create new system threads, which is more resource-efficient.</li>
        <li class="mt-4 text-xl"><i class="fas fa-gear text-teal-300 mr-2"></i><strong>Socket Compatibility:</strong> Eventlet provides an API compatible with Python's standard sockets, making it easier to migrate existing applications to an asynchronous model.</li>
        <li class="mt-4 text-xl"><i class="fas fa-gear text-teal-300 mr-2"></i><strong>WebSocket Support:</strong> Eventlet can be used to manage WebSocket connections, enabling real-time, bidirectional communication between the server and the client.</li>
        <li class="mt-4 text-xl"><i class="fas fa-gear text-teal-300 mr-2"></i><strong>Integration with Other Libraries:</strong> Eventlet can be integrated with other Python libraries to provide asynchronous functionality, such as databases or queue systems.</li>
    </ul>

</section>

<div class="mt-10 mermaid">
  graph TD;
    A[Start] --> B{Condition?};
    B -- Yes --> C[Do something];
    B -- No --> D[Do something else];
    C --> E[End];
    D --> E;
</div>
