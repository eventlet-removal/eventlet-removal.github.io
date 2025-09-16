---
layout: dashboard
title: Migration Testimony - Dmitry Tantsur
permalink: /guide/testimonials/dmitry-tantsur/
description: Dmitry Tantsur shares his experience managing about how Ironic was migrated to native threading, providing insights and practical strategies for a successful transition.
keywords: eventlet migration, openstack ironic, migration testimonial, eventlet removal, threading migration, practical experience, openstack governance, openstack technical commitee
og_type: article
og_title: Eventlet Removal Testimonial - Dmitry Tantsur's Ironic Experience
og_description: Learn from Dmitry Tantsur's firsthand experience migrating OpenStack Ironic away from Eventlet to threading-based solutions.
---

<div>
  <h1 class="text-4xl font-bold mb-8">Migration Testimony: Dmitry Tantsur's Experience</h1>
  
  <div class="flex mb-8 items-center">
    <img src="{{ site.baseurl }}/images/testimonials/dmitry-tantsur.jpg" alt="Dmitry Tantsur" class="w-24 h-24 rounded-full mr-6 object-cover border-2 border-cyan-400">
    <div>
      <h2 class="text-2xl font-bold">Dmitry Tantsur</h2>
      <a href="https://jay.jvf.cc/" target="_blank" class="text-cyan-400">https://www.linkedin.com/in/dtantsur/</a>
      <p class="text-xl text-gray-300">Senior Principal Software Engineer at <a href="https://redhat.com" target="_blank" class="text-cyan-400">Red Hat</a>, OpenStack Contributor.</p>
      <p class="text-gray-400 mt-1">Working on OpenStack Ironic</p>
    </div>
  </div>
  
  <div class="space-y-8">
    <h3 class="text-3xl font-bold mt-12 mb-6">Technical Context</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Which project or component did you migrate away from Eventlet?</h3>
      <p class="text-xl">
        I helped migrate Ironic with a focus on standalone applications.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How deeply was Eventlet integrated into your codebase?</h3>
      <p class="text-xl">
        Probably deeper than average. Not only did Ironic rely on green threads and monkey patching, it used to make certain performance and scaling assumptions inside of its Conductor. We also relies on the oslo.service's WSGI support for standalone applications (something that many OpenStack projects got rid of).
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What were the main technical pain points you identified early or during your migration?</h3>
      <p class="text-xl">
        Since we wanted to keep standalone API services, we needed to find a proper replacement for the WSGI server of oslo.service+eventlet. On top of that, we had to revisit the design of worker threads in the conductor. We also lost an easy path towards very high parallelism - something that we still need to recover for some of our background operations.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Which framework or alternative did you choose to replace Eventlet, and why?</h3>
      <p class="text-xl">
        The Conductor side was relatively simple: we already relied on the Futurist library, so we kept doing so, just with the threaded backend. I had to write a new thread pool implementation that was able to scale down the number of threads as the load falls.
      </p>
      <p class="text-xl">
        The API was a different story. I did a survey of established HTTP frameworks in Python and stumbled upon CheRoot - the HTTP server behind CherryPy. It was not without its own rough edges, but at least it ticked the main boxes and was an active project.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Motivation and Decision</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What motivated you to start this migration?</h3>
      <p class="text-xl">
        We have definitely faced eventlet-related bugs in the past, especially around its TLS handling. The biggest push came from the Eventlet community itself, or rather from the part of the OpenStack community that took over the project. It was clear that the clock was ticking.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Did you have any concerns or doubts before starting?</h3>
      <p class="text-xl">
        Our biggest argument was about the choice between native threads and async/await. 
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What justified your choices?</h3>
      <p class="text-xl">
        I personally advocated for native threads for a very pragmatic reason: I did not believe we had capacity to pull off a complete migration to asyncio within any reasonable timeframe. To be clear, asyncio is not off the table. In fact, we may end up migrating some parts of conductor to it in the future because of the above mentioned parallelism issues.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Migration Process</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How did the migration process go? Where did you start?</h3>
      <p class="text-xl">
        With a very large etherpad :) I created a rough plan based on my knowledge of the code. It proved very incomplete but it allowed us to start and, more importantly, allowed less experienced community members to help. We practiced on Ironic Python Agent first before moving on to Ironic.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What tools or strategies helped you the most? Were there any particularly tricky or painful parts?</h3>
      <p class="text-xl">
        The RPC bus. In addition to oslo.messaging, Ironic also supports JSON RPC and a single all-in-one process without any RPC. It is the latter architecture that caused us the most headache. Unfortunately, I'm guilty of making a pretty bad decision at some point, which we managed to revert in time for the coordinated release.
      </p>
      <p class="text-xl mt-4">
        Julia and I also spent quite some time polishing the Conductor thread pool.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Roughly how long did the migration take?</h3>
      <p class="text-xl">
        I think the core of it started after the previous PTG and finished a couple of weeks before the final 2025.2 release.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Were you able to migrate incrementally? If so, how?</h3>
      <p class="text-xl">
        Sort of. We definitely did not want to migrate the entire Ironic on one patchset. We started with separately migrating API and JSON RPC to CheRoot. Then we migrated the RPC-less process (partly breaking it in the process), which unblocked migrating the Conductor and its threads.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Outcomes and Benefits</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What concrete benefits have you seen after migrating?</h3>
      <p class="text-xl">
        I think it's too early to judge. I'm happy not to depend on the fate of eventlet though.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How did your team react to the change?</h3>
      <p class="text-xl">
        Everyone was on board. Eventlet bugs had been pretty well known before.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Lessons Learned</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What advice would you give to a team that's hesitant to migrate?</h3>
      <p class="text-xl">
        You need to collect people who really know the project and let them brainstorm on the plan. Without that, it's way too easy to make invalid assumptions or underestimate the scope. Create some sort of a scale or performance test in advance, even if a very simple one. The Metal3 project had a test that quickly creates and deletes 100 node resources, and this test saved us from a big embarrassment.
      </p>
    </div>

    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Is there anything you would do differently next time?</h3>
      <p class="text-xl">
        I would rather work on a separate branch and merge everything at once. We ended up releasing an intermediary version of Ironic in a half-migrated state, and its bugs have caused headaches in my downstream.
      </p>
    </div>

    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Have you faced blockers? If so, which?</h3>
      <p class="text-xl">
        Only shortly, when we discovered that the existing Futurist's thread pool did not match our requirements.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Would you like to share a link to a patch, repo, or documentation?</h3>
      <p class="text-xl">
        <a href="https://etherpad.opendev.org/p/ironic-eventlet-removal" target="_blank" class="text-cyan-400 hover:underline">https://etherpad.opendev.org/p/ironic-eventlet-removal</a> was our working document (probably outdated at this point), and patches can be found in project:openstack/ironic topic:eventlet-removal
      </p>
    </div>
    
  </div>
</div>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/testimonials.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>All Testimonials
    </a>
    <div><!-- No next testimonial yet --></div>
</div>
