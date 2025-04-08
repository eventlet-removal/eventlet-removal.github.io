---
permalink: /guide/resources/
title: Common Resources
layout: dashboard
description: Collection of common resources for understanding and implementing alternatives to Eventlet. These resources provide essential references, tools, and community-supported solutions to assist in your migration process.
keywords: eventlet resources, common tools, migration resources, asyncio resources, threading reference, community solutions, developer tools, code libraries, implementation guides
og_type: article
og_title: Common Resources for Eventlet Alternatives
og_description: Essential resources, tools, and reference materials for developers implementing alternatives to Eventlet in their applications.
---
<section>
    <h1 class="text-4xl font-bold">Recommended Resources</h1>
    <p class="mt-10 text-xl">Here is a curated list of resources to help you understand and implement alternatives to Eventlet:</p>

    <h2 class="text-3xl font-bold mt-10">Asyncio Resources</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10 mt-10">
        <!-- Asyncio Resources -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Awesome Asyncio</h3>
            <p>A comprehensive collection of libraries, tools, and resources for working with Python's <code>asyncio</code>.</p>
            <a href="https://github.com/timofurrer/awesome-asyncio" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Python asyncio Documentation</h3>
            <p>Official Python documentation for the <code>asyncio</code> library, including tutorials and API references.</p>
            <a href="https://docs.python.org/3.13/library/asyncio.html" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Awaitlet</h3>
            <p>A library for integrating <code>asyncio</code> with SQLAlchemy, enabling asynchronous database operations.</p>
            <a href="https://awaitlet.sqlalchemy.org/en/latest/" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Eventlet to asyncio Migration Guide</h3>
            <p>A guide to migrating from Eventlet to <code>asyncio</code>.</p>
            <a href="https://eventlet.readthedocs.io/en/latest/asyncio/migration.html#migration-guide" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Trio Documentation</h3>
            <p>A library for structured concurrency in Python, offering an alternative to asyncio.</p>
            <a href="https://trio.readthedocs.io" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Curio Documentation</h3>
            <p>A library for coroutine-based concurrency in Python, focusing on simplicity and performance.</p>
            <a href="https://curio.readthedocs.io" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
    </div>

    <h2 class="text-3xl font-bold mt-10">Threading Resources</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10 mt-10">
        <!-- Threading Resources -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Python threading Documentation</h3>
            <p>Official Python documentation for the <code>threading</code> module, useful for understanding multithreading in Python.</p>
            <a href="https://docs.python.org/3.13/library/threading.html" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
    </div>

    <h2 class="text-3xl font-bold mt-10">General Concurrency Resources</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10 mt-10">
        <!-- General Concurrency Resources -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Concurrency in Python</h3>
            <p>An in-depth article on Python's concurrency models, including threading, asyncio, and multiprocessing.</p>
            <a href="https://realpython.com/python-concurrency/" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Python Issue 22239</h3>
            <p>A Python bug tracker issue discussing the challenges and improvements related to asyncio and concurrency.</p>
            <a href="https://bugs.python.org/issue22239" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">What Color is Your Function?</h3>
            <p>An insightful article discussing the challenges and implications of mixing synchronous and asynchronous code.</p>
            <a href="https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Cotyledon Documentation</h3>
            <p>A framework for defining long-running services in Python, with graceful handling of system signals and worker processes.</p>
            <a href="https://cotyledon.readthedocs.io/en/latest/index.html" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Futurist Documentation</h3>
            <p>A library providing abstractions for asynchronous and synchronous execution patterns, developed by the OpenStack community.</p>
            <a href="https://docs.openstack.org/futurist/latest/" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
    </div>

    <h2 class="text-3xl font-bold mt-10">Eventlet Resources</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10 mt-10">
        <!-- Eventlet Resources -->
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">OpenStack Eventlet Removal Wiki</h3>
            <p>Documentation and guidelines from the OpenStack community on removing Eventlet from projects.</p>
            <a href="https://wiki.openstack.org/wiki/Eventlet-removal" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Eventlet Documentation</h3>
            <p>Official documentation for Eventlet, including installation and usage guides.</p>
            <a href="https://eventlet.readthedocs.io/en/latest/" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">OpenStack Governance Goal: Remove Eventlet</h3>
            <p>OpenStack's technical committee documentation on the goal to remove Eventlet from its ecosystem.</p>
            <a href="https://governance.openstack.org/tc/goals/selected/remove-eventlet.html" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h3 class="text-2xl font-bold mb-3">Greenlet Documentation</h3>
            <p>Official documentation for Greenlet, a lightweight in-process concurrent programming library often used with Eventlet.</p>
            <a href="https://greenlet.readthedocs.io/en/latest/" class="text-cyan-400 hover:underline" target="_blank">Learn more</a>
        </div>
    </div>
</section>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/testimonials.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>Testimonials
    </a>
    <a href="{{ site.baseurl }}{% link guide/tutorials.md %}" class="inline-block bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        Tutorials<i class="fas fa-arrow-right ml-2"></i>
    </a>
</div>