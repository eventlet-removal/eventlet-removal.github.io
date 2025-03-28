---
layout: dashboard
title: Managing Migration Depth
permalink: /guide/managing-migration-depth/
description: Learn strategies for managing the depth of your Eventlet migration project, including incremental approaches and solutions for dealing with deeply rooted Eventlet dependencies in complex applications.
keywords: eventlet migration depth, incremental migration, complex refactoring, deep dependencies, awaitlet, python refactoring strategies
og_type: article
og_title: Strategies for Managing Deep Eventlet Dependencies in Your Migration
og_description: Learn how to approach complex migration scenarios where Eventlet is deeply integrated into your application architecture.
---

<section>
    <h1 class="text-4xl font-bold mb-6">Managing Migration Depth</h1>
    <p class="mt-10 text-xl">In large applications, Eventlet dependencies can become deeply embedded throughout the codebase, making a complete migration to AsyncIO challenging. This chapter explores strategies to manage migration depth, with a special focus on using Awaitlet to limit the impact of your refactoring efforts.</p>
    
    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#understanding-migration-depth" class="text-cyan-400 hover:underline">Understanding Migration Depth</a></li>
                <li><a href="#awaitlet-as-a-migration-tool" class="text-cyan-400 hover:underline">Awaitlet as a Migration Tool</a></li>
                <li><a href="#implementing-awaitlet" class="text-cyan-400 hover:underline">Implementing Awaitlet in Your Migration</a></li>
                <li><a href="#deep-dependencies" class="text-cyan-400 hover:underline">Managing Deep Dependencies</a></li>
                <li><a href="#internal-apis" class="text-cyan-400 hover:underline">Using Awaitlet with Internal Private APIs</a></li>
                <li><a href="#best-practices" class="text-cyan-400 hover:underline">Best Practices and Limitations</a></li>
                <li><a href="#case-study" class="text-cyan-400 hover:underline">Case Study: Incremental Migration of a Complex Application</a></li>
            </ul>
        </div>
    </div>
</section>

<section>
    <h2 id="understanding-migration-depth" class="mt-10 text-3xl font-bold mb-6">Understanding Migration Depth <a href="#understanding-migration-depth" class="text-cyan-400 text-xl">🔗</a></h2>
    
    <p class="mt-4 text-xl">When migrating from Eventlet to AsyncIO, one of the most significant challenges is dealing with the <strong>propagation of async/await keywords</strong> throughout your codebase. This is what we call the "migration depth" problem.</p>
    
    <div class="mt-6 bg-indigo-900 p-6 rounded-lg shadow">
        <h3 class="text-2xl font-bold mb-3">The Async Propagation Challenge</h3>
        <p>In AsyncIO, when a function uses <code class="language-python">await</code>, it must be defined with <code class="language-python">async def</code>. Any function that calls an async function must also be async, creating a chain reaction throughout your codebase.</p>
        
        <pre class="line-numbers mt-4"><code class="language-python"># The await keyword propagates up the call stack
async def low_level_io():
    # Async I/O operation
    return await asyncio.sleep(1)

async def mid_level_function():
    # Must be async to call low_level_io
    return await low_level_io()

async def high_level_function():
    # Must be async to call mid_level_function
    return await mid_level_function()

# Your entire call stack becomes async</code></pre>
    </div>
    
    <p class="mt-6 text-xl">For large applications with deeply nested call hierarchies, this can lead to:</p>
    
    <ul class="mt-4 text-xl list-disc list-inside space-y-2">
        <li><strong>Extensive refactoring:</strong> Changes ripple throughout your codebase, affecting far more code than you initially intended.</li>
        <li><strong>Complex transitions:</strong> APIs that span multiple modules or services become difficult to migrate incrementally.</li>
        <li><strong>Mixed compatibility issues:</strong> During migration, maintaining compatibility between async and sync code becomes challenging.</li>
    </ul>
    
    <div class="mt-6 mermaid">
        %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#312e81', 'primaryTextColor': '#000', 'primaryBorderColor': '#433e99', 'lineColor': '#ffffff', 'secondaryColor': '#312e81', 'tertiaryColor': '#312e81' }}}%%
        graph TD
        A[API Endpoint] --> B[Service Layer]
        B --> C[Business Logic]
        C --> D[Data Access]
        D --> E[Low-level I/O]
        
        style E fill:#e11d48,stroke:#f43f5e,primaryTextColor:#fff
        style D fill:#fb7185,stroke:#f43f5e
        style C fill:#fda4af,stroke:#f43f5e
        style B fill:#fecdd3,stroke:#f43f5e
        style A fill:#fff1f2,stroke:#f43f5e
        
        subgraph Propagation
        F["Low-level async change<br>(e.g., network I/O)"]
        G["Forces changes to propagate<br>up the entire call stack"]
        end
        
        E -.-> F
        F -.-> G
    </div>
    
    <p class="mt-6 text-xl">This is where the <strong>Awaitlet</strong> library comes in, offering a solution to contain the async propagation and manage the depth of your migration more effectively.</p>
</section>

<section>
    <h2 id="awaitlet-as-a-migration-tool" class="mt-10 text-3xl font-bold mb-6">Awaitlet as a Migration Tool <a href="#awaitlet-as-a-migration-tool" class="text-cyan-400 text-xl">🔗</a></h2>
    
    <p class="mt-4 text-xl"><a href="https://awaitlet.sqlalchemy.org/en/latest/" class="text-cyan-400" target="_blank">Awaitlet</a> is a library that helps bridge the gap between synchronous and asynchronous code. Its primary purpose is to limit the "async infection" that naturally occurs when migrating to AsyncIO.</p>
    
    <div class="mt-6 bg-indigo-900 p-6 rounded-lg shadow">
        <h3 class="text-2xl font-bold mb-3">What Awaitlet Does</h3>
        <p class="mb-4">Awaitlet allows you to call asynchronous functions from synchronous code without having to make the calling function async. It acts as a containment mechanism for async/await propagation.</p>
        
        <pre class="line-numbers"><code class="language-python">import awaitlet

# A regular synchronous function
def sync_function():
    # Call an async function without making sync_function async
    result = awaitlet.awaitlet(async_function())
    return result

# An async function that can be called from sync code
async def async_function():
    await asyncio.sleep(1)
    return "result"</code></pre>
    </div>

    <h3 class="mt-6 text-2xl font-bold mb-3">Key Benefits for Eventlet Migration</h3>
    
    <div class="mt-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Incremental Migration</h4>
            <p>Migrate your codebase piece by piece without having to refactor everything at once.</p>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Controlled Refactoring</h4>
            <p>Limit the scope of async changes to only the parts that truly benefit from async I/O.</p>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">API Compatibility</h4>
            <p>Maintain backward compatibility with existing synchronous APIs while introducing async functionality.</p>
        </div>
    </div>
    
    <p class="mt-6 text-xl">Awaitlet essentially provides containment for your AsyncIO changes, allowing you to create "async islands" within your otherwise synchronous codebase.</p>
    
    <div class="mt-6 mermaid">
        %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#312e81', 'primaryTextColor': '#fff', 'primaryBorderColor': '#433e99', 'lineColor': '#ffffff', 'secondaryColor': '#312e81', 'tertiaryColor': '#312e81' }}}%%
        graph TD
        A[Synchronous API Endpoint] --> B[Synchronous Service Layer]
        B --> C[Synchronous Business Logic]
        
        subgraph "Async Island"
        D[Data Access Layer<br>with await_]
        E[Async Low-level I/O]
        end
        
        C --> D
        D --> E
        
        style D fill:#059669,stroke:#10b981
        style E fill:#059669,stroke:#10b981
    </div>
</section>

<section>
    <h2 id="implementing-awaitlet" class="mt-10 text-3xl font-bold mb-6">Implementing Awaitlet in Your Migration <a href="#implementing-awaitlet" class="text-cyan-400 text-xl">🔗</a></h2>
    
    <p class="mt-4 text-xl">Let's explore how to implement Awaitlet in your Eventlet migration project with practical examples.</p>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Installation and Setup</h3>
    
    <pre class="line-numbers"><code class="language-bash">pip install awaitlet</code></pre>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Basic Usage Pattern</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
        <pre class="line-numbers"><code class="language-python">import awaitlet
import asyncio

# The async function you want to call
async def fetch_data_async(url):
    # Some async operation that replaces an Eventlet call
    await asyncio.sleep(1)  # Simulate network I/O
    return f"Data from {url}"

# Your existing synchronous function
def process_data(url):
    # Instead of using eventlet.spawn, use awaitlet
    data = awaitlet.awaitlet(fetch_data_async(url))
    return data

# Usage remains unchanged
result = process_data("https://example.com")</code></pre>
    </div>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Migration Example: From Eventlet to AsyncIO with Awaitlet</h3>
    
    <div class="mt-4 grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">Original Eventlet Code</h4>
            <pre class="line-numbers"><code class="language-python">import eventlet
from eventlet.green import urllib2

def fetch_data(url):
    return urllib2.urlopen(url).read()

def process_urls(urls):
    pool = eventlet.GreenPool()
    results = []
    for body in pool.imap(fetch_data, urls):
        results.append(body)
    return results

# Usage
urls = ['http://example.com', 'http://example.org']
results = process_urls(urls)</code></pre>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">Migrated with Awaitlet</h4>
            <pre class="line-numbers"><code class="language-python">import asyncio
import aiohttp
import awaitlet

async def fetch_data_async(url):
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            return await response.read()

def fetch_data(url):
    # Individual calls use awaitlet to bridge sync and async
    return awaitlet.awaitlet(fetch_data_async(url))

def process_urls(urls):
    # Function remains synchronous
    results = []
    for url in urls:
        results.append(fetch_data(url))
    return results

# Usage remains unchanged
urls = ['http://example.com', 'http://example.org']
results = process_urls(urls)</code></pre>
        </div>
    </div>
    
    <p class="mt-6 text-xl">This approach allows you to introduce AsyncIO at the lower levels where I/O operations happen while keeping your API and business logic layers unchanged.</p>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Handling Multiple Async Calls</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
        <p class="mb-4">For concurrent operations (similar to Eventlet's GreenPool), Awaitlet can work with gathered tasks:</p>
        <pre class="line-numbers"><code class="language-python">import awaitlet
import asyncio
import aiohttp

async def fetch_all_async(urls):
    async with aiohttp.ClientSession() as session:
        tasks = []
        for url in urls:
            tasks.append(fetch_one_async(session, url))
        return await asyncio.gather(*tasks)

async def fetch_one_async(session, url):
    async with session.get(url) as response:
        return await response.text()

def process_urls(urls):
    # Single awaitlet call for multiple concurrent async operations
    results = awaitlet.awaitlet(fetch_all_async(urls))
    return results

# Usage remains unchanged
urls = ['http://example.com', 'http://example.org']
results = process_urls(urls)</code></pre>
    </div>
</section>

<section>
    <h2 id="deep-dependencies" class="mt-10 text-3xl font-bold mb-6">Managing Deep Dependencies <a href="#deep-dependencies" class="text-cyan-400 text-xl">🔗</a></h2>
    
    <p class="mt-4 text-xl">For applications with deeply nested dependencies on Eventlet, Awaitlet enables a targeted migration strategy that minimizes refactoring impact.</p>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">The Depth Containment Strategy</h3>
    
    <div class="mt-4 grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">1. Identify I/O Boundaries</h4>
            <p>Focus on identifying the lowest layers in your application where I/O operations occur - these are ideal points to introduce AsyncIO.</p>
            <ul class="list-disc list-inside mt-2">
                <li>Database access layers</li>
                <li>External API clients</li>
                <li>File system operations</li>
                <li>Network communication components</li>
            </ul>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">2. Create Async Wrappers</h4>
            <p>Create async versions of your I/O operations, but keep the original synchronous API intact by using Awaitlet.</p>
            <pre class="line-numbers"><code class="language-python">import awaitlet

async def db_query_async(query):
    # Async implementation
    
def db_query(query):
    # Keep original API signature
    return awaitlet.awaitlet(db_query_async(query))</code></pre>
        </div>
    </div>
    
    <div class="mt-4 grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">3. Introduce Async Incrementally</h4>
            <p>Begin with the most critical I/O bottlenecks and gradually expand your AsyncIO implementation where it provides the most benefit.</p>
            <pre class="line-numbers"><code class="language-python">import awaitlet

# Phase 1: High-impact components 
def critical_operation():
    # Changed to use awaitlet
    return awaitlet.awaitlet(critical_async())

# Phase 2: Secondary components
def less_critical_operation():
    # Still using eventlet for now
    return eventlet.spawn(operation).wait()</code></pre>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">4. Optional: Expose Async APIs</h4>
            <p>For parts of your application that would benefit from an async interface, consider exposing dual APIs:</p>
            <pre class="line-numbers"><code class="language-python">import awaitlet

# Synchronous interface (original)
def get_user(user_id):
    return awaitlet.awaitlet(get_user_async(user_id))

# Async interface (new)
async def get_user_async(user_id):
    # Async implementation</code></pre>
        </div>
    </div>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Example: Refactoring a Deep Call Chain</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
        <pre class="line-numbers"><code class="language-python">import awaitlet
import asyncio

# Original deep call chain with Eventlet dependencies
def api_endpoint(request):
    data = service_layer(request.params)
    return {"result": data}

def service_layer(params):
    return business_logic(params)

def business_logic(params):
    return data_access_layer(params)

def data_access_layer(params):
    # This uses Eventlet for I/O
    import eventlet
    return eventlet.spawn(fetch_from_db, params).wait()

def fetch_from_db(params):
    # Simulated DB query
    eventlet.sleep(0.1)
    return {"id": params.get("id"), "value": "data"}

# Refactored with Awaitlet - only changing the lower layers
def api_endpoint(request):
    # Unchanged
    data = service_layer(request.params)
    return {"result": data}

def service_layer(params):
    # Unchanged
    return business_logic(params)

def business_logic(params):
    # Unchanged
    return data_access_layer(params)

def data_access_layer(params):
    # Changed to use awaitlet instead of eventlet
    return awaitlet.awaitlet(fetch_from_db_async(params))

async def fetch_from_db_async(params):
    # Async implementation
    await asyncio.sleep(0.1)
    return {"id": params.get("id"), "value": "data"}</code></pre>
    </div>
    
    <p class="mt-6 text-xl">Notice how the higher-level functions remain completely unchanged, while only the I/O layer has been modified to use AsyncIO.</p>
</section>

<section>
    <h2 id="internal-apis" class="mt-10 text-3xl font-bold mb-6">Using Awaitlet with Internal Private APIs <a href="#internal-apis" class="text-cyan-400 text-xl">🔗</a></h2>
    
    <p class="mt-4 text-xl">Internal private APIs present a particular challenge during migration because they often serve as integration points between different components of your application. Awaitlet can be particularly valuable in these scenarios.</p>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Extending the Migration Interface Pattern</h3>
    
    <p class="mt-4 text-xl">Building on the <a href="{{ site.baseurl }}{% link guide/preparing.md %}#migrating-shared-private-apis" class="text-cyan-400">migration interface pattern</a> described earlier, Awaitlet enhances this approach:</p>
    
    <div class="mt-6 bg-indigo-900 p-6 rounded-lg shadow">
        <h4 class="text-xl font-bold mb-3">Creating an API Gateway with Awaitlet</h4>
        <pre class="line-numbers"><code class="language-python"># api_gateway.py
import awaitlet

class InternalAPIGateway:
    """Migration interface that abstracts implementation details."""
    
    def fetch_data(self, resource_id):
        # Version 1: During migration, call the async implementation
        # without requiring callers to change
        return awaitlet.awaitlet(self._fetch_data_async(resource_id))
    
    async def _fetch_data_async(self, resource_id):
        # New async implementation that replaces Eventlet
        # This can be gradually enhanced with AsyncIO features
        import asyncio
        await asyncio.sleep(0.1)  # Simulated async I/O
        return {"id": resource_id, "data": "result"}
    
    # More API methods following the same pattern
    def save_data(self, resource_id, data):
        return awaitlet.awaitlet(self._save_data_async(resource_id, data))
        
    async def _save_data_async(self, resource_id, data):
        # Async implementation
        import asyncio
        await asyncio.sleep(0.1)
        return {"status": "saved", "id": resource_id}</code></pre>
    </div>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Practical Migration Strategy for Internal APIs</h3>
    
    <div class="mt-4 grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">1. Create a Dual-Mode API Layer</h4>
            <p>The API gateway provides both sync and async interfaces:</p>
            <pre class="line-numbers"><code class="language-python">import awaitlet

class DatabaseAPI:
    # Sync interface (for backward compatibility)
    def get_user(self, user_id):
        return awaitlet.awaitlet(self.get_user_async(user_id))
    
    # Async interface (for new async consumers)
    async def get_user_async(self, user_id):
        # Async implementation</code></pre>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">2. Incremental Consumer Migration</h4>
            <p>Consumers can gradually migrate to the async API at their own pace:</p>
            <pre class="line-numbers"><code class="language-python"># Legacy consumer (continues to work)
def process_user(user_id):
    user = db_api.get_user(user_id)
    return user

# New async consumer
async def process_user_async(user_id):
    user = await db_api.get_user_async(user_id)
    return user</code></pre>
        </div>
    </div>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Case Example: Migrating an Internal RPC System</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
        <pre class="line-numbers"><code class="language-python">import awaitlet
import asyncio

# Original implementation using Eventlet
class RPCClientOriginal:
    def __init__(self):
        import eventlet
        self.pool = eventlet.GreenPool()
    
    def call_remote(self, method, *args, **kwargs):
        def _execute():
            # Simulate RPC call with eventlet
            import eventlet
            eventlet.sleep(0.1)
            return {'result': f"Response from {method}"}
        
        return self.pool.spawn(_execute).wait()

# Migrated implementation with Awaitlet
class RPCClient:
    def call_remote(self, method, *args, **kwargs):
        # Same interface, but using awaitlet internally
        return awaitlet.awaitlet(self._call_remote_async(method, *args, **kwargs))
    
    async def _call_remote_async(self, method, *args, **kwargs):
        # Async implementation
        await asyncio.sleep(0.1)
        return {'result': f"Response from {method}"}
    
    # Optionally expose async API for new consumers
    async def call_remote_async(self, method, *args, **kwargs):
        return await self._call_remote_async(method, *args, **kwargs)

# Usage remains unchanged for existing code
client = RPCClient()
result = client.call_remote("get_data", id=123)

# New async code can use the async API
async def new_consumer():
    client = RPCClient()
    result = await client.call_remote_async("get_data", id=123)
    return result</code></pre>
    </div>

    <p class="mt-6 text-xl">This approach allows you to maintain API compatibility while migrating the underlying implementation from Eventlet to AsyncIO.</p>
</section>

<section>
    <h2 id="best-practices" class="mt-10 text-3xl font-bold mb-6">Best Practices and Limitations <a href="#best-practices" class="text-cyan-400 text-xl">🔗</a></h2>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Best Practices</h3>
    
    <div class="mt-4 grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Keep awaitlet Close to I/O</h4>
            <p>Place Awaitlet calls as close as possible to the actual I/O operations to minimize performance overhead and make the code easier to understand.</p>
            <pre class="line-numbers"><code class="language-python">import awaitlet

# Good practice
def get_user(user_id):
    # awaitlet used close to the I/O operation
    return awaitlet.awaitlet(db_query_async(f"SELECT * FROM users WHERE id={user_id}"))</code></pre>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Avoid Excessive Nesting</h4>
            <p>Instead of nesting multiple async calls within a single awaitlet call, combine them into a single async function.</p>
            <pre class="line-numbers"><code class="language-python">import awaitlet

# Avoid this
def process():
    result1 = awaitlet.awaitlet(async_operation1())
    result2 = awaitlet.awaitlet(async_operation2(result1))
    return result2

# Better approach
async def process_async():
    result1 = await async_operation1()
    result2 = await async_operation2(result1)
    return result2

def process():
    return awaitlet.awaitlet(process_async())</code></pre>
        </div>
    </div>
    
    <div class="mt-4 grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Consider Thread Safety</h4>
            <p>Awaitlet creates a new event loop in a separate thread for each call. Be mindful of thread safety when accessing shared resources.</p>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Document Transition Points</h4>
            <p>Clearly document where sync-to-async transitions occur in your codebase to aid maintenance and future refactoring.</p>
        </div>
    </div>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Limitations and Considerations</h3>
    
    <div class="mt-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Performance Overhead</h4>
            <p>Awaitlet incurs some overhead due to thread creation and synchronization. It's most efficient for I/O-bound operations where the async benefit outweighs this cost.</p>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Cannot Use in Async Context</h4>
            <p>The <code class="language-python">awaitlet.awaitlet()</code> function cannot be used inside an async function. It's designed as a bridge from sync to async, not the reverse.</p>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Long-Term Strategy</h4>
            <p>While Awaitlet is excellent for migration, consider whether your long-term architecture should eventually move to fully async patterns for critical paths.</p>
        </div>
    </div>
</section>

<section>
    <h2 id="case-study" class="mt-10 text-3xl font-bold mb-6">Case Study: Incremental Migration of a Complex Application <a href="#case-study" class="text-cyan-400 text-xl">🔗</a></h2>
    
    <p class="mt-4 text-xl">Let's walk through a realistic scenario of migrating a complex application with deeply nested Eventlet dependencies using Awaitlet.</p>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Initial Architecture</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
        <p class="mb-4">A large application with multiple services and deep Eventlet usage:</p>
        <ul class="list-disc list-inside">
            <li>API Server with multiple endpoints</li>
            <li>Worker services processing jobs in the background</li>
            <li>Shared internal libraries used by all components</li>
            <li>Extensive use of Eventlet for concurrency throughout the stack</li>
        </ul>
    </div>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Migration Strategy with Awaitlet</h3>
    
    <div class="mt-4 grid grid-cols-1 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">Phase 1: Create Async Versions of Low-Level Libraries</h4>
            <pre class="line-numbers"><code class="language-python">import awaitlet

# Original Eventlet-based database client
class DBClient:
    def query(self, sql):
        import eventlet
        # Eventlet-based implementation
        conn = self.get_connection()
        return eventlet.spawn(conn.execute, sql).wait()

# Migrated version with Awaitlet
class DBClient:
    def query(self, sql):
        # Same interface, but using awaitlet internally
        return awaitlet.awaitlet(self._query_async(sql))
    
    async def _query_async(self, sql):
        # AsyncIO implementation using asyncpg or similar
        import asyncpg
        conn = await self.get_connection_async()
        return await conn.fetch(sql)
    
    # Optionally expose async API
    async def query_async(self, sql):
        return await self._query_async(sql)</code></pre>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">Phase 2: Migrate Shared Services</h4>
            <pre class="line-numbers"><code class="language-python">import awaitlet

# Shared cache service with dual interface
class CacheService:
    def __init__(self):
        # Initialize connection pool or similar
        pass
    
    def get(self, key):
        # Sync interface using Awaitlet
        return awaitlet.awaitlet(self._get_async(key))
    
    async def _get_async(self, key):
        # Async implementation
        import aioredis
        redis = await aioredis.create_redis_pool('redis://localhost')
        value = await redis.get(key)
        redis.close()
        await redis.wait_closed()
        return value
    
    # Similar pattern for set, delete, etc.</code></pre>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">Phase 3: Optionally Expose Async APIs for New Code</h4>
            <pre class="line-numbers"><code class="language-python">import awaitlet

# New async-aware service that uses async APIs directly
class NewAnalyticsService:
    def __init__(self, db_client, cache_service):
        self.db = db_client
        self.cache = cache_service
    
    async def process_data_async(self, data_id):
        # Use async APIs directly for better performance
        cached_result = await self.cache._get_async(f"analytics:{data_id}")
        if cached_result:
            return cached_result
            
        result = await self.db._query_async(f"SELECT * FROM analytics WHERE id={data_id}")
        # Process result asynchronously
        return result</code></pre>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h4 class="text-xl font-bold mb-3">Phase 4: Gradually Move to Pure AsyncIO Where Beneficial</h4>
            <pre class="line-numbers"><code class="language-python"># Example: Converting an API endpoint to fully async
from aiohttp import web

# Async API handler
async def handle_data_request(request):
    data_id = request.match_info.get('id')
    service = request.app['analytics_service']
    
    # Directly use async methods
    result = await service.process_data_async(data_id)
    return web.json_response(result)

# Configure async web app
app = web.Application()
app.router.add_get('/data/{id}', handle_data_request)

# Initialize services
async def init_app():
    db_client = DBClient()
    cache_service = CacheService()
    app['analytics_service'] = NewAnalyticsService(db_client, cache_service)

if __name__ == '__main__':
    web.run_app(app, host='0.0.0.0', port=8080, on_startup=[init_app])</code></pre>
        </div>
    </div>
    
    <h3 class="mt-6 text-2xl font-bold mb-3">Results and Benefits</h3>
    
    <div class="mt-4 grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Managed Migration Depth</h4>
            <p>The use of Awaitlet contained the async changes to specific layers, preventing widespread refactoring.</p>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Incremental Adoption</h4>
            <p>Each component could be migrated independently without breaking existing functionality.</p>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
            <h4 class="text-xl font-bold mb-3">Improved Architecture</h4>
            <p>The migration resulted in a cleaner separation of concerns and better performance for I/O-bound operations.</p>
        </div>
    </div>
</section>

<section>
    <h2 id="conclusion" class="mt-10 text-3xl font-bold mb-6">Conclusion <a href="#conclusion" class="text-cyan-400 text-xl">🔗</a></h2>
    
    <p class="mt-4 text-xl">Managing migration depth is one of the most challenging aspects of moving from Eventlet to AsyncIO, especially in large, complex applications. Awaitlet provides a practical solution by:</p>
    
    <ul class="mt-4 text-xl list-disc list-inside space-y-2">
        <li>Containing the "async infection" to manageable areas of your codebase</li>
        <li>Enabling incremental migration without disrupting existing functionality</li>
        <li>Providing a bridge between synchronous and asynchronous code</li>
        <li>Supporting the migration of applications with internal private APIs</li>
    </ul>
    
    <p class="mt-4 text-xl">By strategically using Awaitlet, you can transform deeply embedded Eventlet dependencies into modern AsyncIO code with minimal disruption to your application architecture.</p>
    
    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h3 class="text-2xl font-bold mb-4">Next Steps</h3>
            <p class="text-xl mb-4">After successfully managing your migration depth with Awaitlet, consider these next steps:</p>
            <ul class="space-y-2">
                <li class="text-xl"><a href="{{ site.baseurl }}{% link guide/asyncio.md %}" class="text-cyan-400 hover:underline">Complete your migration to AsyncIO</a> for high-performance async components</li>
                <li class="text-xl"><a href="{{ site.baseurl }}{% link guide/eventlet.md %}" class="text-cyan-400 hover:underline">Remove remaining Eventlet dependencies</a> with confidence</li>
                <li class="text-xl">In addtion of Awaitlet you can take a look to the <a href="{{ site.baseurl }}{% link guide/asyncio.md %}#sync-async-adapter" class="text-cyan-400 hover:underline">Sync-Async Adapter Pattern</a> with AsyncIO.</li>
            </ul>
        </div>
    </div>
</section>
