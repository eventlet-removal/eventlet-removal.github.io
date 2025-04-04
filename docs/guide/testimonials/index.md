---
layout: dashboard
title: Migration Testimonials
permalink: /testimonials/
description: Real-world experiences and insights from developers and teams who have successfully migrated away from Eventlet to modern alternatives.
keywords: eventlet migration, testimonials, case studies, migration experiences, eventlet removal stories, success stories
og_type: article
og_title: Eventlet Removal Testimonials - Real-world Migration Experiences
og_description: Read firsthand accounts from developers and projects that have successfully migrated from Eventlet to modern asynchronous alternatives.
---

<h1 class="text-4xl font-bold mb-8">Migration Testimonials</h1>

<p class="mt-6 text-xl">Learn from the experiences of developers and teams who have successfully navigated the transition away from Eventlet. These testimonials provide valuable insights into real-world migration challenges, strategies, and outcomes.</p>

<div class="mt-10 grid grid-cols-1 md:grid-cols-2 gap-8">
  <a href="{{ site.baseurl }}{% link guide/testimonials/arnaud-morin.md %}" class="block bg-gray-800 bg-opacity-70 p-6 rounded-lg hover:bg-gray-700 hover:bg-opacity-70 transition-all hover:scale-105">
    <div class="flex items-center mb-4">
      <img src="{{ site.baseurl }}/images/testimonials/arnaud-morin.jpg" alt="Arnaud Morin" class="w-16 h-16 rounded-full mr-4 object-cover border-2 border-cyan-400">
      <div>
        <h2 class="text-2xl font-bold">Arnaud Morin</h2>
        <p class="text-gray-300">Cloud DevOps & Virtualization Evangelist, OpenStack Contributor</p>
      </div>
    </div>
    <p class="text-xl">Shares his experience migrating OpenStack Mistral from Eventlet to threading-based solutions, focusing on incremental approaches and practical challenges.</p>
    <div class="mt-4 text-cyan-400 flex items-center">
      Read more <i class="fas fa-arrow-right ml-2"></i>
    </div>
  </a>
  
  <!-- Placeholder for future testimonials -->
  <div class="bg-gray-800 bg-opacity-40 p-6 rounded-lg border border-dashed border-gray-600 flex items-center justify-center">
    <div class="text-center">
      <div class="text-5xl text-gray-600 mb-4">
        <i class="fas fa-user-plus"></i>
      </div>
      <p class="text-xl text-gray-400">More testimonials coming soon...</p>
      <p class="mt-2 text-gray-500">Have a migration story to share? <a href="{{ site.github_repo }}" class="text-cyan-400 hover:underline">Contact us</a></p>
    </div>
  </div>
</div>

<div class="mt-16">
  <h2 class="text-3xl font-bold mb-6">Share Your Experience</h2>
  <p class="text-xl">Have you successfully migrated a project from Eventlet? We'd love to hear about your experience and share your insights with the community.</p>
  
  <div class="mt-6">
    <a href="{{ site.github_repo }}" class="inline-block bg-gradient-to-r from-cyan-500 to-blue-600 text-white font-semibold py-3 px-8 rounded-lg hover:scale-105 transition-transform" target="_blank">
      Submit Your Testimonial
    </a>
  </div>
</div>

<div class="mt-10 flex justify-between">
    <a href="{{ site.baseurl }}{% link guide/studies.md %}" class="inline-block bg-gradient-to-r from-yellow-400 to-yellow-600 text-gray-900 font-semibold py-3 px-8 rounded hover:scale-105 transition-transform">
        <i class="fas fa-arrow-left mr-2"></i>Case Studies and Testimonials
    </a>
    <div><!-- No next page --></div>
</div>
