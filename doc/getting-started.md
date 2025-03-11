---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: dashboard
title: Getting Started
---
<section>
    <h1 class="text-4xl font-bold">Getting Started</h1>
    <p class="mt-10 text-xl">Welcome to the Eventlet Migration Guide! This comprehensive resource is designed to assist developers, system administrators, and technical decision-makers in transitioning from Eventlet to more modern and supported concurrency libraries in Python.</p>
</section>

<section>
    <h2 class="mt-10 text-3xl font-bold">Why Migrate from Eventlet?</h2>
    <p class="mt-10 text-xl">Eventlet has been a popular choice for concurrent programming in Python, particularly within the OpenStack community. However, as the library has become deprecated, it is crucial to explore alternatives that offer better support, performance, and future-proofing for your applications.</p>
    <p class="mt-10 text-xl">Migrating to libraries like AsyncIO or Threading can provide numerous benefits, including:</p>
    <div class="grid md:grid-cols-3 gap-6 py-4">
        <div class="bg-gray-900 p-6 rounded-lg shadow">
            <h2 class="text-2xl font-bold">Improved Performance</h2>
            <p class="mt-10 text-xl">Modern libraries often offer better performance and scalability for handling concurrent tasks. Modern libraries like Asyncio introduce new paradigms like real cooperative task handling.</p>
        </div>
        <div class="bg-gray-900 p-6 rounded-lg shadow">
            <h2 class="text-2xl font-bold">Active Maintenance</h2>
            <p class="mt-10 text-xl">Actively maintained libraries receive regular updates, bug fixes, and new features, ensuring long-term viability. Active maintenance also mean more resources, tutorials, and peers to learn from and collaborate with.</p>
        </div>
        <div class="bg-gray-900 p-6 rounded-lg shadow">
            <h2 class="text-2xl font-bold">Future-Proofing</h2>
            <p class="mt-10 text-xl">Adopting widely-used and supported libraries helps future-proof your applications against obsolescence.</p>
        </div>
    </div>
</section>

<section>
    <h2 class="mt-10 text-3xl font-bold">Objectives of This Guide</h2>
    <p class="mt-10 text-xl">The primary goal of this guide is to provide a structured and informative pathway for migrating from Eventlet. By the end of this guide, you will:</p>
    <ul class="mt-10 text-xl">
        <li class="mt-5 text-xl"><i class="fas fa-check-square text-teal-300 mr-2"></i>Understand the key differences between Eventlet and alternative concurrency libraries.</li>
        <li class="mt-5 text-xl"><i class="fas fa-check-square text-teal-300 mr-2"></i>Learn how to assess your current Eventlet-based applications for migration readiness.</li>
        <li class="mt-5 text-xl"><i class="fas fa-check-square text-teal-300 mr-2"></i>Follow step-by-step instructions for migrating to AsyncIO or threading.</li>
        <li class="mt-5 text-xl"><i class="fas fa-check-square text-teal-300 mr-2"></i>Access resources, including code examples and best practices, to support your migration efforts.</li>
        <li class="mt-5 text-xl"><i class="fas fa-check-square text-teal-300 mr-2"></i>Hear from others who have successfully navigated this transition, gaining insights from their experiences.</li>
    </ul>
</section>

<section>
    <h2 class="mt-10 text-3xl font-bold">How to Use This Guide</h2>
    <p class="mt-10 text-xl">This guide is organized into sections that address various aspects of the migration process. You can navigate through the sections sequentially or jump to specific areas of interest. Whether you are just beginning to explore migration options or are ready to dive into code refactoring, this guide aims to support you at every stage.</p>
    <p class="mt-10 text-xl">Join us on this journey to modernize your Python applications and unlock the full potential of contemporary concurrency libraries!</p>
</section>

<div class="mt-10 text-right">
    <a href="{{ site.baseurl }}{% link eventlet.md %}" class="inline-block bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">Understand Eventlet<i class="fas fa-arrow-right ml-2"></i></a>
</div>
