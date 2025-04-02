---
layout: dashboard
title: Sequencing the Migration
permalink: /guide/sequencing-the-migration/
id: sequencing
description: Learn how to properly sequence your Eventlet migration project with timelines and Gantt charts. This guide covers planning your transition, aligning with Eventlet's retirement timeline, and the OpenStack community's migration approach.
keywords: eventlet migration timeline, migration planning, project sequencing, gantt chart, openstack migration, eventlet retirement, phased migration
og_type: article
og_title: Sequencing Your Eventlet Migration Project - Timeline Planning Guide
og_description: Create an effective timeline for your Eventlet migration project with Gantt charts and practical milestone planning based on the OpenStack community's approach.
---
<section>
    <h1 class="text-4xl font-bold mb-6">Sequencing the Migration</h1>
    <p class="mt-10 text-xl">This chapter aims to give you an overview of the various timeline of the migration. From your point of view. From the life cycle of Eventlet point of view and its integration with versions of CPython. And to finish from the point of view of the migration of OpenStack.</p>

    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#your-migration" class="text-cyan-400 hover:underline">Your Migration</a></li>
                <li><a href="#eventlet-retirement" class="text-cyan-400 hover:underline">How Is Sequenced the Global Retirement Of Eventlet</a></li>
                <li><a href="#openstack-migration" class="text-cyan-400 hover:underline">How Is Sequenced the abandon of Eventlet in OpenStack</a></li>
            </ul>
        </div>
    </div>
    <h2 id="your-migration" class="mt-10 text-3xl font-bold mb-6">Your Migration <a href="#your-migration" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">The Gantt diagram below aims to give you an overview of how the removal of Eventlet from your deliverable could looks if started at 2025 March 19th.</p>
    <div class="mt-10 mermaid">
        %%{init: {'theme': 'dark', 'gantt': {'titleColor': '#ffffff', 'sectionColor': '#ffffff', 'taskColor': '#ffffff', 'taskTextColor': '#ffffff'}}}%%
        gantt
            title Hypothetical Eventlet Migration Plan For Your Deliverables
            dateFormat YYYY-MM-DD
            axisFormat %d/%m/%Y
            todayMarker stroke:red
            
            section Preparation
            Locate usages              :prep1, 2025-03-19, 7d
            Audit code                 :prep2, after prep1, 14d
            Check disablable           :prep3, after prep2, 3d
            
            section Alternatives
            Assess app structure       :choice1, after prep3, 5d
            Evaluate integration       :choice2, after choice1, 5d
            Make decision              :milestone, after choice2, 0d
            
            section Organization
            Isolate parts              :org1, after choice2, 10d
            Services first             :org2, after org1, 3d
            
            section Migration
            Implement deprecation      :mig1, after org2, 14d
            Migrate executors          :mig2, after mig1, 21d
            Migrate monkey patching    :mig3, after mig2, 28d
            
            section Validation
            Regression testing         :val1, after mig3, 14d
            Gradual deployment         :val2, after val1, 14d
            Removing requirements      :val3, after val2, 2d
    </div>
    <h2 id="eventlet-retirement" class="mt-10 text-3xl font-bold mb-6">How Is Sequenced the Global Retirement Of Eventlet <a href="#eventlet-retirement" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">The Gantt diagram below aims to give you an overview of the global retirement plan of Eventlet and what support and next steps you should expect.</p>
    <p class="mt-10 text-xl mt-4 text-xl text-yellow-300"><strong>Support of Eventlet will end by April 2027. Past this date if your migration is not finished, you will have to pin your requirements and you will have to stick to the CPython versions supported by the version of Eventlet you are using. We strongly recommend you to not pass over this date.</strong></p>
    <div class="mt-10 mermaid">
        %%{init: {'theme': 'dark', 'gantt': {'titleColor': '#ffffff', 'sectionColor': '#ffffff', 'taskColor': '#ffffff', 'taskTextColor': '#ffffff'}}}%%

        gantt
            title Eventlet's Retirement Plan
            dateFormat YYYY-MM-DD
            axisFormat %d/%m/%Y
            todayMarker stroke:red
            
            section Preparation
            Propose Eventlet's retirement           :prep1, 2023-12-08, 4w
            Discuss about the terms of retirement   :prep2, 2024-01-01, 14d
            Document the migration                  :prep3, after prep2, 52w
            Maintenance mode                        :prep4, after prep2, 166w
            Final retirement                        :prep5, after prep4, 4w
            
            section Compatibility
            Implementing Support for Python 3.12    :alt1, after prep2, 10w
            Implementing Support for AsyncIO        :alt2, after alt1, 20w
            Implementing Support for Python 3.13    :alt3, after alt2, 15w
            Implementing Support for Python 3.14    :alt4, after alt3, 45w
            Remove Support for Python 3.9           :alt5, after alt4, 4w
            Implementing Support for Python 3.15    :alt6, after alt5, 45w
            Remove Support for Python 3.10          :alt7, after alt6, 4w
            
            section Support
            Stop supporting new features            :sta1, 2024-02-01, 164w
            Deprecating not AsyncIO hubs            :sta2, 2025-04-01, 54w
            Stop supporting not AsyncIO hubs        :sta3, 2026-04-01, 51w
            Stop fixing bugs                        :sta4, 2027-01-01, 12w
        </div>
    <h2 id="openstack-migration" class="mt-10 text-3xl font-bold mb-6">How Is Sequenced the abandon of Eventlet in OpenStack <a href="#openstack-migration" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">The Gantt diagram below aims to give you an overview of the Eventlet removal plan OpenStack and what support and next steps you should expect.</p>
    <div class="mt-10 mermaid">
        %%{init: {'theme': 'dark', 'gantt': {'titleColor': '#ffffff', 'sectionColor': '#ffffff', 'taskColor': '#ffffff', 'taskTextColor': '#ffffff'}}}%%

        gantt
            title OpenStack's Migration Plan
            dateFormat YYYY-MM-DD
            axisFormat %d/%m/%Y
            todayMarker stroke:red
            
            section Migration
            Discuss the Eventlet problem            :prep1, 2023-11-20, 2w
            Propose a community goal                :prep2, after prep1, 34w
            Select the community goal               :prep3, after prep2, 15w
            Start officially the migration          :prep4, after prep3, 2w
            Prepare Oslo (deprecate/ new drivers)   :prep5, after prep4, 24w
            Accelerate the migration of services    :prep6, 2025-04-04, 52w
            Remove deprecated parts of services     :prep7, 2026-04-01, 27w
            Cleans up Eventlet from services        :prep8, 2026-10-01, 12w
            Remove deprecated parts of libraries    :prep9, after prep8, 12w
            Drop Eventlet from requirements         :prep10, after prep9, 2w
        </div>
    <div class="mt-10 flex justify-between">
        <a href="{{ site.baseurl }}{% link guide/preparing.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
            <i class="fas fa-arrow-left mr-2"></i>Preparing for Migration
        </a>
        <a href="{{ site.baseurl }}{% link guide/asyncio.md %}" class="inline-block bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
            Migrating to AsyncIO<i class="fas fa-arrow-right ml-2"></i>
        </a>
    </div>
</section>