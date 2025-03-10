---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: base
---
<!-- Hero Section -->
<section class="h-screen flex items-center justify-center bg-cover bg-center" style="background-image: url('images/hero.jpg'); background-position: center 40%; background-repeat: no-repeat; background-size: cover;">
    <div class="bg-gray-950 bg-opacity-75 p-10 rounded-lg text-center">
    <h1 class="text-5xl font-extrabold mb-4">This website learn you<br>How to <span class="text-yellow-500">Remove</span><span class="text-teal-300"> Eventlet</span><br> from your software</h1>
    <p class="mb-6 text-xl">We are not going to turn around pot,<br>Eventlet is a technology from another age,<br>which produce more headaches than solutions.</p>
    <p class="mb-6 text-xl"><strong>If you want your project to keep running then it is time to replace it!</strong></p>
    <a href="{{ site.baseurl }}{% link getting-started.md %}" class="mt-4 inline-block bg-gradient-to-r from-cyan-400 to-blue-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">Getting Started</a>
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
        <h2 class="text-4xl font-bold mb-6">À Propos de Nous</h2>
        <p>Nous sommes des pionniers passionnés par le développement informatique futuriste. Notre mission est d'intégrer des technologies émergentes dans vos projets, pour vous propulser vers l'avenir.</p>
    </div>
</section>


<!-- Section About -->
<section id="about" class="py-20">
    <div class="container mx-auto px-10">
        <h2 class="text-4xl font-bold mb-6">À Propos de Nous</h2>
        <p>Nous sommes des pionniers passionnés par le développement informatique futuriste. Notre mission est d'intégrer des technologies émergentes dans vos projets, pour vous propulser vers l'avenir.</p>
    </div>
</section>
