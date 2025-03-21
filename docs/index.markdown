---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: base
footer: true
title: Eventlet Removal - Stop Using Eventlet and Make your Python Software Future-Proofing
permalink: /
---
<!-- Hero Section -->
<section class="h-screen flex items-center justify-center bg-cover bg-center" style="background-image: url('images/hero.jpg'); background-position: center 40%; background-repeat: no-repeat; background-size: cover;">
    <div class="bg-gray-950 bg-opacity-80 p-10 rounded-lg text-center">
    <h1 class="text-5xl font-extrabold mb-4">This website learn you<br>How to <span class="text-yellow-500">Remove</span><span class="text-teal-300"> Eventlet</span><br> from your software</h1>
    <p class="mb-6 text-xl">We are not going to turn around pot,<br>Eventlet is a technology from another age,<br>which produce more headaches than solutions.</p>
    <p class="mb-6 text-xl"><strong>If you want your project to keep running then it is time to replace it!</strong></p>
    <a href="{{ site.baseurl }}{% link guide/getting-started.md %}" class="mt-4 inline-block bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">Getting Started</a>
    </div>
</section>

<!-- Section Reasons -->
<section id="reasons" class="py-20 px-10">
    <h2 class="text-4xl font-bold text-center mb-16">3 Reasons to Remove Eventlet</h2>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-10">
        <div class="bg-gray-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h2 class="text-2xl font-bold mb-3">Eventlet is Broken</h2>
            <p>Eventlet is aging and increasingly unsupported, making your applications vulnerable with each new Python or dependency update. Removing it ensures your project's long-term stability and security.</p>
        </div>
        <div class="bg-gray-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h2 class="text-2xl font-bold mb-3">Maintenance Complexity</h2>
            <p>Eventlet’s complex handling of green threads and compatibility issues with modern Python libraries (like Asyncio) make maintenance difficult, increasing technical debt and operational risks.</p>
        </div>
        <div class="bg-gray-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
            <h2 class="text-2xl font-bold mb-3">Explicit is Better Than Implicit</h2>
            <p>Eventlet heavily rely on monkey patching to make synchronous features asynchronous. This kind of voodoo can quickly became nightmare to maintain, making your code difficult to understand
            and to debug.</p>
        </div>
    </div>
</section>

<!-- Section About -->
<section id="about" class="py-20 futuristic-section">
    <div class="container mx-auto px-10">
        <h2 class="text-4xl font-bold mb-6">Who This Website is For</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10">
            <div class="bg-gray-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300 bg-opacity-70">
                <h3 class="text-2xl font-bold mb-3">Python Developers</h3>
                <p>Those who have been using Eventlet in their projects and are looking for guidance on migrating to alternatives like AsyncIO, Threading, etc.</p>
            </div>
            <div class="bg-gray-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300 bg-opacity-70">
                <h3 class="text-2xl font-bold mb-3">OpenStack Users</h3>
                <p>Given Eventlet's historical use in OpenStack, this guide is particularly relevant for users and developers within the OpenStack community seeking to update their infrastructure.</p>
            </div>
            <div class="bg-gray-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300 bg-opacity-70">
                <h3 class="text-2xl font-bold mb-3">Project Managers</h3>
                <p>Individuals overseeing projects that currently rely on Eventlet and need to understand the benefits and process of transitioning to more modern solutions.</p>
            </div>
            <div class="bg-gray-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300 bg-opacity-70">
                <h3 class="text-2xl font-bold mb-3">DevOps Engineers</h3>
                <p>Professionals responsible for maintaining and deploying applications that use Eventlet, seeking to improve system reliability and performance.</p>
            </div>
            <div class="bg-gray-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300 bg-opacity-70">
                <h3 class="text-2xl font-bold mb-3">Technical Leads</h3>
                <p>Leaders who need to make informed decisions about the technical direction of their projects, including the adoption of newer concurrency models.</p>
            </div>
            <div class="bg-gray-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300 bg-opacity-70">
                <h3 class="text-2xl font-bold mb-3">Software Architects</h3>
                <p>Experts designing the architecture of software systems who want to ensure their projects are built on robust and future-proof technologies.</p>
            </div>
        </div>
    </div>
</section>

<!-- Section Call to Action -->
<section id="call-to-action" class="py-20">
    <div class="container mx-auto px-10">
        <h2 class="text-4xl font-bold mb-6">Call to Action</h2>
        <p class="mb-6 text-xl">Ready to start your migration journey? Here's how you can take the next steps:</p>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10 mt-10">
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Subscribe to Our Newsletter</h3>
                <p>Stay updated with the latest migration tips, best practices, and success stories.</p>
                <a href="https://groups.google.com/u/1/g/eventlet-removal" target="_blank" class="mt-4 inline-block bg-teal-500 text-gray-900 font-semibold py-2 px-4 rounded hover:bg-teal-400 transition-colors">Join Google Group</a>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Follow Us on Social Media</h3>
                <p>Join our community on platforms like <a href="https://www.linkedin.com/groups/13183090/" target="_blank" class="text-teal-400">LinkedIn</a>, and <a href="https://github.com/eventlet/eventlet" target="_blank" class="text-teal-400">GitHub</a> to engage with fellow developers, ask questions, and share your experiences.</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Contribute to the Guide</h3>
                <p>We welcome contributions from the community. If you have insights, code examples, or success stories to share, please <a href="https://github.com/4383/eventlet-removal/issues/new" class="text-teal-400" target="_blank">reach out</a>. Your input can help others navigate their migration more smoothly.</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Explore Our Resources</h3>
                <p>Dive into our comprehensive <a href="{{ site.baseurl }}{% link guide/getting-started.md %}" class="text-teal-400">guides</a>, <a href="{{ site.baseurl }}{% link guide/tutorials.md %}" class="text-teal-400">tutorials</a>, and <a href="{{ site.baseurl }}{% link guide/faq.md %}" class="text-teal-400">FAQs</a> to gain a deeper understanding of the migration process. Whether you're just starting or looking to optimize, we have the resources to support you.</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Get in Touch</h3>
                <p>Have specific questions or need personalized advice? Feel free to contact us. We are here to assist you every step of the way.</p>
            </div>
            <div class="bg-indigo-900 p-6 rounded-lg shadow hover:shadow-xl hover:scale-110 transition-transform duration-300">
                <h3 class="text-2xl font-bold mb-3">Sponsor Us</h3>
                <p>Your financial support helps us continue to provide valuable resources and maintain the quality of our guides. Consider sponsoring us to ensure we can keep helping the community.</p>
                <a href="https://github.com/sponsors/4383" target="_blank" class="mt-4 inline-block bg-teal-500 text-gray-900 font-semibold py-2 px-4 rounded hover:bg-teal-400 transition-colors">Become a Sponsor</a>
            </div>
        </div>
        <p class="mt-10 text-xl">Don't wait—take the first step towards modernizing your Python applications today!</p>
    </div>
</section>
