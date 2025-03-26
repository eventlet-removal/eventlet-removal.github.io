---
# Feel free to add content and custom Front Matter to this file.
# To modify the layout, see https://jekyllrb.com/docs/themes/#overriding-theme-defaults

layout: dashboard
title: Eventlet's Risks
permalink: /guide/eventlet-risks/
description: Understand the legal, technical, ethical, and business risks of continuing to use Eventlet in your Python applications, including security vulnerabilities, productivity impacts, and user experience degradation.
keywords: eventlet risks, security vulnerabilities, technical debt, software maintenance, legal compliance, productivity impact, reliability issues, user experience
og_type: article
og_title: The Critical Risks of Continuing to Use Eventlet in Production
og_description: An in-depth analysis of the legal, technical, ethical, and business risks posed by continued reliance on the deprecated Eventlet library.
---
<section>
    <h1 class="text-4xl font-bold">Risks of Eventlet</h1>
    <p class="mt-10 text-xl">If you are already convinced that you have to migrate off of Eventlet, then you can simply skip this chapter and jump directly to the next one, else, you are at the right place.</p>

    <div class="mt-6 p-4 rounded-lg futuristic-section">
        <div class="bg-gray-900 bg-opacity-70 p-6 rounded-lg">
            <h2 class="text-2xl font-bold mb-4">Table of Contents</h2>
            <ul class="space-y-2">
                <li><a href="#arguments-against-eventlet" class="text-cyan-400 hover:underline">Arguments Against Eventlet</a></li>
                <li class="ml-4"><a href="#legal-danger" class="text-cyan-400 hover:underline">Legal Danger</a></li>
                <li class="ml-4"><a href="#productivity-killer" class="text-cyan-400 hover:underline">Productivity Killer</a></li>
                <li class="ml-4"><a href="#ethical-problem" class="text-cyan-400 hover:underline">Ethical Problem</a></li>
                <li class="ml-4"><a href="#credibility-destroyer" class="text-cyan-400 hover:underline">Credibility Destroyer</a></li>
                <li><a href="#our-motivations" class="text-cyan-400 hover:underline">What Are Our Motivations</a></li>
            </ul>
        </div>
    </div>
</section>
<section>
    <h2 id="arguments-against-eventlet" class="mt-10 text-3xl font-bold">Arguments Against Eventlet <a href="#arguments-against-eventlet" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">It do not matter if you are a a developer, a project owner, a planning maker, or a business owner, everyone will find what they are looking for in the arguments set out below.</p>

    <h3 id="legal-danger" class="mt-10 text-2xl font-bold">Eventlet is a Legal Danger <a href="#legal-danger" class="text-cyan-400 text-xl">🔗</a></h3>
    <p class="mt-10 text-xl">Using obsolete and poorly maintained technologies, such as Eventlet, exposes companies to increased security vulnerabilities, thereby compromising the protection of personal data of their product users and their compliance with the GDPR and with data protection authority rules. In many cases, companies have been ordered to pay millions of dollars by courts around the world (<a href="https://en.wikipedia.org/wiki/Equifax#March_2017_security_breach" class="text-cyan-400" target="_blank">USA</a>, <a href="https://fr.wikipedia.org/wiki/Affaire_de_la_fuite_de_donn%C3%A9es_de_sant%C3%A9_de_laboratoires_fran%C3%A7ais" class="text-cyan-400" target="_blank">Europe</a>) due to a security breach in their systems. Data protection authorities, like the <a href="https://www.cnil.fr/en" class="text-cyan-400" target="_blank">CNIL</a> in France, often have the power to impose financial penalties reaching several million euros or, in the case of a company, up to 4% of its annual worldwide turnover.​</p>

    <p class="mt-10 text-xl">Obsolete technologies are more likely to contain unpatched vulnerabilities, increasing the risk of unauthorized access to personal data. The <a href="https://gdpr-info.eu/" class="text-cyan-400" target="_blank">GDPR</a> and data protection authorities require companies to implement appropriate measures to ensure the security of personal data, and failure to comply with these obligations can lead to severe sanctions.​</p>

    <p class="mt-10 text-xl">However, by adopting modern and well-maintained technologies, training staff in cybersecurity, and conducting data protection impact assessments, companies can significantly reduce these risks. It is therefore strongly recommended that companies avoid using obsolete technologies like Eventlet and adopt modern, well-maintained solutions to ensure the security of personal data.​</p>

    <p class="mt-10 text-xl text-yellow-300"><strong>Failing to remove Eventlet from your products exposes your business to legal penalties, for which you could be held personally liable in court</strong>.</p>

    <h3 id="productivity-killer" class="mt-10 text-2xl font-bold">Eventlet is a Productivity Killer <a href="#productivity-killer" class="text-cyan-400 text-xl">🔗</a></h3>
    <p class="mt-10 text-xl">The use of Eventlet reduces the efficiency of development and maintenance teams by increasing cognitive load, debugging time, and coordination efforts. Due to monkey patching, standard libraries exhibit unpredictable behavior, forcing developers to spend considerable time identifying and fixing <a href="https://github.com/eventlet/eventlet/issues?q=is%3Aissue%20" class="text-cyan-400" target="_blank">hard-to-trace bugs</a>. Additionally, learning Eventlet presents an unnecessarily steep curve for both newcomers and experienced developers, requiring them to invest time in an obsolete tool.</p>

    <p class="mt-10 text-xl">This complexity leads to a fragmentation of skills within teams. Collaboration is also affected, as developers must document and explain the side effects of monkey patching, which burdens communication and disrupts workflow fluidity.</p>

    <p class="mt-10 text-xl">An efficient team must rely on predictable, well-documented tools that are widely adopted by the community. The more uncertainty and friction a tool introduces, the more it slows down collective execution. Projects that have already migrated away from Eventlet report a significant reduction in time spent managing anomalies and maintaining their code, allowing them to focus on higher-value tasks. Likewise, onboarding new developers becomes easier, accelerating their learning curve and integration into the team.</p>

    <p class="mt-10 text-xl">Some may argue that Eventlet still works and that the migration cost would be too high. However, a tool that "works" while reducing team efficiency ultimately hinders productivity. The hidden cost of time lost dealing with unpredictable behavior is far greater than a well-planned, gradual transition to a modern alternative. A carefully phased migration would allow teams to progressively reduce their dependence on Eventlet without disrupting their daily work while improving their efficiency right away. That's the goal of this guide.​</p>

    <p class="mt-10 text-xl text-yellow-300"><strong>Failing to remove Eventlet from your products destroy the productivity of your team</strong>.</p>

    <h3 id="ethical-problem" class="mt-10 text-2xl font-bold">Eventlet is An Ethical Problem <a href="#ethical-problem" class="text-cyan-400 text-xl">🔗</a></h3>
    <p class="mt-10 text-xl">The continued use of Eventlet constitutes a failure to protect users, as it exposes their sensitive data and infrastructures to avoidable security risks. Due to its reliance on monkey patching, Eventlet dynamically modifies the behavior of standard libraries, introducing unpredictable and hard-to-detect side effects. This instability can lead to data leaks, service disruptions, or even exploitable security vulnerabilities. A user relying on your Eventlet based project to store critical data—such as personal information, medical records, or financial transactions—could see their data compromised without any way to protect themselves, simply because the platform is built on outdated technology.</p>

    <p class="mt-10 text-xl">The consequences of a security breach are severe for end users. Losing control over their data can result in privacy violations, identity theft, or even significant financial losses. If a vulnerability in Eventlet were to be exploited, it could have an irreversible impact on user trust in your products and all services that depend on it.</p>

    <p class="mt-10 text-xl">Recent history has shown that neglecting outdated technologies has already led to disasters. In 2014, the <a href="https://heartbleed.com/" class="text-cyan-400" target="_blank">Heartbleed vulnerability</a> in OpenSSL allowed attackers to extract sensitive data for years without users realizing it. In 2021, the <a href="https://en.wikipedia.org/wiki/Log4Shell" class="text-cyan-400" target="_blank">Log4Shell vulnerability</a> led to massive intrusions, affecting thousands of critical systems simply because a widely used library had not been updated. In 2017, <a href="https://www.blackduck.com/blog/equifax-apache-struts-vulnerability-cve-2017-5638.html" class="text-cyan-400" target="_blank">the attack on Equifax exposed the personal information of 140 million people</a> due to the use of an outdated version of Apache Struts containing a known critical flaw.</p>

    <p class="mt-10 text-xl">Keeping Eventlet in place means accepting the risk that one day, a similar incident could impact your projects and its users. Some may argue that no major attack related to Eventlet has been reported yet and that removing it would come at a significant cost. However, waiting for a disaster to happen before taking action is irresponsible. A security flaw can go unnoticed for months or even years, and by the time it is exploited, it is often too late for the victims.</p>

    <p class="mt-10 text-xl">A well-planned migration, like the one proposed in this guide, would allow teams to gradually reduce reliance on Eventlet without disrupting users, while ensuring a level of security and reliability that meets their legitimate expectations. Continuing to use Eventlet is a conscious decision to expose users to data breaches and consequences they cannot anticipate or prevent themselves.</p>

    <p class="mt-10 text-xl text-yellow-300"><strong>Continuing to use Eventlet is like knowingly putting passengers on a defective plane — it might stay in the air for now, but a crash is inevitable. Choosing to ignore it is both unethical and irresponsible.</strong></p>

    <h3 id="credibility-destroyer" class="mt-10 text-2xl font-bold">Eventlet Destroy the credibility of your projects <a href="#credibility-destroyer" class="text-cyan-400 text-xl">🔗</a></h3>
    <p class="mt-10 text-xl">The use of Eventlet degrades the user experience by making application performance unpredictable and causing sudden interruptions, leading to frustration and a loss of trust among end users.</p>  

    <p class="mt-10 text-xl">Users expect consistency and responsiveness when interacting with a service, yet with Eventlet, they may experience smooth performance at one moment, only to face unexplained slowdowns the next, even under similar conditions. This inconsistency stems from Eventlet’s inefficient thread and I/O management, which can cause some requests to be delayed unpredictably. Worse still, Eventlet’s reliance on monkey patching introduces unforeseen behaviors and hard-to-trace errors, leading to sudden crashes that interrupt user actions without warning. As a result, users may lose progress, face downtime, or experience disruptions at critical moments, all without any clear explanation.</p>

    <p class="mt-10 text-xl">A great user experience depends on reliability and predictability. A product that behaves inconsistently and interrupts users unexpectedly erodes trust and causes frustration. Users expect the services they rely on to function smoothly and predictably without having to worry about technical failures behind the scenes.</p>

    <p class="mt-10 text-xl">When applications become unstable, users abandon them. <a href="https://moldstud.com/articles/p-the-impact-of-error-handling-on-user-experience" class="text-cyan-400" target="_blank">Studies in user experience (UX)</a> show that inconsistent performance and random failures are among the top reasons users stop using a service. Even if an application works well most of the time, occasional unexplained failures create a negative perception that is difficult to erase. Additionally, unreliable applications generate a flood of customer complaints and support tickets, increasing operational costs and overloading technical teams.</p>

    <p class="mt-10 text-xl">Some may argue that Eventlet’s instability is tolerable since it doesn’t always manifest and that workarounds exist. However, the real problem is not systematic failures but unpredictable ones—users might have a seamless experience today and a terrible one tomorrow, which is even more frustrating than a consistently slow service. Furthermore, existing fixes only mask the symptoms without addressing the root cause. No amount of patching can change the fundamental fact that Eventlet is an outdated and unreliable technology.</p>

    <p class="mt-10 text-xl">If immediate removal is not feasible, a structured plan to phase out Eventlet must be put in place to ensure a stable and predictable user experience, that's the goal of this guide. Ignoring this issue means knowingly accepting a degraded user experience and exposing end users to frustration, lost productivity, and service disruptions—none of which are acceptable in a modern application.</p>

    <p class="mt-10 text-xl text-yellow-300"><strong>Would you buy a product that is known not to work?</strong></p>  
</section>

<section>
    <h2 id="our-motivations" class="mt-10 text-3xl font-bold">What Are Our Motivations <a href="#our-motivations" class="text-cyan-400 text-xl">🔗</a></h2>
    <p class="mt-10 text-xl">We are the core maintainers of Eventlet. We argument against the product we maintain. Do not get us wrong, this is not a gratuitous attack on someone else. We daily observe all these problems from the inside, and, we think, that it is our responsability to inform our end users.</p>
    <p class="mt-10 text-xl">We think, that it is our responsability to lead our end users toward a solution. This is why <a href="https://github.com/eventlet/eventlet/issues/824" class="text-cyan-400" target="_blank">we decided to abandon the maintenance of Eventlet</a> in <a href="https://review.opendev.org/c/openstack/governance/+/902585" class="text-cyan-400" target="_blank">a planified way</a>, and this is why we decided to create this guide.</p>
</section>
