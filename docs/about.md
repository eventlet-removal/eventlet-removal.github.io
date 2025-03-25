---
layout: base
title: About Us
permalink: /about/
description: Learn about the creators of the Eventlet Removal guide, their background in the Python ecosystem, and the collective effort behind this initiative to help developers transition away from Eventlet.
keywords: eventlet removal, python ecosystem, migration guide, asynchronous programming, eventlet alternatives, openstack community
og_type: website
og_title: About the Eventlet Removal Guide Team and Project
og_description: Meet the team behind the Eventlet Removal guide and learn about our mission to help Python developers migrate to better asynchronous solutions.
---

<div class="container mx-auto px-6 py-10">
    <h1 class="text-4xl font-bold mb-8">About This Project</h1>
    
    <section class="mb-12">
        <h2 class="text-3xl font-bold mb-6">About the Author</h2>
        <div class="flex flex-col md:flex-row items-start md:items-center gap-8">
            <div class="w-full md:w-1/5 flex">
                <img src="https://avatars.githubusercontent.com/u/1541489?v=4" alt="Hervé Beraud" class="rounded-lg shadow-lg w-1/4 md:w-1/2 lg:w-3/4">
            </div>
            <div class="w-full ">
                <p class="text-xl mb-4">This website was created by <a href="https://herve.beraud.io/" class="text-cyan-400 hover:underline" target="_blank">Hervé Beraud</a>, a passionate software engineer with extensive experience in open source technologies and the Python ecosystem.</p>
                <p class="text-xl mb-4">Hervé has been actively involved in the OpenStack community for many years, where he gained firsthand experience with the challenges of using Eventlet and the need for more modern asynchronous approaches.</p>
                <p class="text-xl">You can find more of Hervé's work on <a href="https://github.com/4383" class="text-cyan-400 hover:underline" target="_blank">GitHub</a>.</p>
            </div>
        </div>
    </section>
    
    <section class="mb-12">
        <h2 class="text-3xl font-bold mb-6">A Collective Effort</h2>
        <div class="bg-indigo-900 p-8 rounded-lg">
            <p class="text-xl mb-4">This website represents the culmination of a collective effort that began in early 2024. It emerged from numerous discussions, working sessions, and collaborative problem-solving among developers facing the challenges of Eventlet in their projects.</p>
            <p class="text-xl mb-4">Throughout 2024, we've collected materials, code samples, migration approaches, and real-world experiences from various sources across the Python ecosystem, with particular focus on the OpenStack community where Eventlet has been historically prevalent.</p>
            <p class="text-xl">This knowledge base represents the combined wisdom of many developers who have tackled the complex task of migrating away from Eventlet to more modern, maintainable concurrency solutions.</p>
        </div>
    </section>
    
    <section class="mb-12">
        <h2 class="text-3xl font-bold mb-6">Why This Matters</h2>
        <p class="text-xl mb-4">Eventlet has served many projects well for years, but as the Python ecosystem evolves, more robust and standardized approaches to concurrency have emerged. The transition away from Eventlet represents an important step forward for many projects in terms of:</p>
        <ul class="list-disc pl-8 text-xl space-y-2 mb-4">
            <li>Long-term sustainability</li>
            <li>Compatibility with modern Python features</li>
            <li>Performance improvements</li>
            <li>Code maintainability</li>
            <li>Developer experience</li>
        </ul>
        <p class="text-xl">This website aims to make that transition as smooth as possible by providing comprehensive guidance based on real-world experience.</p>
    </section>
    
    <section class="futuristic-section p-8 rounded-lg">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-3xl font-bold mb-6">Contribute Your Experience</h2>
            <p class="text-xl mb-4">The journey of migrating away from Eventlet continues, and we welcome your contributions to this knowledge base. If you've successfully migrated a project from Eventlet to alternative approaches, we'd love to hear about your experience.</p>
            <p class="text-xl mb-6">Together, we can build a comprehensive resource that helps the entire Python community move toward more maintainable and future-proof concurrency patterns.</p>
            <a href="{{ site.github_repo }}/issues/new" class="bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform inline-block">Share Your Experience</a>
        </div>
    </section>
    
    <section>
        <h2 class="mt-10 text-3xl font-bold mb-6">Acknowledgments</h2>
        <p class="text-xl mb-4">We'd like to thank all the individuals and organizations that have contributed their expertise, code samples, testing, and feedback to make this guide possible.</p>
        <p class="text-xl">Special thanks to the OpenStack community, which has been at the forefront of addressing these migration challenges and whose collective experience forms a substantial part of the knowledge shared on this site.</p>
    </section>
</div>


