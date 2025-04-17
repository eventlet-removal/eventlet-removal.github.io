---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: dashboard
title: OpenStack 2025.1 Flamingo PTG - Eventlet Removal Discussions
permalink: /guide/openstack/flamingo-ptg/
description: Analysis of the discussions about removing Eventlet from OpenStack projects during the April 2025 Project Teams Gathering (PTG), including migration strategies and project-specific plans.
keywords: openstack, eventlet removal, flamingo ptg, migration strategies, threading, asyncio, python 3.13, project teams gathering
og_type: article
og_title: OpenStack Flamingo PTG Eventlet Removal Analysis
og_description: Analysis of OpenStack's community-wide effort to migrate away from Eventlet, based on April 2025 Project Teams Gathering discussions.
---
<section>
    <h1 class="text-4xl font-bold">Report on Eventlet Discussions at the OpenStack Flamingo PTG</h1>
    <p class="mt-4 text-lg"><em>Report created by <a href="https://herve.beraud.io" class="text-cyan-400 hover:underline">Hervé Beraud</a> at April 17, 2025</em></p>
    <p class="mt-10 text-xl">This report presents a full analysis of all the transversal discussions regarding the migration away from Eventlet in OpenStack, as documented in the <a href="https://ptg.opendev.org/etherpads.html" class="text-cyan-400" target="_blank">etherpads from the April 2025 Project Teams Gathering (PTG)</a>.</p>

    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#context-importance" class="text-cyan-400 hover:underline">Context and Importance</a></li>
                <li><a href="#specific-issues" class="text-cyan-400 hover:underline">Specific Issues Identified</a></li>
                <li><a href="#migration-strategy" class="text-cyan-400 hover:underline">Recommended Migration Strategy</a></li>
                <li><a href="#challenges" class="text-cyan-400 hover:underline">Challenges and Open Questions</a></li>
                <li><a href="#project-status" class="text-cyan-400 hover:underline">Current Migration Status by Project</a></li>
                <li><a href="#nova-plan" class="text-cyan-400 hover:underline">Nova's Specific Migration Plan</a></li>
                <li><a href="#resources" class="text-cyan-400 hover:underline">Resources</a></li>
                <li><a href="#conclusion" class="text-cyan-400 hover:underline">Conclusion</a></li>
            </ul>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="context-importance" class="text-3xl font-bold mb-6">Context and Importance <a href="#context-importance" class="text-cyan-400 text-xl">🔗</a></h2>
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
            <p class="mb-4 text-xl">OpenStack has extensively used Eventlet as a concurrency solution for its services. However, this dependency has become problematic for several reasons:</p>
            <ol class="list-decimal pl-6 space-y-2 text-xl">
                <li><strong>Incompatibilities with future Python versions</strong>: Eventlet has significant issues with Python 3.13 and PEP 703 ("GILectomy").</li>
                <li><strong>Declining maintenance</strong>: Eventlet's maintenance activity is decreasing, making it difficult to resolve numerous bugs.</li>
                <li><strong>Integration problems</strong>: Several incompatibilities have been identified between Eventlet and various libraries such as RabbitMQ.</li>
            </ol>
            <p class="mt-4 text-xl">The OpenStack community has launched an official goal to remove Eventlet due to growing issues with this library, particularly in light of future Python evolutions.</p>
        </div>
        
        <h3 class="text-2xl font-bold mt-6 mb-4">Timeline and Coordination</h3>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-4">
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h4 class="font-bold text-xl mb-2">Initial Schedule</h4>
                <p>Planned for the 2025.1/2025.2 cycles</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h4 class="font-bold text-xl mb-2">New Timeline</h4>
                <p>The new timeline is available <a href="{{ site.baseurl }}{% link guide/sequencing.md %}#openstack-migration" class="text-cyan-400" target="_blank">here</a></p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h4 class="font-bold text-xl mb-2">Coordination</h4>
                <p>OFTC channel #openstack-eventlet-removal established to facilitate collaboration</p>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="specific-issues" class="text-3xl font-bold mb-6">Specific Issues Identified <a href="#specific-issues" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4 mb-6">
            <h3 class="text-2xl font-bold mb-3">RabbitMQ Heartbeat</h3>
            <p class="mb-2 text-xl">A problem mentioned concerns heartbeat failures with RabbitMQ:</p>
            <ul class="list-disc pl-6 space-y-2 text-xl">
                <li>Timeouts and API failures occur because RabbitMQ does not support Eventlet's "green" environments well</li>
                <li>This is a well know problem</li>
                <li>A partial solution using heartbeats in pthread exists but has problems with logging</li>
                <li>A patch for oslo.log is pending: <a href="https://review.opendev.org/c/openstack/oslo.log/+/937729" class="text-cyan-400" target="_blank">review.opendev.org/937729</a></li>
            </ul>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h3 class="text-2xl font-bold mb-3">WSGI Server Performance</h3>
            <p class="mb-4 text-xl">Comparative tests mentioned in Swift's etherpad show:</p>
            <div class="overflow-x-auto">
                <table class="min-w-full bg-gray-800 rounded-lg">
                    <thead>
                        <tr>
                            <th class="py-2 px-4 border-b border-gray-700 text-left">Server</th>
                            <th class="py-2 px-4 border-b border-gray-700 text-left">1 worker</th>
                            <th class="py-2 px-4 border-b border-gray-700 text-left">4 workers</th>
                            <th class="py-2 px-4 border-b border-gray-700 text-left">8 workers</th>
                            <th class="py-2 px-4 border-b border-gray-700 text-left">16 workers</th>
                            <th class="py-2 px-4 border-b border-gray-700 text-left">Notes</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td class="py-2 px-4 border-b border-gray-700">Eventlet WSGI</td>
                            <td class="py-2 px-4 border-b border-gray-700">8,135</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                            <td class="py-2 px-4 border-b border-gray-700">Only uses 1 process/core</td>
                        </tr>
                        <tr>
                            <td class="py-2 px-4 border-b border-gray-700">Gunicorn WSGI</td>
                            <td class="py-2 px-4 border-b border-gray-700">5,465</td>
                            <td class="py-2 px-4 border-b border-gray-700">19,868</td>
                            <td class="py-2 px-4 border-b border-gray-700">25,146</td>
                            <td class="py-2 px-4 border-b border-gray-700">38,474</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                        </tr>
                        <tr>
                            <td class="py-2 px-4 border-b border-gray-700">FastWsgi WSGI</td>
                            <td class="py-2 px-4 border-b border-gray-700">83,766</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                            <td class="py-2 px-4 border-b border-gray-700">Only uses 1 thread/core</td>
                        </tr>
                        <tr>
                            <td class="py-2 px-4 border-b border-gray-700">Uvicorn ASGI</td>
                            <td class="py-2 px-4 border-b border-gray-700">4,169</td>
                            <td class="py-2 px-4 border-b border-gray-700">2,048</td>
                            <td class="py-2 px-4 border-b border-gray-700">2,161</td>
                            <td class="py-2 px-4 border-b border-gray-700">2,156</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                        </tr>
                        <tr>
                            <td class="py-2 px-4 border-b border-gray-700">Bjoern WSGI</td>
                            <td class="py-2 px-4 border-b border-gray-700">2,028</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                            <td class="py-2 px-4 border-b border-gray-700">-</td>
                            <td class="py-2 px-4 border-b border-gray-700">Only uses 1 thread/core</td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <p class="mt-4 text-xl mb-4">These figures (requests/second) show significant performance differences between servers:</p>
<ul class="list-disc pl-6 space-y-2 text-xl">
    <li><strong>FastWsgi</strong>: Appears significantly superior with <code>83,766 req/s</code> on a single thread/core.</li>
    <li><strong>Gunicorn</strong>: Demonstrates impressive scalability with increasing worker counts (up to <code>38,474 req/s</code> with 16 workers).</li>
    <li><strong>Eventlet</strong>: Offers good performance for a single process (<code>8,135 req/s</code>) but is limited to a single process/core.</li>
    <li><strong>Uvicorn (ASGI)</strong> and <strong>Bjoern</strong>: Show more modest performance.</li>
</ul>    </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-6">
            <h3 class="text-2xl font-bold mb-3">Technical Committee (TC) Perspective</h3>
            <ul class="list-disc pl-6 space-y-2 text-xl">
                <li>Eventlet removal is identified as one of the top priorities for the community</li>
                <li>Sees it as more important than ever due to upcoming Python changes</li>
                <li>Recognizes that it requires a shift in thinking about project ownership, focusing more on accomplishing goals collectively</li>
                <li class="text-xl">Suggests potential rootwrap deprecation during the Eventlet removal process</li>
                <li class="text-xl">Planning to collect information about the current status of migration across projects</li>
            </ul>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-6">
            <h3 class="text-2xl font-bold mb-3">Oslo.service New Backend</h3>
            <p class="text-xl mb-4">A key development is the new Threading backend for oslo.service that no longer depends on Eventlet:</p>
            <ul class="list-disc pl-6 space-y-2 text-xl">
                <li class="text-xl">Change in progress: <a href="https://review.opendev.org/c/openstack/oslo.service/+/945720" class="text-cyan-400" target="_blank">review.opendev.org/945720</a></li>
                <li class="text-xl">No longer provides WSGI support</li>
                <li class="text-xl">Each service will need to deprecate implementations that depend on Eventlet's WSGI server</li>
                <li class="text-xl">Documentation patch: <a href="https://review.opendev.org/c/openstack/oslo.service/+/940664" class="text-cyan-400" target="_blank">review.opendev.org/940664</a></li>
            </ul>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="migration-strategy" class="text-3xl font-bold mb-6">Recommended Migration Strategy <a href="#migration-strategy" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-3">Prioritization</h3>
                <ul class="list-disc pl-6 space-y-2 text-xl">
                    <li class="text-xl">Start with services rather than libraries</li>
                    <li class="text-xl">Focus first on eliminating the Eventlet WSGI server</li>
                    <li class="text-xl">Then migrate other concurrency features (greenpools, threadpools)</li>
                </ul>
            </div>
            
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-3">Practical Steps</h3>
                <ul class="list-disc pl-6 space-y-2 text-xl">
                    <li class="text-xl">Identify Eventlet use cases specific to each project</li>
                    <li class="text-xl"><a href="{{ site.baseurl }}{% link guide/preparing.md %}" class="text-cyan-400" target="_blank">Prepare migration by following the guidelines</a></li>
                    <li class="text-xl">Gradually deprecate Eventlet-related features</li>
                    <li class="text-xl">Develop alternatives for shared and private APIs</li>
                </ul>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="challenges" class="text-3xl font-bold mb-6">Challenges and Open Questions <a href="#challenges" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
            <h3 class="text-2xl font-bold mb-3">Dual Compatibility During Transition</h3>
            <p class="mb-4 text-xl">An important debate concerns the possibility of temporarily maintaining services that work both with and without Eventlet:</p>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4 mb-6">
                <div class="bg-blue-800 p-4 rounded-lg">
                    <h4 class="font-bold mb-2">Arguments for</h4>
                    <ul class="list-disc pl-6 space-y-1 text-xl">
                        <li>Would allow falling back to Eventlet if bugs are discovered in the native thread mode</li>
                        <li>Would facilitate a gradual and safer migration</li>
                    </ul>
                </div>
                
                <div class="bg-red-900 p-4 rounded-lg">
                    <h4 class="font-bold mb-2">Arguments against</h4>
                    <ul class="list-disc pl-6 space-y-1 text-xl">
                        <li>Considerable complexity in maintaining two different concurrency modes</li>
                        <li>Eventlet itself already contains numerous bugs, making a fallback strategy difficult to implement</li>
                        <li>Difficulties related to side effects of Eventlet's monkey patching</li>
                    </ul>
                </div>
            </div>
            
            <h4 class="font-bold text-xl mt-6 mb-2">Project-specific decisions</h4>
            <ul class="list-disc pl-6 space-y-2 text-xl">
                <li><strong>Nova</strong>: After discussions, the project has decided to support both modes (eventlet and native threads) during the transition to avoid a "big bang" migration. A variable environment mechanism (<code>OS_NOVA_DISABLE_EVENTLET_PATCHING=1</code>) is planned to control the concurrency mode.</li>
                <li><strong>Glance</strong>: Has been usable with both eventlet and native threads for several years, demonstrating the feasibility of dual mode support.</li>
            </ul>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-6">
            <h3 class="text-2xl font-bold mb-3">Partial vs Complete Migration</h3>
            <p class="text-xl mb-4">Experiences vary across projects:</p>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Neutron</strong>: Encountered challenges with a gradual approach</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Glance</strong>: Successfully maintained compatibility with both modes for several years</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Swift</strong>: Considering a "canary" node approach starting with proxies</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Nova</strong>: Plans to go service by service, with some services running in native thread mode while others still use eventlet in the same release</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Octavia</strong>: Has demonstrated a successful complete migration, now serving as a case study</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Mistral</strong>: Has chosen a comprehensive approach, nearly completing its migration with minimal incremental steps</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Manila</strong>: Adopting a phased approach, planning for completion over multiple cycles</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Cinder</strong>: Taking a component-by-component approach, starting with Volume Manager</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Heat</strong>: Planning a complete discontinuation of WSGI server implementations</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Ironic</strong>: Facing complex migration challenges, particularly with IPA</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Designate</strong>: Focusing on a complete migration as their top priority for the Flamingo cycle</p>
                </div>
                <div class="bg-blue-800 p-4 rounded-lg">
                    <p class="text-xl"><strong>Blazar</strong>: Considering a gradual transition, evaluating different WSGI alternatives</p>
                </div>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="project-status" class="text-3xl font-bold mb-6">Current Migration Status by Project <a href="#project-status" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <h3 class="text-2xl font-bold mb-4">Projects with Significant Progress</h3>
        
         <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
            <div class="bg-green-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Octavia</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Has successfully migrated away from Eventlet since 2017</li>
                    <li>Their approach is now documented as a case study to help other projects</li>
                    <li>Case study available at: <a href="{{ site.baseurl }}{% link guide/studies/octavia.md %}" class="text-cyan-400">removal.eventlet.org/guide/case-studies/octavia/</a></li>
                    <li>Uses cotyledon</li>
                </ul>
            </div>
            
            <div class="bg-green-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Neutron</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Has made significant progress in removing its dependency on Eventlet through numerous changes</li>
                    <li>Successfully implemented an approach where code can work with both Eventlet and native threads during transition</li>
                    <li>Many patches found under topic "eventlet-removal": <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/neutron" class="text-cyan-400" target="_blank">Neutron patches</a></li>
                </ul>
            </div>
            
            <div class="bg-green-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Mistral</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Has almost completed its migration from Eventlet</li>
                    <li>Many of the changes are found under: <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/mistral" class="text-cyan-400" target="_blank">Mistral patches</a></li>
                </ul>
            </div>
            
            <div class="bg-green-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Glance</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Can be deployed without Eventlet</li>
                    <li>Some optional features (scrubber) still depend on it</li>
                    <li>Tests still depend on Eventlet</li>
                    <li>Has been usable with both Eventlet and native threads for several years</li>
                </ul>
            </div>

            <div class="bg-green-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Oslo Libraries</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Have deprecated all their Eventlet compatibility features</li>
                    <li>Working on a new Threading backend for oslo.service that doesn't depend on Eventlet</li>
                    <li>Added asyncio support in oslo.db, enabling asynchronous database operations</li>
                    <li>Significant progress in making core libraries Eventlet-free</li>
                </ul>
            </div>
        </div>
        
        <h3 class="text-2xl font-bold mt-8 mb-4">Projects with Plans in Progress</h3>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Nova</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Has decided to support both modes during the transition</li>
                    <li>Planning to use an environment variable mechanism (<code>OS_NOVA_DISABLE_EVENTLET_PATCHING=1</code>)</li>
                    <li>Plans to go service-by-service with some services running in native thread mode while others still use Eventlet</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Swift</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Has started discussions about alternative WSGI servers</li>
                    <li>Considering options including Gunicorn, Uvicorn, FastWsgi, or Bjoern</li>
                    <li>Planning a Proof of Concept (POC) using the proxy server as a starter</li>
                    <li>Considering a "canary node" approach to migration, starting with proxies</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Manila</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Removed monkey patching in the client during the Epoxy cycle</li>
                    <li>Planning to remove WSGI uses and adopt oslo.service's new Threading-based backend</li>
                    <li>Working on this in the Flamingo cycle, aiming for completion in the G cycle (Guppy)</li>
                    <li>Looking at Neutron's progress with periodic tasks for inspiration</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Cinder</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Has started working on removing Eventlet dependencies during the Flamingo cycle</li>
                    <li>Taking a gradual approach with multiple team members involved</li>
                    <li>Planning to start with the Volume Manager, which is a key component</li>
                    <li>Will apply lessons learned from Volume Manager work to the Backup Manager later</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Heat</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Has identified Eventlet removal as one of their upcoming changes</li>
                    <li>Planning to use the new oslo.service implementation without Eventlet once available</li>
                    <li>Will not provide a WSGI server implementation with thread model</li>
                    <li>Preferring external server mechanisms such as uwsgi or httpd+mod_wsgi</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Designate</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Has identified Eventlet removal as their top priority for the Flamingo cycle</li>
                    <li>Started tracking relevant changes in their Gerrit repository</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Blazar</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Acknowledges the need to move away from Eventlet's WSGI implementation</li>
                    <li>Planning to reuse examples from other projects as they migrate away from Eventlet</li>
                    <li>Considering alternatives including mod_wsgi and uwsgi</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Watcher</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Needs to examine how Eventlet is used in the project</li>
                    <li>Start proof of concept work for removal</li>
                    <li>Acknowledges timing pressure due to Ubuntu 2025.4 shipping with Python 3.13 by default</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Ironic</h4>
                <ul class="list-disc pl-6 space-y-1 text-xl">
                    <li>Has identified Eventlet removal as one of their priorities for the Flamingo cycle</li>
                    <li>Facing several challenges with different components such as IPA</li>
                    <li>Recognizes that one person can't own the entire migration</li>
                    <li>Team members are volunteering for specific components</li>
                </ul>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
         <h2 id="nova-plan" class="text-3xl font-bold mb-6">Nova's Specific Migration Plan <a href="#nova-plan" class="text-cyan-400 text-xl">🔗</a></h2>
                
        <p class="text-xl mb-4">Nova has dedicated significant discussion to their Eventlet removal approach during the Flamingo PTG:</p>
                
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4 mb-6">
            <h3 class="text-2xl font-bold mb-3">Flamingo Cycle Tasks (2025.2)</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                    <h4 class="font-bold text-xl mb-2">API Modernization</h4>
                    <ul class="list-disc pl-6 space-y-1 text-xl">
                        <li>Deprecate the oslo.service.wsgi based standalone eventlet server mode for nova-api</li>
                        <li>Remove direct eventlet imports where possible</li>
                        <li>Replace eventlet primitives with stdlib primitives where possible</li>
                    </ul>
                </div>
                
                <div>
                    <h4 class="font-bold text-xl mb-2">Architecture Changes</h4>
                    <ul class="list-disc pl-6 space-y-1 text-xl">
                        <li>Create an entry point wrapper to configure concurrency mode via ENV variable</li>
                        <li>Move Nova commands that don't use Eventlet to a separate module without monkey_patch</li>
                        <li>Check novncproxy Eventlet usage and non-monkey-patched performance</li>
                    </ul>
                </div>
                
                <div>
                    <h4 class="font-bold text-xl mb-2">Performance Improvements</h4>
                    <ul class="list-disc pl-6 space-y-1 text-xl">
                        <li>Remove Eventlet from nova-api scatter/gather logic</li>
                        <li>Add metric gathering about threadpool state when load is high</li>
                        <li>Add SQL statement timeout for nova-api</li>
                        <li>Enable connection pooling (qpool) for timeout implementation</li>
                    </ul>
                </div>
                
                <div>
                    <h4 class="font-bold text-xl mb-2">Testing Strategy</h4>
                    <ul class="list-disc pl-6 space-y-1 text-xl">
                        <li>Not maintaining functional tests to work in both threading modes</li>
                        <li>Moving tests to native threading mode to uncover issues early</li>
                        <li>Using Tempest jobs to test both threading modes</li>
                    </ul>
                </div>
            </div>
        </div>
                
        <div class="bg-indigo-900 p-6 rounded-lg shadow mb-6">
            <h3 class="text-2xl font-bold mb-3">Guppy Cycle Tasks (2026.1)</h3>
            <ul class="list-disc pl-6 space-y-1 text-xl">
                <li>Converting the core event loop to native threads</li>
                <li>Using the new thread-based backend from oslo.service when available</li>
            </ul>
        </div>
                
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <h3 class="text-2xl font-bold mb-3">Tracking</h3>
            <ul class="list-disc pl-6 space-y-1 text-xl">
                <li>Blueprint: <a href="https://blueprints.launchpad.net/nova/+spec/eventlet-removal-part-1" class="text-cyan-400" target="_blank">blueprints.launchpad.net/nova/+spec/eventlet-removal-part-1</a></li>
                <li>Progress etherpad: <a href="https://etherpad.opendev.org/p/nova-eventlet-removal" class="text-cyan-400" target="_blank">etherpad.opendev.org/p/nova-eventlet-removal</a></li>
                <li>Developer blog: <a href="https://gibizer.github.io/categories/eventlet/" class="text-cyan-400" target="_blank">Balazs Gibizer's blog on Eventlet removal progress</a></li>
            </ul>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="resources" class="text-3xl font-bold mb-6">Resources <a href="#resources" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-3">Documentation & Guidelines</h3>
                <ul class="list-disc pl-6 space-y-2 text-xl">
                    <li><a href="https://governance.openstack.org/tc/goals/selected/remove-eventlet.html" class="text-cyan-400" target="_blank">Official Goal</a></li>
                    <li><a href="https://wiki.openstack.org/wiki/Eventlet-removal" class="text-cyan-400" target="_blank">Wiki</a></li>
                    <li><a href="{{ site.baseurl }}{% link guide/getting-started.md %}" class="text-cyan-400">Getting Started Guide</a></li>
                </ul>
            </div>
            
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-3">Development Resources</h3>
                <ul class="list-disc pl-6 space-y-2 text-xl">
                    <li><a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22" class="text-cyan-400" target="_blank">Gerrit Changes</a></li>
                    <li><a href="https://ptg.opendev.org/etherpads.html" class="text-cyan-400" target="_blank">Flamingo PTG etherpads</a></li>
                </ul>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="conclusion" class="text-3xl font-bold mb-6">Conclusion <a href="#conclusion" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <p class="text-xl">The migration away from Eventlet represents a major technical challenge for OpenStack but is becoming increasingly urgent given Python's evolution and growing issues with Eventlet. Discussions during the April 2025 PTG show consensus on the necessity of this migration, with significant progress in certain projects and an increasingly clear strategy.</p>
            
            <p class="text-xl mt-4">The success of this migration will require continuous coordination between teams and a pragmatic approach to managing the transition, taking into account the specificities of each project. Projects like Nova are adopting a dual-mode strategy to ensure stability during the transition period, while leveraging lessons learned from projects that have already made significant progress such as Octavia, Neutron, and Mistral. Octavia's complete migration since 2017 serves as a particularly valuable case study, demonstrating that a full transition is achievable and sustainable over the long term.</p>
        </div>
    </div>
</section>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/openstack.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>OpenStack Resources
    </a>
</div>