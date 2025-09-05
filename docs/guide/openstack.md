---
permalink: /guide/openstack/
title: OpenStack Eventlet Removal Resources
layout: dashboard
description: Documentation and resources about OpenStack's community-wide effort to migrate away from Eventlet. Learn about the project's approach, timeline, and progress across different OpenStack components.
keywords: openstack eventlet removal, openstack migration, flamingo ptg, python 3.13 compatibility, openstack concurrency model, openstack threading
og_type: article
og_title: OpenStack Eventlet Removal Progress and Resources
og_description: Track OpenStack's community-wide effort to migrate away from Eventlet, including PTG discussions, migration strategies, and component-specific plans.
---

<h1 class="text-4xl font-bold mb-8">OpenStack Eventlet Removal Resources</h1>

<p class="mt-6 text-xl">OpenStack has embarked on a significant technical initiative to remove Eventlet dependencies across all its components. This section provides resources, updates, and insights into this community-wide effort.</p>

<div class="mt-10 bg-gray-800 bg-opacity-70 p-6 rounded-lg">
  <h2 class="text-2xl font-bold mb-4">Why is OpenStack Removing Eventlet?</h2>
  <p class="text-xl mb-6">
    The OpenStack Technical Committee (TC) has established Eventlet removal as a top priority for several key reasons:
  </p>
  
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-6">
    <div class="border border-cyan-800 p-4 rounded-lg">
    <h3 class="text-xl font-bold mb-2">Python Evolution</h3>
    <p>Eventlet faces significant compatibility issues with all recent Python versions starting from Python 3.12. These challenges are further exacerbated by Python 3.13 and the PEP 703 "GILectomy" implementation, making migration critical for ensuring future compatibility.</p>
    </div>
    
    <div class="border border-cyan-800 p-4 rounded-lg">
      <h3 class="text-xl font-bold mb-2">Maintenance Challenges</h3>
      <p>Diminishing maintenance activity in the Eventlet project means bugs accumulate faster than they're fixed, creating reliability issues.</p>
    </div>
    
    <div class="border border-cyan-800 p-4 rounded-lg">
      <h3 class="text-xl font-bold mb-2">Compatibility Problems</h3>
      <p>Integration issues between Eventlet and important libraries like RabbitMQ cause persistent operational problems in OpenStack deployments.</p>
    </div>
    
    <div class="border border-cyan-800 p-4 rounded-lg">
      <h3 class="text-xl font-bold mb-2">Performance Optimization</h3>
      <p>Benchmarks have shown performance advantages with alternative concurrency implementations compared to Eventlet's approach.</p>
    </div>
  </div>
</div>

<div class="mt-10 grid grid-cols-1 md:grid-cols-2 gap-8">
  <div class="bg-gray-800 bg-opacity-70 p-6 rounded-lg">
    <h2 class="text-2xl font-bold mb-4">OpenStack PTG Updates</h2>
    <p class="text-xl">
      The OpenStack Project Teams Gathering (PTG) events have featured significant discussions about Eventlet removal. Explore the documentation from these sessions to understand the community's approach and progress.
    </p>
    <ul class="mt-4 list-disc pl-6">
      <li><a href="{{ site.baseurl }}{% link guide/openstack/flamingo.md %}" class="text-cyan-400 hover:underline">Flamingo PTG (April 2025): Eventlet Discussions</a></li>
    </ul>
    <p class="mt-4 text-gray-400 italic">Updates from future PTGs will be added here</p>
  </div>

  <div class="bg-gray-800 bg-opacity-70 p-6 rounded-lg">
    <h2 class="text-2xl font-bold mb-4">Project-Specific Resources</h2>
    <p class="text-xl">
      Different OpenStack projects have adopted various approaches to Eventlet removal. Below are examples and resources from specific components.
    </p>
    <ul class="mt-4 list-disc pl-6">
      <li><a href="{{ site.baseurl }}{% link guide/studies/octavia.md %}" class="text-cyan-400 hover:underline">Octavia: A Complete Migration Case Study</a></li>
      <li><a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/neutron" class="text-cyan-400 hover:underline" target="_blank">Neutron: Eventlet Removal Patches</a></li>
      <li><a href="https://docs.openstack.org/oslo.service/latest/user/backend.html" class="text-cyan-400 hover:underline" target="_blank">Oslo.service: New Threading Backend</a></li>
      <li><a href="https://etherpad.opendev.org/p/nova-eventlet-removal" class="text-cyan-400 hover:underline" target="_blank">Nova: Eventlet Removal Progress Tracking</a></li>
      <li><a href="https://gibizer.github.io/categories/eventlet/" class="text-cyan-400 hover:underline" target="_blank">Nova: Balazs Gibizer's Blog on Eventlet Migration</a> - Detailed articles tracking Nova's progress</li>
    </ul>
    <p class="mt-4 text-gray-400 italic">Additional project resources will be added as they become available</p>
  </div>
</div>

<div class="mt-10 bg-indigo-900 bg-opacity-50 p-6 rounded-lg">
  <h2 id="migration-status" class="text-2xl font-bold mb-4">Current Migration Status Summary <a href="#migration-status" class="text-cyan-400 text-xl">🔗</a></h2>
  <p class="text-xl mb-6">
    The progress of Eventlet removal varies across OpenStack projects. Here's a high-level overview as of September 2025:
  </p>
  
  <div class="overflow-x-auto">
    <table class="min-w-full bg-gray-800 rounded-lg">
      <thead>
        <tr>
          <th class="py-2 px-4 border-b border-gray-700 text-left">Project</th>
          <th class="py-2 px-4 border-b border-gray-700 text-left">Status</th>
          <th class="py-2 px-4 border-b border-gray-700 text-left">Approach</th>
        </tr>
      </thead>
      <tbody>
        <!-- Projects with Completed or Significant Progress -->
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Octavia</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-green-800 rounded px-2 py-1">Completed</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Migration completed since 2017
            <a href="https://review.opendev.org/q/eventlet+project:openstack/octavia" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Mistral</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-green-800 rounded px-2 py-1">Completed</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Migration completed since September 2025.
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/mistral" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Ironic</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-green-800 rounded px-2 py-1">Completed</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Migration completed since August 2025.
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/ironic" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Barbican</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-green-800 rounded px-2 py-1">Completed</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Migration completed since August 2025.
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/barbican" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Heat</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-green-800 rounded px-2 py-1">Completed</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Migration completed since July 2025.
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/heat" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <!-- Projects In Progress -->
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Neutron</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-purple-800 rounded px-2 py-1">In Progress</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Dual-mode support during transition
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/neutron" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Oslo Libraries</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-purple-800 rounded px-2 py-1">In Progress</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Deprecated Eventlet features, oslo.db supports asyncio, oslo.service's threading backend is implemented
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+oslo" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Glance</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-purple-800 rounded px-2 py-1">In Progress</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Core works without Eventlet; some optional features still depend on it
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/glance" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Nova</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-purple-800 rounded px-2 py-1">In Progress</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Service-by-service approach with dual-mode support
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/nova" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Cinder</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-purple-800 rounded px-2 py-1">In Progress</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Starting with Volume Manager, then other components
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/cinder" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Manila</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-purple-800 rounded px-2 py-1">In Progress</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Multi-cycle approach, targeting completion in Guppy cycle
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/manila" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Designate</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-purple-800 rounded px-2 py-1">In Progress</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Top priority for Flamingo cycle
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/designate" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Watcher</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-purple-800 rounded px-2 py-1">In Progress</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Support running under threading mode
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/watcher" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Swift</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-purple-800 rounded px-2 py-1">In Progress</span></td>
          <td class="py-2 px-4 border-b border-gray-700">"Canary node" approach starting with proxies
            <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/swift" class="text-cyan-400 hover:underline" target="_blank">Show the migration history</a>
          </td>
        </tr>
        
        <!-- Projects in Planning Phase -->
        <tr>
          <td class="py-2 px-4 border-b border-gray-700">Blazar</td>
          <td class="py-2 px-4 border-b border-gray-700"><span class="bg-yellow-800 rounded px-2 py-1">Planning</span></td>
          <td class="py-2 px-4 border-b border-gray-700">Evaluating alternatives to Eventlet WSGI</td>
        </tr>
      </tbody>
    </table>
  </div>
</div>

<div class="mt-10 bg-gray-800 bg-opacity-70 p-6 rounded-lg">
  <h2 class="text-2xl font-bold mb-4">OpenStack Community Resources</h2>
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <div class="border border-gray-700 p-4 rounded-lg">
      <h3 class="text-xl font-bold mb-2">Official Documentation</h3>
      <ul class="list-disc pl-6">
        <li><a href="https://governance.openstack.org/tc/goals/selected/remove-eventlet.html" class="text-cyan-400 hover:underline" target="_blank">OpenStack TC Goal: Remove Eventlet</a></li>
        <li><a href="https://wiki.openstack.org/wiki/Eventlet-removal" class="text-cyan-400 hover:underline" target="_blank">OpenStack Wiki: Eventlet Removal</a></li>
        <li><a href="https://eventlet.readthedocs.io/en/latest/" class="text-cyan-400 hover:underline" target="_blank">Eventlet Documentation</a></li>
      </ul>
    </div>
    
    <div class="border border-gray-700 p-4 rounded-lg">
      <h3 class="text-xl font-bold mb-2">Development Resources</h3>
      <ul class="list-disc pl-6">
        <li><a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22" class="text-cyan-400 hover:underline" target="_blank">All Eventlet Removal Patches in Gerrit</a></li>
        <li><a href="https://ptg.opendev.org/etherpads.html" class="text-cyan-400 hover:underline" target="_blank">PTG Etherpads Archive</a></li>
        <li><a href="https://webchat.oftc.net/?nick=chan&channels=%23openstack-eventlet-removal" class="text-cyan-400 hover:underline" target="_blank">#openstack-eventlet-removal IRC Channel</a></li>
        <li><a href="https://meetings.opendev.org/#Eventlet_Removal_Pop_Up_Team_Meeting" class="text-cyan-400 hover:underline" target="_blank">OpenStack Meeting Logs</a></li>
      </ul>
    </div>
  </div>
</div>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/testimonials.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>Testimonials
    </a>
    <a href="{{ site.baseurl }}{% link guide/resources.md %}" class="inline-block bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        Additional Resources<i class="fas fa-arrow-right ml-2"></i>
    </a>
</div>
