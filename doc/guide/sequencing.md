---
layout: dashboard
title: Sequencing the Migration
permalink: /guide/sequencing-the-migration/
id: sequencing
---
<section>
    <h2 class="mt-10 text-4xl font-bold mb-6">Sequencing the Migration</h2>
    <p class="mt-10 text-xl">The Gantt diagram below aim to give you an overview of how the removal of Eventlet from your deliverable could looks if started at 2025 March 19th.</p>
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
    <h2 class="mt-10 text-4xl font-bold mb-6">How Is Sequenced the Global Retirement Of Eventlet</h2>
    <p class="mt-10 text-xl">The Gantt diagram below aim to give you an overview of the global retirement plan of Eventlet and what support and next steps you should expect.</p>
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
            
            section Alternatives
            Implementing Support for Python 3.12       :alt1, after prep2, 10w
            Implementing Support for AsyncIO       :alt2, after alt1, 15w
            Make decision              :milestone, after alt2, 0d
            
            section Organization
            Isolate parts              :org1, after alt2, 10d
            Services first             :org2, after org1, 3d
            
            section Migration
            Implement deprecation      :mig1, after org2, 14d
            Migrate executors          :mig2, after mig1, 21d
            Migrate monkey patching    :mig3, after mig2, 28d
            
            section Validation
            Regression testing         :val1, after mig3, 14d
            Performance tuning         :val2, after val1, 7d
            Abandon of the maintenance         :val3, 2027-04-01, 1d
            Archiving the official repository         :val4, after val3, 1d
        </div>
</section>