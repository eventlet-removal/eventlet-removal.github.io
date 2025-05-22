---
layout: dashboard
title: Post-Migration Guidance
permalink: /guide/post-migration-guidance/
description: Essential steps to take after completing your migration away from Eventlet to ensure it doesn't return to your codebase, including code checks, dependency monitoring, and team education.
keywords: post-migration, eventlet removal, dependency monitoring, code checks, hacking checks, migration maintenance
og_type: article
og_title: Post-Migration Guidance for Eventlet Removal Projects
og_description: Learn how to prevent Eventlet from returning to your codebase with code checks, dependency monitoring, and team education after successful migration.
---
<section>
    <h1 class="text-4xl font-bold">Post-Migration Guidance</h1>
    <p class="mt-10 text-xl">Now that your project is free from Eventlet, the journey isn't over. This guide provides actionable recommendations to ensure Eventlet doesn't sneak back in and compromise the stability, performance, or compatibility of your codebase.</p>

    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#hacking-checks" class="text-cyan-400 hover:underline">Add Custom Hacking Checks</a></li>
                <li><a href="#monitor-dependencies" class="text-cyan-400 hover:underline">Monitor Your Dependencies</a></li>
                <li><a href="#review-practices" class="text-cyan-400 hover:underline">Review Practices</a></li>
                <li><a href="#education" class="text-cyan-400 hover:underline">Educate Your Team</a></li>
                <li><a href="#community" class="text-cyan-400 hover:underline">Join the Community</a></li>
            </ul>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="hacking-checks" class="text-3xl font-bold mb-6">Add Custom Hacking Checks <a href="#hacking-checks" class="text-cyan-400 text-xl">🔗</a></h2>
        <p class="mt-10 text-xl">To prevent Eventlet from being reintroduced accidentally, we recommend adding specific checks using the <a href="https://docs.openstack.org/hacking/latest/" class="text-cyan-400" target="_blank">OpenStack Hacking</a> framework or similar linting tools.</p>

        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-6">
            <h3 class="text-2xl font-bold mb-3">Example: H999 – Ban Eventlet Import</h3>
            <pre class="line-numbers mt-4"><code class="language-python"># In your custom hacking plugin:
import re

EVENTLET_IMPORT_RE = re.compile(r'^\s*(import|from)\s+eventlet')

def check_no_eventlet_import(logical_line):
    if EVENTLET_IMPORT_RE.search(logical_line):
        return (0, "H999: eventlet must not be imported.")</code></pre>

            <p class="mt-4 text-xl">To enable this check, register it in your setup.cfg:</p>
            <pre class="line-numbers mt-4"><code class="language-ini">[flake8]
enable-extensions = H999</code></pre>
            <p class="mt-4 text-xl">This ensures that any import of eventlet is blocked at code review time.</p>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="monitor-dependencies" class="text-3xl font-bold mb-6">Monitor Your Dependencies <a href="#monitor-dependencies" class="text-cyan-400 text-xl">🔗</a></h2>
        <p class="mt-10 text-xl">Make sure none of your dependencies reintroduce Eventlet indirectly:</p>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-10 mt-10">
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Dependency Auditing</h3>
                <p class="text-xl">Use tools like <a href="https://pypi.org/project/pipdeptree/" class="text-cyan-400 hover:underline" target="_blank">pipdeptree</a> or <code>pip freeze</code> to regularly audit your dependencies and their dependencies for eventlet usage.</p>
                <pre class="line-numbers mt-4"><code class="language-bash">pipdeptree | grep -i eventlet</code></pre>
            </div>
            
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Constraints Files</h3>
                <p class="text-xl">Use constraints files or negative requirements to enforce Eventlet's absence:</p>
                <pre class="line-numbers mt-4"><code class="language-text"># In constraints.txt
eventlet==none

# Or in requirements.txt
exclude eventlet</code></pre>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="review-practices" class="text-3xl font-bold mb-6">Review Practices <a href="#review-practices" class="text-cyan-400 text-xl">🔗</a></h2>
        <p class="mt-10 text-xl">Update your code review checklist to include Eventlet-specific checks:</p>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-6">
            <h3 class="text-2xl font-bold mb-3">Code Review Checklist</h3>
            <ul class="list-disc pl-6 space-y-2 text-xl">
                <li>No direct or transitive Eventlet usage</li>
                <li>Green light from custom hacking checks</li>
                <li>Async compatibility maintained</li>
                <li>No monkey-patching in the codebase</li>
                <li>Proper error handling for async operations</li>
            </ul>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="education" class="text-3xl font-bold mb-6">Educate Your Team <a href="#education" class="text-cyan-400 text-xl">🔗</a></h2>
        <p class="mt-10 text-xl">Add a section in your team's onboarding and review documentation explaining:</p>
        
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-10">
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Why Eventlet Was Removed</h3>
                <p class="text-xl">Document the technical debt, performance issues, and compatibility problems that led to the removal decision.</p>
            </div>
            
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Risks of Reintroduction</h3>
                <p class="text-xl">Explain the potential consequences of reintroducing Eventlet: incompatibility issues, maintenance burden, and performance problems.</p>
            </div>
            
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Preferred Alternatives</h3>
                <p class="text-xl">Clearly document which alternatives (AsyncIO, Threading) are now being used and provide guidance on how to use them correctly.</p>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="community" class="text-3xl font-bold mb-6">Join the Community <a href="#community" class="text-cyan-400 text-xl">🔗</a></h2>
        <p class="mt-10 text-xl">Stay connected with the broader community working on Eventlet removal:</p>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-6">
            <h3 class="text-2xl font-bold mb-3">Community Resources</h3>
            <ul class="list-disc pl-6 space-y-2 text-xl">
                <li>Subscribe to the <strong>#openstack-eventlet-removal</strong> OFTC IRC channel</li>
                <li>Join the <a href="https://www.linkedin.com/groups/13183090/" class="text-cyan-400 hover:underline" target="_blank">Eventlet Removal LinkedIn group</a> to stay informed of the latest updates</li>
                <li>Share your experiences and tools back through this community-driven resource</li>
            </ul>
        </div>
        
        <p class="mt-10 text-xl">This is not just a cleanup – it's a commitment. A commitment to long-term maintainability, performance, and clarity in your codebase.</p>
    </div>
</section>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/depth.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>Managing Migration Depth
    </a>
    <a href="{{ site.baseurl }}{% link guide/debug-asyncio.md %}" class="inline-block bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        Debugging AsyncIO<i class="fas fa-arrow-right ml-2"></i>
    </a>
</div>