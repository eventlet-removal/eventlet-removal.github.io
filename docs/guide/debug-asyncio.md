---
layout: dashboard
title: Debugging Async Code with AsyncIO Inspector
permalink: /guide/debug-asyncio/
id: debug-asyncio
description: Learn how to debug your async code with Python 3.14's new asyncio inspector tools. This guide covers how to identify tasks, deadlocks, and other common issues when migrating from Eventlet to AsyncIO.
keywords: asyncio debugging, asyncio inspector, async code debugging, python 3.14, asyncio ps, asyncio pstree, eventlet debug, async debugging tools
og_type: article
og_title: Debug Async Code with Python 3.14's AsyncIO Inspector Tools
og_description: Discover how to effectively debug asynchronous Python code with Python 3.14's new asyncio inspector tools after migrating from Eventlet.
---
<section>
    <h1 class="text-4xl font-bold mb-6">Debugging Async Code with AsyncIO Inspector (Python 3.14)</h1>
    <p class="mt-10 text-xl">Learn how to keep—and improve—your debugging workflow when migrating from Eventlet to AsyncIO using Python 3.14's new external inspector tools.</p>
    
    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#why-this-page-exists" class="text-cyan-400 hover:underline">Overview of the AsyncIO Inspector</a></li>
                <li><a href="#audience-prerequisites" class="text-cyan-400 hover:underline">Audience & Prerequisites</a></li>
                <li><a href="#quick-demo" class="text-cyan-400 hover:underline">Quick Demo — 60 Seconds to First Stack Trace</a></li>
                <li><a href="#eventlet-vs-asyncio" class="text-cyan-400 hover:underline">Eventlet versus AsyncIO Introspection</a></li>
                <li><a href="#understanding-output" class="text-cyan-400 hover:underline">Understanding the Output</a></li>
                <li><a href="#best-practices" class="text-cyan-400 hover:underline">Best Practices During Migration</a></li>
                <li><a href="#troubleshooting" class="text-cyan-400 hover:underline">Troubleshooting the Inspector</a></li>
                <li><a href="#quick-reference" class="text-cyan-400 hover:underline">Quick Reference Commands</a></li>
                <li><a href="#further-reading" class="text-cyan-400 hover:underline">Further Reading</a></li>
            </ul>
        </div>
    </div>
</section>
<section>
    <h2 id="why-this-page-exists" class="mt-10 text-3xl font-bold mb-6">Overview of the AsyncIO Inspector <a href="#why-this-page-exists" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">This guide is about <strong>how to keep—and improve—your debugging workflow</strong> once you migrated. Python 3.14 introduces a pair of command-line tools that let you inspect a live <code class="language-python">asyncio</code> program <strong>from outside the process</strong>, no telnet consoles or custom logging required.</p>
    
    <div class="mt-6 overflow-x-auto">
        <table class="min-w-full bg-gray-800 text-white rounded-lg overflow-hidden">
            <thead class="bg-gray-700">
                <tr>
                    <th class="px-4 py-2 text-left">New tool</th>
                    <th class="px-4 py-2 text-left">What it does</th>
                    <th class="px-4 py-2 text-left">One-liner</th>
                </tr>
            </thead>
            <tbody>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2"><code>python -m asyncio ps &lt;PID&gt;</code></td>
                    <td class="px-4 py-2">Dump a <strong>flat table</strong> of every running task plus its coroutine stack.</td>
                    <td class="px-4 py-2"><code>python -m asyncio ps 12345</code></td>
                </tr>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2"><code>python -m asyncio pstree &lt;PID&gt;</code></td>
                    <td class="px-4 py-2">Show a <strong>hierarchical await tree</strong> (who awaits whom).</td>
                    <td class="px-4 py-2"><code>python -m asyncio pstree 12345</code></td>
                </tr>
            </tbody>
        </table>
    </div>
    
    <p class="mt-6 text-xl">These cover the same needs Eventlet users traditionally solved with <code class="language-python">eventlet.debug</code>, <code class="language-python">hub_blocking_detection()</code>, and the backdoor telnet console—only now they're:</p>
    
    <ul class="mt-4 text-xl list-disc list-inside">
        <li><strong>Zero-intrusion</strong> (no code changes, works on prod PIDs).</li>
        <li><strong>Read-only & safe</strong> (uses the debugger API; won't freeze your loop).</li>
        <li><strong>Structured</strong> (TaskGroup names, await relationships, cycle detection).</li>
    </ul>
</section>
<section>
    <h2 id="audience-prerequisites" class="mt-10 text-3xl font-bold mb-6">Audience & Prerequisites <a href="#audience-prerequisites" class="text-cyan-400 text-xl">🔗</a></h2>
    <ul class="mt-4 text-xl list-disc list-inside">
        <li>You operate services that still use <strong>Eventlet</strong> in places and have begun migrating parts to <strong>asyncio</strong>.</li>
        <li>You're already running <strong>Python 3.14</strong> (beta or newer) in staging or prod.</li>
        <li>You want to identify deadlocks, long-running coroutines, or await cycles quickly.</li>
    </ul>
    <p class="mt-4 text-xl">No prior knowledge of <code class="language-python">pdb</code> or low-level CPython internals required.</p>
</section>
<section>
    <h2 id="quick-demo" class="mt-10 text-3xl font-bold mb-6">Quick Demo — 60 Seconds to First Stack Trace <a href="#quick-demo" class="text-cyan-400 text-xl">🔗</a></h2>
    <div class="bg-indigo-900 p-6 rounded-lg shadow">
        <pre class="language-bash"><code class="language-bash"># 1. Start any asyncio program (PID will vary)
python my_server.py &      # assume PID=12345

# 2. Flat view
python -m asyncio ps 12345

# 3. Hierarchical view
python -m asyncio pstree 12345</code></pre>
    </div>
    
    <p class="mt-6 text-xl">Sample <code>ps</code> output (first 3 rows):</p>
    <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
        <pre class="language-bash"><code class="language-bash">python -m asyncio ps 12345

tid       task_id           task_name     coroutine_chain                       awaiter_name  runtime
--------- ----------------- ------------- ------------------------------------- ------------- -------
1407356   0x7f8b1b5d9e90    tg_workers    crawl_site -> parse_links             main          12.4s
1407356   0x7f8b1b5da250    heartbeat     heartbeat -> asyncio.sleep            main          0.5s
1407356   0x7f8b1b5db610    crawl_site    parse_links -> fetch_url              tg_workers    3.2s</code></pre>
    </div>
    
    <p class="mt-6 text-xl">Sample <code>pstree</code> snippet (abridged):</p>
    <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
        <pre class="language-bash"><code class="language-bash">python -m asyncio pstree 12345

└── (T) tg_workers [12 tasks]
    ├── crawl_site('https://example.com')
    │   └── parse_links()
    │       ├── fetch_url('https://example.com/a')  [3.2 s]
    │       └── fetch_url('https://example.com/b')  [3.1 s]
    ├── crawl_site('https://example.org')
    │   └── parse_links()
    │       ├── fetch_url('https://example.org/a')  [3.0 s]
    │       └── fetch_url('https://example.org/b')  [3.0 s]
    └── heartbeat()  [sleeping 0.5 s]</code></pre>
    </div>
    
    <p class="mt-6 text-xl"><strong>Red flags:</strong></p>
    <ul class="mt-4 text-xl list-disc list-inside">
        <li>Leaf tasks stuck &gt; N seconds.</li>
        <li>A branch that keeps growing (possible memory leak).</li>
    </ul>
</section>
<section>
    <h2 id="eventlet-vs-asyncio" class="mt-10 text-3xl font-bold mb-6">Eventlet versus AsyncIO Introspection—Cheat Sheet <a href="#eventlet-vs-asyncio" class="text-cyan-400 text-xl">🔗</a></h2>
    <div class="mt-6 overflow-x-auto">
        <table class="min-w-full bg-gray-800 text-white rounded-lg overflow-hidden">
            <thead class="bg-gray-700">
                <tr>
                    <th class="px-4 py-2 text-left">Concern</th>
                    <th class="px-4 py-2 text-left">Eventlet tooling</th>
                    <th class="px-4 py-2 text-left">AsyncIO 3.14 equivalent</th>
                    <th class="px-4 py-2 text-left">Pros of new tool</th>
                </tr>
            </thead>
            <tbody>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2">Flat list of green threads / tasks</td>
                    <td class="px-4 py-2"><code>eventlet.debug.format_threads_info()</code></td>
                    <td class="px-4 py-2"><code>python -m asyncio ps</code></td>
                    <td class="px-4 py-2">No in-process call needed; includes call stacks.</td>
                </tr>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2">Call hierarchy</td>
                    <td class="px-4 py-2">N/A (manual walk in backdoor)</td>
                    <td class="px-4 py-2"><code>python -m asyncio pstree</code></td>
                    <td class="px-4 py-2">Readable tree; highlights cycles automatically.</td>
                </tr>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2">Blocking detector</td>
                    <td class="px-4 py-2"><code>hub_blocking_detection(timeout)</code></td>
                    <td class="px-4 py-2">Implicit: long leaf tasks in <code>pstree</code></td>
                    <td class="px-4 py-2">Non-intrusive; no thread-interrupt side-effects.</td>
                </tr>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2">Live console</td>
                    <td class="px-4 py-2"><code>eventlet.backdoor.backdoor_server</code> (telnet)</td>
                    <td class="px-4 py-2">Use external CLI + <code>await asyncio.to_thread(pdb.set_trace)</code> if needed</td>
                    <td class="px-4 py-2">No open port; safer in prod.</td>
                </tr>
            </tbody>
        </table>
    </div>
</section>
<section>
    <h2 id="understanding-output" class="mt-10 text-3xl font-bold mb-6">Understanding the Output <a href="#understanding-output" class="text-cyan-400 text-xl">🔗</a></h2>
    
    <h3 class="mt-6 text-2xl font-bold">PS Columns</h3>
    <div class="mt-4 overflow-x-auto">
        <table class="min-w-full bg-gray-800 text-white rounded-lg overflow-hidden">
            <thead class="bg-gray-700">
                <tr>
                    <th class="px-4 py-2 text-left">Column</th>
                    <th class="px-4 py-2 text-left">Meaning</th>
                </tr>
            </thead>
            <tbody>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2"><strong>tid</strong></td>
                    <td class="px-4 py-2">Native thread ID (useful if you run multiple loops).</td>
                </tr>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2"><strong>task id</strong></td>
                    <td class="px-4 py-2">Memory address of the <code>asyncio.Task</code> object.</td>
                </tr>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2"><strong>task name</strong></td>
                    <td class="px-4 py-2">Comes from <code>task.set_name()</code> or TaskGroup child naming.</td>
                </tr>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2"><strong>coroutine chain</strong></td>
                    <td class="px-4 py-2">Top-of-stack coroutine → … → root.</td>
                </tr>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2"><strong>awaiter</strong></td>
                    <td class="px-4 py-2">Which task (if any) is currently awaiting this task.</td>
                </tr>
            </tbody>
        </table>
    </div>
    
    <h3 class="mt-6 text-2xl font-bold">PSTREE Glyphs</h3>
    <ul class="mt-4 text-xl list-disc list-inside">
        <li><code>(T)</code> — a <em>task</em> node (leaf or internal).</li>
        <li>Indentation shows await nesting.</li>
        <li>Cycle detection: if the await graph forms a loop, the tool aborts and prints the exact path twice (start ↔︎ end).</li>
    </ul>
</section>
<section>
    <h2 id="best-practices" class="mt-10 text-3xl font-bold mb-6">Best Practices During Migration <a href="#best-practices" class="text-cyan-400 text-xl">🔗</a></h2>
    <ol class="mt-4 text-xl list-decimal list-inside">
        <li><strong>Name your tasks</strong> — <code class="language-python">asyncio.create_task(coro, name="crawler_page")</code> or inside <code class="language-python">TaskGroup</code> use <code class="language-python">create_task(coro, name=...)</code>. These names appear in both inspectors.</li>
        <li><strong>Keep Eventlet debug hooks until removed</strong> — they still help with the legacy side; use the asyncio CLI for the new side.</li>
        <li><strong>Script it</strong> — add a CI health check:
            <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
                <pre class="language-bash"><code class="language-bash">python -m asyncio ps $PID | grep -q "TaskGroup.*blocked" && exit 1</code></pre>
            </div>
            <p class="mt-2">Fail fast if a task exceeds your SLA.</p>
        </li>
        <li><strong>Combine with profiling</strong> — long-running leaves can be profiled via <code class="language-python">asyncio.run(await aiomonitor.start_server())</code> <strong>inside staging</strong> only.</li>
    </ol>
</section>
<section>
    <h2 id="troubleshooting" class="mt-10 text-3xl font-bold mb-6">Troubleshooting the Inspector Itself <a href="#troubleshooting" class="text-cyan-400 text-xl">🔗</a></h2>
    <div class="mt-6 overflow-x-auto">
        <table class="min-w-full bg-gray-800 text-white rounded-lg overflow-hidden">
            <thead class="bg-gray-700">
                <tr>
                    <th class="px-4 py-2 text-left">Symptom</th>
                    <th class="px-4 py-2 text-left">Cause</th>
                    <th class="px-4 py-2 text-left">Remedy</th>
                </tr>
            </thead>
            <tbody>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2"><code>PermissionError</code> when attaching</td>
                    <td class="px-4 py-2">Different user or container namespace</td>
                    <td class="px-4 py-2">Run as same UID / enter the pid-ns via <code>nsenter</code>.</td>
                </tr>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2">Empty output</td>
                    <td class="px-4 py-2">Wrong PID or Python interpreter < 3.14</td>
                    <td class="px-4 py-2">Verify with <code>ps aux</code> and <code>python -V</code>.</td>
                </tr>
                <tr class="border-t border-gray-600">
                    <td class="px-4 py-2">Unicode gibberish in stacks</td>
                    <td class="px-4 py-2">Terminal's locale mismatch</td>
                    <td class="px-4 py-2"><code>export LC_ALL=en_US.UTF-8</code>.</td>
                </tr>
            </tbody>
        </table>
    </div>
</section>
<section>
    <h2 id="quick-reference" class="mt-10 text-3xl font-bold mb-6">Quick Reference Commands <a href="#quick-reference" class="text-cyan-400 text-xl">🔗</a></h2>
    <div class="bg-indigo-900 p-6 rounded-lg shadow">
        <pre class="language-bash"><code class="language-bash"># Flat view, filter by task name
python -m asyncio ps &lt;PID&gt; | grep loader

# Limit output lines (rough approximation of depth)
python -m asyncio pstree &lt;PID&gt; | head -n 100</code></pre>
    </div>
</section>
<section>
    <h2 id="further-reading" class="mt-10 text-3xl font-bold mb-6">Further Reading <a href="#further-reading" class="text-cyan-400 text-xl">🔗</a></h2>
    <ul class="mt-4 text-xl list-disc list-inside">
        <li><a href="https://docs.python.org/3.14/whatsnew/3.14.html#asyncio-introspection-capabilities" class="text-cyan-400" target="_blank">What's New in Python 3.14 — asyncio introspection</a></li>
        <li><a href="https://github.com/python/cpython/issues/91048" class="text-cyan-400" target="_blank">Async Call-Stack Reconstruction</a></li>
        <li>Eventlet docs: <a href="https://eventlet.readthedocs.io/en/latest/modules/debug.html" class="text-cyan-400" target="_blank">Debugging utilities</a></li>
    </ul>
</section>
<section>
    <h2 class="mt-10 text-3xl font-bold mb-6">One-Pager Takeaway</h2>
    <div class="bg-indigo-900 p-6 rounded-lg shadow">
        <p class="text-xl"><strong>Python 3.14's <code>ps</code> and <code>pstree</code> put x-ray goggles on your event loop.</strong></p>
        <p class="mt-2 text-xl">They replace telnet backdoors and ad-hoc logging with an <em>outside-looking-in</em> view that's safer, richer, and ready for production.</p>
    </div>
</section>
<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/post-migration.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>Post-Migration Guidance
    </a>
    <a href="{{ site.baseurl }}{% link guide/studies.md %}" class="inline-block bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        Case Studies<i class="fas fa-arrow-right ml-2"></i>
    </a>
</div>
