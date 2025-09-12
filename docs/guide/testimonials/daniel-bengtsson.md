---
layout: dashboard
title: Migration Testimony - Daniel Mats Niklas Bengtsson
permalink: /guide/testimonials/daniel-bengtsson/
description: Daniel Bengtsson shares his experience migrating oslo.service from Eventlet to native threading, providing insights and practical strategies for a successful transition.
keywords: eventlet migration, openstack oslo.service, migration testimonial, eventlet removal, threading migration, practical experience
og_type: article
og_title: Eventlet Removal Testimonial - Daniel Bengtsson's oslo.service Experience
og_description: Learn from Daniel Bengtsson's firsthand experience migrating OpenStack oslo.service away from Eventlet to threading-based solutions.
---

<div>
  <h1 class="text-4xl font-bold mb-8">Migration Testimony: Daniel Bengtsson's Experience</h1>
  
  <div class="flex mb-8 items-center">
    <img src="{{ site.baseurl }}/images/testimonials/daniel-bengtsson.png" alt="Daniel Bengtsson" class="w-24 h-24 rounded-full mr-6 object-cover border-2 border-cyan-400">
    <div>
      <h2 class="text-2xl font-bold">Daniel Bengtsson</h2>
      <!--<a href="https://www.arnaudmorin.fr/" target="_blank" class="text-cyan-400">https://www.arnaudmorin.fr/</a>-->
      <p class="text-xl text-gray-300">Software Engineer at <a href="https://redhat.com/" target="_blank" class="text-cyan-400">Red Hat</a>, OpenStack Contributor</p>
      <p class="text-gray-400 mt-1">Working on OpenStack Oslo Libraries</p>
    </div>
  </div>
  
  <div class="space-y-8">
    <h3 class="text-3xl font-bold mt-12 mb-6">Technical Context</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Which project or component did you migrate away from Eventlet?</h3>
      <p class="text-xl">
        I worked on the oslo.service library, a core component of the OpenStack ecosystem.
      </p>
      <p class="mt-4 text-xl">
        It provides the foundation for managing long-running services, multi-process workers, and periodic tasks used across many OpenStack projects like Nova and Neutron.
      </p>
      <p class="mt-4 text-xl">
        Historically, oslo.service relied entirely on Eventlet for concurrency and networking — it was the only available backend.
      </p>
      <p class="mt-4 text-xl">
        I designed and implemented a new Threading backend to provide a modern, more compatible, and future-proof alternative.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How deeply was Eventlet integrated into your codebase?</h3>
      <p class="text-xl">
        Eventlet was deeply embedded in oslo.service’s architecture:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>It powered green threads and cooperative multitasking.</li>
        <li>It relied on eventlet.monkey_patch() to make standard libraries non-blocking.</li>
        <li>Core abstractions like ThreadGroup, LoopingCall, ServiceLauncher, and ProcessLauncher were all built around Eventlet’s concurrency model.</li>
      </ul>
      <p class="mt-4 text-xl">
        Because of this, replacing Eventlet required re-engineering significant internal logic while ensuring we kept the same public API for consumers.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Which framework or alternative did you choose to replace Eventlet, and why?</h3>
      <p class="text-xl">
        I chose a combination of threading, futurist, and cotyledon:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>threading: for native Python concurrency and better long-term support.</li>
        <li>futurist: provides thread pools, futures, and task management for safe async execution.</li>
        <li>cotyledon: handles process management, graceful shutdowns, and worker lifecycle.</li>
      </ul>
      <p class="mt-4 text-xl">
        The main reason for this choice was simplicity, maintainability, and forward compatibility.<br>
        Native threads are better supported in modern Python and avoid the issues caused by Eventlet’s monkey-patching and green-thread model.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Motivation and Decision</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What motivated you to start this migration?</h3>
      <p class="text-xl">
        Several factors drove this migration:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>Python 3.12+ compatibility: Eventlet had serious issues with recent Python versions.</li>
        <li>Stability: Monkey-patching caused subtle, unpredictable bugs.</li>
        <li>Performance & maintainability: A threading-based solution is easier to reason about and debug.</li>
        <li>Community alignment: Many OpenStack components are progressively moving away from Eventlet; this was the natural next step.</li>
      </ul>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Did you have any concerns or doubts before starting?</h3>
      <p class="text-xl">
        Absolutely. My main concern was backward compatibility.<br>
        Since oslo.service is used by many OpenStack services, any behavioral change could cause major breakages.<br>
        That’s why I focused on ensuring the new Threading backend behaved identically to the Eventlet backend from the perspective of external consumers.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Migration Process</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How did the migration process go? Where did you start?</h3>
      <p class="text-xl">
        I started by deep-diving into the Eventlet backend, analyzing how ProcessLauncher, ServiceLauncher, ThreadGroup, and LoopingCall interacted.
      </p>
      <p class="text-xl mt-4">
        Then, I implemented a new backend using threading, futurist, and cotyledon — while keeping the same public interface.
      </p>
      <p class="text-xl mt-4">
        Finally, I refactored the functional tests so that both Eventlet and Threading backends could be tested side by side to validate identical behavior.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What tools or strategies helped you the most?</h3>
      <ul class="text-xl list-disc list-inside">
        <li>Futurist: simplified thread pools and async task management.</li>
        <li>Cotyledon: provided robust multi-process worker handling.</li>
        <li>Functional test parity: I reused the same test suite for both backends to guarantee API compatibility.</li>
      </ul>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Were there any particularly tricky or painful parts?</h3>
      <p class="text-xl">
        Definitely:
      </p>
      <ul class="text-xl list-disc list-inside">
        <li>Removing Eventlet’s monkey-patching without breaking existing services.</li>
        <li>Handling signal propagation and worker restarts via Cotyledon.</li>
        <li>Ensuring LoopingCall and ThreadGroup behaved identically across both backends.</li>
        <li>Managing Python 3.12+ “spawn” default start method compatibility, since Eventlet depended heavily on os.fork().</li>
      </ul>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Roughly how long did the migration take?</h3>
      <p class="text-xl">
        From initial design to a fully working implementation, it took around three months, including testing, reviews, and refinements.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Were you able to migrate incrementally? If so, how?</h3>
      <p class="text-xl">
        Yes. We introduced the Threading backend as an alternative, not a replacement.
      </p>
      <p class="text-xl mt-4">
        Users can now choose between Eventlet and Threading via a configuration flag.
      </p>
      <p class="text-xl mt-4">
        This incremental approach minimized risk and allowed us to collect real-world feedback before considering Eventlet deprecation.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Outcomes and Benefits</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What concrete benefits have you seen after migrating?</h3>
      <ul class="text-xl list-disc list-inside">
        <li>Compatibility: Fully works with Python 3.12+.</li>
        <li>Stability: Fewer unexpected bugs due to removing monkey-patching.</li>
        <li>Testability: Much easier to debug and maintain.</li>
        <li>Maintainability: The codebase is simpler, modern, and easier for new contributors to understand.</li>
      </ul>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How did your team react to the change?</h3>
      <p class="text-xl">
        Feedback has been very positive.<br>
        Reviewers appreciated the incremental approach and especially the fact that we preserved API parity.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Lessons Learned</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What advice would you give to a team that's hesitant to migrate?</h3>
      <ul class="text-xl list-disc list-inside">
        <li>Start small: Break down the migration into isolated components.</li>
        <li>Preserve the API: Keeps downstream consumers safe.</li>
        <li>Write tests early: Use the same suite to validate both old and new implementations.</li>
        <li>Leverage mature libraries: futurist and cotyledon saved significant time and effort.</li>
      </ul>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Is there anything you would do differently next time?</h3>
      <p class="text-xl">
        I would prepare a migration strategy document earlier, align it with the community, and validate the plan upfront.<br>
        That would have saved some back-and-forth during reviews.
      </p>
    </div>

    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Have you faced blockers?</h3>
      <p class="text-xl">
        Yes, especially regarding Python 3.12+ and multiprocessing.set_start_method("spawn").<br>
        Cotyledon required some adjustments to manage process spawning cleanly and avoid incompatibilities.
      </p>
    </div>

    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Would you like to share a link to a patch, repo, or documentation?</h3>
      <p class="text-xl">
        Sure, here are the main Gerrit reviews for this migration:
      </p>
      <ul class="text-xl list-disc list-inside">
        <li><a href="https://review.opendev.org/c/openstack/oslo.service/+/935783" target="_blank" class="text-cyan-400">https://review.opendev.org/c/openstack/oslo.service/+/935783</a></li>
        <li><a href="https://review.opendev.org/c/openstack/oslo.service/+/937010" target="_blank" class="text-cyan-400">https://review.opendev.org/c/openstack/oslo.service/+/937010</a></li>
        <li><a href="https://review.opendev.org/c/openstack/oslo.service/+/945720" target="_blank" class="text-cyan-400">https://review.opendev.org/c/openstack/oslo.service/+/945720</a></li>
        <li><a href="https://review.opendev.org/c/openstack/oslo.service/+/951505" target="_blank" class="text-cyan-400">https://review.opendev.org/c/openstack/oslo.service/+/951505</a></li>
        <li><a href="https://review.opendev.org/c/openstack/oslo.service/+/952656" target="_blank" class="text-cyan-400">https://review.opendev.org/c/openstack/oslo.service/+/952656</a></li>
        <li><a href="https://review.opendev.org/c/openstack/oslo.service/+/940664" target="_blank" class="text-cyan-400">https://review.opendev.org/c/openstack/oslo.service/+/940664</a></li>
        <li><a href="https://review.opendev.org/c/openstack/oslo.service/+/956739" target="_blank" class="text-cyan-400">https://review.opendev.org/c/openstack/oslo.service/+/956739</a></li>
        <li><a href="https://review.opendev.org/c/openstack/oslo.service/+/955062" target="_blank" class="text-cyan-400">https://review.opendev.org/c/openstack/oslo.service/+/955062</a></li>
        <li><a href="https://review.opendev.org/c/openstack/oslo.service/+/957302" target="_blank" class="text-cyan-400">https://review.opendev.org/c/openstack/oslo.service/+/957302</a></li>
        <li><a href="https://review.opendev.org/c/openstack/oslo.service/+/956843" target="_blank" class="text-cyan-400">https://review.opendev.org/c/openstack/oslo.service/+/956843</a></li>
      </ul>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Final Thoughts</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Is there anything else you'd like to share with the community about your experience?</h3>
      <p class="text-xl">
        This migration marks an important step forward for oslo.service and the OpenStack ecosystem.<br>
        By moving away from Eventlet, we’re making the library more compatible, easier to maintain, and friendlier for new contributors.<br>
        It also opens the door for future optimizations and better long-term performance, without being tied to an outdated concurrency model.
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
