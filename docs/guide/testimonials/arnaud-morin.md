---
layout: dashboard
title: Migration Testimony - Arnaud Morin
permalink: /testimonials/arnaud-morin/
description: Arnaud Morin shares his experience migrating Mistral from Eventlet to native threading, providing insights and practical strategies for a successful transition.
keywords: eventlet migration, openstack mistral, migration testimonial, eventlet removal, threading migration, practical experience
og_type: article
og_title: Eventlet Removal Testimonial - Arnaud Morin's Mistral Experience
og_description: Learn from Arnaud Morin's firsthand experience migrating OpenStack Mistral away from Eventlet to threading-based solutions.
---

<div>
  <h1 class="text-4xl font-bold mb-8">Migration Testimony: Arnaud Morin's Experience</h1>
  
  <div class="flex mb-8 items-center">
    <img src="{{ site.baseurl }}/images/testimonials/arnaud-morin.jpg" alt="Arnaud Morin" class="w-24 h-24 rounded-full mr-6 object-cover border-2 border-cyan-400">
    <div>
      <h2 class="text-2xl font-bold">Arnaud Morin</h2>
      <a href="https://www.arnaudmorin.fr/" target="_blank" class="text-cyan-400">https://www.arnaudmorin.fr/</a>
      <p class="text-xl text-gray-300">OpenStack Contributor</p>
      <p class="text-gray-400 mt-1">Working on OpenStack Mistral</p>
    </div>
  </div>
  
  <div class="space-y-8">
    <h3 class="text-3xl font-bold mt-12 mb-6">Technical Context</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Which project or component did you migrate away from Eventlet?</h3>
      <p class="text-xl">
        I migrated Mistral and related mistral projects (like mistral-lib) away from Eventlet. Note that this is not yet entirely finished, so the code is still relying on eventlet on Epoxy release, but I plan to evacuate this during the F cycle.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How deeply was Eventlet integrated into your codebase?</h3>
      <p class="text-xl">
        Mistral was relying a lot on eventlet greenthreads for multiple functionalities:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>Thread pools: Mistral uses threads to spawn tasks for workflows, tasks, queues and other concurrency operations</li>
        <li>Monkey patching: Mistral is monkey patching all eventlet resources (threads, queues, etc.), even for unit tests</li>
        <li>Mistral is also relying on oslo projects that use eventlet, like oslo-messaging and oslo-service</li>
      </ul>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Which framework or alternative did you choose to replace Eventlet, and why?</h3>
      <p class="text-xl">
        From a code perspective, I wanted to change the minimum things, so I discarded the async alternative and took a look at native threading/multi-processing from Python. Most of the time, classes and objects are pretty similar and may be replaced with minimum code change, like eventlet.sleep by time.sleep. It's also true with other classes like Thread pool executors.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Motivation and Decision</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What motivated you to start this migration?</h3>
      <p class="text-xl">
        My team is using mistral in production, so the first thing that motivated me was to have a clean mistral without eventlet. I also had very bad experiences with eventlet in the past in neutron/oslo.messaging and I always wanted to get rid of this, so that was my way of moving this out! And finally, I was willing to learn more about how eventlet/concurrency is done in mistral, so that was a perfect occasion to dig into code.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Did you have any concerns or doubts before starting?</h3>
      <p class="text-xl">
        My main concern was: is it even possible to get rid of eventlet without rewriting completely the framework around it? Also, I wasn't sure about where to start, but I was very happy to listen/watch what other projects were doing. From the beginning, I wasn't sure that I would be able to completely get rid of it, but I was willing to do simple tasks, collect where eventlet is used.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Migration Process</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How did the migration process go? Where did you start?</h3>
      <p class="text-xl">
        I started by taking a deep breath and read the <a href="https://wiki.openstack.org/wiki/Eventlet-removal" target="_blank" class="text-cyan-400">wiki about eventlet removal</a>. That was very useful to me to identify what solutions were possible to replace eventlet code with other python code.
      </p>
      <p class="text-xl mt-4">
        I then did very small/simple tasks, the first one was to replace all eventlet.sleep references by time.sleep.
      </p>
      <p class="text-xl mt-4">
        The second thing I did was to reference where eventlet was used, and try to identify similar patterns that could be replaced by similar solutions.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What tools or strategies helped you the most?</h3>
      <p class="text-xl">
        My strategy was the following:
      </p>
      <ul class="mt-4 text-xl list-disc list-inside">
        <li>Make sure the CI is green before my changes (unit/devstack tests)</li>
        <li>Make sure I have a mistral running somewhere with "master" branch code</li>
        <li>Try to replace one occurrence of eventlet, then run unit tests (they are pretty quick, so it can be done easily from my dev env)</li>
        <li>If that works, push the change to gerrit and wait for devstack results</li>
        <li>In parallel, start pushing my change on my mistral on "master" and execute basic tests to make sure everything is still ok</li>
      </ul>
      <p class="text-xl mt-4">
        Repeat this process as many times as possible!
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Were there any particularly tricky or painful parts?</h3>
      <p class="text-xl">
        I had mostly two kinds of issues:
      </p>
      <ol class="mt-4 text-xl list-decimal list-inside">
        <li class="mt-2">
          <strong>Unit tests failing because of the way the unit test is started/written</strong> - It's not always easy to identify that it's not your change in code which is wrong, but the way the unit test was calling it. When that occurred, the fix was usually pretty easy: rewrite/refactor the test to fit the new code.
        </li>
        <li class="mt-4">
          <strong>Eventlet green threads execution order is not the same as native threads</strong> - Sometimes it's a little bit trickier because eventlet green threads and python threads are not executed in the same order. When a test relies on an assertion that may occur later if replaced by a native thread, then it's pretty hard to identify. If, by luck it works from time to time, it's even harder to debug.
        </li>
      </ol>
      <p class="text-xl mt-4">
        One other thing that comes to mind is that changing a very small piece of code is sometimes not enough and needs more work to be done to make sure the code works as expected, but then you start refactoring too much code. By chance, I did not encounter this situation very often in mistral.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Roughly how long did the migration take?</h3>
      <p class="text-xl">
        That's hard to say, but I would say at least one full cycle. And it's not yet finished, but most of the big work has been done.
      </p>
      <p class="text-xl mt-4">
        I spent multiple hours/days on this, but I was not focused on it 100% of my time. I tried to focus on removing one eventlet call per week and spent something like 2-3 hours each time to make sure everything works fine.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Were you able to migrate incrementally? If so, how?</h3>
      <p class="text-xl">
        Yes, I love the gerrit workflow for this, I can split my work by commits/patchsets which are then tested incrementally in the CI. Mistral CI is pretty consistent/reliable, which helped identifying where my changes were failing.
      </p>
      <p class="text-xl mt-4">
        My patchsets were usually working on a small subset of eventlet removal. Sometimes, where the pattern is similar in multiple places, I did the code change in only one patchset (e.g. eventlet.sleep removal was done only in one patchset per repo).
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Outcomes and Benefits</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What concrete benefits have you seen after migrating?</h3>
      <p class="text-xl">
        The main benefits are for maintainability and testability. Without eventlet, it's much easier to understand what is going on behind the scene. Debugging sessions with eventlet has always been painful, this is not the case anymore!
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How did your team react to the change?</h3>
      <p class="text-xl">
        Mistral team is pretty small, and so far, everyone is happy to see that happening.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Lessons Learned</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What advice would you give to a team that's hesitant to migrate?</h3>
      <p class="text-xl">
        Eventlet is deprecated, complex to maintain and leads to very weird behavior. While migrating out of it may sound like a hard thing to do, the benefits in maintainability and testability are huge!
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Would you like to share a link to a patch, repo, or documentation?</h3>
      <p class="text-xl">
        <a href="https://review.opendev.org/q/topic:%22eventlet-removal%22+project:openstack/mistral" class="text-cyan-400 hover:underline">Mistral Eventlet Removal Patches</a>
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Final Thoughts</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Is there anything else you'd like to share with the community about your experience?</h3>
      <p class="text-xl">
        One thing I did before starting was to clean the CI so it's reliable and produces reproducible tests. You also want to have more than a devstack environment to make sure the system won't be broken under real conditions.
      </p>
    </div>
  </div>
</div>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/testimonials/index.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>All Testimonials
    </a>
    <div><!-- No next testimonial yet --></div>
</div>
