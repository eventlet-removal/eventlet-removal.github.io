---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: dashboard
title: Getting Started
permalink: /guide/getting-started/
---
<section>
    <h1 class="text-4xl font-bold">Getting Started</h1>
    <p class="mt-10 text-xl">Welcome to the Eventlet Migration Guide! This comprehensive resource is designed to assist developers, system administrators, and technical decision-makers in transitioning from Eventlet to more modern and supported concurrency libraries in Python.</p>
</section>

<section>
    <h2 class="mt-10 text-3xl font-bold">Why Migrate from Eventlet?</h2>
    <p class="mt-10 text-xl">Eventlet has been a popular choice for concurrent programming in Python, particularly within the OpenStack community. However, as <a href="https://github.com/eventlet/eventlet/issues/972" class="text-cyan-400" target="_blank">the library has become deprecated</a>, it is crucial to explore alternatives that offer better support, performance, and future-proofing for your applications.</p>

    <p class="mt-10 text-xl">There is <a href="{{ site.baseurl }}{% link guide/risks.md %}"  class="text-cyan-400" target="_blank">plenty of reasons</a> to abandon your Eventlet usages. Keeping Eventlet means exposing your company to major legal risks, harming your team's productivity, compromising user security, and destroying your project's credibility. As Eventlet maintainers, we have witnessed these issues firsthand and believe it is our responsibility to help users move away from it. This guide is here to support you in a necessary and well-managed transition.</p>

    <p class="mt-10 text-xl text-yellow-300"><strong>Neglecting necessary updates nurtures needless risks</strong>.</p>
</section>
<section>
    <h2 class="mt-10 text-3xl font-bold">What Benefits Will You Get From Migrating Your Products?</h2>
    <p class="mt-10 text-xl">Migrating to libraries like AsyncIO or Threading can provide numerous benefits, including:</p>
    <div class="grid md:grid-cols-3 gap-6 py-4">
        <div class="bg-gray-900 p-6 rounded-lg shadow hover:scale-105 transition-transform duration-300">
            <h2 class="text-2xl font-bold">Improved Performance</h2>
            <p class="mt-10 text-xl">Modern libraries often offer better performance and scalability for handling concurrent tasks. Modern libraries like Asyncio introduce new paradigms like real cooperative task handling. Threads, with modern versions of Python can benefits from <a href="https://peps.python.org/pep-0703/" class="text-cyan-400" target="_blank">PEP 703</a> to introduce real parallelism in your Python software.</p>
        </div>
        <div class="bg-gray-900 p-6 rounded-lg shadow hover:scale-105 transition-transform duration-300">
            <h2 class="text-2xl font-bold">Active Maintenance</h2>
            <p class="mt-10 text-xl">Actively maintained libraries receive regular updates, bug fixes, and new features, ensuring long-term viability. Active maintenance also mean more resources, tutorials, and peers to learn from and collaborate with.</p>
        </div>
        <div class="bg-gray-900 p-6 rounded-lg shadow hover:scale-105 transition-transform duration-300">
            <h2 class="text-2xl font-bold">Future-Proofing</h2>
            <p class="mt-10 text-xl">Adopting widely-used and supported libraries helps future-proof your applications against obsolescence. They provides guarantees of sustainability to your customers. AsyncIO and threading are both part of the Python standard library, making them a safe choice.</p>
        </div>
    </div>
</section>

<section class="mt-10 futuristic-section p-8 rounded-lg">
    <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
        <h2 class="text-3xl font-bold mb-6">Objectives of This Guide</h2>
        <p class="text-xl mb-4">The OpenStack community <a href="https://review.opendev.org/c/openstack/governance/+/902585" target="_blank" class="text-cyan-400">began to pave the way for a structured migration</a> and <a href="https://review.opendev.org/q/prefixtopic:%22eventlet-removal%22" target="_blank" class="text-cyan-400">pionneered bulk migration</a>. The OpenStack community is now eager to share his experience with other. This guide is the result of the work done by the OpenStack community. The intent of this guide is to help other Eventlet users in their migration journey.</p>
        <p class="text-xl mb-4">The primary goal of this guide is to provide a structured and informative pathway for migrating from Eventlet. By the end of this guide, you will:</p>
        <ul class="mb-6 text-xl">
            <li class="mt-3"><i class="fas fa-check-square text-teal-300 mr-2"></i>Understand the key differences between Eventlet and its alternatives.</li>
            <li class="mt-3"><i class="fas fa-check-square text-teal-300 mr-2"></i>Learn how to assess your current Eventlet-based applications for migration readiness.</li>
            <li class="mt-3"><i class="fas fa-check-square text-teal-300 mr-2"></i>Follow step-by-step instructions for migrating to AsyncIO or Threading.</li>
            <li class="mt-3"><i class="fas fa-check-square text-teal-300 mr-2"></i>Access resources, including code examples and best practices, to support your migration efforts.</li>
            <li class="mt-3"><i class="fas fa-check-square text-teal-300 mr-2"></i>Hear from others who have successfully navigated this transition, gaining insights from their experiences.</li>
        </ul>
    </div>
</section>

<section>
    <h2 class="mt-10 text-3xl font-bold">How to Use This Guide</h2>
    <p class="mt-10 text-xl">This guide is organized into sections that address various aspects of the migration process. You can navigate through the sections sequentially or jump to specific areas of interest. Whether you are just beginning to explore migration options or are ready to dive into code refactoring, this guide aims to support you at every stage.</p>
    <p class="mt-10 text-xl">Join us on this journey to modernize your Python applications and unlock the full potential of contemporary alternatives!<br>Together let's make our products safer by removing eventlet!</p>
    <p class="mt-10 text-xl">Let's get started!</p>
</section>

<div class="mt-10 text-right">
    <a href="{{ site.baseurl }}{% link guide/eventlet.md %}" class="inline-block bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">Understand Eventlet<i class="fas fa-arrow-right ml-2"></i></a>
</div>
