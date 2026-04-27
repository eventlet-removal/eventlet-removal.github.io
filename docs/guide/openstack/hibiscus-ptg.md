---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: dashboard
title: OpenStack 2027.1 Hibiscus PTG - Eventlet Removal Retrospective
permalink: /guide/openstack/hibiscus-ptg/
description: Analysis of the Eventlet removal progress during the Gazpacho cycle and call to action for Hibiscus, based on the April 2026 Project Teams Gathering (PTG) office hours session.
keywords: openstack, eventlet removal, hibiscus ptg, gazpacho retrospective, migration strategies, threading, asyncio, python 3.14, project teams gathering
og_type: article
og_title: OpenStack Hibiscus PTG Eventlet Removal Retrospective
og_description: Comprehensive retrospective of Gazpacho achievements and roadmap for Hibiscus cycle in OpenStack's eventlet removal initiative.
---
<section>
    <h1 class="text-4xl font-bold">Eventlet Removal PTG Retrospective - Gazpacho to Hibiscus</h1>
    <p class="mt-4 text-lg"><em>Report created by <a href="https://herve.beraud.io" class="text-cyan-400 hover:underline">Hervé Beraud</a> on April 27, 2026</em></p>
    <p class="mt-10 text-xl">This report presents a comprehensive analysis of the eventlet removal office hours session held during the April 2026 Project Teams Gathering (PTG), covering the retrospective from the Gazpacho cycle and outlining the call to action for Hibiscus.</p>

    <div class="mt-6 mb-6">
        <h2 class="text-2xl font-bold mb-4">Session Recording</h2>
        <div class="aspect-w-16 aspect-h-9 bg-gray-900 rounded-lg overflow-hidden shadow-lg">
            <iframe 
                width="100%" 
                height="500" 
                src="https://www.youtube.com/embed/dDgUdKPRNZU" 
                title="OpenStack Eventlet Removal Office Hours - Hibiscus PTG" 
                frameborder="0" 
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" 
                allowfullscreen
                class="rounded-lg">
            </iframe>
        </div>
        <p class="mt-4 text-lg">
            <strong>Resources:</strong> 
            <a href="https://etherpad.opendev.org/p/openstack-eventlet-removal" class="text-cyan-400 hover:underline" target="_blank">Session Etherpad</a>
        </p>
    </div>

    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#gazpacho-retrospective" class="text-cyan-400 hover:underline">Gazpacho Retrospective</a></li>
                <li><a href="#project-progress" class="text-cyan-400 hover:underline">Project-by-Project Progress</a></li>
                <li><a href="#hibiscus-action" class="text-cyan-400 hover:underline">Call to Action for Hibiscus</a></li>
                <li><a href="#concerns" class="text-cyan-400 hover:underline">Concerns and Noticed Problems</a></li>
                <li><a href="#performance" class="text-cyan-400 hover:underline">Performance Testing Insights</a></li>
                <li><a href="#achievements" class="text-cyan-400 hover:underline">Notable Achievements</a></li>
                <li><a href="#next-steps" class="text-cyan-400 hover:underline">Next Steps Summary</a></li>
            </ul>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="gazpacho-retrospective" class="text-3xl font-bold mb-6">Gazpacho Retrospective <a href="#gazpacho-retrospective" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
            <h3 class="text-2xl font-bold mb-4">Core Libraries Progress</h3>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
                <div class="bg-green-800 p-6 rounded-lg">
                    <h4 class="text-xl font-bold mb-3">Eventlet Library</h4>
                    <ul class="list-disc pl-6 space-y-2 text-xl">
                        <li>✅ Upgraded from Python 3.14 alpha to stable</li>
                        <li>✅ Dropped Python 3.9 support (now 3.10-3.14)</li>
                        <li>🔄 Monitoring Python 3.15 (waiting for stable)</li>
                        <li>✅ Strengthened asyncio integration</li>
                        <li>✅ Fixed Python 3.14 compatibility with asyncio</li>
                        <li>✅ Refactored patching logic for API stability</li>
                    </ul>
                </div>
                
                <div class="bg-green-800 p-6 rounded-lg">
                    <h4 class="text-xl font-bold mb-3">Oslo Services</h4>
                    <ul class="list-disc pl-6 space-y-2 text-xl">
                        <li>✅ Introduced multiprocessing spawn support</li>
                        <li>✅ Reduced fork instability in multi-threaded environments</li>
                        <li>✅ Two patches merged</li>
                        <li>🔄 One patch pending (requires POC)</li>
                        <li>Primary benefit: Manila stability improvements</li>
                    </ul>
                </div>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="project-progress" class="text-3xl font-bold mb-6">Project-by-Project Progress <a href="#project-progress" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <h3 class="text-2xl font-bold mb-4 text-green-400">✅ Completed Migrations</h3>
        
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-4 mb-8">
            <div class="bg-green-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Designate</h4>
                <p class="text-lg mb-2"><strong>Status: COMPLETE</strong></p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>Full migration finished</li>
                    <li>Major milestone achieved in Gazpacho</li>
                </ul>
            </div>
            
            <div class="bg-green-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Neutron</h4>
                <p class="text-lg mb-2"><strong>Status: COMPLETE</strong></p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>Moved Oslo services to threading</li>
                    <li>Removed all eventlet imports</li>
                    <li>Fully operational</li>
                </ul>
            </div>
            
            <div class="bg-green-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Cyborg</h4>
                <p class="text-lg mb-2"><strong>Status: EVENTLET-FREE</strong></p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>Removed monkey patch</li>
                    <li>Fixed DB deadlock</li>
                    <li>Native WSGI server removed</li>
                </ul>
            </div>
        </div>
        
        <h3 class="text-2xl font-bold mb-4 text-yellow-400">🟢 Significant Progress</h3>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Nova</h4>
                <p class="text-lg mb-2"><strong>Status: MOSTLY COMPLETE</strong></p>
                <p class="mb-3 text-green-300">Gazpacho Defaults to Threading:</p>
                <ul class="list-disc pl-6 space-y-1 mb-3">
                    <li>✅ API service</li>
                    <li>✅ Metadata API</li>
                    <li>✅ Scheduler</li>
                </ul>
                <p class="mb-3 text-blue-300">Hibiscus Merged Defaults:</p>
                <ul class="list-disc pl-6 space-y-1 mb-3">
                    <li>✅ Compute</li>
                    <li>✅ Conductor</li>
                </ul>
                <p class="mb-2 text-orange-300">Pending:</p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>Console proxies (noVNC, spice, serial)</li>
                    <li>CLI gaps</li>
                    <li>Functional tests conversion</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Manila</h4>
                <p class="text-lg mb-2"><strong>Status: ADVANCED</strong></p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>✅ Environment option for threading</li>
                    <li>✅ Share manager migrated</li>
                    <li>✅ Rally performance framework developed</li>
                    <li>✅ No significant performance regression</li>
                    <li>🔄 Planning functional tests for Hibiscus</li>
                    <li>🔄 Considering eventlet default switch</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Cinder</h4>
                <p class="text-lg mb-2"><strong>Status: GOOD SHAPE</strong></p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>✅ Volume and scheduler operational</li>
                    <li>✅ Aligned with Oslo services</li>
                    <li>🔄 Backup service in progress</li>
                    <li>🔄 Multiple patches targeting Hibiscus</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Glance</h4>
                <p class="text-lg mb-2"><strong>Status: GOOD SHAPE</strong></p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>✅ Removed eventlet from utils</li>
                    <li>✅ Unit tests use native threading</li>
                    <li>🔄 Client migration pending (waiting for consumers)</li>
                </ul>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Watcher</h4>
                <p class="text-lg mb-2"><strong>Status: ADVANCED</strong></p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>✅ All components migrated (API, Decision Engine, Applier)</li>
                    <li>✅ <strong>Switched to threading by default in early Hibiscus</strong></li>
                    <li>✅ CI passing with eventlet code deleted</li>
                    <li>🔄 Considering full removal in 2027 or end of cycle</li>
                </ul>
            </div>
        </div>
        
        <h3 class="text-2xl font-bold mb-4 mt-8 text-orange-400">🟡 In Progress</h3>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
            <div class="bg-orange-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-2">Swift</h4>
                <p class="text-lg mb-2"><strong>Status: IN PROGRESS</strong></p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>✅ 100% functional tests passing</li>
                    <li>✅ 99% unit tests passing</li>
                    <li>⚠️ Remaining 1% contains hardest problems</li>
                    <li>🔧 Erasure coding predictability challenges</li>
                    <li>🔧 Evaluating test assumptions vs. real requirements</li>
                </ul>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="hibiscus-action" class="text-3xl font-bold mb-6">Call to Action for Hibiscus <a href="#hibiscus-action" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-4">
            <h3 class="text-2xl font-bold mb-4">Primary Goals</h3>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
                <div class="bg-blue-800 p-6 rounded-lg">
                    <h4 class="text-xl font-bold mb-3">1. Continue Migration Momentum</h4>
                    <ul class="list-disc pl-6 space-y-2">
                        <li>Projects not yet migrated maintain active development</li>
                        <li>Default to threading as soon as technically feasible</li>
                    </ul>
                </div>
                
                <div class="bg-blue-800 p-6 rounded-lg">
                    <h4 class="text-xl font-bold mb-3">2. Gradual Default Switching</h4>
                    <ul class="list-disc pl-6 space-y-2">
                        <li><strong>Release N:</strong> Both modes (eventlet default)</li>
                        <li><strong>Release N+1:</strong> Both modes (threading default)</li>
                        <li><strong>Release N+2:</strong> Remove eventlet support</li>
                    </ul>
                </div>
                
                <div class="bg-blue-800 p-6 rounded-lg">
                    <h4 class="text-xl font-bold mb-3">3. Testing Infrastructure</h4>
                    <ul class="list-disc pl-6 space-y-2">
                        <li>Convert functional tests to threading mode</li>
                        <li>Expand Python 3.14 testing coverage</li>
                        <li>Prepare for Ubuntu 26.04 (release: April 29, 2026)</li>
                        <li>Rally performance comparison jobs</li>
                    </ul>
                </div>
                
                <div class="bg-blue-800 p-6 rounded-lg">
                    <h4 class="text-xl font-bold mb-3">4. Library Cleanup</h4>
                    <ul class="list-disc pl-6 space-y-2">
                        <li>Address unconditional eventlet imports</li>
                        <li>Make eventlet dependencies optional</li>
                        <li>Clean up transitive dependencies</li>
                    </ul>
                </div>
                
                <div class="bg-blue-800 p-6 rounded-lg">
                    <h4 class="text-xl font-bold mb-3">5. Documentation</h4>
                    <ul class="list-disc pl-6 space-y-2">
                        <li>Thread pool sizing guidance for operators</li>
                        <li>Performance tuning recommendations</li>
                        <li>Migration paths for deployments</li>
                    </ul>
                </div>
            </div>
        </div>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow mt-6">
            <h3 class="text-2xl font-bold mb-4">Example Migration Timeline</h3>
            <div class="bg-gray-800 p-4 rounded">
                <p class="mb-2"><strong>Nova Console Proxies:</strong></p>
                <ul class="list-disc pl-6 space-y-1">
                    <li><strong>Hibiscus:</strong> Support threading (eventlet default)</li>
                    <li><strong>Next cycle:</strong> Switch to threading default</li>
                    <li><strong>Following cycle:</strong> Remove eventlet support</li>
                </ul>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="concerns" class="text-3xl font-bold mb-6">Concerns and Noticed Problems <a href="#concerns" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <h3 class="text-2xl font-bold mb-4 text-red-400">🔴 Critical Issues</h3>
        
        <div class="grid grid-cols-1 gap-6 mt-4">
            <div class="bg-red-900 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-3">1. Picklable Problem (Multiprocessing Spawn)</h4>
                <p class="mb-3"><strong>Impact:</strong> High | <strong>Affected:</strong> Neutron, Nova, Multiple projects</p>
                
                <div class="bg-gray-800 p-4 rounded mb-3">
                    <p class="mb-2"><strong>Issue:</strong></p>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Objects passed to multiprocessing spawn must be picklable</li>
                        <li>Config objects from Oslo.config are not picklable</li>
                        <li>Python 3.14 changes default context to <code>fork_server</code></li>
                        <li><code>fork</code> context being deprecated (removal timeline unknown)</li>
                        <li>Warning system implemented but fixes pending</li>
                    </ul>
                </div>
                
                <div class="bg-blue-900 p-4 rounded">
                    <p class="mb-2"><strong>Actions Needed:</strong></p>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Investigate common patterns for making objects picklable</li>
                        <li>File bugs in Oslo.config if root cause confirmed</li>
                        <li>Test spawn method compatibility across projects</li>
                        <li>Determine if fork_server is acceptable fallback</li>
                    </ul>
                </div>
            </div>
            
            <div class="bg-red-900 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-3">2. Root Wrap Compatibility</h4>
                <p class="mb-3"><strong>Impact:</strong> High | <strong>Affected:</strong> Nova, Cinder (via OS-brick)</p>
                
                <div class="bg-gray-800 p-4 rounded mb-3">
                    <p class="mb-2"><strong>Issue:</strong></p>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Root wrap may not work properly in threaded mode</li>
                        <li>OS-brick uses root wrap and is consumed by multiple services</li>
                        <li>Services can be eventlet-free but blocked by library dependencies</li>
                        <li>Proof uses pickle for transport over Unix sockets (related to picklable issue)</li>
                    </ul>
                </div>
                
                <div class="bg-blue-900 p-4 rounded">
                    <p class="mb-2"><strong>Actions Needed:</strong></p>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Test root wrap in threading mode</li>
                        <li>Identify workarounds or alternatives</li>
                        <li>Coordinate with OS-brick maintainers</li>
                    </ul>
                </div>
            </div>
            
            <div class="bg-red-900 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-3">3. Heartbeat Performance Regression</h4>
                <p class="mb-3"><strong>Impact:</strong> Medium | <strong>Status:</strong> Solution identified</p>
                
                <div class="bg-gray-800 p-4 rounded mb-3">
                    <p class="mb-2"><strong>Issue:</strong></p>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Half-second sleep in while-true loop causes thread blocking</li>
                        <li>Only affects <code>heartbeat_in_pthread=true</code> deployments</li>
                        <li>Feature being deprecated/removed anyway</li>
                    </ul>
                </div>
                
                <div class="bg-green-900 p-4 rounded">
                    <p class="mb-2"><strong>Actions Agreed:</strong></p>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Revert change in master</li>
                        <li>Handle stable branch backports carefully</li>
                        <li>Investigate TCP keepalive as alternative (already working for Tooz/OBS)</li>
                    </ul>
                </div>
            </div>
        </div>
        
        <h3 class="text-2xl font-bold mb-4 mt-8 text-yellow-400">🟡 Medium Priority Issues</h3>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-3">4. Library Eventlet Dependencies</h4>
                <p class="mb-3"><strong>Impact:</strong> Medium</p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>Oslo.service still depends on eventlet as default</li>
                    <li>Oslo.log unconditionally imports eventlet (pipe mutex fix)</li>
                    <li>Oslo.concurrency checks eventlet at import time</li>
                    <li>Multiple libraries have unnecessary imports</li>
                </ul>
                <p class="mt-3 text-blue-300"><strong>Actions:</strong> Audit libraries, convert to conditional imports, make eventlet optional</p>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-3">5. Long Synchronous API Calls</h4>
                <p class="mb-3"><strong>Impact:</strong> Medium | <strong>Affected:</strong> Nova, potentially others</p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>Eventlet: blocks one greenthread (cheap)</li>
                    <li>External WSGI + threading: blocks entire process (expensive)</li>
                    <li>Nova fixed volume attach/detach</li>
                    <li>Interface detach still pending</li>
                </ul>
                <p class="mt-3 text-blue-300"><strong>Actions:</strong> Audit APIs, convert to async, document operator impacts</p>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-3">6. Python 3.14 Testing Coverage</h4>
                <p class="mb-3"><strong>Impact:</strong> Medium</p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>Most CI on Python 3.12 (Ubuntu 24.04)</li>
                    <li>Nova testing on 3.13 (Debian) for eventlet-free jobs</li>
                    <li>Limited 3.14 testing despite compatibility work</li>
                    <li>Ubuntu 26.04 releases April 29, 2026</li>
                </ul>
                <p class="mt-3 text-blue-300"><strong>Actions:</strong> Enable 3.14 in devstack, test threading-default services</p>
                <p class="mt-2 text-orange-300"><strong>Known issue:</strong> sudo-rs (Rust sudo) doesn't launch properly - use GNU version</p>
            </div>
            
            <div class="bg-yellow-800 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-3">7. Glance Client Migration Timing</h4>
                <p class="mb-3"><strong>Impact:</strong> Low</p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>Has eventlet checks that appear unnecessary</li>
                    <li>Imports explicit eventlet classes if patcher applied</li>
                    <li>Likely not needed (patching happens automatically)</li>
                    <li>Waiting for consuming services first</li>
                </ul>
                <p class="mt-3 text-blue-300"><strong>Actions:</strong> Test removal in tips job, coordinate with Nova/Cinder</p>
            </div>
        </div>
        
        <h3 class="text-2xl font-bold mb-4 mt-8 text-purple-400">🟣 Architectural Concerns</h3>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
            <div class="bg-purple-900 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-3">8. Async.io Pressure from Libraries</h4>
                <p class="mb-3"><strong>Impact:</strong> Low (future concern)</p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>Some libraries forcing async.io only (Pyro2, Blazer)</li>
                    <li>Pyro2 impacts Swift and Nova</li>
                    <li>SQLAlchemy and Oslo.db support async.io (nothing using it yet)</li>
                </ul>
                <div class="mt-3 p-3 bg-gray-800 rounded">
                    <p class="font-bold mb-2">Community Position:</p>
                    <p>Avoid three runtimes (eventlet + threading + async.io). Complete eventlet removal first, then evaluate async.io adoption.</p>
                </div>
            </div>
            
            <div class="bg-purple-900 p-6 rounded-lg shadow">
                <h4 class="text-xl font-bold mb-3">9. Timeline Alignment Challenges</h4>
                <p class="mb-3"><strong>Impact:</strong> Low (process concern)</p>
                <ul class="list-disc pl-6 space-y-1">
                    <li>Projects moved at different paces</li>
                    <li>Some eventlet-free for years (Placement, Keystone, Ironic)</li>
                    <li>Can't enforce uniform "all support eventlet in 2027.1"</li>
                </ul>
                <div class="mt-3 p-3 bg-gray-800 rounded">
                    <p class="font-bold mb-2">Agreed Timeline:</p>
                    <ul class="list-disc pl-6 space-y-1 text-sm">
                        <li><strong>2026.1:</strong> Threading potentially supported everywhere (eventlet still works)</li>
                        <li><strong>2027.1:</strong> Threading supported everywhere (ideally default, eventlet may work)</li>
                        <li><strong>2027.2:</strong> Eventlet removal from Oslo and dependencies</li>
                    </ul>
                </div>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="performance" class="text-3xl font-bold mb-6">Performance Testing Insights <a href="#performance" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-4">
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-3">Manila Rally Framework</h3>
                <ul class="list-disc pl-6 space-y-2">
                    <li>✅ Developed reusable job template (Grenade-based pattern)</li>
                    <li>✅ Runs: stable (eventlet) → upgrade → new (threading)</li>
                    <li>✅ Generates comparative reports</li>
                    <li>✅ <strong>Finding: No significant performance differences</strong></li>
                    <li>✅ Framework adaptable for other projects</li>
                </ul>
                <p class="mt-4 text-green-300 font-bold">Key Result: Performance parity achieved</p>
            </div>
            
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-3">Nova Considerations</h3>
                <ul class="list-disc pl-6 space-y-2">
                    <li>Thread pool sizing affects memory usage significantly</li>
                    <li>Native threads more expensive than greenthreads</li>
                    <li>Working to tune defaults for similar performance + memory</li>
                    <li>Developing operator tuning documentation</li>
                </ul>
                <p class="mt-4 text-yellow-300 font-bold">Focus: Tuning thread pools for production workloads</p>
            </div>
            
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-3">Swift Challenges</h3>
                <ul class="list-disc pl-6 space-y-2">
                    <li>Erasure coding requires predictable request ordering</li>
                    <li>Fragment reconstruction depends on query sequencing</li>
                    <li>Eventlet co-routine switching is deterministic</li>
                    <li>Threading introduces non-determinism</li>
                    <li>Evaluating where predictability is actually required vs. test artifacts</li>
                </ul>
                <p class="mt-4 text-orange-300 font-bold">Challenge: Distinguishing real requirements from test assumptions</p>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="achievements" class="text-3xl font-bold mb-6">Notable Achievements <a href="#achievements" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <div class="bg-gradient-to-r from-green-900 to-blue-900 p-6 rounded-lg shadow">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <div class="bg-gray-800 bg-opacity-70 p-4 rounded-lg">
                    <h4 class="text-xl font-bold mb-2 text-green-400">✅ Complete Migrations</h4>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Designate</li>
                        <li>Neutron</li>
                        <li>Cyborg</li>
                    </ul>
                </div>
                
                <div class="bg-gray-800 bg-opacity-70 p-4 rounded-lg">
                    <h4 class="text-xl font-bold mb-2 text-blue-400">🔄 Default Threading</h4>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Nova (5/7 services)</li>
                        <li>Watcher (all components)</li>
                    </ul>
                </div>
                
                <div class="bg-gray-800 bg-opacity-70 p-4 rounded-lg">
                    <h4 class="text-xl font-bold mb-2 text-purple-400">🧪 Testing Infrastructure</h4>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Rally comparison framework</li>
                        <li>Performance parity validated</li>
                    </ul>
                </div>
                
                <div class="bg-gray-800 bg-opacity-70 p-4 rounded-lg">
                    <h4 class="text-xl font-bold mb-2 text-yellow-400">🐍 Python Support</h4>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Eventlet 3.14 compatible</li>
                        <li>Monitoring 3.15</li>
                    </ul>
                </div>
                
                <div class="bg-gray-800 bg-opacity-70 p-4 rounded-lg">
                    <h4 class="text-xl font-bold mb-2 text-cyan-400">📊 Performance</h4>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>No major regressions</li>
                        <li>Manila shows parity</li>
                    </ul>
                </div>
                
                <div class="bg-gray-800 bg-opacity-70 p-4 rounded-lg">
                    <h4 class="text-xl font-bold mb-2 text-pink-400">📚 Knowledge Sharing</h4>
                    <ul class="list-disc pl-6 space-y-1">
                        <li>Reusable patterns</li>
                        <li>Cross-project collaboration</li>
                    </ul>
                </div>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 id="next-steps" class="text-3xl font-bold mb-6">Next Steps Summary <a href="#next-steps" class="text-cyan-400 text-xl">🔗</a></h2>
        
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-4">
            <div class="bg-blue-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-4">Immediate (Hibiscus Cycle)</h3>
                <ul class="space-y-2">
                    <li>☐ Nova: Complete console proxy migration</li>
                    <li>☐ Address picklable warnings (file bugs, develop patterns)</li>
                    <li>☐ Revert heartbeat sleep patch</li>
                    <li>☐ Enable 3.14 testing when Ubuntu 26.04 releases</li>
                    <li>☐ Convert more functional tests to threading mode</li>
                    <li>☐ More projects default to threading</li>
                </ul>
            </div>
            
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-4">Medium Term (2027.1)</h3>
                <ul class="space-y-2">
                    <li>☐ All projects support threading mode</li>
                    <li>☐ Threading is default for most/all projects</li>
                    <li>☐ Eventlet still available but not recommended</li>
                    <li>☐ Comprehensive operator documentation</li>
                </ul>
            </div>
            
            <div class="bg-purple-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-4">Long Term (2027.2+)</h3>
                <ul class="space-y-2">
                    <li>☐ Remove eventlet from Oslo libraries</li>
                    <li>☐ Remove transitive eventlet dependencies</li>
                    <li>☐ Evaluate async.io adoption where beneficial</li>
                    <li>☐ Consider OpenStack SDK migration for clients</li>
                </ul>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 class="text-3xl font-bold mb-6">Resources</h2>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-3">Session Materials</h3>
                <ul class="list-disc pl-6 space-y-2">
                    <li><a href="https://www.youtube.com/watch?v=dDgUdKPRNZU" class="text-cyan-400 hover:underline" target="_blank">YouTube Recording</a></li>
                    <li><a href="https://etherpad.opendev.org/p/openstack-eventlet-removal" class="text-cyan-400 hover:underline" target="_blank">Session Etherpad</a></li>
                    <li><a href="https://ptg.opendev.org/etherpads.html" class="text-cyan-400 hover:underline" target="_blank">PTG Etherpads</a></li>
                </ul>
            </div>
            
            <div class="bg-indigo-900 p-6 rounded-lg shadow">
                <h3 class="text-2xl font-bold mb-3">Documentation & Guidelines</h3>
                <ul class="list-disc pl-6 space-y-2">
                    <li><a href="https://governance.openstack.org/tc/goals/selected/remove-eventlet.html" class="text-cyan-400 hover:underline" target="_blank">Official Goal</a></li>
                    <li><a href="https://wiki.openstack.org/wiki/Eventlet-removal" class="text-cyan-400 hover:underline" target="_blank">Wiki</a></li>
                    <li><a href="{{ site.baseurl }}{% link guide/getting-started.md %}" class="text-cyan-400 hover:underline">Getting Started Guide</a></li>
                </ul>
            </div>
        </div>
    </div>
</section>

<section>
    <div class="mt-10">
        <h2 class="text-3xl font-bold mb-6">Conclusion</h2>
        
        <div class="bg-indigo-900 p-6 rounded-lg shadow">
            <p class="text-xl mb-4">The Gazpacho cycle demonstrated significant progress across the OpenStack ecosystem in removing eventlet dependencies. With two projects fully migrated (Designate, Neutron), one completely eventlet-free (Cyborg), and major advances in Nova, Manila, Watcher, and others, the community has established clear momentum.</p>
            
            <p class="text-xl mb-4">The development of reusable testing infrastructure, particularly Manila's Rally comparison framework showing performance parity, provides confidence that this transition can be achieved without significant performance degradation. Nova's gradual service-by-service approach and Watcher's successful switch to threading by default in Hibiscus demonstrate multiple viable migration strategies.</p>
            
            <p class="text-xl mb-4">Key challenges remain, particularly around picklable objects for multiprocessing spawn, root wrap compatibility, and library dependency cleanup. However, with Ubuntu 26.04 bringing Python 3.14 by default and the deprecation timeline for eventlet becoming clearer, the urgency and path forward are well understood.</p>
            
            <p class="text-xl">The Hibiscus cycle represents a critical phase where the focus shifts from initial migration to defaulting to threading mode, expanding test coverage, and addressing the remaining technical blockers. The community's collaborative approach through dedicated office hours sessions and the #openstack-eventlet-removal channel continues to facilitate knowledge sharing and coordinated progress toward a threading-based future for OpenStack.</p>
        </div>
    </div>
</section>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/openstack.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>OpenStack Resources
    </a>
    <a href="{{ site.baseurl }}{% link guide/openstack/flamingo.md %}" class="inline-block bg-gradient-to-r from-blue-400 to-blue-600 text-white font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        Flamingo PTG Report<i class="fas fa-arrow-right ml-2"></i>
    </a>
</div>
