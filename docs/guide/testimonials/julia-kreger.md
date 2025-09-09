---
layout: dashboard
title: Migration Testimony - Julia Kreger
permalink: /guide/testimonials/julia-kreger/
description: Julia Kreger shares her experience migrating Ironic from Eventlet to native threading, providing insights and practical strategies for a successful transition.
keywords: eventlet migration, openstack mistral, migration testimonial, eventlet removal, threading migration, practical experience
og_type: article
og_title: Eventlet Removal Testimonial - Julia Kreger's Ironic Experience
og_description: Learn from Julia Kreger's firsthand experience migrating OpenStack Ironic away from Eventlet to threading-based solutions.
---

<div>
  <h1 class="text-4xl font-bold mb-8">Migration Testimony: Julia Kreger's Experience</h1>
  
  <div class="flex mb-8 items-center">
    <img src="{{ site.baseurl }}/images/testimonials/julia-kreger.jpg" alt="Julia Kreger" class="w-24 h-24 rounded-full mr-6 object-cover border-2 border-cyan-400">
    <div>
      <h2 class="text-2xl font-bold">Julia Kreger</h2>
      <a href="https://www.linkedin.com/in/juliaashleykreger/" target="_blank" class="text-cyan-400">https://www.linkedin.com/in/juliaashleykreger/</a>
      <p class="text-xl text-gray-300">Senior Principal Software Engineer at <a href="https://redhat.com/" target="_blank" class="text-cyan-400">Red Hat</a>, Chair of the OpenStack Governing Board</p>
      <p class="text-gray-400 mt-1">Working on OpenStack Ironic</p>
    </div>
  </div>
  
  <div class="space-y-8">
    <h3 class="text-3xl font-bold mt-12 mb-6">Technical Context</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Which project or component did you migrate away from Eventlet?</h3>
      <p class="text-xl">
        Ironic
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How deeply was Eventlet integrated into your codebase?</h3>
      <p class="text-xl">
        Ironic's codebase made heavy use of green-threads and the monkey patching provided by eventlet. Coupled with the WSGI server provided, it made a lot of sense for the Ironic project as a model to feel like we were making it simpler. In reality, we have learned Eventlet brought us more complexity, but being on the journey and learning as we go is the critical aspect.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Which framework or alternative did you choose to replace Eventlet, and why?</h3>
      <p class="text-xl">
        Ironic is a service which can be deployed in numerous use cases. As a result, we have a number of processes, some which also offer an operating model to enable Ironic's use without a RPC Message Bus like many other services which came from OpenStack. This drove us towards using <a href="https://cheroot.cherrypy.dev/en/latest/pkg/cheroot.wsgi/" target="_blank" class="text-cyan-400">cheroot</a> which was an item of community consensus. Cheroot is by no means perfect, but it allowed us to move our JSON-RPC and Restful API endpoints without much heartache.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Motivation and Decision</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What motivated you to start this migration?</h3>
      <p class="text-xl">
        Ironic chose to perform this migration largely because of the mounting evidence of issues creeping in from Eventlet. Ironic has always been very unit test heavy, and we were finding that we were seeing more and more "weird" test failures which we could begin to tell were rooted in Eventlet's operation and monkey-patching of the test and ultimately service code as it related to the Python version in use.
      </p>
      <p class="mt-4 text-xl">
        <a href="https://www.gresearch.com/teams/open-source-software/" target="_blank" class="text-cyan-400">G-Research Open Source Software</a> was able to lend some additional Python expertise to Ironic to help us reach an understanding of what was going on and that actually ended up in the initial call which started the groundswell to migrate off Eventlet in the OpenStack context. 
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Did you have any concerns or doubts before starting?</h3>
      <p class="text-xl">
        I don't think any of us had any real concerns or doubts. Obviously, before starting any major migration in a community which strives for perfection, we had to recognize and find the balance which would work for us as a group of contributors. The biggest concerns we had were rooted largely around the core of Ironic which is our conductor service because it performs the heaviest workload. It was extremely reliant upon green-threads, and we had to find that right balance. Part of that balance was gaining an understanding, then building consensus around the possible problems or risk areas which would require work. One contributor did that and then others reviewed, commented, and tried to understand their perception. In this process we were able to gain a better mutual understanding, but you can only do so much of that before you really need to experiment and begin to eliminate possible issues and begin experimenting. Once we started experimenting, the speed of our progress grew dramatically.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Migration Process</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How did the migration process go? Where did you start?</h3>
      <p class="text-xl">
        So, I'll first stress this was a team effort. CID started by working through the list of areas raised by Dmitry for smaller isolated possible areas. I and others started chipping away at aspects like tests which helped model the eventlet behavior.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What tools or strategies helped you the most?</h3>
      <p class="text-xl">
        Etherpad was most likely the most critical tool that we used to help move this effort along.
      </p>
      <p class="mt-4 text-xl">
        Combined with starting small, iterating, and getting to a point where we could load test with some fake data really helped us gain velocity as we went.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Were there any particularly tricky or painful parts?</h3>
      <p class="text-xl">
        There were two painful and tricky parts. Perhaps three as this morning we just had reports of a breakage but it was easy to fix.
      </p>
      <p class="mt-4 text-xl">
        Two of the issues were largely rooted around the final step to remove Eventlet and launch Ironic without Eventlet being loaded. Turned out oslo.service introduced some changes in the process launch model, new sub-process via python multiprocessing by default instead of inside a single process. This cascaded to our custom shutdown code because this change in oslo.service introduced a change of behavior from eventlet which forced us to retool some of our shutdown handling. Like any project with a variety of use cases and customer demands, we carry some additional features and logic for shutting down our ironic-conductor service. Once we understood what we needed to do, it was fairly straightforward.
      </p>
      <p class="mt-4 text-xl">
        The third issue was our TaskManager and the resulting worker threads... and the resulting memory impacts.
      </p>
      <p class="mt-4 text-xl">
        We knew due to our internal structural model of interaction that we had to maintain the model of executing any task or user requested work item on its own thread, while also multiplexing our existing maintenance work across threads as well. We knew the basic pattern to expect. These threads were going to be IO bound. Secondly we knew that as we increased the number of threads, our memory footprint would swell. This quickly led us to a pattern of setting up several simple tests with mock data and beginning to measure the memory impact. We quickly realized we were going to have a huge issue. Many of Ironic's users are systems operators where they are improving their quality of life through Ironic. We are also highly scalable and tunable and generally recognized at a certain scale operators may need to further tune aspects. The end result, we have many configuration settings which ship with reasonable defaults but that could be tuned to support operators. And often, those operators don't actually understand how the knobs can impact their performance. For example, we've had operators come in and say "I set it to a thousand threads and each timeout to like 10 minutes" and then complain of issues. It becomes a teaching opportunity for the maintainers of Ironic. So when we launched Ironic with four hundred thread threads in a simulated load test and could trigger the host's OOM-Killer process, we knew we had to address the thread worker model. Luckily, Dmitry Tantsur was up for that challenge. This resulted in a few weeks of collaboration, testing. Ironic now has what might be a better "rejector" function than what the futurist library carries, but time will tell.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Roughly how long did the migration take?</h3>
      <p class="text-xl">
        Discussion really getting started and being executed on to completion felt about a four or five month process. Ironic did have to take a little more time to address some of its various use cases and ensure they were going to work as we expected. Which led to additional work and some undoing of some of the work as we were able to measure performance in some different cases. Ultimately, leading to a more performant and capable service in the end. 
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Were you able to migrate incrementally? If so, how?</h3>
      <p class="text-xl">
        Ironic started with the low-hanging-fruit approach. Like unit tests, explicit invocations of eventlet. Specific threading invocation. This then shifted to some testing, updating our code around process launch and shutdown, and then ultimately addressing the more specific use cases around Ironic like those leveraged by Metal3, in which Ironic is embedded.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Outcomes and Benefits</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What concrete benefits have you seen after migrating?</h3>
      <p class="text-xl">
        First off, our unit tests now seem to be less "flaky". It was uncommon before, but I don't think I've seen a spurious failure of our unit tests since we've migrated.
      </p>
      <p class="mt-4 text-xl">
        Secondly, we've seen some dramatic performance improvements, but not only that we were able to identify some additional areas where performance was degraded and we just didn't really "see" it before. For example, Metal3 had a test which with Eventlet took about a minute and twenty seconds. That runtime spiked up beyond two minutes with some changes, and we were able to see some of the root causes and get that down somewhere in the neighborhood of six seconds at last report.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How did your team react to the change?</h3>
      <p class="text-xl">
        This was really a collaborative effort amongst CID, Jay Faulkner, Dmitry Tantsur, and myself for the core of the Ironic service. Ironic also has neutron plugins and some other components, and some other code and others jumped in to help. Once everyone understood the reasons and the impacts, it just made sense and we worked through it one step at a time. We did also decide not to migrate one service in our project scope, but that service was previously deprecated. 
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Lessons Learned</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What advice would you give to a team that's hesitant to migrate?</h3>
      <p class="text-xl">
        Start small. Iterate. Try to move with purpose. Embrace the change, and don't let fear or uncertainty or fear slow you down. Just take one step at a time with a basic plan. Expect the plan to need to change. It will be okay.
      </p>
    </div>

    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Is there anything you would do differently next time?</h3>
      <p class="text-xl">
        I think it is too early to speak to this for the Ironic project. We could have moved slower, but it would have just elongated the process and made the effort harder.
      </p>
    </div>

    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Have you faced blockers? If so, which?</h3>
      <p class="text-xl">
        Our biggest blocker was in the futurist library when we realized that we needed to balance our worker threads and memory usage. Through collaboration, we were able to make progress.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Would you like to share a link to a patch, repo, or documentation?</h3>
      <p class="text-xl">
        We ended up drafting a blog post for Ironic's blog as a result of this work. In part to bring awareness to the impact, tunables, and areas operators might want to take a look at. This was critical for us as Ironic's use base is spread from "embedded" use cases to highly scaled environments. This seemed like the right balance to take in the context of the effort. <a href="https://ironicbaremetal.org/blog/coming-soon-threading/" target="_blank" class="text-cyan-400">https://ironicbaremetal.org/blog/coming-soon-threading/</a>.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Final Thoughts</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Is there anything else you'd like to share with the community about your experience?</h3>
      <p class="text-xl">
        Embrace change. Move forward. The grass is definitely "greener" on the side of real threads.
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
