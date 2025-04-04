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

<div class="mt-10">
  <h2 class="text-3xl font-bold mb-6">Frequently Asked Questions</h2>
  
  <div class="space-y-8">
    <div class="bg-gray-800 bg-opacity-70 p-6 rounded-lg">
      <h3 class="text-2xl font-bold mb-3">Is this migration initiative an official initiative?</h3>
      <p class="text-xl">
        Yes, this initiative is supported by the OpenStack community. This initiative found its roots in <a href="https://review.opendev.org/c/openstack/governance/+/902585" class="text-cyan-400 hover:underline">an OpenStack community goal</a>.
      </p>
    </div>
    
    <div class="bg-gray-800 bg-opacity-70 p-6 rounded-lg">
      <h3 class="text-2xl font-bold mb-3">What will happen to eventlet once the openstack migration is complete?</h3>
      <p class="text-xl">
        Eventlet will be simply abandoned officially. No more maintenance will be given to the official Eventlet repository. The official Eventlet repository will be archived.
      </p>
    </div>
    
    <div class="bg-gray-800 bg-opacity-70 p-6 rounded-lg">
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
  </div>
</div>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/tutorials.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>Tutorials
    </a>
    <div><!-- No next page --></div>
</div>

