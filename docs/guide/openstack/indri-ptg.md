---
layout: dashboard
title: OpenStack Indri PTG - Eventlet Removal Retrospective
permalink: /guide/openstack/indri-ptg/
description: Eventlet removal progress during Hibiscus and priorities for Indri, based on the October 2026 OpenStack Project Teams Gathering session.
keywords: openstack, eventlet removal, indri ptg, hibiscus retrospective, native threading, migration, python 3.15, project teams gathering
og_type: article
og_title: OpenStack Indri PTG Eventlet Removal Retrospective
og_description: Hibiscus migration progress, testing results, remaining blockers and next steps for removing Eventlet from OpenStack.
---

<section aria-labelledby="report-title">
    <h1 id="report-title" class="text-4xl font-bold">Eventlet Removal PTG Retrospective - Hibiscus to Indri</h1>
    <p class="mt-4 text-lg"><em>Report prepared on <time datetime="2026-11-02">November 2, 2026</time></em></p>
    <p class="mt-6 text-xl">OpenStack's Eventlet removal effort advanced during Hibiscus, with several services now using native threading by default or completing their migration. At the October 2026 PTG session, participants identified Swift, Cinder and Nova as the main concentrations of remaining implementation work. Priorities for Indri include reviewing those changes, removing residual dependencies and testing services with Eventlet absent from the environment.</p>
    <div class="mt-6 bg-indigo-900 p-6 rounded-lg shadow">
        <p class="text-lg"><strong>Session:</strong> <time datetime="2026-10-13T13:00:00Z">October 13, 2026 at 13:00 UTC</time></p>
        <p class="mt-3 text-lg">This report covers the progress and plans recorded in the <a href="https://etherpad.opendev.org/p/oct2026-ptg-eventlet-removal" class="text-cyan-400 hover:underline">session Etherpad</a>.</p>
    </div>
    <nav aria-labelledby="report-contents" class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 id="report-contents" class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#hibiscus-retrospective" class="text-cyan-400 hover:underline">Hibiscus Retrospective</a></li>
                <li><a href="#project-progress" class="text-cyan-400 hover:underline">Progress by Project</a></li>
                <li><a href="#testing-performance" class="text-cyan-400 hover:underline">Testing and Performance</a></li>
                <li><a href="#dependencies" class="text-cyan-400 hover:underline">Dependencies and Testing without Eventlet</a></li>
                <li><a href="#python-compatibility" class="text-cyan-400 hover:underline">Python Compatibility</a></li>
                <li><a href="#coordination" class="text-cyan-400 hover:underline">Projects Requiring Coordination</a></li>
                <li><a href="#next-steps" class="text-cyan-400 hover:underline">Next Steps for Indri</a></li>
                <li><a href="#resources" class="text-cyan-400 hover:underline">Session Resources</a></li>
            </ul>
        </div>
    </nav>
</section>

<section aria-labelledby="hibiscus-retrospective" class="mt-10">
    <h2 id="hibiscus-retrospective" class="text-3xl font-bold mb-6 scroll-mt-28">Hibiscus Retrospective</h2>
    <p class="text-xl mb-6">The September 2026 scan presented at the session covered 98 projects and approximately 1,781 baseline Eventlet usages. These figures describe the September scan; subsequent project updates are covered below.</p>
    <dl class="grid grid-cols-2 lg:grid-cols-5 gap-4 mb-6">
        <div class="bg-green-800 p-4 rounded-lg shadow">
            <dt class="text-lg">Fully migrated</dt>
            <dd class="text-3xl font-bold mt-2">59</dd>
        </div>
        <div class="bg-indigo-900 p-4 rounded-lg shadow">
            <dt class="text-lg">In progress</dt>
            <dd class="text-3xl font-bold mt-2">12</dd>
        </div>
        <div class="bg-indigo-900 p-4 rounded-lg shadow">
            <dt class="text-lg">Deprecating Eventlet</dt>
            <dd class="text-3xl font-bold mt-2">19</dd>
            <dd class="mt-2">Eventlet already optional</dd>
        </div>
        <div class="bg-yellow-800 p-4 rounded-lg shadow">
            <dt class="text-lg">Stalled</dt>
            <dd class="text-3xl font-bold mt-2">4</dd>
        </div>
        <div class="bg-gray-800 p-4 rounded-lg shadow">
            <dt class="text-lg">Abandoned</dt>
            <dd class="text-3xl font-bold mt-2">4</dd>
        </div>
    </dl>
    <p class="text-xl">The notes also reported 131 merged and 79 open Gerrit changes under <code>topic:eventlet-removal</code> since March 2026. The review priority is to help complete Swift, Cinder and Nova while moving projects that already make Eventlet optional toward full removal.</p>
</section>

<section aria-labelledby="project-progress" class="mt-10">
    <h2 id="project-progress" class="text-3xl font-bold mb-6 scroll-mt-28">Progress by Project</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
        <article class="bg-green-800 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-3">Glance</h3>
            <p class="text-lg">Fully migrated to uWSGI and native threading; Eventlet removed from requirements.</p>
        </article>
        <article class="bg-indigo-900 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-3">Manila</h3>
            <p class="text-lg">Eventlet free at runtime, with native threading as the default.</p>
            <p class="text-lg mt-3"><strong>Remaining:</strong> Eventlet backend support retained until Milestone 1.</p>
        </article>
        <article class="bg-green-800 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-3">Neutron and Ironic</h3>
            <p class="text-lg">No substantive Eventlet use remains in production code.</p>
            <p class="text-lg mt-3"><strong>Remaining:</strong> Test re-enablement and cleanup.</p>
        </article>
        <article class="bg-green-800 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-3">Blazar</h3>
            <p class="text-lg">Fully migrated, using a threading API and asyncio.</p>
        </article>
        <article class="bg-indigo-900 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-3">Swift</h3>
            <p class="text-lg">Work continues on <code>feature/threaded</code>, including Gunicorn WSGI and replacement of Eventlet primitives.</p>
            <p class="text-lg mt-3"><strong>Next:</strong> Finish the branch work and bring changes to <code>master</code>, potentially beginning with individual components.</p>
        </article>
        <article class="bg-indigo-900 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-3">Cinder</h3>
            <p class="text-lg">Migration proceeds driver by driver, including FC zoning, DS8K, Datera and NetApp.</p>
            <p class="text-lg mt-3"><strong>Remaining:</strong> Two patches remained for <code>cinder-volume</code>; API service work was still ongoing.</p>
        </article>
        <article class="bg-indigo-900 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-3">Nova</h3>
            <p class="text-lg">Native threading is the default for conductor and compute. The proxy can also run in native threading mode.</p>
            <p class="text-lg mt-3"><strong>Next:</strong> Make threading the default for console proxies; continue functional and performance testing and address earlier technical debt.</p>
        </article>
        <article class="bg-indigo-900 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-3">Tacker</h3>
            <p class="text-lg">Changes target removal of the Eventlet WSGI server and the requirements dependency.</p>
            <p class="text-lg mt-3"><strong>Next:</strong> Complete those changes, after which the notes expect only samples to remain.</p>
        </article>
        <article class="bg-green-800 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-3">Watcher</h3>
            <p class="text-lg">Threading is the default; Eventlet code removal merged in Indri.</p>
            <p class="text-lg mt-3"><a href="https://review.opendev.org/q/topic:%22indri-removals%22" class="text-cyan-400 hover:underline">Eventlet removal changes</a></p>
        </article>
        <article class="bg-indigo-900 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-3">Trove</h3>
            <p class="text-lg">Migration is at an early stage, with one preparation change merged.</p>
            <p class="text-lg mt-3"><strong>Next:</strong> Continue the migration work.</p>
        </article>
    </div>
    <p class="text-xl mt-6">Smaller changes remained for <code>oslo.log</code>, <code>oslo.concurrency</code>, <code>glance_store</code> and <code>python-keystoneclient</code>.</p>
</section>

<section aria-labelledby="testing-performance" class="mt-10">
    <h2 id="testing-performance" class="text-3xl font-bold mb-6 scroll-mt-28">Testing and Performance</h2>
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <article class="bg-indigo-900 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-4">Swift</h3>
            <p class="text-lg mb-4">Swift reported working unit, functional and probe tests, with non-voting testing in Zuul. Approximately 70 percent of the patches on <code>feature/threaded</code> had merged; the team explicitly noted that this percentage does not represent all the work needed for the migration.</p>
            <p class="text-lg">Initial benchmarks showed a measurable context switching cost, particularly at high I/O rates. The plan is to continue work on the branch and propose a small number of larger changes back to <code>master</code> when ready. Individual components, such as the object server, may move first.</p>
        </article>
        <article class="bg-indigo-900 p-6 rounded-lg shadow min-w-0">
            <h3 class="text-2xl font-bold mb-4">Nova</h3>
            <p class="text-lg mb-4">Scale testing initially showed higher memory use with native threading. Scheduler and conductor used roughly twice as much memory in the initial runs, although Eventlet could reach similar peak values. Reducing <code class="break-all">executor_thread_pool_size</code> lowered memory consumption without degrading performance in the subsequent tests described in the notes.</p>
            <p class="text-lg">All unit tests worked in threading mode. Most functional tests also worked, with one still skipped. Continued performance testing and console proxy migration remain priorities for Indri.</p>
        </article>
    </div>
</section>

<section aria-labelledby="dependencies" class="mt-10">
    <h2 id="dependencies" class="text-3xl font-bold mb-6 scroll-mt-28">Dependencies and Testing without Eventlet</h2>
    <div class="bg-indigo-900 p-6 rounded-lg shadow">
        <p class="text-lg mb-4">Participants called for jobs that demonstrate services can run with Eventlet completely uninstalled. Switching to native threading or deleting a direct dependency is insufficient if another package still installs Eventlet in the shared environment.</p>
        <p class="text-lg mb-4">The discussion explored making Eventlet an optional dependency in shared libraries around Milestone 2 or earlier. Projects that still need it could declare it explicitly or use an <code>eventlet</code> extra. Nova expected to retain Eventlet in its requirements for 2027.1 while also seeking coverage with the package absent.</p>
        <p class="text-lg mb-4">A reusable DevStack Ansible role or playbook was proposed to uninstall Eventlet after environment preparation. This would let projects test its absence without changing jobs that still require it. Because DevStack uses a global virtual environment, such jobs may initially need to exclude Swift and Cinder until those projects no longer require Eventlet.</p>
        <p class="text-lg">Debian packaging feedback also showed remaining reverse dependencies, including services and shared libraries. This reinforces the need to distinguish runtime migration, removal of compatibility code and removal of the package dependency.</p>
    </div>
    <div class="bg-yellow-800 p-6 rounded-lg shadow mt-6">
        <h3 class="text-2xl font-bold mb-4">Rootwrap and privsep</h3>
        <p class="text-lg">The session called for revisiting the Rootwrap and privsep transition. Privsep was identified as the preferred direction, but participants questioned whether existing fallbacks already allow operation without Eventlet and how deployments should launch privsep in that configuration. Verifying that behavior and documenting the deployment path remain open tasks.</p>
    </div>
</section>

<section aria-labelledby="python-compatibility" class="mt-10">
    <h2 id="python-compatibility" class="text-3xl font-bold mb-6 scroll-mt-28">Python Compatibility</h2>
    <p class="text-xl mb-4">The discussion noted that Python 3.15 testing is not mandatory for the 2027.1 release and highlighted a Debian 14 requirement for Indri to work with Python 3.15. Library jobs using the <code>-next</code> templates were already running voting Python 3.15 tests; participants also wanted non-voting coverage for other deliverables.</p>
    <p class="text-xl mb-6">Python 3.14 end-to-end coverage with Eventlet was raised as a question. The pad links existing Swift Python 3.14 unit tests, which do not by themselves establish end-to-end coverage.</p>
    <div class="bg-indigo-900 p-6 rounded-lg shadow">
        <h3 class="text-2xl font-bold mb-4">Runtime and Testing References</h3>
        <ul class="list-disc pl-6 space-y-3 text-lg">
            <li><a href="https://governance.openstack.org/tc/reference/runtimes/2027.1.html" class="text-cyan-400 hover:underline">2027.1 runtime policy</a></li>
            <li><a href="https://review.opendev.org/c/openstack/openstack-zuul-jobs/+/992771/" class="text-cyan-400 hover:underline">Python 3.15 job change</a></li>
            <li><a href="https://opendev.org/openstack/openstack-zuul-jobs/src/commit/913885b578b905e66c7f0b0ddfd8c996e7859cef/zuul.d/project-templates.yaml#L686-L691" class="text-cyan-400 hover:underline">Library job templates cited in the session</a></li>
        </ul>
    </div>
</section>

<section aria-labelledby="coordination" class="mt-10">
    <h2 id="coordination" class="text-3xl font-bold mb-6 scroll-mt-28">Projects Requiring Coordination</h2>
    <p class="text-xl mb-6">The stalled group comprised Zun, Storlets, Masakari and Masakari Monitors. Their situations require different follow-up:</p>
    <div class="bg-yellow-800 p-6 rounded-lg shadow">
        <ul class="list-disc pl-6 space-y-4 text-lg">
            <li>Storlets depends on Swift moving to threading first.</li>
            <li>Masakari Monitors needs functional testing and follow-up with the core team on the <a href="https://review.opendev.org/c/openstack/masakari-monitors/+/1009699" class="text-cyan-400 hover:underline">pending removal change</a>.</li>
            <li>Zun still uses monkey patching, <code>spawn_n</code>, <code>Timeout</code> and an Eventlet executor, with no migration changes in progress in the update.</li>
            <li>Masakari remains in the group needing ownership or a decision on its migration.</li>
        </ul>
    </div>
    <p class="text-xl mt-6">The Neutron team indicated that Eventlet support in <code>os-ken</code> would be dropped during Indri, with discussion planned at the PTG.</p>
    <p class="text-xl mt-4">For the abandoned repositories, participants considered <code>osops</code> acceptable to ignore, requested a check of <code>swift-bench</code>, and recorded retirement of <code>upstream-institute</code> for 2027.1. <code>openstack-ansible-ops</code> remained on the list requiring a decision.</p>
</section>

<section aria-labelledby="next-steps" class="mt-10">
    <h2 id="next-steps" class="text-3xl font-bold mb-6 scroll-mt-28">Next Steps for Indri</h2>
    <p class="text-xl mb-6">The session's priorities are to maintain migration work, switch projects to native threading as early as feasible, focus reviews on Swift, Cinder and Nova, and remove Eventlet from requirements where it is no longer needed. Participants also called for testing with Eventlet absent alongside that dependency cleanup.</p>
    <div class="bg-indigo-900 p-4 sm:p-6 rounded-lg shadow">
        <table class="w-full text-left text-lg">
            <caption class="text-xl font-bold text-left mb-4">Follow-up Actions</caption>
            <thead>
                <tr class="border-b border-indigo-700">
                    <th scope="col" class="py-3 pr-4">Follow-up</th>
                    <th scope="col" class="py-3">Owner in the notes</th>
                </tr>
            </thead>
            <tbody>
                <tr class="border-b border-indigo-700">
                    <td class="py-3 pr-4 align-top">Raise Zun's migration situation on the mailing list.</td>
                    <td class="py-3 align-top">damani</td>
                </tr>
                <tr class="border-b border-indigo-700">
                    <td class="py-3 pr-4 align-top">Check the plan for completing and merging the Masakari Monitors change with its core team.</td>
                    <td class="py-3 align-top">tkajinam</td>
                </tr>
                <tr class="border-b border-indigo-700">
                    <td class="py-3 pr-4 align-top">Check the status of <code>swift-bench</code>.</td>
                    <td class="py-3 align-top">cschwede</td>
                </tr>
                <tr>
                    <td class="py-3 pr-4 align-top">Handle retirement of <code>upstream-institute</code> in 2027.1.</td>
                    <td class="py-3 align-top">gouthamr</td>
                </tr>
            </tbody>
        </table>
    </div>
    <p class="text-xl mt-6">The strategy for removing the remaining Eventlet dependent code in 2027.2 was raised as an open question. It still needs a concrete plan alongside the Indri work on optional dependencies, testing and deployment guidance.</p>
</section>

<section aria-labelledby="resources" class="mt-10">
    <h2 id="resources" class="text-3xl font-bold mb-6 scroll-mt-28">Session Resources</h2>
    <div class="bg-indigo-900 p-6 rounded-lg shadow">
        <ul class="list-disc pl-6 space-y-3 text-lg">
            <li><a href="https://etherpad.opendev.org/p/oct2026-ptg-eventlet-removal" class="text-cyan-400 hover:underline">October 2026 Eventlet Removal Etherpad</a></li>
            <li><a href="https://meetpad.opendev.org/oct2026-ptg-eventlet-removal" class="text-cyan-400 hover:underline">Session meeting room</a></li>
            <li><a href="{{ site.baseurl }}{% link guide/openstack/hibiscus-ptg.md %}" class="text-cyan-400 hover:underline">Previous Hibiscus PTG report</a></li>
        </ul>
    </div>
</section>

<nav aria-label="Related OpenStack resources" class="mt-10 flex flex-col sm:flex-row justify-between gap-4">
    <a href="{{ site.baseurl }}{% link guide/openstack.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-6 rounded text-center hover:underline">OpenStack Resources</a>
    <a href="{{ site.baseurl }}{% link guide/openstack/hibiscus-ptg.md %}" class="inline-block bg-gradient-to-r from-blue-400 to-blue-600 text-white font-semibold py-3 px-6 rounded text-center hover:underline">Hibiscus PTG Report</a>
</nav>
