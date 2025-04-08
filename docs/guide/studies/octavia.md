---
permalink: /guide/studies/octavia/
title: Case Study - Removal of Eventlet from OpenStack Octavia
layout: dashboard
description: Explore the systematic removal of Eventlet from OpenStack Octavia, including motivations, steps undertaken, and lessons learned.
keywords: eventlet removal, openstack octavia, python concurrency, cotyledon, technical debt
og_type: article
og_title: Case Study - Removal of Eventlet from OpenStack Octavia
og_description: Learn how OpenStack Octavia successfully removed Eventlet, addressing technical debt and improving performance with Cotyledon.
---

<section>
    <h1 class="text-4xl font-bold">Case Study: Removal of Eventlet from OpenStack Octavia</h1>
    <p class="mt-10 text-xl">The OpenStack Octavia project embarked on a systematic removal of the Eventlet library to address technical debt, improve performance, and ensure compatibility with Python 3 by adopting more suitable libraries such as Cotyledon. This case study outlines the timeline, key contributors, and critical steps undertaken during the removal process.</p>

    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#motivation" class="text-cyan-400 hover:underline">Motivation for Removing Eventlet</a></li>
                <li><a href="#timeline" class="text-cyan-400 hover:underline">Timeline and Contributors</a></li>
                <li><a href="#steps" class="text-cyan-400 hover:underline">Steps Undertaken</a></li>
                <li><a href="#outcomes" class="text-cyan-400 hover:underline">Outcomes and Benefits</a></li>
                <li><a href="#lessons" class="text-cyan-400 hover:underline">Lessons Learned</a></li>
                <li><a href="#recommendations" class="text-cyan-400 hover:underline">Recommendations for Other Projects</a></li>
                <li><a href="#conclusion" class="text-cyan-400 hover:underline">Conclusion</a></li>
                <li><a href="#call-to-action" class="text-cyan-400 hover:underline">Call to Action</a></li>
            </ul>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="motivation" class="text-3xl font-bold mb-6">Motivation for Removing Eventlet <a href="#motivation" class="text-cyan-400 text-xl">🔗</a></h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Complex Debugging</h3>
                <p class="text-xl">Difficulties arising from monkey-patching made debugging complex and error-prone.</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Python Version Compatibility</h3>
                <p class="text-xl">Eventlet posed compatibility challenges, particularly with the evolution toward Python 3.</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Maintenance Overhead</h3>
                <p class="text-xl">The slow evolution and reduced community support for Eventlet created long-term sustainability concerns.</p>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="timeline" class="text-3xl font-bold mb-6">Timeline and Contributors <a href="#timeline" class="text-cyan-400 text-xl">🔗</a></h2>
        <p class="text-xl mb-6">The following timeline illustrates the key milestones and contributions during the migration process:</p>
        <div class="mt-10 mermaid">
            %%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#312e81', 'primaryTextColor': '#fff', 'primaryBorderColor': '#433e99', 'lineColor': '#ffffff', 'secondaryColor': '#312e81', 'tertiaryColor': '#312e81' }}}%%
            timeline
                title Eventlet Removal Timeline
                April 8, 2017: Adam Harwell initiated the removal by eliminating direct Eventlet dependencies.
                May 3, 2017: Michael Johnson introduced enhanced hacking checks.
                March 22, 2018: Doug Hellmann implemented a lower-constraints testing framework.
                January 10, 2019: Michael Johnson further enhanced hacking checks and test coverage.
                March 29, 2020: Andreas Jaeger upgraded coding standards to Python 3 compatibility.
                July 6, 2022: Gregory Thiemonge finalized dependency management adjustments and concluded Eventlet support.
        </div>
        <p class="text-xl mt-6">
            This timeline highlights the incremental steps taken by contributors to remove Eventlet, ensuring a smooth and systematic migration process.
        </p>
    </div>
</section>


<div class="mt-10 bg-gray-800 bg-opacity-70 p-6 rounded-lg">
    <h2 class="text-2xl font-bold mb-4">Additional Details</h2>
    <p class="text-xl">
        For more information about the migration process, you can explore the relevant Git history using the following command:
    </p>
    <pre class="bg-gray-900 text-gray-100 p-4 rounded-lg mt-4 text-xl">
git log --since="2016-01-01" --until="2025-03-10" -S "eventlet" -p
    </pre>
    <p class="text-xl mt-4">
        This command filters commits between January 1, 2016, and March 10, 2025, that reference "eventlet" and displays the associated changes.
    </p>
</div>

<section>
    <div class="mt-10">
        <h2 id="steps" class="text-3xl font-bold mb-6">Steps Undertaken <a href="#steps" class="text-cyan-400 text-xl">🔗</a></h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <!-- Initial Dependency Removal -->
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">1. Initial Dependency Removal</h3>
                <p class="text-xl">Identified and removed direct dependencies on Eventlet and related Oslo libraries.</p>
                <p class="mt-4"><strong>Before:</strong></p>
                <pre class="line-numbers"><code class="language-python">import eventlet
eventlet.monkey_patch()</code></pre>
                <p class="mt-4"><strong>After:</strong></p>
                <pre class="line-numbers"><code class="language-python"># Removed all references to eventlet</code></pre>
            </div>
            <!-- Adoption of Cotyledon -->
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">2. Adoption of Cotyledon</h3>
                <p class="text-xl">Transitioned to Cotyledon for improved asynchronous performance and maintainability.</p>
                <p class="mt-4"><strong>Eventlet:</strong></p>
                <pre class="line-numbers"><code class="language-python">from oslo_service import service
launcher = service.launch(CONF, server)</code></pre>
                <p class="mt-4"><strong>Cotyledon:</strong></p>
                <pre class="line-numbers"><code class="language-python">import cotyledon
manager = cotyledon.ServiceManager()
manager.run()</code></pre>
            </div>
            <!-- Significant Refactoring -->
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">3. Significant Refactoring</h3>
                <p class="text-xl">Removed Eventlet-specific constructs and ensured compatibility with alternative frameworks.</p>
                <p class="mt-4"><strong>Eventlet:</strong></p>
                <pre class="line-numbers"><code class="language-python">eventlet.sleep(1)</code></pre>
                <p class="mt-4"><strong>Standard Library:</strong></p>
                <pre class="line-numbers"><code class="language-python">import time
time.sleep(1)</code></pre>
            </div>
            <!-- Code Quality and Testing Enhancements -->
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">4. Code Quality and Testing Enhancements</h3>
                <p class="text-xl">Introduced robust hacking checks and improved test coverage.</p>
                <p class="mt-4"><strong>Example Hacking Check:</strong></p>
                <pre class="line-numbers"><code class="language-python">def check_no_eventlet_imports(logical_line):
    if 'eventlet' in logical_line:
        yield logical_line.index('eventlet'),
            "O345: Usage of eventlet is prohibited."</code></pre>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="outcomes" class="text-3xl font-bold mb-6">Outcomes and Benefits <a href="#outcomes" class="text-cyan-400 text-xl">🔗</a></h2>
        <ul class="list-disc pl-6 space-y-2 text-xl">
            <li><strong>Simplified and Maintainable Codebase:</strong> Extensive refactoring and adoption of Cotyledon streamlined the codebase.</li>
            <li><strong>Performance and Stability Improvements:</strong> Transitioning to Cotyledon improved performance and service management.</li>
            <li><strong>Enhanced Compatibility:</strong> Established ongoing compatibility with modern Python environments.</li>
            <li><strong>Security Enhancement:</strong> Proactive dependency management mitigated potential vulnerabilities.</li>
        </ul>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="lessons" class="text-3xl font-bold mb-6">Lessons Learned <a href="#lessons" class="text-cyan-400 text-xl">🔗</a></h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Structured Approach is Key</h3>
                <p class="text-xl">Incremental removal combined with robust testing strategies is essential to safely refactor complex systems.</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Community Alignment</h3>
                <p class="text-xl">Regular alignment with community standards and leveraging collective knowledge significantly reduced refactoring risks.</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Clear Goals and Early Wins</h3>
                <p class="text-xl">Clearly defined goals and achieving early incremental successes motivated continued progress and stakeholder buy-in.</p>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="recommendations" class="text-3xl font-bold mb-6">Recommendations for Other Projects <a href="#recommendations" class="text-cyan-400 text-xl">🔗</a></h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Evaluate Dependency Risks Early</h3>
                <p class="text-xl">Regularly re-evaluate library dependencies to avoid accruing significant technical debt.</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Adopt Robust Testing Practices</h3>
                <p class="text-xl">Enhance automated testing and static checks early in the refactoring cycle to ensure stability.</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-105 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Choose Sustainable Alternatives</h3>
                <p class="text-xl">Select technologies like Cotyledon for long-term maintainability and compatibility with modern standards.</p>
            </div>
        </div>
    </div>
</section>

<section>
    <h2 id="conclusion" class="mt-10 text-3xl font-bold">Conclusion <a href="#conclusion" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">The successful removal of Eventlet from OpenStack Octavia highlights the value of systematic refactoring, proactive dependency management, and careful technology selection. Octavia's transition to Cotyledon illustrates effective software engineering practices that benefit maintainability, performance, and future adaptability.</p>
</section>

<section id="call-to-action" class="mt-10 bg-gray-800 bg-opacity-70 p-6 rounded-lg">
    <h2 class="text-2xl font-bold mb-4">Call to Action <a href="#call-to-action" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="text-xl">Considering removing Eventlet from your project?</p>
    <ul class="mt-4 text-xl list-disc list-inside">
        <li>Review <a href="https://opendev.org/openstack/octavia" class="text-cyan-400 hover:underline" target="_blank">Octavia’s repository</a> for detailed commits and best practices.</li>
        <li>Engage with the community and share experiences in the dedicated OFTC IRC channel <strong>#openstack-eventlet-removal</strong>.</li>
        <li>Share <a href="https://removal.eventlet.org" class="text-cyan-400 hover:underline" target="_blank">removal.eventlet.org</a> with your colleagues to give them additional guidance, tools, and support from the broader community.</li>
    </ul>
</section>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/studies.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>Back to Studies
    </a>
</div>

