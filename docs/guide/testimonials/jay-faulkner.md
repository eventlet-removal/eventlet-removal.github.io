---
layout: dashboard
title: Migration Testimony - Jay Faulkner
permalink: /guide/testimonials/jay-faulkner/
description: Jay Faulkner shares his experience managing the migration from Eventlet at the OpenStack technical commitee level and how Ironic was migrated to native threading, providing insights and practical strategies for a successful transition.
keywords: eventlet migration, openstack ironic, migration testimonial, eventlet removal, threading migration, practical experience, openstack governance, openstack technical commitee
og_type: article
og_title: Eventlet Removal Testimonial - Jay Faulkner's Ironic and TC Experience
og_description: Learn from Jay Faulkner's firsthand experience migrating OpenStack Ironic away from Eventlet to threading-based solutions.
---

<div>
  <h1 class="text-4xl font-bold mb-8">Migration Testimony: Jay Faulkner's Experience</h1>
  
  <div class="flex mb-8 items-center">
    <img src="{{ site.baseurl }}/images/testimonials/jay-faulkner.jpg" alt="Jay Faulkner" class="w-24 h-24 rounded-full mr-6 object-cover border-2 border-cyan-400">
    <div>
      <h2 class="text-2xl font-bold">Jay Faulkner</h2>
      <a href="https://jay.jvf.cc/" target="_blank" class="text-cyan-400">https://jay.jvf.cc/</a>
      <p class="text-xl text-gray-300">Open Source Developer at <a href="https://www.gresearch.com/teams/open-source-software/" target="_blank" class="text-cyan-400">G-Research</a>, OpenStack Contributor & Former Technical Commitee Member</p>
      <p class="text-gray-400 mt-1">Working on OpenStack Ironic</p>
    </div>
  </div>
  
  <div class="space-y-8">
    <h3 class="text-3xl font-bold mt-12 mb-6">Technical Context</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Which project or component did you migrate away from Eventlet?</h3>
      <p class="text-xl">
        My primary technical work was focused on Ironic sub-projects, like Ironic Python Agent (IPA) or Networking Generic Switch (NGS), but I was involved in the migration at an OpenStack level very early. While serving as chair of the Technical Committee in November 2023, I <a href="https://lists.openstack.org/archives/list/openstack-discuss@lists.openstack.org/thread/YO5CZDVAJ6QSF734ALWSGNOQDDAIOXKI/?sort=date" target="_blank" class="text-cyan-400">emailed the list</a> about the dire state of eventlet in python 3.12. This was the kickoff to the years-long project of eventlet removal.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How deeply was Eventlet integrated into your codebase?</h3>
      <p class="text-xl">
        Ironic Python Agent (IPA) has traditionally had an interesting relationship with eventlet. One of my earliest encounters with eventlet was troubleshooting deadlocks for weeks when adding TLS support to IPA. To me, this added a real sense of the unknown: I wasn’t entirely sure how integrated it was with our codebase, because so much of it was implicit via monkey patching and use of the eventlet WSGI server. It was clear that getting rid of the use of the eventlet WSGI server would be the first step to determining how tangled the removal would be.
      </p>
      <p class="text-xl">
        We had an interesting problem though: we cannot use a traditional WSGI server (e.g. gunicorn), which expects to be run directly rather than started via a python method. The nature of IPA being run from a read-only image means it configures itself on the fly when started up. Using a token and URL from the kernel command line, it authenticates to the Ironic API which populates the remaining IPA configuration. Only at this point can the IPA API server startup; leaving us with a choice: turn this flow into a two step, two process flow, or find a WSGI server that can startup after the process.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Which framework or alternative did you choose to replace Eventlet, and why?</h3>
      <p class="text-xl">
        While we were trying to figure out how to solve the WSGI server problem, Dmitry Tantsur from Red Hat — a longtime Ironic and Metal3 contributor — found cheroot. As the basic WSGI library that powers cherrypy, it was already designed to be imported and started up directly from python. This is exactly what we needed!
      </p>
      <p class="text-xl">
        Once the eventlet WSGI dependency was removed from IPA, it became clear how integrated eventlet was with IPA: not very much at all. The only remaining tasks were replacing a few eventlet.sleep() invocations with time.sleep(). With these changes, we were able to <a href="https://opendev.org/openstack/ironic-python-agent/commit/c03021fee25f47ccd2c04e0d91341cd829c9e600" target="_blank" class="text-cyan-400">complete the removal of eventlet from IPA</a>.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Motivation and Decision</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What motivated you to start this migration?</h3>
      <p class="text-xl">
        I've been in the OpenStack community for a while, and often we struggle finding enough resources to perform the important, but less visible work in the community such as security, release management, or QA. The eventlet migration — a large undertaking to remove technical debt — is an example of this type of less-visible work. I was highly motivated to kick the community into gear and help Ironic be an example of migration in the hope that it would help others understand the importance of this migration.
      </p>
      <p class="text-xl">
        Additionally, we promise our operators eighteen months of support. As someone who started out in operations, I know people rely on those promises. I was extremely concerned we'd have a major bug (or worse; security issue), involving eventlet, that we might be unable to fix due to that project being unmaintained at the time.
      </p>
      <p class="text-xl">
        I am extremely grateful for the community stepping up when the call for action went out initially and for those — like <a href="https://herve.beraud.io/" target="_blank" class="text-cyan-400">Hervé Beraud</a> and <a href="{{ site.baseurl }}{% link about.md %}" target="_blank" class="text-cyan-400"> others building this website</a> — are helping to keep a spotlight on this important work as we drive it to completion.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Did you have any concerns or doubts before starting?</h3>
      <p class="text-xl">
        Moving a large community like OpenStack — literally hundreds of contributors spread across the world — in the same direction is incredibly difficult. Getting them to a consensus on something technical, like an eventlet migration, is even tougher. When this started, I didn't see how it would end — or if it would end. Looking back to November 2023 when I originally wrote that email, if you told me we'd have Ironic migrated and many other projects well on the way by the 2025.2 release, I would've been thrilled.
      </p>
      <p class="text-xl">
        For me personally, I found the project very intimidating at first. I learned how to write python working on OpenStack and have written more python code running in context with monkey patched eventlet than without. In general, approaching the questions around threading models, asyncio, and how to safely extricate eventlet at a technical level were difficult – other members of the Ironic community breaking it down into digestible pieces helped me significantly with this.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Migration Process</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How did the migration process go? Where did you start?</h3>
      <p class="text-xl">
        Like I said above, this started for me before the technical migration started — in my role as Technical Committee Chair I documented the alarming state of eventlet on the mailing list in November, 2023. While I'm sure the issues were not news to anyone involved with the project, it was valuable to collate the various issues and state the business case around migrating from eventlet. OpenStack really had no choice — as has been well-documented already, changes to core python functionality meant that model was not going to last much longer and the project was in peril.
      </p>
      <p class="text-xl mt-4">
        It takes more than just words though; as a member of the G-Research Open Source Software (GR-OSS) team, I am lucky to have collaborators whose expertise in this area exceeds my own — in this case, we asked <a href="https://pythonspeed.com/about/" target="_blank" class="text-cyan-400">Itamar Turner-Trauring</a> to take a look at the eventlet codebase. He, along with contributors like <a href="https://herve.beraud.io/" target="_blank" class="text-cyan-400">Herve Beraud</a> at Red Hat, worked together to bring the library to a functional state — buying OpenStack (and other eventlet using projects) the badly needed time to migrate.
      </p>
      <p class="text-xl mt-4">
        After working to shore up the status quo and spur the community into action, the hard technical work of migration began. Myself, CID, and Adam MacArthur from GR-OSS started digging into Ironic Python Agent, the small daemon in the Bare Metal project which runs on machines being actively provisioned or cleaned with the Ironic direct driver. It was our hope that trying to migrate this early would shine lights on potential problem points and help supply information for the migration of Ironic; it did this successfully, as the cheroot library piloted as IPA’s replacement WSGI server is also in use in Ironic.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What tools or strategies helped you the most?</h3>
      <p class="text-xl">
        Trusting CI. There were a lot of times, such as during the migration for NGS, where we removed explicit eventlet references and the monkey patch and it just worked. It required having faith that the CI would find issues and knowing our tests well enough to identify if a failure was environmental or some new issue.
      </p>
      <p class="text-xl mt-4">
        Trusting the community and finding your role. My background is in operations and Linux systems and I'm less experienced than many others Ironic contributors in the realm that eventlet code lives in. I tried to find spots to help where I could, and asked the community for help where I couldn't. In fact, during the most technically intensive portion of the Ironic migration I was on a sabbatical. It was very refreshing to return and see how much progress had been made.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Were there any particularly tricky or painful parts?</h3>
      <p class="text-xl">
        With Ironic — and even NGS — we often write and maintain code that interacts with hardware we don't possess or have a method to test. Even at this writing, a part of me is a little nervous about when Ironic's 2025.2 release gets used in the real world.
      </p>
      <p class="text-xl mt-4">
        Another tricky part is still ongoing, even though the migration is code complete, which is fully understanding the change in performance. Ironic has multiple different use cases and operating modes; while we have a lot of confidence we've sped things up, the shape of the performance has changed. I anticipate we'll continue working through bottlenecks and improving on our post-eventlet architecture in the coming months and years.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Roughly how long did the migration take?</h3>
      <p class="text-xl">
        Experiments around removing eventlet from IPA started as early as November 2024, however the bulk of the Ironic-specific eventlet removal work occurred during the OpenStack 2025.2 release cycle. It's important to note that the long tail of prework — early research identifying potential issues, the oslo libraries updating to support being used without eventlet, and the updates to eventlet itself all were part of enabling Ironic to get it wrapped up so quickly.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Were you able to migrate incrementally? If so, how?</h3>
      <p class="text-xl">
        Incrementally can mean many things. Ironic migrated in multiple commits, with CI passing between the commits — but realistically, I wouldn't want to run on any of those incremental commits. Ironic decided that the best thing for our operators was to migrate entirely on a release boundary — users of 2025.1 will use eventlet; 2025.2 will not. This allows us to have one performance model to worry about -- if an operator gets a performance regression, we want to fix it and roll forward.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Outcomes and Benefits</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What concrete benefits have you seen after migrating?</h3>
      <p class="text-xl">
        <a href="{{ site.baseurl }}{% link guide/testimonials/julia-kreger.md %}" target="_blank" class="text-cyan-400">Julia</a> covered this well in her blogpost on <a href="https://ironicbaremetal.org/blog/coming-soon-threading/" target="_blank" class="text-cyan-400">ironicbaremetal.org</a> — the shape of the performance, particularly around memory usage and memory usage reporting changed significantly. Our post-migration testing showed an approximately 10% speed boost in some internal benchmarks, but I expect that some operators may be able to tune for their environment and get even more of a boost. I expect that over time, we may see this improve further as we learn more about the performance of a post-eventlet Ironic.
      </p>
      <p class="text-xl">
        Primarily for me the concrete benefit is that we've disarmed the ticking time bomb. OpenStack Ironic will continue to work even if the next version of python completely breaks eventlet. This was an existential threat to OpenStack as we know it, and it's nice to know we're going to overcome it as a community.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">How did your team react to the change?</h3>
      <p class="text-xl">
        I don't know how everyone felt, but I know I observed something that I think speaks volumes: we had a large amount of community participation in this project. Multiple Ironic contributors across multiple companies all were able to contribute something to get it done. Usually you only get reviews or engagement from a portion of the community interested in whatever the feature is; it was very nice to see all corners of the Ironic community rally around the work needed and accomplish it.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Lessons Learned</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">What advice would you give to a team that's hesitant to migrate?</h3>
      <p class="text-xl">
        If you need advice on how to start your migration when reading this, you're already starting too late. This is not the kind of thing that can be done at the last minute. Make a list of what you need to do and start on the first item right away. The window for migrating off eventlet while it’s still working is closing rapidly.
      </p>
    </div>

    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Is there anything you would do differently next time?</h3>
      <p class="text-xl">
        Start a year earlier, maybe more – we started too late. The state of eventlet had been dubious for a while, and I think as a TC member before that I could have done more to raise awareness and possibly get us started earlier. As it is now, we're looking at some OpenStack projects having to continue to support code running on eventlet through 2028 — even Ironic's last eventlet-dependent release, 2025.1, will be supported until mid-2026, so even Ironic is not yet out of the woods.
      </p>
    </div>

    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Have you faced blockers? If so, which?</h3>
      <p class="text-xl">
        There were some technical problems during the Ironic migration which were worked out, but once the oslo libraries supported a threading-based backend, we were mostly free to migrate Ironic.
      </p>
    </div>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Would you like to share a link to a patch, repo, or documentation?</h3>
      <p class="text-xl">
        I have a few videos about OpenStack, as well as episodes of the GR-OSS OUT podcast at <a href="https://youtube.com/@oss-gr" class="text-cyan-400 hover:underline">https://youtube.com/@oss-gr</a>. Keep an eye there for content about a post-eventlet Ironic.
      </p>
    </div>
    
    <h3 class="text-3xl font-bold mt-12 mb-6">Final Thoughts</h3>
    
    <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
      <h3 class="text-2xl font-bold mb-3">Is there anything else you'd like to share with the community about your experience?</h3>
      <p class="text-xl">
In my <a href="https://podcast.gr-oss.io" class="text-cyan-400 hover:underline">podcast</a>, we talk about "gross" moments in open source, where something happens that's unpleasant and you have to work through it. This really was one for the entire OpenStack ecosystem. Rallying around taking working code using one model and refactoring it to another model and hoping nothing breaks is extremely intimidating — and that's before thinking about the sheer scope of OpenStack: hundreds of individual services that all needed this done. In November 2023, I was afraid this might be impossible. Now we can see the finish line — and the community and software are better for it.
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
