---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: dashboard
title: Understand Eventlet
---
<section>
    <h1 class="text-4xl font-bold">Understanding Eventlet</h1>
    <p class="mt-10 text-xl">Eventlet is a Python concurrent network programming library created nearly 18 years ago. Its intent is to propose asynchronous feature at destination of the Python ecosystem. Eventlet has been created at a time where the Python standard library was not designed to support async. Since then, a lot of water has flowed under the bridge, and AsyncIO emerged from async waves.</p>
</section>
<section>
    <h2 class="mt-10 text-2xl font-bold">The promises of Eventlet</h2>
    <p class="mt-10 text-xl">Eventlet was created with good intentions. The main promise of Eventlet was to allow people to transform existing synchronous code in an asynchronous code without even rewriting one line of that existing code. Making asynchronous implicit. The magic behind this behavior is due to <a href="https://en.wikipedia.org/wiki/Monkey_patch" class="text-cyan-400" target="_blank">monkey patching</a>.</p>
    <p class="mt-10 text-xl"> This trick, monkey patching, is used to transform all the blocking IO of the stack loaded at runtime into non blocking IO. This is done by Eventlet by modifying the internal of the standard library through specific monkey patches.</p>
    <p class="mt-10 text-xl">The problem is that it make Eventlet makes Eventlet sensitive to each new version of Python. In parallel of that, one version of Eventlet needs to be compatible with several versions of Python at the same time, between 4 or 5 versions.</p>
</section>
<section>
    <h2 class="mt-10 text-2xl font-bold">Anatomy Eventlet</h2>
    <p class="mt-10 text-xl">Modern libraries often offer better performance and scalability for handling concurrent tasks. Modern libraries like Asyncio introduce new paradigms like real cooperative task handling. Threads, with modern versions of Python can benefits from <a href="https://peps.python.org/pep-0703/" class="text-cyan-400" target="_blank">PEP 703</a> to introduce real parallelism in your Python software.</p>
</section>
<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link getting-started.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>Getting Started
    </a>
    <a href="{{ site.baseurl }}{% link alternatives.md %}" class="inline-block bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        Understanding the Alternatives<i class="fas fa-arrow-right ml-2"></i>
    </a>
</div>