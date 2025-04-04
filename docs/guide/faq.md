---
layout: dashboard
title: Frequently Asked Questions
permalink: /guide/faq/
description: Find answers to commonly asked questions about migrating from Eventlet, including technical challenges, performance considerations, and practical solutions for specific use cases.
keywords: eventlet faq, migration questions, eventlet replacement, asyncio vs eventlet, threading vs eventlet, eventlet migration problems, transition challenges
og_type: article
og_title: Eventlet Removal FAQ - Answering Your Migration Questions
og_description: Comprehensive answers to the most common questions about migrating away from Eventlet to modern asynchronous alternatives.
---

<div>
  <h1 class="text-4xl font-bold mb-6">Frequently Asked Questions</h1>
  
  <div class="space-y-8">
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Is this migration initiative an official initiative?</h3>
      <p class="text-xl">
        Yes. This initiative is supported by the OpenStack community. This initiative found its roots in <a href="https://review.opendev.org/c/openstack/governance/+/902585" class="text-cyan-400 hover:underline">an OpenStack community goal</a>.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What will happen to eventlet once the openstack migration is complete?</h3>
      <p class="text-xl">
        Eventlet will be simply abandoned officially. No more maintenance will be given to the official Eventlet repository. The official Eventlet repository will be archived.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Why not simply maintaining Eventlet?</h3>
      <p class="text-xl">
        We have many reasons for that:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>
          The main reason is that, since years, to keep running, Eventlet started to fork deprecated  old CPython stdlib modules that are now removed from CPython. With coming versions of Python this kind of workaround will surely have to repeat. This trend seems already coming with Python 3.13 and the threading module (see the discussion: <a href="https://github.com/eventlet/eventlet/pull/966#issuecomment-2344252826" class="text-cyan-400 hover:underline">GitHub comment</a>). We do not want to transform Eventlet into a poorly maintained fork of CPython. A poor full of security breaches. Moving away of that trend would almost require a complete rewrite of Eventlet.
        </li>
        <li class="mt-2">
          The second reason is more about resources. Eventlet is maintained by 1 or 2 part time core maintainers, that's not enough to keep its head above water, especially if we take account of the previous reason. We do not want to repeat the log4shell story.
        </li>
        <li class="mt-2">
          The third reason is sustainability. When the migration will be done, even if the migration was difficult, your projects will be safe for next decades. We want to encourage a sustainable solution.
        </li>
      </ul>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Should I choose AsyncIO or Threading to replace Eventlet?</h3>
      <p class="text-xl">
        The choice depends on your use case and system architecture:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>
          <strong>AsyncIO</strong> is generally recommended for new development and I/O-intensive applications. It offers better performance for handling many simultaneous connections, but requires more significant code refactoring.
        </li>
        <li class="mt-2">
          <strong>Threading</strong> may be easier to implement if your current code already heavily uses Eventlet's thread patterns. It's often a more accessible transition step, especially for complex existing applications.
        </li>
      </ul>
      <p class="mt-4 text-xl">
        In all cases, we recommend analyzing your code and specific needs before making a choice. Check our dedicated guides for <a href="{{ site.baseurl }}{% link guide/asyncio.md %}" class="text-cyan-400 hover:underline">AsyncIO</a> and <a href="{{ site.baseurl }}{% link guide/threading.md %}" class="text-cyan-400 hover:underline">Threading</a>.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How do I manage the transition period when some parts of the code are still using Eventlet?</h3>
      <p class="text-xl">
        Progressive migration is often unavoidable in large projects. Here are some strategies:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>
          Isolate components using Eventlet in separate modules with clearly defined interfaces.
        </li>
        <li class="mt-2">
          Use adapters or wrappers to enable communication between migrated code and code still using Eventlet.
        </li>
        <li class="mt-2">
          Prioritize migrating the least interdependent components to reduce complexity.
        </li>
        <li class="mt-2">
          Implement robust integration tests to verify that both paradigms work correctly together.
        </li>
      </ul>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How can I estimate the difficulty and time needed for migration?</h3>
      <p class="text-xl">
        The migration effort varies considerably depending on:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>
          The size of your codebase and the depth of Eventlet usage.
        </li>
        <li class="mt-2">
          The number of external dependencies that also use Eventlet.
        </li>
        <li class="mt-2">
          The use of global monkey patching vs. specific Eventlet features.
        </li>
      </ul>
      <p class="mt-4 text-xl">
        We recommend:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>
          Starting with an impact analysis to identify all Eventlet usage points.
        </li>
        <li class="mt-2">
          Conducting a pilot project on a less critical component to assess specific challenges.
        </li>
        <li class="mt-2">
          Planning the migration in phases, starting with the most accessible layers.
        </li>
      </ul>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How can I effectively test after migration?</h3>
      <p class="text-xl">
        Post-migration validation is critical to ensure stability:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>
          Use load and concurrency tests to verify that performance remains acceptable.
        </li>
        <li class="mt-2">
          Implement specific tests for timeout scenarios and error handling.
        </li>
        <li class="mt-2">
          Ensure you test asynchronous behaviors under real conditions.
        </li>
        <li class="mt-2">
          Carefully monitor performance metrics and logs during initial deployment.
        </li>
      </ul>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Are there OpenStack projects that have already successfully completed this migration?</h3>
      <p class="text-xl">
        Yes, several OpenStack projects have already undertaken or completed this migration:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>
          <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/neutron" class="text-cyan-400 hover:underline">Neutron</a> has started its migration several months ago and its state is now well advanced.
        </li>
        <li class="mt-2">
          <a href="https://github.com/openstack/octavia/commit/9027154a5a56a3cd23150415c78fe50af2124a2e" class="text-cyan-400 hover:underline">Octavia</a> completely removed its dependency on Eventlet since 2017.
        </li>
        <li class="mt-2">
          <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22+project:openstack/mistral" class="text-cyan-400 hover:underline">Mistral</a> has completed most of its migration.
        </li>
      </ul>
      <p class="mt-4 text-xl">
        These projects offer concrete examples and lessons learned that can be valuable for your own migration.
      </p>
    </div>
  </div>
</div>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/tutorials.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>Tutorials
    </a>
    <div><!-- No next page --></div>
</div>

