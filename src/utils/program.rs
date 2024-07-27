// Mock data for the program to not massively spam the API
pub fn program() -> String {
    r#"
    {
"sessions": [
{
"intendedAudience": "Topic: General\nAudience: Everyone (no experience needed)\n\nWe live in the world of information chaos. Isn’t easier just to ask your colleague than to find an answer in documentation? Or probably there is no answer as the development team doesn't write documentation because of the \"it's obvious for me, so it's obvious to everyone\" trap. They don't write because they are too busy making the project exist. \nGood documentation remains the best communication tool for groups and projects. This is especially true considering that projects tend to get bigger over time.\nI will give an example of how to describe a routine process for creating documentation in a software development team, show tools and share tips how to do it. These are easy to implement and use in everyday software developers’ life.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Breaking news, Instagram Reels, TikTok, and YouTube Shorts,… Living in the era of information intoxication means the average person’s attention span was only ~8 seconds in 2022. No wonder it’s hard to conquer readers’ attention with your documentation. That’s why it’s important to make your documentation exciting and interesting to read still keeping it informative.\n\nDocumentation is communication. And it is an important part of a community builder. It is also a visit card of your project. Via documentation you bring a message to your clients or community. It is a way of communication. And without communication there is no sustainability, consistency, etc.\nHow to communicate to succeed with your community?\n\nDocumentation within a group or a company may also be a single source of truth.\nDocumenting processes and protocols may be even more important than documenting your project itself, especially if it is some kind of a library.\n\nAnother biggest issue in creating documentation is that your team doesn't know how to do it. If you make the process understandable and a standard one, there will be no problem.\n\nI will tell what types of documentation exist, share some tips and tools to simplify the process of writing documentation.",
"title": "Does reading documentation make your eyes glaze over? 5 tips to make documentation more successful.",
"id": "aa141e3a-32e5-4b53-997f-b1a875a3be32",
"sessionId": "aa141e3a-32e5-4b53-997f-b1a875a3be32",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Olga Merkulova",
"twitter": "",
"bio": "I am a solutions analyst and a software engineer. Started as a Java developer back in 2007. After 4 years in the big international companies, realized I am also keen on analysis and requirements capture. Founded an IT consulting company in 2016 and worked as a solutions analyst optimizing processes, writing prototypes and proof of the concepts for laboratories in the big EU research facilities such as ESFR, DESY. Now I am a software engineer and project manager of the Information System for Protein Crystallography in DESY.\n\nI am an open-source enthusiast and most of the projects I have been working on were open-source, really support the idea of knowledge sharing. \n\nLove traveling and learning different cultures. That is the reason I have lived in different EU countries and the US. One of my most memorable adventures is the US coast to coast road trip."
}
]
},
{
"intendedAudience": "developers interested in cross-platform development; project managers, technical leads and architects seeking insights into efficient code sharing strategies; anyone curious about Kotlin's capabilities & versatility beyond a single platform\n",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Do you have a complex business rule-set problem? You know the kind where you have business rules (logic) spread across multiple platforms, systems and applications? Some of them might even be alter egos of the same rule hiding in different apps. You never know which one might turn up where, there is no single source of truth, solving errors takes hours of debugging, and making changes requires (oftentimes) painful coordination across teams and applications (ugh). \n\nWe had that problem at Posten & Bring, and we solved it using Kotlin Multiplatform. \n\nIn my session I’m spilling the tea. \n\nWe’ll look through real-world use cases at Posten & Bring. We’ll reflect on what we did - key learnings, what went well and most importantly  - what didn’t quite go as planned. We’ll also look through some resources to get started. We'll learn how KMM provides a unified framework for code sharing and execution across various platforms to solve the challenge of different codebases targeting different runtimes.\n\nThis talk is for you if troublesome and unruly business logic keeps you up at night and you want that to end NOW. It is also for you if you like to hear a good story. ",
"title": "Unraveling Chaos with Kotlin Multiplatform: Story of how we brought order to Business Rules Spaghetti at Posten & Bring",
"id": "a1d9aeac-ffc3-4b1d-ba08-a0568f415a02",
"sessionId": "a1d9aeac-ffc3-4b1d-ba08-a0568f415a02",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Anshika Koul",
"twitter": "",
"bio": "Anshika has 10 years experience developing software at Posten Bring, building the logistics systems and APIs that enable millions of people in the Nordics to order and receive packages. She values studying users, and fostering cross-departmental collaboration to demystify unruly business rules. A typical day at work involves climbing stairs to the 17th floor of Posthuset, where she will transform whiteboard scribbles into deployed features. Off-duty you'll find her chasing sunsets and culinary adventures, sampling flavours from around the globe."
}
]
},
{
"intendedAudience": "Dette vil være en talk som er passende for utviklere som allerede jobber med Spring Boot, eller som kan tenke seg å utforske denne teknologien. Målet er at lytterene skal få et innblikk i mer deklarative metoder for å lage Spring Boot apper enn hva man tradisjonelt er vant til, og kunne få en bedre forståelse for hva \"automagien\" til Spring Boot vanligvis håndterer for deg. ",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Har du noen gang lurt på hvordan Spring Boot applikasjonen din fungerer, hva alle annotasjonene gjør, og ikke minst om det trenger å være på denne måten? Da er denne talken for deg. Jeg kommer til å vise konkrete kodeeksempler på hvordan man kan gjøre Spring Boot mer deklarativt og funksjonelt, og se på hvordan dette kan fungere veldig bra med Kotlin",
"title": "Deklarativ Spring Boot med Kotlin, mindre automagi",
"id": "e56a4835-0561-40db-a464-8644d99e2a85",
"sessionId": "e56a4835-0561-40db-a464-8644d99e2a85",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Ruben Eide Saur",
"twitter": "",
"bio": "Jobbet som utvikler i SpareBank 1 Utvikling DA i ca. 5 år, og har kjent på både fordelene og ulempene med Spring Boot"
}
]
},
{
"intendedAudience": "Developers who want to improve their code reading skills. \nExamples will be in Java, the IDE used will be IntelliJ IDEA.\nWhile this talk is probably more useful for beginners, more experienced developers might pick up a trick or two. ",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "As developers, we spend a lot of time learning to write code, while spending little to no time learning to read code. Meanwhile, we often spend more time reading code than actually writing it. Shouldn’t we be spending at least the same amount of time and effort improving this skill? Deliberate practice can help us get better at reading code. Learning how to better read and understand code, can in turn teach us what makes code readable. This might even help us to write code that is easier to read.\n\nIn this talk we will discuss the benefits of deliberately practicing reading code in a code reading club or session without an IDE, as well as common strategies to navigate a new codebase and familiarise ourselves with the code using the IDE.",
"title": "Reading code",
"id": "e6abb708-8f32-41a8-9c37-26346226de0b",
"sessionId": "e6abb708-8f32-41a8-9c37-26346226de0b",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Marit van Dijk",
"twitter": "@MaritvanDijk77",
"bio": "Marit van Dijk is a software developer with 20 years of experience in different roles and companies. She is a Java Champion and works as a Developer Advocate at JetBrains. She loves building awesome software with amazing people and making developers lives better.\n\nShe enjoys learning new things as well as sharing knowledge on programming, software development, testing & test automation, and more. She has contributed to open-source projects like Cucumber and several other projects.\n\nMarit speaks at international conferences, in webinars, and on podcasts, occasionally writes blog posts, and contributed to the book “97 Things Every Java Programmer Should Know” (O’Reilly Media)."
}
]
},
{
"intendedAudience": "Developers that interact with data both at frontend & backend",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "For a long time the first thing that came to mind when mentioning databases was relational data and SQL. This changed when the NoSQL wave came and an explosion of several formats for storing data came to light, with JSON becoming a favorite because it gain traction as the preferred format for interchanging data between clients (mobile & browsers) and servers. Both JSON and relational formats have their advantages and disadvantages, we'll have to pick one when implementing a given solution. But, this is not a zero-sum game. What if we're able to use both formats inside the same database engine? In this session we'll explore different options that relational databases offer for dealing with JSON data.",
"title": "Dealing with JSON in the relational world",
"id": "75aac348-0650-49ba-afdb-0e97a186ec77",
"sessionId": "75aac348-0650-49ba-afdb-0e97a186ec77",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Andres Almiray",
"twitter": "@aalmiray",
"bio": "Andres is a Java/Groovy developer and a Java Champion with more than 2 decades of experience in software design and development. Andres is a true believer in Open Source."
}
]
},
{
"intendedAudience": "Developers/architects interested in the \"cloud native\" ecosystem and improving build pipelines and Continuous Delivery\n",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Progressive Delivery makes it easier to adopt Continuous Delivery, by deploying new versions to a subset of users before rolling them to the totality of the users, and rolling them back if not matching some key metrics, using techniques like feature flags and canary deployments.\n\nFor workloads running on Kubernetes it is very easy to adopt Progressive Delivery using Argo Rollouts. At Adobe Experience Manager we deploy over 10k customer services to Kubernetes. Changes can occur multiple times per day both internal and from code. A new feature can work fine for 99% of customers but still affect the other 1%, and detecting this just from tests is costly.\n\nWe will show how to implement a Progressive Delivery pipeline with Argo Rollouts to improve the reliability of the service and prevent regressions. It allows the protection of the service and automation of roll backs to a previous version if needed. This allows for faster delivery with more confidence.\n",
"title": "Progressive Delivery Made Easy with Argo Rollouts",
"id": "6d5b81c2-8fdc-4f04-95a7-9cb2c4aa06ab",
"sessionId": "6d5b81c2-8fdc-4f04-95a7-9cb2c4aa06ab",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Carlos Sanchez",
"twitter": "csanchez",
"bio": "Carlos Sanchez is a Principal Scientist at Adobe Experience Manager, specializing in software automation, from build tools to Continuous Delivery and Progressive Delivery. Involved in Open Source for over 15 years, he is the author of the Jenkins Kubernetes plugin, contributor to Kubernetes, and a member of the Apache Software Foundation amongst other open source groups.\n"
}
]
},
{
"intendedAudience": "Java Developers and architects. Anyone interested in beautiful code.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Often times Java SDKs for APIs look more like the API they represent than idiomatic Java.\n\nThis talk examines how great SDKs are built using design patterns.\n\nIt covers: enforced separation of Java api and implementation, use of generics, a caching layer and hiding network traffic.\n\n---\n\nThe (now defunct) Stormpath SDK had a great design approach from its original author, Les Hazlewood.\n\nIt has an enforced separation of api and implementation. It does this by using the api module as a compile time dependency and the implementation module as a runtime dependency.\n\nIt has a DataStore interface that makes heavy use of generics to support CRUD operations for all objects represented in the API. The implementation hides the actual network traffic and includes rich support for retry with backoff and error handling. Developers only ever have to deal with the DataStore, POJOs and method calls to “interact” with the API.\n\nIt also has a rich, interface-based caching layer. The default implementation is robust and suitable for single-JVM environments. It’s easy to drop in a distributed caching layer, such as Redis or Hazelcast.\n\nAll of this combined makes this one of the best designed SDKs in Java. In this talk, all these secrets are revealed against a completely different API: DigitalOcean’s Droplet API. There’s a few slides and lots of code, including some live-coding.",
"title": "Beautiful SDK Design in Java for APIs",
"id": "2239b7c7-18e0-4aaa-95a3-f7aa3134528d",
"sessionId": "2239b7c7-18e0-4aaa-95a3-f7aa3134528d",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Micah Silverman",
"twitter": "",
"bio": "Micah Silverman leads Security Advocacy for Snyk. With 29 years of Java Experience (yup, that's from the beginning), he's authored numerous articles, co-authored a Java EE book and spoken at many conferences. He's a maker, who's built full size MAME arcade cabinets and repaired old electronic games (http://afitnerd.com/2011/10/16/weekend-project-fix-dark-tower/). He brings his love of all things Security and Java to a conference near you!"
}
]
},
{
"intendedAudience": "Utviklere med interesse for databaser.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Datomic er en jæskla interessant database fordi den er så radikalt annerledes. For de av oss som er melka opp på SQL, så er det spennende å se hvor nytt det kan tenkes rundt lagring og henting av data. Her forteller jeg litt om hva jeg har lært etter 11 år med Datomic og hva som gjør databasen så interessant.",
"title": "11 innsikter etter 11 år med den rare databasen Datomic",
"id": "bbbc0c4c-bb2e-4198-8c27-ffba112e3d63",
"sessionId": "bbbc0c4c-bb2e-4198-8c27-ffba112e3d63",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Magnar Sveen",
"twitter": "magnars",
"bio": "Jeg heter Magnar og er en glad programmør hos Mattilsynet. Jeg har laget noen videoserier (ZombieTDD, Emacs Rocks! og Parens of the Dead), holder gjerne på med sånn open source-greier, og er begeistret for funksjonell programmering."
}
]
},
{
"intendedAudience": "This session will mainly be for junior java developers or developers that never really thought about Garbage Collection but would like to know more. They don't need to be experienced to attend this session. This is NOT a garbage collector deep dive session but should more explain the principles and ideas behind the existing garbage collectors.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "In the realm of Java programming, understanding memory management and garbage collection mechanisms cannot only be helpful for optimizing performance and resource utilization but also in general will help you to use the right garbage collector for your application. This session will be about memory allocation, object lifecycle, and garbage collection strategies within the Java Virtual Machine (JVM). Attendees will gain insights into memory allocation strategies, object retention policies, and various garbage collection algorithms like generational and concurrent collection. So when you are interested in getting more knowledge about which garbage collector to use best for your application or about memory management in the JVM, this session is for you.",
"title": "TrashTalk - Exploring the JVM memory management",
"id": "842aa401-587b-4553-a17a-f01e099722b3",
"sessionId": "842aa401-587b-4553-a17a-f01e099722b3",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Gerrit Grunwald",
"twitter": "@hansolo_",
"bio": "Gerrit Grunwald is a software engineer that loves coding for around 40 years already. He is a true believer in open source and has participated in popular projects like JFXtras.org as well as his own projects (TilesFX, Medusa, Enzo, SteelSeries Swing, SteelSeries Canvas, JDKMon). \nGerrit blogs regularly at http://harmonic-code.org, he is an active member of the Java community, where he founded and leads the Java User Group Münster (Germany), he is a JavaOne rockstar and a Java Champion. He is a speaker at conferences and user groups internationally and writes for several magazines."
}
]
},
{
"intendedAudience": "Utviklere og designere som er eller kan bli involvert i innkjøpsprosesser\n\nPresentasjonen krever ingen forkunnskaper, så passer for alle interesserte",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Både utviklere og brukere må forholde seg til innkjøpte systemer. Ofte kan de være tunge å bruke, og enda vanskeligere å integrere med nye og eksisterende IT-løsninger.\n\nKrav om APIer og design må komme når systemet kjøpes inn. Jeg deler mine erfaringer med innkjøp av nytt køsystem til legevakta i Oslo. Her jeg satte APIer på agendaen og stilte de nødvendige spørsmålene.",
"title": "De beste råvarene finner kokkene selv - en utviklers erfaringer fra ny legevakt i Oslo",
"id": "0b2b793d-124f-4ae4-94b0-14c20cad75b0",
"sessionId": "0b2b793d-124f-4ae4-94b0-14c20cad75b0",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Marit Ånestad",
"twitter": "",
"bio": "Marit Ånestad har 12 års erfaring som utvikler. Hun er opptatt av å lage gode løsninger for brukerne. De siste to årene har hun jobbet i Oslo Origo  med systemer for legevakta i Oslo"
}
]
},
{
"intendedAudience": "This talk is primarily for developers and technical architects who are interested in solving problems in a distributed environment using Redis, although techniques and algorithms described in this talk can also be implemented using other tools like PostgreSQL. Some basic coding skills would be beneficial to understand the code examples given in this talk.\n",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "If you are designing or building back-ends which are deployed as multiple instances, then chances are you  have come across challenges which require some sort of coordination across those instances. Redis is a brilliant lightweight in-memory key/value store which is easy to reason about, super fast and perfect for sharing short-lived state across instances to facilitate things like transactions, data consistency and enable reactive applications.\nWelcome to this deep dive into how we use Redis at the Norwegian Tax Administration to solve problems like: distributed locks, pushing events to frontends to create a responsive experience, managing background processes, producing large file downloads without blocking the frontend and managing Kafka consumers using Redis.\n",
"title": "How Redis plays a key role in the world's coolest case management solution at the Norwegian Tax Administration",
"id": "0748c9b0-f779-47af-ae5b-ef3fa678de00",
"sessionId": "0748c9b0-f779-47af-ae5b-ef3fa678de00",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Sebastian Dehne",
"twitter": "",
"bio": "Sebastian Dehne is a senior consultant at Systek, with 20+ years experience in software engineering. His main expertise and passion lies in designing and building systems with regards to distributed computing, transaction handling, scalability, robustness and zero-downtime.\n"
}
]
},
{
"intendedAudience": "Programmerere som jobber med eller kanskje kommer til å jobbe med en kodebase med Spring.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Jeg programmerer i Spring. Jeg lærer ting hver dag. Noen ganger går det galt. Her deler jeg tre feil jeg har gjort, som jeg egentlig visste var feil. Her deler jeg dem slik at du kanskje kan unngå dem.\n",
"title": "Tre spring grøfter jeg har vært i",
"id": "ef8f75cf-3c26-4c8d-b3b8-b9bd6ed9f9d7",
"sessionId": "ef8f75cf-3c26-4c8d-b3b8-b9bd6ed9f9d7",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Anders Karlsen",
"twitter": "",
"bio": "Anders har blitt betalt for å programere i over 20 år. Han har jobbet både i store offentlige prosjekter og små private, Han får nå betalt som konsulent ansatt i Jpro. På fritiden hjelper han til med å arrangere JavaZone"
}
]
},
{
"intendedAudience": "This talk is aimed at developers or infrastructure folks who use git as a part of their day job, but who are uncomfortable with their level of understanding of how git really works. Familiarity with basic usage of git (status/diff/add/commit/checkout) is assumed. However, if you understand how commits, trees, blobs, and the index are structured and how they work together, you already know what this talk will cover.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Do you use git while having a nagging feeling that you don't quite understand what's going on? Are you comfortable with day-to-day diffing, committing, and checking out, but struggle once you get to a complicated merge or rebase? Do you prefer not to think about what that \"index\" thing really is that the docs talk about all the time? This talk will explain the internal data model of git and connect it to the everyday git commands, to help you overcome tricky git situations.",
"title": "Git demystified",
"id": "bd6dd20a-4da6-4212-bed0-851b25a2b791",
"sessionId": "bd6dd20a-4da6-4212-bed0-851b25a2b791",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Åsmund Eldhuset",
"twitter": "@aasmundeldhuset",
"bio": "Åsmund Eldhuset is a geek and a jack-of-all-trades software engineer who loves to spread knowledge about complicated subjects. He is a Principal Software Engineer at Cognite, and occasionally provides professional training at his side business Eld."
}
]
},
{
"intendedAudience": "Experienced Java programmers who also need to do scripting/exploratory programming. ",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "For small projects, we often reach for a scripting language, such as Python or Bash, because Java seems too heavy handed. In this interactive presentation, I want to convince you to give Java a try for those tasks. With the right tooling (and some innovative features in Java 21 and 22), Java can be a great choice for scripts, Jupyter notebooks, and even code in the browser. The benefit: compile-time typing, great IDEs, and a growth path when those small projects don't stay small.",
"title": "Java for Small Coding Tasks",
"id": "d58af4f0-c5d7-4231-93c3-6dae254ce684",
"sessionId": "d58af4f0-c5d7-4231-93c3-6dae254ce684",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Cay Horstmann",
"twitter": "",
"bio": "Cay has been involved with Java since 1995. In his copious spare time he writes books, including the international best-seller Core Java, and develops online learning experiences for beginning and professional programmers. "
}
]
},
{
"intendedAudience": "Back-end software developers and architects",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Too often, truly intelligent software seems achievable only by AI experts wielding complex algorithms. What if that isn't the case? In this talk, you'll discover how leveraging basic event-driven architecture allows coherent behaviors to emerge spontaneously; no advanced degree is required!\n\nDrawing from biology and complex systems research, I'll explain how decentralized software components exhibiting simple stimulus-response logic can self-organize into an intelligent entity with capabilities far exceeding the sum of its parts. Like neurons forming a brain, the interactions between individual \"dumb\" nodes lead to emergent intelligence.\n\nYou will learn how shifting day-to-day coding practice toward reactive, event-driven programming opens the door to building surprisingly capable and scalable behaviors into your application architecture. I'll share Java code examples showing how this plays out in practice. You'll also see a unique 3D visualization showing how your software functions more like a mind than a ball of mud. \n\nLearn how clever, emergent behavior software does not exclusively belong to Ph.D. Roombas. By intelligently relinquishing control and letting order bubble up from the ground floor, developers of all skill levels can slash complexity while enabling their systems to display dynamic decision-making, environmental adaptation, resilience, and more!",
"title": "No PhD Required: How to Build Smart Software Systems on Your Own",
"id": "2315a135-faef-4f1b-b33f-760681e0679c",
"sessionId": "2315a135-faef-4f1b-b33f-760681e0679c",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Hugh McKee",
"twitter": "mckeeh3",
"bio": "Hugh McKee is a skilled back-end developer and developer advocate with decades of experience building enterprise applications, specializing in large-scale, cloud-based systems. As a developer advocate, he speaks at conferences worldwide. With a passion for teaching and evolving his software engineering skills, Hugh is a valuable contributor to the tech community."
}
]
},
{
"intendedAudience": "This talk is for anyone.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "How do hidden vulnerabilities become a hacker's treasure trove and how can they be avoided? This talk addresses the underlying security issues that arises during application development, highlighting how a shift in mindset among developers and management can preemptively address and mitigate these risks.\nBy the end, you'll gain actionable insights to fortify your applications against unseen digital threats.",
"title": "A hackers guide to software engineering",
"id": "a50b04e0-f254-44cc-8aaa-80cff1e02245",
"sessionId": "a50b04e0-f254-44cc-8aaa-80cff1e02245",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Martin Londal",
"twitter": "",
"bio": "Martin Londal, formerly a developer, now works as a penetration tester and security advisor at the Police IT Unit(PIT) in Oslo. I am one of the leaders of PIT's Security Champions, focusing on imparting security concepts through hands-on training and engaging presentations. I also lead PIT's after-work hacking team, Abyss. I travel to schools where I talk about a career in IT with a special focus on a job role in cybersecurity. I also holds talks, training and presentations for public & private cooperation focusing on cybersecurity for both technical and non-technical staff."
}
]
},
{
"intendedAudience": "Interested into information security.\nWondering how the security industry manage to detect incidents.\nWanna hear about challenges and solutions with analyzing \"big\" data.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "The lightning talk will describe how the security incident detection pipeline works. And how to scale to be able to handle 70 billion security events a day. In order to find potential hacker activities.",
"title": "Catch the hacker among 70 billion events per day",
"id": "29f83286-431b-4cba-b555-5974465ed422",
"sessionId": "29f83286-431b-4cba-b555-5974465ed422",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Kai Cao",
"twitter": "",
"bio": "Team lead of Argus Distributed team at mnemonic AS, responsible for develop and maintain incident detection pipelines.\nBackend developers for 14 years. Worked with projects such as Rovio cloud platform to support Angry birds games. "
}
]
},
{
"intendedAudience": "People working with people from other disciplines",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Most of my career I have been a part of the majority. As a developer working in large software organizations, most of the people I work with are also developers. Technical people. Same kind of technical as me. Programmers. Of course with other disciplines mixed in, but always outnumbered by programmers.\n\nA few years back I joined new organization. Suddenly I was not part of the majority. Software developers were not at the top of the food chain there either. And our competency were scarce compared to other discipline. This made me think.\n\nNow I am back in a typical software development set up, again, a part of the majority. However, my experiences from being a minority has changed how I interact with the minorities that I now work with.",
"title": "Minority Report",
"id": "f499a603-f463-4903-be18-5fb0a13fb0a9",
"sessionId": "f499a603-f463-4903-be18-5fb0a13fb0a9",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Janniche Lange",
"twitter": "",
"bio": "Developer @ Storebrand. I like pair programming and delivering value to customers. Care about testing, readable code and frequent deliveries. And naming things. Also people and interactions more than processes and tools. And agile.\nInvolved in communities for knowledge sharing; javaBin, Stavanger Agile Meetup and the new software conference HelloStavanger.no\n"
}
]
},
{
"intendedAudience": "De som jobber i produktutviklingsteam og ledere knyttet digitalisering og produktutvikling.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Etter alt maset om at jurister må involveres i produktutvikling - hvordan er ståa? \n\nHar juristen blitt en masete bedreviter som tvinger seg på overalt? Eller, er juristen fremdeles en fjern slektning som ingen forholder seg til med mindre man må? Ligger jussen nå som en klam hånd på alle kreative initiativ og utviklerne får ikke jobbe i fred?  Eller har juristen blitt en naturlig del av  gjengen som bringer noe verdifullt til bords? \n\nVi vet ikke hva DU tenker, men vi har snakka med folka hos oss og vet hva vi selv mener. I Oslo Origo har vi inkludert jurister helt inn på teamnivå i produktutviklingen, og vi har fått erfaringer på godt og vondt. Disse erfaringene vil vi dele med dere! \n\nMed denne presentasjonen vil vi utfordre deg til å reflektere over din egen organisasjons kultur og kanskje se nye muligheter for hvordan jurister og utviklere kan samarbeide mer effektivt.\n",
"title": "Jurist i produktutvikling - hår i suppa eller din beste venn?",
"id": "591c5bad-d2e0-4c39-95f4-df3fe33a10db",
"sessionId": "591c5bad-d2e0-4c39-95f4-df3fe33a10db",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Kathinka Fjærli",
"twitter": "",
"bio": "Kathinka er utdannet jurist fra Universitetet i Oslo. Hun har lang erfaring med juss i krysningfeltet teknologi og tverrfaglighet, fra Tolletaten, Skattetaten og nå Oslo Origo. I Origo jobber hun i produktområdet \"Bo hjemme lenger\" med fokus på personvern og generell jus. Hun jobber aktivt inne i produktutviklingen sammen med både utviklere, designere, organisasjonsutviklere, forretningsutvikler og teamlead."
},
{
"name": "Siri Eriksen",
"twitter": "",
"bio": "Siri har mange års erfaring med å jobbe med de særlige juridiske problemstillingene som oppstår ved digitalisering og datadeling. Hun har blant annet erfaring fra Skatteetaten, Nasjonalt ressurssenter for deling av data (Digdir) og jobber for tiden i Oslo Origo der hun er juridisk ressurs i produktområdet Helsedata i Oslo.\n"
}
]
},
{
"intendedAudience": "Backend engineers, microservice developers, security engineers and architects, DevOps\n\nThe attendees will learn about the upcoming IETF specification which has been created to help make the internal microservice perimeter more secure. The talk will also feature a case study for Transaction Token Service implementation with a stripped-down, stateless variant of Keycloak, an highly extensible lightweight open-source IAM platform.\n\nThe participants should be familiar with OAuth and OpenID Connect, and should also understand the concept of workloads. Experience with Keycloak will be a plus.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "For the modern computing architectures involving multiple independent workloads, it is important that the calls between the workloads be properly authenticated and authorized. SPIFFE/SPIRE does solve the authentication part; however, it does not take into account the request context and other dynamic data.\n\nA new Internet draft called Transaction Tokens has been created by the IETF OAuth Working Group in order to address the authorization part. A transaction token is a short-lived, cryptographically signed, request-specific token obtained from the new Transaction Token Service in exchange for the external OAuth/OIDC access token and other context-dependent data. The token is then included into every inter-workload call, which guarantees that only non-spurious calls between the workloads can take place. From this talk, the attendees will learn about how Transaction Tokens work, how they help to make the internal perimeter more secure, how we implemented this upcoming specification using a customized version of Keycloak, what challenges we faced and how we solved them.",
"title": "Securing workloads with Transaction Tokens and Minicloak",
"id": "5e1526f2-c6a2-463a-8a6b-e20080f72ccb",
"sessionId": "5e1526f2-c6a2-463a-8a6b-e20080f72ccb",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Dmitry Telegin",
"twitter": "",
"bio": "Principal backend engineer at Backbase UK, independent Keycloak expert and contributor, IETF OAuth WG participant.\n\nLinkedIn: https://www.linkedin.com/in/d-telegin/"
}
]
},
{
"intendedAudience": "open for everyone",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "TV 2 is known by many for being one the most popular television broadcasters, while also being the largest commercial broadcaster in Norway. TV 2 is also the fourth largest online news outlet in Norway. But keeping everything alive requires efficient infrastructure, emergency response ability, availability and capacity planning.\n\nAt TV 2 our teams and developers create products that makes all of TV2's offers and content possible, and it is made possible through ElasticSearch, Aiven, Kafka and other platform-centric tech. In this talk I will share our real-life experiences with On-Site Reliability (SRE) within a big media outlet. You will not only how we do SRE, but more importantly we are able to coordinate the communication with all our teams across the whole of Norway.\n\nI will take you behind the scenes to show how we take care of some of the most important infrastructure that is critical for not only broadcasting but also all the online content. After listening to this talk you will have a good understanding of how modern open-source technology makes the foundation of TV 2.",
"title": "On-Site Reliability For Uninterrupted Broadcasting at TV 2",
"id": "2936cb83-f8bc-40e6-926e-a6a460f90600",
"sessionId": "2936cb83-f8bc-40e6-926e-a6a460f90600",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Ewa Kabza",
"twitter": "ewakabza",
"bio": "System architect at TV 2. Member of javaBin (the team behind JavaZone). I have M.Sc. in architecture, model-based system development & teaching."
}
]
},
{
"intendedAudience": "Alle som er nysgjerrige på hvordan fremtidens strømnett blir utviklet. ",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Kraft er blitt en mangelvare, strømmen blir dyrere og samtidig foregår en hard kamp om tilgangen på den. Industrien står i kø for å koble seg på strømnettet og bedriftene stilles harde krav. Økningen av ulike fornybare energikilder medfører uforutsigbarhet og utfordrer tradisjonelle måter å drifte på. For å møte disse utfordringene står kraftbransjen ovenfor en radikal omstilling av energisystemet – verdens største og mest kompliserte maskin.\n\nDerfor satser Statnett stort på utvikling av automatiserte løsninger som tar oss fra reaktiv til proaktiv systemdrift. I dette foredraget skal jeg fortelle om hvordan vi bruker prediktive metoder for å varsle om kommende driftsforstyrrelser og hvordan dette tillater oss å presse ut de siste elektronene fra strømnettet imens vi ivaretar forsyningssikkerheten.",
"title": "Hvordan Statnett gjør Smartnett",
"id": "09e48339-45a0-4ce5-8688-e1f5660d6729",
"sessionId": "09e48339-45a0-4ce5-8688-e1f5660d6729",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Nemanja Lakicevic",
"twitter": "",
"bio": "Nemanja Lakicevic jobber som utvikler i Team Volt på Statnett, elsker all-things Linux og bruker (for?) mye tid på å konfigurere vim og unix ricing. Er interessert i alle lag av techstacken, men fascineres mest av SRE hvor han skrev\nmasteroppgave om bruk av eBPF for sikkerhetsmonitorering. Oh, I use Arch btw. \n"
}
]
},
{
"intendedAudience": "The story is mainly from a developers perspective. There other developers will benefit from attending this talk. It helps if participants have a few years of experience to recognise the problems I highlight.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Ever heard of the IKEA effect or the bandwagon-effect? In our efforts to delivering great software, there are numerous occasions where our brain tries to trick us. Don’t fall for them!\n\nIn this talk we will dive into some well-known and researched cognitive biases that impact our judgments in our day-to-day jobs as software creators.\nWhen do they typically occur? And why are we falling into these traps from time to time?\n\nAfter this talk you'll have a better understanding of how your brain tricks you, how to make better decisions and in the end how to deliver some great software.\n\nA word of warning; during this talk you may get tricked!",
"title": "Battling your Biased Brain",
"id": "049ff325-bc59-4582-9290-b8bde1453e5c",
"sessionId": "049ff325-bc59-4582-9290-b8bde1453e5c",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Peter Wessels",
"twitter": "@peterwessels",
"bio": "Peter Wessels is a Java Developer, Speaker, and Teacher at Info Support in the Netherlands. In addition, he leads the Java Community within Info Support. \n\nHe loves working with talented people to develop software that's not only effective but also fun to create. In his career, Peter has worn many hats, from lead engineer and project manager to scrum master and product owner. In Dutch, he would be described as a 'manusje van alles,' which roughly translates to a person who seizes every opportunity to learn new technologies and (soft)skills to create impact. This attitude helps him get diverse perspectives on software development. \n\nAs a speaker, Peter has had the privilege of presenting at prestigious events such as Devoxx Belgium, Devnexus, Devoxx UK, J-Fall, and Developer Week."
}
]
},
{
"intendedAudience": "Java devs that love to work with things like Chat GPT and Co-pilot",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Explore the world of generative AI with us and learn about the hidden challenges it presents. Are you intrigued by how tools like Copilot, ChatGPT, and Code Whisperer can revolutionize your coding workflow? But did you know that GenAI can also introduce (new) problems? This hands-on session is designed for developers eager to leverage the speed of AI while avoiding the traps it might set, such as misleading information or \"AI hallucinations\" that compromise the security and reliability of your projects.\nJoin us as we live code a Spring coffee shop application, demonstrating in real-time how to spot and fix vulnerabilities introduced by AI, from simple SQL injections to more complicated issues. This session is crafted to provide practical insights into identifying and addressing the security risks of AI-generated code.\nThis session is all about balancing the excitement of fast-paced development with the crucial aspect of security. Maximizing AI's potential while safeguarding your code against its hidden dangers. \n",
"title": "Secure Your Java: AI Hallucinations Demystified",
"id": "a01a3109-e019-413f-89ce-426f3ebda191",
"sessionId": "a01a3109-e019-413f-89ce-426f3ebda191",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Brian Vermeer",
"twitter": "@BrianVerm",
"bio": "Brian Vermeer is a Staff Developer Advocate for Snyk, Java Champion, and Software Engineer with over a decade of hands-on experience in creating and maintaining (web)applications. He is passionate about Java, (Pure) Functional Programming and Cybersecurity. Brian is a JUG leader for the Virtual JUG and the NLJUG. He also co-leads the DevSecCon community and is a community manager for Foojay. He is a regular international speaker on mostly Java-related conferences like JavaOne, Devnexus, Devoxx, Jfokus, JavaZone and many more. Besides all that, Brian is a military reserve for the Royal Netherlands Air Force and a Taekwondo Master / Teacher."
}
]
},
{
"intendedAudience": "Engineers who are responsible for running services in production. They will learn about best industry practices in general, and lessons learned at Grafana when adopting these best practices.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "This talk highlights industry best practices for setting up a healthy SRE culture, and lessons learned when implementing these practices at Grafana labs.\n\nWe will cover dashboarding strategies, alerting best practices, healthy on-call team structures, service level objectives, how to manage incidents, production readiness reviews, post mortems, and more.\n\nWe'll use Grafana as an example, but we believe that the general lessons learned and best practices are independent of the tools being used, and can be applied in any engineering organization.",
"title": "The SRE Journey - Best Practices at Grafana Labs",
"id": "68f693b8-1003-4d89-a5d1-56d5db4d05a5",
"sessionId": "68f693b8-1003-4d89-a5d1-56d5db4d05a5",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Fabian Stäber",
"twitter": "",
"bio": "Dr. Fabian Stäber is engineering manager and observability enthusiast at Grafana. He is a member of the Prometheus open source project, where he is maintainer of the Prometheus Java client library and the JMX exporter. At Grafana Fabian has his focus on application observability with OpenTelemetry."
}
]
},
{
"intendedAudience": "Frontend developers, no experience needed.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Det er blitt en hype rundt signals i det siste, flere og flere frontendrammeverk kaster seg på. Men hva er egentlig signals? Og hvordan ser TC39 sitt proposal ut for at det implementeres direkte i JavaScript?",
"title": "Signals i frontend, og kan det bli en JavaScript standard?",
"id": "d1f7e34d-c482-4fab-bfb9-4f1e6cad695e",
"sessionId": "d1f7e34d-c482-4fab-bfb9-4f1e6cad695e",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Gaute Meek Olsen ",
"twitter": "GauteMeekOlsen",
"bio": "Gaute er en utvikler med ekstra lidenskap for frontend. Gaute er konsulent i Capra og med i fagteamet frontend. "
}
]
},
{
"intendedAudience": "Anyone who works in a team or with teams. The audience will receive valuable input to start the necessary disussions regarding team autonomy that need to happen both within the team and with the teams surroundings. ",
"length": "10",
"format": "lightning-talk",
"language": "en",
"abstract": "The goal of the talk is to give the audience some input to the necessary disussions regarding team autonomy that need to happen both within the team and with the teams surroundings. The talk discusses different areas in which a team can be autonomous, and what might constitute a low and a high level of autonomy within these areas. In addition, I give input to what type of organizational or situational settings might play a part in how autonomous a team can ever become. This is a presentation for anyone who is a part of a team or who contributes to the teams goals and direction.\n",
"title": "Autonomy aint anarchy ",
"id": "1c226c1a-4a0f-44ae-b5b5-c63f6ef96f09",
"sessionId": "1c226c1a-4a0f-44ae-b5b5-c63f6ef96f09",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Anne Høymyr",
"twitter": "",
"bio": "Anne leder  seksjonen for prosjektledere og teamledere i Politiets IT-enhet og vil sette søkelyset på at det ikke bare er å sette sammen tverrfaglige team og tro at de blir autonome av seg selv. Både innad i teamet og med teamets omgivelser må man diskutere: \"Hvordan bør teamautonomi se ut for vårt team?\""
}
]
},
{
"intendedAudience": "This talk is for all Java developers who would like to taste functional programming. Modern Java seems to take a lot of inspiration from Scala - after this talk you will be able to think in a more functional way and implement some functional concepts in Java.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "For newcomers with a Java background, Functional Programming can seem counter-intuitive at first. Using immutable values and data structures, pattern matching, handling errors without exceptions, or avoiding dependency injection frameworks, not to mention dealing with abstractions like product/union types or monads - all these can blow your mind. But fear not the unknown, my friend. \n\nIn this session I’m going to demonstrate to you a couple of common battle-proven techniques from the Scala (and FP) world that can be easily applied to modern Java. Come and see how you can write even better code!",
"title": "Pain-free Functional Programming with Java 22",
"id": "6aab0e8e-0851-4450-9b39-a3555949cb03",
"sessionId": "6aab0e8e-0851-4450-9b39-a3555949cb03",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jacek Kunicki",
"twitter": "@rucek",
"bio": "I'm a passionate software engineer living in the JVM land, where I mostly code and teach Scala and functional programming.\n\nI believe that software craftsmanship is technology-agnostic, thus I try not to limit my portfolio to a narrow set of technologies.\n\nWhen sharing my knowledge, I always keep in mind that a working example is worth a thousand words, so you are very likely to see me in action during a live coding session."
}
]
},
{
"intendedAudience": "Man bør helst ha brukt noe kodekvalitetsverktøy før, men ellers ingen erfaring påkrevd.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Har du opplevd verktøy som skal holde kvaliteten på koden oppe, gjør det motsatte? Jeg også.\n\nI dette foredraget skal vi se på hvordan slike verktøy, som er satt opp i beste mening, i noen tilfeller kan skape frustrasjon og hvordan utviklere finner kreative måter å omgå reglene på.\n\nVi skal se på for eksempel \"hvordan lure code coverage\" og noen kreative omveier rundt regler og standarder satt opp for å sjekke koden. Kanskje er det reglene som bør endres, eller er det utviklerne som må bruke verktøyene annerledes? Forhåpentligvis får jeg deg til å tenke litt, men ikke forvent noen fasitsvar.\n\nHusk, dette foredraget er ment for å være morsomt og jeg anbefaler IKKE på noen som helst måte å bruke noen av disse \"triksene\" selv.",
"title": "Statisk kodeanalyse for den late utvikler",
"id": "b87dfe61-03da-409b-90dc-41006f1e1fff",
"sessionId": "b87dfe61-03da-409b-90dc-41006f1e1fff",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jarle Hansen",
"twitter": "",
"bio": "Jarle har mer enn 17 års erfaring som utvikler og arkitekt og er ansatt i Systek.\nHan har lenge hatt en kjærlighet for Kotlin, og var med på å starte opp og driver Oslo Kotlin meetup.\n\nHar erfaring fra en rekke bransjer som feks bank og finans, telekom, olje og gass og er for tiden hos Skatteetaten hvor han jobber med skyplattformen."
}
]
},
{
"intendedAudience": "Alle kan delta på dette foredraget, men de som får mest ut av dette vil være de som er interessert i å utvikle seg selv enten personlig eller som en form for leder.",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Det er to grunner til at alle bør bry seg om transformerende ledelse. 1) Ledelse er ikke noe som blir gjort på andre, men det er en gjensidig prosess. 2) Vi er alle ledere, og hvis du vil være del av et høytpresterende selvorganisert team, så må du ta din del av ansvaret. \n\nI dette foredraget går vi kort gjennom de viktigste elementene i transformerende ledelse, før vi dykker inn i et lederutviklingsrammeverk basert på forskning innen utviklingspsykologi. \n\nEtter foredraget vil du sitte igjen med et nytt perspektiv på deg selv som leder og teammedlem. Du også vil sitte igjen med konkrete tips om hvordan du kan utvikle deg personlig og i din karriere. Og ikke minst du vil vite hvordan du bedre kan lede og møte andre der de faktisk er, på en måte som gir mening for dem. ",
"title": "Transformerende ledelse for alle",
"id": "93598c63-5b10-483b-bd5f-ee2e71b6a1e3",
"sessionId": "93598c63-5b10-483b-bd5f-ee2e71b6a1e3",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "John Inge S Hervik",
"twitter": "",
"bio": "John Inge er en engasjerende Enterprise Agile Coach med mer enn 15 års erfaring i smidige arbeidsmåter og organisasjoner. Han begynte som utvikler med interesse for metodikk i arkitektonisk arbeid, og har tatt videreutdannelse i psykologi og profesjonell coaching (sertifisert på PCC-nivå fra ICF). Han bruker sin kunnskap og reflekterende praksis for å støtte organisasjoner, team og ledere til å utvikle seg mot høyere prestasjon og bedre resultater. Hans fremgangsmåte basert på systemisk coaching og vertikal ledelsesutvikling, hjelper kundene hans til å navigere i de komplekse tidene vi lever i, og han får stadig skryt for å kunne engasjere og lede andre til nye perspektiver. John Inge tar oppdrag som mentor og coach for ledere, team, smidige coacher, produkteiere med flere – i tillegg til at han holder foredrag, kurs og workshops. John Inge leder coaching- og kursarbeidet for Adventures with Agile i Norden."
}
]
},
{
"intendedAudience": "Developers/architects collecting ideas and inspiration for integrated solutions.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Device integrations have for decades been a problem for many business web apps. Input devices, printers, sensors and security gadgets would have their use case in many typical business apps. The \"sandbox\" by browsers has been worked around with several hacks ranging from browser plugins to native wrappers, but native apps have ruled when you need to connect to custom devices. Emerging new web standards may be changing the game forever.\n\nThis presentation will overview the possibilities (including workarounds and dirty hacks) to connect to various devices from your web apps. As a hands-on code example, we use Web Bluetooth API available in Chrome to collect ECG precise data from a wireless heart monitor belt. The data is then moved over to a Java server for further analysis.",
"title": "Web apps and device integrations - Analyzing heart arrhythmias using Java and Web Bluetooth API",
"id": "f104c1a4-3142-48eb-9e85-3e23bbe9f3fc",
"sessionId": "f104c1a4-3142-48eb-9e85-3e23bbe9f3fc",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Matti Tahvonen",
"twitter": "@MattiTahvonen",
"bio": "Java and OSS enthusiast working at Vaadin.\n\nMatti Tahvonen has worked on web apps for over two decades. The first-ever JS developer at Vaadin has turned into Java/JVM enthusiast over the years when working with a pure Open Source Java web framework and doing some business on top of it. When Matti is not working, time is mostly spent on nature-related hobbies and software to support those."
}
]
},
{
"intendedAudience": "If you want to solve your tasks faster and be more effective, this talk is for you",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "There are myriads of ideas on how to improve the way you work inside software and outside. They all boil down to two simple tricks: Understand the goal state and check your thinking at multiple levels. In this talk, I will show examples on how to approach programming tasks, skills acquisition, software architecture, project planning and life effectively\n\nThere will be examples from code, from professional work and from purely non-work settings\n\nUnlocking these skills can help you be more productive and achieve goals that matter for you. It will probably make you more successful in your career. But I cannot claim that it will make you happy or loved.",
"title": "Secrets of effective software development",
"id": "06004f26-84b8-4d1c-8fde-2ac7db352fb7",
"sessionId": "06004f26-84b8-4d1c-8fde-2ac7db352fb7",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Johannes Brodwall",
"twitter": "jhannes",
"bio": "Johannes Brodwall is principal software engineer for Sopra Steria. He currently works in public safety in Norway where he writes code to save lives. He works part time as a teacher in Java, React and GIS for Høyskolen Kristiania (Kristiania University College). In addition, he juggles responsibilities like being the treasurer of the local school marching band, representing the Green party in local (burrough) and AI politics, promoting the professional community for developers and being a dad of two boys. He just manages to fit everything into a 26-hour daily schedule."
}
]
},
{
"intendedAudience": "Any developer can follow the presentation, but some previous experience with using cryptographic protocols such as TLS, SSH or JWS, as well as some knowledge of hash functions will make it easier to follow.\n\nParticipants will learn how to avoid many of the common pitfalls when trying to use cryptographic primitives. The talk will give practical advise that will aid the participants in choosing suitable algorithms for future problems, as well as some of the theory behind these algorithms. The talk will also give some (very limited) practical advise regarding post-quantum readiness. ",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Have you ever wondered about the difference between a Digital Signature and a Message Authentication Code? Or what security guarantees you actually get from a Hash function such as SHA-256?\nThis talk will do an intermediate-level dive into the various properties of the most used cryptographic primitives and give practical advise to aid you in choosing between the various constructions.",
"title": "The Cryptographer's Toolbox",
"id": "2ee6a5c8-5d01-48cd-a371-e75882582b70",
"sessionId": "2ee6a5c8-5d01-48cd-a371-e75882582b70",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Stian Svedenborg",
"twitter": "SSvedenborg",
"bio": "Stian is a cryptographer by education and is currently working as the Security Architect for BankID. Stian has a talent for simpifying difficult security topics to make them more accessible for a wider audience."
}
]
},
{
"intendedAudience": "Experienced developers with basic knowledge of Gradle looking for a tool to coordinate the release process of services inside monorepo.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "There are few tools for mono repo and JVM applications. Unlike Bazel or Gradle Multi-Project, NX offers robust CI/CD tools tailored for monorepo, simplifying service management and deployment, including generating changelogs and enforcing semantic versioning. In this talk, I’ll show how to integrate NX into a repository with Gradle services.",
"title": "Using NX for Monorepo with Gradle Services",
"id": "175b58d9-2778-4e63-b74b-97c3c675ca16",
"sessionId": "175b58d9-2778-4e63-b74b-97c3c675ca16",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Matěj 'Horm' Horák",
"twitter": "HormCodes",
"bio": "Backend developer focused on clean code, functional programming, and testing.\n\nBesides work and personal projects, I'm a smart home enthusiast and a big fan of developer communities.\n\n🇨🇿 living in 🇳🇴"
}
]
},
{
"intendedAudience": "This session is designed for any developer not already familiar with the Java Microbenchmark Harness.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Some years ago, I wrote a program whose purpose was to demonstrate the performance of different Java collections when searching for, inserting, and deleting items to my students. The differences between the collections were what I expected but the actual values varied significantly for each run. Then I discovered the Java Microbenchmark Harness or JMH. Think of it as unit testing for performance rather than correctness. In this presentation I will talk about the original performance program and the enhanced version using JMH. Along the way you will learn how to configure and use the JMH.",
"title": "I Just Discovered the Java Microbenchmark Harness and You Should To",
"id": "18784091-0370-44b4-94cf-3cb6028a80a6",
"sessionId": "18784091-0370-44b4-94cf-3cb6028a80a6",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Ken Fogel",
"twitter": "@omniprof",
"bio": "Ken is a Java Champion and member of the Java Community Process Executive Committee. He is also the organizer of the annual JChampions Conference, He retired from the classroom after 31 years teaching software development at Dawson College, 25 of those years as the chair and program coordinator of the Computer Science Technology Program. He is currently a Research Scholar in Residence at the college. His first book, Transitioning to Java, was recently published by Packt. He has spoken at numerous conferences. "
}
]
},
{
"intendedAudience": "All developers that use, like or would like to create or contribute to open-source projects. \nOpen source 101",
"length": "10",
"format": "lightning-talk",
"language": "en",
"abstract": "Creating and open-sourcing a rather small project for the first time. What did this teach us, what should one be aware of before doing this, what caveats are there and would we do it againg?\nFocused on the process around this and how one should proceed if you want to do this for yourself.",
"title": "Ten things I wish I knew before i created my first piece of open-source software",
"id": "2d24450d-ec8f-4002-a47d-6fb19776d844",
"sessionId": "2d24450d-ec8f-4002-a47d-6fb19776d844",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Andreas Thuen",
"twitter": "",
"bio": "Andreas is a Platform developer and product owner of the platform team in Sparebanken vest. Really enjoys working in the intersection between technology and process, and is passionate about explaining complex tech in an understandable way."
}
]
},
{
"intendedAudience": "This session should be interesting for anyone contributing to open source projects or aspiring to do so as well as other committers/maintainers.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Almost exactly 10 years ago, I had the first discussions about how to keep evolving JUnit at conferences. I had been a maintainer for JUnit for 2 years and recently been promoted to ”keeper of the green bar,“ i.e., project/team lead of the project. While being a wildly popular testing framework, maintaining JUnit was challenging due to IDEs, build tools, and third-party extensions relying on implementation details. Plus, I felt I never had enough time to even work through my GitHub notifications.\n\nAs I shared these frustrations with fellow developers at conferences, the idea arose to start a crowdfunding campaign to have actual dedicated time to solve the underlying issues. The campaign launched a few months later and came to a successful conclusion in September 2015. We formed a team, had a kickoff with relevant stakeholders (IDEs, build tools, …), and set to work. Finally, six weeks of time to work on JUnit! As you can imagine, that time passed relatively quickly and the project was far from finished. Despite that we eventually released JUnit 5.0 in September 2017. But development didn’t stop there and has kept going since. In this talk, I will share insights about the most important factors for keeping the project running successfully.",
"title": "Running an independent open source project by example",
"id": "203ec22b-ec45-41c5-9f61-e29b0ff01538",
"sessionId": "203ec22b-ec45-41c5-9f61-e29b0ff01538",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Marc Philipp",
"twitter": "marcphilipp",
"bio": "Marc Philipp is a software engineer with extensive experience in developing business and consumer applications, as well as training and coaching other developers. One of his focus areas has always been providing tools for fellow software developers and improving their productivity. He is a long-time core committer and maintainer of JUnit and initiator of the JUnit Lambda crowdfunding campaign that started what has become JUnit 5."
}
]
},
{
"intendedAudience": "Developers with an interest in learning/trying Golang",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "This session is a quick practical start guide, exploring core Golang concepts compared to Java language. We will discuss Golang best use-cases and go through code examples to make the comparison clear.",
"title": "Introduction to Golang for Java developers",
"id": "bd8879b9-f64c-42b7-8e2c-f222c6f3d1ce",
"sessionId": "bd8879b9-f64c-42b7-8e2c-f222c6f3d1ce",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Oleksandr Dushyn",
"twitter": "https://twitter.com/ODushyn",
"bio": "I am a software developer who believes in using the right tool for the job and like to explore and experiment with a wide range of technologies to achieve that. Java has been my primary language, but I'm always eager to explore new frontiers, like Golang, which I've been actively using for the past two and a half years."
}
]
},
{
"intendedAudience": "Enterprise Java developers",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "In this session, we'll have a look why you want to try out \"supersonic, subatomic Java\" with Quarkus. If you're familiar with enterprise development with Spring or Java/Jakarta EE, you'll be delighted to see the effective way of working that Quarkus provides. We'll why coding with Quarkus not only makes our work more effective but is also fun. We will have a look at the coding experience, turnaround times, Quarkus' dev mode, container & Kubernetes support, persistence, and templating. Join us for this live-coding-only session!",
"title": "Why You Should Use Quarkus For Your Next Project",
"id": "256d34b4-8645-4034-939d-2113e6ec3f2b",
"sessionId": "256d34b4-8645-4034-939d-2113e6ec3f2b",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Sebastian Daschner",
"twitter": "@DaschnerS",
"bio": "Sebastian Daschner is a self-employed Java consultant, author and trainer and is enthusiastic about programming and Java. He is the author of the book “Architecting Modern Java EE Applications”. Sebastian is participating in open source standardization processes such as the JCP or the Eclipse Foundation, helping forming the future standards of Enterprise Java, and collaborating on various open source projects. For his contributions in the Java community and ecosystem he was recognized as a Java Champion, Oracle Developer Champion, and JavaOne Rockstar. Besides Java, Sebastian is a heavy user of cloud native technologies and anything related to enterprise software. Sebastian evangelizes computer science practices on his blog, newsletter, podcast, and videos. He kickstarted the JOnsen and jSpirit unconferences that connect Java developers throughout the globe."
}
]
},
{
"intendedAudience": "All developers with intermediate knowledge of Java ",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Event streaming will fix all your problem!\nBut why this approach, and how can you pull off your changes, ensuring compatibility and minimizing risk, without introducing new complex, expensive systems?\nThis talk is based on the speakers experience in rewriting complex, distributed systems, showcasing a small toolbox based on the Xorcery project.",
"title": "Tricks when rewriting a complex app/distributed system",
"id": "56e2637a-5065-4b10-874f-7333ae4603b1",
"sessionId": "56e2637a-5065-4b10-874f-7333ae4603b1",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Stig Lau",
"twitter": "",
"bio": "Contributor to XORcery and Java dev since 2004."
}
]
},
{
"intendedAudience": "Software Security is on everyone's mind. We devs depend on open source so much but can we trust it. The aim of this talk is to educate the audience on the basics of supply chain security and show me some tools and techniques that can be used to mitigate risk",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "We depend on OSS so much; no one is writing a cURL or Math Library anymore; everyone is just maven or npm pulling a bunch of stuff from the Internet, and that’s scary. How do you know your dependencies are free of backdoors or vulnerabilities? Have you heard of SLSA, SBOM, or the new fuzzy word in the street, “Software Supply Chain Security'' before? Maybe yes, if you are an avid reader of some tech publications. But what does this all mean? Or rather, should you care? Well, the answer is it depends. In this talk, the speaker will attempt to clarify these words and what they mean and present a state of the security world with tools and methodologies people and organizations are implementing to ensure the software is secured from dev to production.",
"title": "Our dependency on Open Source is scary. SLSA, SBOM and Sigstore to the rescue",
"id": "12f0f8f3-c1fb-4c3e-840f-2287c8e84455",
"sessionId": "12f0f8f3-c1fb-4c3e-840f-2287c8e84455",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Abdel Sghiouar",
"twitter": "boredabdel",
"bio": "GKE/Kubernetes & Service Mesh Senior Developer Advocate\nBio: Abdel Sghiouar is a senior Cloud Developer Advocate @Google Cloud. A co-host of the Kubernetes Podcast by Google and a CNCF Ambassador. His focused areas are GKE/Kubernetes, Service Mesh, and Serverless. Abdel started his career in data centers and infrastructure in Morocco, where he is originally from, before moving to Google's largest EU data center in Belgium. Then in Sweden, he joined Google Cloud Professional Services and spent five years working with Google Cloud customers on architecting and designing large-scale distributed systems before turning to advocacy and community work."
}
]
},
{
"intendedAudience": "This presentation is tailored for a wide audience, including professionals in the transportation and technology sectors and researchers interested in the application of quantum computing and AI.\nAnyone interested in learning about Quantum computing and its real world use cases would benefit from this talk. This is not a technical talk",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Ruter (together with TET Digital), has done two projects which use Quantum Computing (QC) and Artificial Intelligence (AI):\n\n1. Passenger Quantity Prediction: In our first project we used QC with AI, to accurately predict passenger volumes. \n2. Route Optimization for Ticket Controllers: Building on our initial successes, we further explored the capabilities of QC by applying it to optimize the routes of our ticket controllers. \n\nIn this talk, I will show how we worked Quantum Computing, highlighting the challenges encountered along the way and the valuable insights we learned. \n",
"title": "Quantum computing at Ruter (& TET Digitial)",
"id": "f16ae659-1576-4680-829c-a731b3aced81",
"sessionId": "f16ae659-1576-4680-829c-a731b3aced81",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Umair Mehmood Imam",
"twitter": "https://twitter.com/Umair_",
"bio": "Umair is the CDO, CAIO at TET Digital (Ruter). Additionally he is also the Founder for a startup called Bineric AI and Bineric Data. In the academic world, he is working as an assistant professor at OsloMet."
}
]
},
{
"intendedAudience": "Developers interested in the progress of programming languages, especially Kotlin enthusiasts.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Kotlin 2.0 just shipped. The huge enabler for the new major version is the new compiler front-end, codenamed K2. The release delivered better performance and stabilization of the language features and library functions across multiple compilation targets. Even though 2.0 is a major release, it brings no major language changes, providing a smooth migration path for the users. But the progress doesn’t stop, and now it’s time for more experiments!\n\nIn this talk, we will provide an overview of the state of Kotlin at the point of the 2.0 release: the outcomes of K2 stabilization and the state of Kotlin Multiplatform, including the WASM target.\n\nNext, we will dive into the upcoming ideas – what’s on Kotlin’s roadmap for the next year. You’ll see static extensions, collection literals, named-based destructuring, explicit fields, and context parameters design proposals.\n",
"title": "Kotlin 2.0 and beyond",
"id": "cf39cb14-3786-45df-a1bc-d14f70bb9d78",
"sessionId": "cf39cb14-3786-45df-a1bc-d14f70bb9d78",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Anton Arhipov",
"twitter": "@antonarhipov",
"bio": "Anton is a Developer Advocate in the Kotlin team at JetBrains. With a professional background in server-side development, Anton has been building tools for developers for more than ten years. He has been recognized as a Java Champion since 2014. He often presents as a speaker at large software conferences and contributes to the Kotlin YouTube channel.\n"
}
]
},
{
"intendedAudience": "Developers, architects, product people, managers, and others who are curious about data mesh",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Today, many organizations have a data mess where data are not discoverable and are hard to understand and use. How can we go to data mesh where we avoid bottlenecks and have clear data products that are available, discoverable, complete, and easy to use? This talk will introduce data mesh, and show what we have done so far in FINN.no / Schibsted Nordic Marketplaces.",
"title": "From Data Mess to Data Mesh",
"id": "2f4903b4-fdf6-435a-ac6b-253e677a95f5",
"sessionId": "2f4903b4-fdf6-435a-ac6b-253e677a95f5",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Lotte Johansen",
"twitter": "@lotte_johansen",
"bio": "has talked at JavaZone 5 times before. She has worked as a backend Java developer since 2006, and since 2008 in FINN. She is enthusiastic about accessibility, process, and development, and is now an Engineering Manager for the Data Enablement & Infrastructure team. "
}
]
},
{
"intendedAudience": "Målgruppen er alle som er interessert i generativ AI for norske forhold. Det kreves ingen spesielle forkunnskaper, men det er en fordel med en viss kjennskap til generativ AI.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Målgruppen er alle som er interessert i generativ AI for norsk. Jeg vil fortelle hva Nasjonalbiblioteket gjør på generativ AI-området, og hvordan Nasjonalbiblioteket bidrar til at maskinene kan forstå norsk språk, kultur, kunst, historie, geografi og samfunnsliv - den norske erfaringen. \n\nJeg kommer inn på:\n• Nasjonalbibliotekets digitalisering av den norske kulturarven\n• Utvikling av et norsk tekstkorpus (datasett) for trening av generative modeller\n• Trening av språkmodeller på ulike infrastrukturer\n• Evaluering/testing av språkmodeller for norsk\n• NB-Whisper - tale til tekst for norsk",
"title": "Hvordan få maskinene til å forstå norsk og den norske erfaringen?",
"id": "d441d220-4494-40a1-a1e1-84eab6c0ff06",
"sessionId": "d441d220-4494-40a1-a1e1-84eab6c0ff06",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Wilfred Østgulen",
"twitter": "",
"bio": "Wilfred Østgulen er IT-direktør i Nasjonalbiblioteket og leder også AI-laben der. Jeg har 30 års erfaring fra utvikling av digitale løsninger og organisering og ledelse av IT. Jeg har tidligere jobbet i Forsvaret, Accenture, Aetat/NAV, Altinn, Sykehuspartner og politiet."
}
]
},
{
"intendedAudience": "Any developer dealing with DTO or database layers of any experience level.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "One of Java's most prominent features is its type system. While its strict typing is normally a useful aid in writing consistent programs, it can also lead to verbosity. This is especially the case when processing similar types that represent the same or similar data, something that is common in DTO layers, or when versioning data types, two problems that are also current at the Norwegian tax authority (Skatteetaten). This presentation showcases how the introduction of structural types avoids boilerplate in the Norwegian taxation process, without loosing any type safety.",
"title": "Bringing structural types to Java and Typescript",
"id": "43583af3-0362-403c-a454-e85d134f9159",
"sessionId": "43583af3-0362-403c-a454-e85d134f9159",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Rafael Winterhalter",
"twitter": "rafaelcodes",
"bio": "Rafael works as a software consultant in Oslo, Norway. He is a proponent of static typing and a JVM enthusiast with particular interest in code instrumentation, concurrency and functional programming. Rafael blogs about software development, regularly presents at conferences and was pronounced a JavaOne Rock Star. When coding outside of his work place, he contributes to a wide range of open source projects and often works on Byte Buddy, a library for simple runtime code generation for the Java virtual machine. For his work, Rafael received a Duke's Choice award, an Oracle groundbreaker award and was elected a Java Champion."
}
]
},
{
"intendedAudience": "The workshop targets an intermediate Java user, however, a beginner should be able to follow too.",
"length": "120",
"format": "workshop",
"language": "en",
"abstract": "Join us for an in-depth workshop designed for developers looking to understand better how to use GraalVM Native Image via a Maven plugin and build size-optimized Java applications.\nDiscover ways to minimize application footprint taking advantage of different Native Image linking options and packaging into various base container images. Finally, streamline your development process by automating builds with CI/CD pipelines. Enhance your multi-cloud application development skills and take your applications to the next level with GraalVM!",
"title": "Multi-Cloud Apps with GraalVM - Up and Running",
"workshopPrerequisites": "The attendees are not required to install any software for or during the workshop.\nThe session will run on an interactive workshop platform in the Oracle Cloud Infrastructure (OCI): https://luna.oracle.com/. Some key facts:\n• A user is provided with an ephemeral Oracle Cloud account and necessary OCI resources for the time of a workshop.\n• The development environment is pre-configured with GraalVM JDK as a default JDK.\n• All resources will be cleaned up after completing a workshop.\n• Remote control over users' sessions is possible in case of trouble.\n• The desktop environment runs as a container image with 32GB of memory and 2 CPUs.\n• A user only needs to sign in to the workshop platform with a regular Oracle account that he can create in advance at \nhttps://profile.oracle.com/myprofile/account/create-account.jspx",
"id": "a519538a-4ebc-4c44-a722-3ec159d5d809",
"sessionId": "a519538a-4ebc-4c44-a722-3ec159d5d809",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Olga Gupalo",
"twitter": "@Olya_Gupalo",
"bio": "A member of the GraalVM Developer Relations team.My current focus is the GraalVM website (graalvm.org), content and documentation management, accessibility, public outreach, producing different learning materials, and participation in the GraalVM releases."
},
{
"name": "Kris Foster",
"bio": "Kris Foster has been working in software development for over 20 years, a large part of that work building Java applications as an independent consultant. Currently works at Oracle as a part of the GraalVM team. Focused on the product management of the developer tooling for GraalVM, Micronaut® framework, and the Graal Development Kit. We have released several VS Code extensions that aim to make the life of Java developers better."
}
]
},
{
"intendedAudience": "Arkitekter og utviklere som er interesserte i sikkerhet, ytelse og applikasjonsarkitektur, eller er interesserte i den nasjonale felleskomponenten ID-porten.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "ID-porten har 300 millioner innlogginger i året.  Det siste året er ID-porten omskrevet fra bunnen av og all trafikken er flyttet til ny løsning - uten nedetid.  \n\nModerniseringen har vært omfattende: Vi har skrevet all kode på nytt, fra Java 8/11 til Java 21 og tilpasset til en Kubernetes-basert plattform.  Fra å bestille manuell deploy til automatisering på Kubernetes-plattform.  Byttet ut kjerneprogramvare.  Konvertert data fra ldap til nye databaser.  Sikkerhetsprotokollene OpenID Connect og SAML2 har byttet plass.  Samtidig har ikke verden stått stille: Antall tjenester som er integrert med ID-porten øker og stadig flere logger seg inn til offentlige tjenester.  Eksterne avhengigheter har endret seg.  Skyplattformen vi skulle bruke kunne ikke brukes likevel.\n\n\nVi har planlagt og bommet, feilet og krasjet og blitt overrasket mange ganger.  Likevel rakk vi å få kundene over til nye ID-porten før kontrakten på den gamle driftsplattformen gikk ut.  Vi vil dele våre erfaringer med transisjonsarkitektur: Om å kjøre to versjoner av et system samtidig med full trafikk, om både å migrere kunder brått, men skånsomt eller migrere gradvis, om å ha data 2 ulike steder på helt ulike formater. Vi har designet tjenester bare for å kaste dem etter kort tid, utvidet sikkerhetsprotokoller for å koordinere ulike sikkerhetsmodeller og laget mye testverktøy.\n\nKom og le av våre feil! (Kanskje vi har noe å lære bort?)\n",
"title": "ID-porten til sky uten nedetid",
"id": "fcb7a30d-0482-4193-97e2-3ae82c2feb3b",
"sessionId": "fcb7a30d-0482-4193-97e2-3ae82c2feb3b",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Thomas Reppesgård",
"twitter": "",
"bio": "Thomas er protokollnerd og utvikler i Digitaliseringsdirektoratet.  Han har jobbet mer enn 24 år som utvikler innen offentlig sektor, bank og forsikring.  Han har jobbet som utvikler på ID-porten og relaterte systemer i over 10 år.  "
},
{
"name": "Anne Marte Hjemås",
"twitter": "",
"bio": "Anne Marte er en utvikler og arkitekt i JProfessionals som har jobbet med utvikling og applikasjonsarkitektur i Java-økosystemet i over 25 år. Hun er interessert i sikkerhet, integrasjon og arkitektur.\nDe siste 5 årene har hun jobbet med ID-Porten og Maskinporten for Digitaliseringsdirektoratet."
}
]
},
{
"intendedAudience": "This talk is for anyone interested in learning about geodata, geometric processing and how to create data sets suitable for 3D visualization. The main takeaway is that it is not hard to create great-looking models of the surroundings when you are a bit familiar with the domain, and use the right tools.\n\nParticipants should have a software development background, but otherwise no particular skill set is required.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "We use computers to model and simulate the physical world - both for entertainment (games) and for business (construction and manufacturing). With increasing computational power, and VR headsets on top of that, we are able to make more and more immersive experiences. In short, our virtual and physical worlds blend together. Graphics technology has made it possible, and open geodata has made it accessible.\n\nThis talk will walk you through how you can create your own virtual 3D model of some location on the globe. You will learn how to find, evaluate, gather, process, compose and visualize geodata, and we will end up with (in my opinion) a rather nice 3D model.\n\nWe will use data sets describing terrains, buildings, roads and aerial views, and all of them will be open and free. You will also learn more about the geodata domain and common techniques, and the main software tools and libraries in use (mostly in Python and JavaScript). You will get links to further resources, including documentation and code for the model building process shown in this presentation.",
"title": "Building virtual worlds using open geodata",
"id": "c9ed22e0-1417-4ae5-af9e-b489a9374123",
"sessionId": "c9ed22e0-1417-4ae5-af9e-b489a9374123",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Kristoffer Dyrkorn",
"twitter": "kristofd",
"bio": "Kristoffer is a senior principal software engineer at Autodesk. He holds an MSc in computer graphics and computational geometry, and works on processing, optimizing and visualizing geodata for architectural design."
}
]
},
{
"intendedAudience": "The talk is intended for anyone wanting to stop using passwords. There will be some technical details on how the technology works.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Who doesn’t want to get rid of passwords and all the problems they bring us? This is why passwordless is probably one of the greatest buzzwords ever made. But can passwordless deliver on such a utopic future?\nThe goal of the talk is to introduce passwordless, and more specifically FIDO2. How does it work and is it safer than your standard passwords and other MFAs like time-based one-time passwords? Even passwords we consider safe due to length and entropy are a risk factor. The talk will give quick real-life coverage of how no password is truly safer. It will go through what FIDO2 is, details on the CTAP2 and Webauthn specifications that FIDO2 encompasses, including covering what passkeys are.\nAt the end you should know what options you have right now as an end user to start going passwordless, and what you as a developer can do to help secure others.\n",
"title": "Imagine: A world without passwords, it is easy if you try",
"id": "fa614957-9dbb-4726-b5d1-73239e3474eb",
"sessionId": "fa614957-9dbb-4726-b5d1-73239e3474eb",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Sigurhjörtur Snorrason",
"twitter": "",
"bio": "Sigurhjörtur currently works as the Head of Software Development for Pone Biometrics AS in Oslo. He has done multiple FIDO implementations, both as virtual devices and as real hardware devices in a multitude of programming languages. He has previously worked as Head of Architecture and Security at Distribution Innovation."
}
]
},
{
"intendedAudience": "Etter vår erfaring er kurset givende både for begynnere og for erfarne utviklere. En grunnleggende forståelse for objektorientering er nyttig, men ingen forutsetning. Ingen Rust-forkunnskaper kreves. ",
"length": "240",
"format": "workshop",
"language": "no",
"abstract": "Bli fortrolig med programmeringsspråket Rust gjennom å implementere sjakk!\n\nRust har i åtte år blitt kåret til det høyest elskede programmeringsspråket på Stack Overflow. Også i SpareBank 1 Utvikling er vi flere som har lagt vår elsk på språket, og vi har laget denne workshopen for å gi en god innføring i idiomatisk Rust og hvordan det lar oss skrive bedre og mer konsis kode – gjennom praktisk erfaring.\n\nI kurset starter du med et tomt sjakkbrett, og ender opp (om du er rask, eller flittig til å bruke hint) med et fullverdig sjakkspill. Underveis blir du eksponert for syntaks og datastrukturer i Rust, blir kjent med Rust-tankegangen og lånesjekkeren, og ikke minst Cargo, vårt essensielle Rust-verktøy.\n\nNB! Det er stor fare for at du selv blir glad i Rust i løpet av workshopen. Vi tar ikke ansvar for eventuelle konsekvenser av dette.",
"title": "Idiomatisk Rust for begynnere",
"workshopPrerequisites": "For å delta i kurset behøver du en bærbar PC med git, og en kodeeditor du er komfortabel med. Du kan installere Rust ved å følge instruksjoner på https://doc.rust-lang.org/book/ch01-01-installation.html for å komme fort i gang, men dette kan fint gjøres ved starten av workshopen.",
"id": "b5cc6d47-d9d7-4bf1-a229-9fbcb796d1e3",
"sessionId": "b5cc6d47-d9d7-4bf1-a229-9fbcb796d1e3",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Isak Kyrre Lichtwarck Bjugn",
"twitter": "",
"bio": "Isak er siving i Nanoteknologi og Entreprenørskap fra NTNU, og engasjert mannskorsanger. Han er fagleder i SpareBank 1 Utvikling, og trives både fremst og bakerst i teknologistakken, med henholdsvis React/TypeScript og Kotlin/Spring Boot. Han er dessuten svært språkinteressert, og har utviklet Fagord.no for å gjøre norsk fagspråk mer tilgjengelig."
},
{
"name": "Geir Olav Alsvik",
"twitter": "",
"bio": "Geir Olav har i et tidligere JavaZone-foredrag blitt omtalt som «fullstack-ninja». Han har studert informatikk ved UiO – men rakk også to år med japansk, og lager for tiden et verktøy i Rust for å lære seg koreansk – som gjør ham allsidig i verdensspråk så vel som programmeringsspråk.  I SpareBank 1 Utvikling er han fullstack-utvikler i Bli kunde-teamet, og engasjert leder i Rust-faggruppen."
}
]
},
{
"intendedAudience": "Everybody has used CSS at some point. This talk gives you an opportunity to hear about it from the person who created it.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "In my talk, I plan to tell why CSS was invented, tell some stories from the early days, outlining the philosophy behind CSS: aestetics is important! keep things simple! Don't create another programming language!\n",
"title": "CSS @ 30, join the party!",
"id": "7356d949-ab6d-41ba-9c44-e5727b47d70e",
"sessionId": "7356d949-ab6d-41ba-9c44-e5727b47d70e",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Håkon Wium Lie",
"twitter": "",
"bio": "Håkon Wium Lie created Cascading Style Sheets (CSS) while working with Tim Berners-Lee at CERN in 1994. Along with Bert Bos, he developed CSS into a W3C standard. As CTO of Opera Software he but browsers into mobile devices, and he proposed the Acid2 test make sure all browser supported the same standard. He is the chairman of YesLogic, which makes the Prince CSS-to-PDF formatter. In 2015, Håkon sailed from Lima to Easter Island on a balsa raft, making sure the world-wide web is truly world-wide. In 2023 he sailed a viking ship down the Danube. He has an apple orchard (real apples, not computers) in his native Norway, and also owns a pipe organ workshop there. He holds degrees from West Georgia College, the MIT Media Lab, and the University of Oslo.\n"
}
]
},
{
"intendedAudience": "Target audience is anyone interested in practical applications of Large Language Models (LLMs) and Generative AI. This includes AI and machine learning engineers, data scientists, business leaders and decision-makers, CIOs, as well as legal professionals. ",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "In the past year, the conversation in AI has been dominated by the remarkable capabilities of Large Language Models, with ChatGPT leading the charge. These models, such as those developed by OpenAI, have changed our approach to tasks ranging from text to image generation. However, these closed source models are not without their shortcomings, particularly in areas of information security and GDPR compliance – restricting their applicability beyond Proof of Concepts (PoCs)\n\nOpen-source models, for example Llama from Meta, on the other hand fall short in handling specific languages, for example, Norwegian. \n\nWe developed RuterGPT to tackle these challenges related to the Norwegian language and AI privacy concerns. \n\nIn this session, we'll explore the inspiration behind RuterGPT, its development journey, and the challenges we faced along the way. Additionally, we'll share insights into how RuterGPT is being utilized inside Ruter by combining the capabilities of Large Language Models with Ruter's specialized domain data using RAG (Retrieval Augmented Generation). ",
"title": "RuterGPT: Unveiling an Open-Source Large Language Model for Norwegian",
"id": "5b67ff17-28b7-4112-9f6f-8c3ce598e021",
"sessionId": "5b67ff17-28b7-4112-9f6f-8c3ce598e021",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jawad Saleemi",
"twitter": "",
"bio": "Jawad Saleemi is the team lead in Data & AI department in Ruter and is responsible for leading AI initiatives. He led the team which developed and released RuterGPT."
},
{
"name": "Alexander Amiri",
"twitter": "",
"bio": "Alexander is datascientist in Ruter who works on Generative AI initiatives."
}
]
},
{
"intendedAudience": "Anyone who builds software with other people. (Or at least tries to.)",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "We make decisions all the time in software - our architectures are the sum of them; both conscious and unconscious.\n\nYet we have so little awareness about *what* decisions are and *how* we decide. This is not only the source of great friction and waste, it is leading to terrible outcomes for our software.\n\nIn this talk I’ll describe what architectural decisions are, and the different ways that we approach them (individually and collectively) making clear the pros and cons of each. I’ll then compare them all, describing the ideal characteristics of a decision-process for modern software development.\n\nAs a consequence of this talk not only will you be able to decide better as an individual, you will also be able to diagnose failings in your group decision processes and resolve them.",
"title": "How we Decide",
"id": "9574e6dc-483b-46e4-bbd1-c4bb9002f13c",
"sessionId": "9574e6dc-483b-46e4-bbd1-c4bb9002f13c",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Andrew Harmel-Law",
"twitter": "@al94781",
"bio": "A Tech Principal at Thoughtworks and author/trainer for O'Reilly, Andrew specialises in Java / JVM technologies, agile delivery, build tools and automation, and domain driven design. Experienced across the software development lifecycle and in many sectors what motivates him is the efficient delivery of large-scale software solutions, fulfilling complex user needs. He understands that people, architecture, process and tooling all have key roles to play in achieving this. Andrew has a passion for open source software and its communities. He has been involved with OSS to a greater or lesser extent since his career began; as a user, contributor, expert group member, or paid advocate - most famously as one of the Jenkins JobDSL originators. Andrew enjoys sharing his experience as much as possible. This sharing is not only seen in his formal consulting engagements, but also informally through mentoring, blog posts, conferences (speaking and organising), and open-sourcing his code"
}
]
},
{
"intendedAudience": "Utviklere som har mange applikasjoner å vedlikeholde, er lei av Maven eller bare har lyst til å lære noe nytt.",
"length": "120",
"format": "workshop",
"language": "no",
"abstract": "Med mikroarkitektur kommer flere apper, som alle gjerne skal bygges, testes og kjøres lokalt. I tillegg skal man ha CI pipeline med statisk kodeanalyse, produsering og signering av containere, autodeployment, notifications ved feilede bygg og så videre.\n\nMed et monorepo kan man få samme CI pipeline for flere applikasjoner i samme språk, men hva om man kunne bruke den for flere språk, og i tillegg få synergier mellom forskjellige teknologier?\n\nI Fremtind har vi brukt Bazel for produksjonsbygg i over ett år, og har per nå 60 applikasjoner og 70 biblioteker som bygger med samme GitHub Workflows.\n\nKom hit for å lære om Bazel og hva som gjør det til et bra verktøy for monorepo både med et eller flere språk! Etter denne workshoppen vil du ha oversikt over hvordan man kan bruke Bazel både til lokal utvikling, og et eksempeloppsett for en CI pipeline i GitHub Workflows.",
"title": "Bygg et flerspråklig monorepo med Bazel",
"workshopPrerequisites": "Linux/Mac laptop - kan ikke garanterere at det funker på Windows (selv om Bazel fungerer der)\n\nInstaller Bazelisk (https://github.com/bazelbuild/bazelisk)\n\nVi tar utgangspunkt i dette git repoet: https://github.com/fremtind/bazel_workshop",
"id": "2a43f2c7-d45f-4dc6-9e47-ded5d0088317",
"sessionId": "2a43f2c7-d45f-4dc6-9e47-ded5d0088317",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Knut Eirik Leira Hjelle",
"twitter": "",
"bio": "Knut Eirik jobber med arkitektur og utvikleropplevelse i Fremtind og har 15 år bak seg som profesjonell utvikler i hele stacken, og er vandt til å jobbe med forskjellige programmeringsspråk. Etter mange år i forskjellige bransjer har han vært innom mange forskjellige byggsystemer og behov.\n\nNår Knut Eirik ikke programmerer, finner du han på konserter, på pub eller i skogen på en stisykkel."
},
{
"name": "Magnus Raaum",
"twitter": "",
"bio": "Magnus er Techlead for motor-relaterte produkter for privatmarkedet i Fremtind. Han har mer enn 10 års erfaring som Java utvikler, og er ikke redd for å bryne seg på arkitektur og infrastruktur! \n\nNår Magnus ikke er på jobb, finner du ham gjerne på en golfbane der han øver på svingen sin."
}
]
},
{
"intendedAudience": "Developers working on microservices setups who are interested in how to better organize testing them. This is a relatively introductory session, so no major experience is required, ",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Starting with cloud-native apps may represent a steep learning curve! It’s not only a matter of architecture or technology but also a paradigm shift on developer experience and workflow. Even before writing code, configuring your entire test environment might be intimidating! And after that you still have to figure out how to validate your microservices with great confidence to release them independently!\n\nIn this session, we present tools to ease your life when setting up your development environment and testing with databases, middlewares, cloud services and your regular business services and API! Considering the diversity of styles and protocols - REST, gRPC, GraphQL, Async - and cloud-native stacks - this latter is not an easy task.\n\nThis session provides hands-on experience on: how to equip your team with a consistent and polyglot approach. And how to let them develop locally, but providing confidence with a unified policy between the inner and the outer loops.",
"title": "Smooth Sailing in the Cloud-Native Storm: Tools for better confidence in microservices development",
"id": "d42eb9e5-1d6f-4652-9d69-e6f6f62fc37e",
"sessionId": "d42eb9e5-1d6f-4652-9d69-e6f6f62fc37e",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Oleg Šelajev",
"twitter": "shelajev",
"bio": "Oleg Šelajev is a developer advocate at Docker working mainly on developer productivity, Testcontainers, and improving how we set up local development environments and tests. Developer. Author. Speaker. Java Champion. Docker captain.\nLoves all languages."
}
]
},
{
"intendedAudience": "Dette fordraget er for alle som utvikler eller dokumenterer API-er eller jobber med integrasjon. Det kan også være nyttig for produkteiere som har API-er i sin portefølje.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Open API Specification (OAS) er blitt vår standard for å dokumentere i detalj hver enkelt operasjon i vårt API. Vi har også AsyncAPI for å dokumentere event-drevne og asynkrone API-er, men alle disse har én mangel: Hvordan kan vi dokumentere i hvilken sekvens API-operasjonene skal kalles for å få gjennomført en større, forretningsmessig transaksjon som å opprette handlekurv, legge produkter i den, sjekke ut og betale? Arazzo-spesifikasjonen fra OpenApi Initiative (OAI) er definert nettopp for å dokumentere slike arbeidsflyter i API-et på en menneskelig og maskinlesbar måte, slik som OAS er for enkelt-operasjoner i API-et.\n\nI dette foredraget vil jeg si noe om bakgrunnen for denne spesifikasjonen, vise noen ekle eksempler og si noe kort om hvilke verktøy som er kommet rundt denne spesifikasjonen.",
"title": "Bedre API-dokumentasjon med Arazzo",
"id": "9b4811ec-2a9f-429d-9712-d3a06b2afee9",
"sessionId": "9b4811ec-2a9f-429d-9712-d3a06b2afee9",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Bjørn Hamre",
"twitter": "@javaguruen",
"bio": "Bjørn jobber som løsningsarkitekt med spesielt ansvar og interesse for det meste relatert til API-er. Han har holdt en rekke foredrag og workshops på konferanser og i communities de siste årene. Har en forkjærlighet for Kotlin og funksjonell programmering og er ellers engasjert i javaBin Bergen."
}
]
},
{
"intendedAudience": "This presentation is for developers and anyone else that would like to learn how to build shared understanding on data. There will be no code lines in the presentation, but intuitive visual graph nodes that anyone can understand.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "The Norwegian Police is all about protecting human beings, fighting crime, and adding light to difficult cases. This is managed through many valuable datasets stored in different formats and systems. Would it be possible to understand our data better, and even save lives if our data was linked together and optimized for reuse? This talk is for developers and everyone else who wants to understand Semantic Web and Data Mesh concepts for achieving exactly that.\n\nTeam Knowledge Graphs at the Norwegian Police IT Unit have started a journey to explore how to make data a first-class citizen of our ecosystem. In this talk we will share our data-centric approach on saving lives through building meaningful data products and generic applications. By listening to this talk, you will learn how we are working to transform the Police into a data-centric organization.\n\nAs a developer, you will learn how to reduce application complexity and build knowledge that will outlive the lifespan of any application that you will ever build. Not convinced? I will present how it can be done through real use case examples from our work at the Norwegian Police.",
"title": "Norwegian Police Knowledge Graph: How to save lives through shared understanding of data",
"id": "5aa3c7fb-68ad-4745-8d20-cd16fc97ac8c",
"sessionId": "5aa3c7fb-68ad-4745-8d20-cd16fc97ac8c",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Lars Berg Hustveit",
"twitter": "hustveit",
"bio": "Lars is a system/knowledge architect at the Norwegian Police IT Unit. He has been fascinated with using web technologies for knowledge representation ever since he studied and wrote a master thesis about the topic at the University of Bergen. Throughout his career he has been developing semantic knowledge graphs within the fields of cultural history archives, life science, banking, and marine vessel regulations."
}
]
},
{
"intendedAudience": "Developers, AI Enthusiasts, Computer Vision interested\nNo experience needed. ",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "We introduce an innovative application of computer vision and artificial intelligence to analyze training videos of canoe athletes preparing for the Olympic Games. Our method employs foreground-background separation for canoe detection and waterline derivation. Through pose detection, we identify the paddle and have trained a neural network to recognize essential paddle positions for routine training analysis. Additionally, we incorporate biomechanical insights in a post-processing step to refine AI results and enhance analysis accuracy. Traditionally, biomechanics engineers manually screen training videos frame by frame to locate specific paddle positions and measure the paddle's angle relative to the waterline; a process taking about 20 minutes per athlete. Our approach significantly streamlines this process, reducing the workload by an order of magnitude.",
"title": "From Athlete to Algorithm: Transforming Canoe Technique Analysis with AI",
"id": "2a3b78a2-935a-4f2a-8706-8e155aeb73a5",
"sessionId": "2a3b78a2-935a-4f2a-8706-8e155aeb73a5",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Marc Schuh",
"twitter": "",
"bio": "Dr. Marc Schuh is a Principal Consultant at TNG Technology Consulting in Munich. In addition to managing customer projects, ranging from the development of a Data Lake in the AWS Cloud to the migration of legacy applications (e.g. Visual Basic) to Azure, he works on various innovative prototype projects together with the Innovation Hacking. Among them are the automated technical analysis of high-performance canoe athletes of the German National Team, and experiments with a Brain Computer Interface.\n\nPrior to his start at TNG, Marc Schuh received his doctorate in physics and was a wheelchair sprinter over 400m at three Paralympic Games, in addition to being a World Champion and holding the European record."
}
]
},
{
"intendedAudience": "Denne lyntalen vil være interessant for utviklere som jobber med eller er interessert i infrastruktur og testing, samt arkitekter eller andre med teknisk ledelse som er opptatt av automatisering og metrikker.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Samordna opptak er det systemet de fleste bruker når de søker til høyere utdanning i Norge. Vi som jobber med Samordna opptak lever i en verden der lasten er svært ujevnt fordelt utover året. Det betyr at vi jobber lenge med kode og system mellom hver gang vi blir utsatt for mye trafikk. Det har derfor vært vanlig prosedyre å bruke Gatling for å gjøre lasttesting av Samordna opptak i forkant av de to periodene i året med høyest last. Lasttestene har vært svært nyttige og har hjulpet oss med å finne og forbedre ytelsesutfordringer.\n\nI denne lyntalen vil dere høre om hvordan vi nå har automatisert lasttest-riggen vår. Dere vil lære litt om Gatling som lasttest-verktøy, samt om hvordan vi har automatisert lasttestene gjennom GitLab pipeline. Dere vil også se hvordan man kan bruke Opentelemetry for å spore metrikker som vises på Grafana. Vi vil dele de viktigste lærdommene fra dette arbeidet og gi noen tips til de som kan tenke seg å gjøre lignende.",
"title": "Automatisk lasttesting av Samordna opptak",
"id": "16de8fc2-4e54-4432-977a-e4400de271b1",
"sessionId": "16de8fc2-4e54-4432-977a-e4400de271b1",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Karl Yngve Lervåg",
"twitter": "",
"bio": "Karl Yngve er en utvikler med forskerbakgrunn som nå jobber i Sikt. Han har skiftet karriere fordi han synes det er spesielt artig å jobbe med kode og systemer som er i produksjon og som påvirker mange folks liv. Han bryr seg spesielt om dokumentasjon, kodekvalitet og infrastruktur og er en engasjert Neovim-bruker."
}
]
},
{
"intendedAudience": "Utviklere i alle aldre",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Når man lager nye systemer, påvirker man fremtiden til alle de som skal vedlikeholdet systemet. Det gjelder både teknologivalg, teststrategi og hvordan koden er strukturert. I dette foredraget vil jeg dele anektoder, både fra ting jeg har overtatt, og fra intervjuer jeg har gjort med de som har måttet leve med mine valg. For vi må finne ut hvorfor vedlikehold blir så dyrt og vanskelig, sånn at vi kan lage flere nye system!",
"title": "Tanker om vedlikehold versus nyutvikling",
"id": "888fe26f-7652-434c-a984-7f410c06e393",
"sessionId": "888fe26f-7652-434c-a984-7f410c06e393",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Trond Pedersen",
"twitter": "",
"bio": "I have developed solutions for businesses for 20 years, as a consultant, as an employee or as part of a startup, and see no reason to stop. Sometimes I do this as a manager or architect, sometimes I help with advice or finding out what we need to make, and a lot of the time I do full-stack development as well.\n\nI am a proud member of the Agile Community in Oslo, and have been an advocate for lean self-organizing development teams since 2001. Enabling a team to do quality work, by giving them the power and authority to make the right decisions is key to a successful product. \n\nTo contribute to my professional community, and to share what I learn makes me happy. That is one of the reasons I started the Agile Conference with some friends back in 2007, a conference that is still going strong. Nowadays I contribute as a organizer of the Lean Coffee meetup, and as a speaker at conferences.\n\nMy friends described me as energetic, pragmatic and inspiring to work with."
}
]
},
{
"intendedAudience": "Foredraget passer for alle som har en interesse for teknologi og miljø. Vi kommer til å introdusere konseptene rundt digitale utslipp fra bunnen av. Foredraget vil  passe for en et tverrfaglig publikum - både de med mye og mindre erfaring. ",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Den økende bruken av teknologi, spesielt den eksplosive bruken av kunstig intelligens, vil få store konsekvenser for IT-bransjens klimautslipp og vår bransjes evne til å støtte målet om bærekraftig vekst. \n \nVi lever i et stadig mer digitalisert samfunn, der IT-sektoren er en bærebjelke for å holde livsviktige tjenester oppe. Med kunnskapen og erfaringen vi har som teknologer har vi ansvaret for å ta gode valg når vi bygger, drifter, og videreutvikler digitale løsninger. Et godt valg i 2024 skal også være det mest bærekraftige valget. Dette gjør det helt nødvendig at teknologene må med i selskapenes samtaler om bærekraft og utslippskutt. \n \nI dette foredraget vil Thea og Hans Kristian snakke om IT-bransjens utslipp, hvordan AI setter fart på dette og hva vi kan gjøre for å utvikle bærekraftige digitale løsninger. De vil vise til konkrete eksempler for hvordan man kan måle digitale utslipp, kutte utslipp på systemnivå og i produktutvikling. Samt diskutere hvordan du kan gå frem for å bygge en digital bærekraftskultur som ivaretar bærekraftig utvikling over tid. \n",
"title": "Med AI i førersetet blir IT-sektoren en klimaversting: Nå må teknologene på banen! ",
"id": "c88c5bcd-3e45-4b4c-aa83-ddffa93bf517",
"sessionId": "c88c5bcd-3e45-4b4c-aa83-ddffa93bf517",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Hans Kristian Henriksen",
"twitter": "",
"bio": "Hans Kristian er sikkerhetsutvikler i Bekk og brenner for klima og bærekraft. Han har siden ungdomstiden vært opptatt både av IT og miljø, og bruker nå hverdagen til å forsøke å gjøre produkter og systemer hos noen av Norges største selskaper litt bedre for verden. Han har praktisk erfaring med reduksjon av utslipp fra IT-systemer, og rådgir selskaper på hvilke tiltak de kan gjennomføre for å levere grønnere digitale løsninger."
},
{
"name": "Thea Thorleifsson",
"twitter": "",
"bio": "Thea er siviløkonom med spesialisering i energi og bærekraft. Hun er opptatt av at vi som jobber med teknologi må integrere bærekraft i håndtverket vårt, for at teknologi skal forbli drivkraften til et bedre og mer bærekraftig samfunn for alle. I rollen som forretningsutvikler og bærekraftsansvarlig i Bekk er Thea sterkt engasjert i å spre kunnskap og interesse for digital bærekraft i IT-bransjen\n"
}
]
},
{
"intendedAudience": "Developers, architects, designers, management, anyone interested in product development",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "When designing products for large organizations, it's easy to get carried away trying to understand the user's combined and complex overall needs.  WHY are we making this software? What VALUE are we trying to deliver? We've all heard the story of the bricklayers asked about what they are doing - are they laying bricks or are they building a cathedral? Software is a tool that we use to achieve some goal. It's important to focus on the goal - the cathedral,  but should we be delivering \"cathedral building\" products? \nNo.\nIn order to build a cathedral, we need a large set of _simple_ tools. The tools themselves need little knowledge of the overall goals involved. Building in this knowledge makes them complex and inflexible. To build a cathedral, one needs hammers, drills, bricks... simple tools that can be composed with others to achieve the overall goal. The same goes for software",
"title": "To build a cathedral, the user needs brick laying tools",
"id": "92f3bc90-e257-4ef8-854f-d4521de74349",
"sessionId": "92f3bc90-e257-4ef8-854f-d4521de74349",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Christin Gorman",
"twitter": "ChristinGorman",
"bio": "Christin has more than 20 years experience as a software developer, and 13 years experience as a speaker at JavaZone. Her talks are generally seen as both informative and entertaining.  For the last 7 years, she's been working as a contractor/consultant for Kodemaker. "
}
]
},
{
"intendedAudience": "Dette foredraget krever hverken høy teknisk eller juridisk kompetanse. Foredraget er veldig aktuelt for personer som har en interesse for kunstig intelligens og/eller personvern samt de som skal jobbe med anskaffelse av fremtidig teknologi i virksomheten. \n",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Kunstig intelligens (KI) er ikke lenger en fjern fremtidshorisont, men en realitet som griper inn i stadig flere aspekter av våre liv og samfunn. Som med enhver teknologisk revolusjon, kommer også KI med utfordringer som krever grundig forberedelse og regulering. I 2016, da GDPR trådte i kraft, var denne forordningen et banebrytende skritt mot å regulere hvordan personlig data behandles og beskyttes i en digital tidsalder. Mens det primært fokuserer på personvern, kan vi trekke verdifulle lærdommer fra GDPR som er relevante for å forberede oss på KI. \nI dette foredraget skal vi ta opp både suksesser og smertepunkter fra GDPR og se på hvilke praktiske læringspunkter vi kan ta med oss videre for å sikre god etterlevelse av regelverket ved implementering av KI. ",
"title": "Hvordan forbereder vi oss på KI - læringspunkter fra GDPR",
"id": "ce4d3d75-0989-4e6e-a6ea-76fc8d108662",
"sessionId": "ce4d3d75-0989-4e6e-a6ea-76fc8d108662",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Katarina Torgersen",
"twitter": "",
"bio": "Katarina er personvernsjurist og jobber hos Oslo Origo, digitaliseringsetaten til Oslo kommune. Hun skrev masteroppgaven sin om de rettslige problemstillingene rundt opplæring av maskinlæringsmodeller i forvaltningen og har stor interesse for forholdet mellom KI og personvern. \n"
}
]
},
{
"intendedAudience": "Alle programmerere som har lyst til å lage bedre API'er.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Har du noen gang bestilt noe på nett, hvor du har fått samme \"vare\" mange ganger? Har du noen sinne opplevd feilmelding på feilmelding, men så viser det seg at ting gikk bra likevel?\n\nI denne lyntalen vil jeg demonstrere og forklare en robust teknikk for å få bukt med denne type problemer.",
"title": "Nøkkelen til robuste API'er",
"id": "35d8c1c7-122e-4c79-b077-5b3174d7834d",
"sessionId": "35d8c1c7-122e-4c79-b077-5b3174d7834d",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Alf Kristian Støyle",
"twitter": "",
"bio": "Alf Kristian er konsulent i Kodemaker med 20 års erfaring. Han har jobbet backend, frontend, fullstack, og med med mange programmeringsspråk. Han mener at det er en ting vi alle kan bli bedre på uavhengig av teknologi og plattform, lage bedre API'er."
}
]
},
{
"intendedAudience": "The talk is designed to be accessible to a broad audience covering attendees with a basic understanding of AI concepts or a keen interest in art and digital humanities ",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "This talk showcases a collaborative project between the Munch Museum in Norway and Bineric (startup in Norway), leveraging Dall-E and NorskGPT to explore Edvard Munch's extensive archive of 47,000 artworks and texts. We introduce an innovative approach to cultural heritage, using Dall-E for detailed analysis of Munch's paintings and NorskGPT, the leading Scandinavian language LLM, for interpreting texts. Our project aims to make Munch's legacy accessible and navigable through advanced AI, offering fresh insights into his work. This presentation is designed for AI enthusiasts, digital humanities scholars, and art lovers eager to see how technology transforms our understanding of cultural artifacts. Attendees will learn about the integration of AI in art analysis, challenges in digital heritage preservation, and the potential of AI to bridge history with the future. Join us for a glimpse into how art and AI converge to preserve and interpret Munch's profound legacy.",
"title": "Art Meets AI: Understanding Munch Museums art through Dall-E and NorskGPT",
"id": "3ff39708-fa60-4db4-b1dc-6ab993f0b6dc",
"sessionId": "3ff39708-fa60-4db4-b1dc-6ab993f0b6dc",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Bjørn Erik Lie",
"twitter": "",
"bio": "Bjørn Erik Lie is the CTO at Munch Museum."
},
{
"name": "Hira Mehmood",
"twitter": "https://www.linkedin.com/in/hiramehmood/",
"bio": "Hira is the CEO and cofounder for Bineric AI and Bineric Data. \nShe has an MBA from INSEAD business school, France and is also a chartered accountant by education.\nShe has around 15 years of professional experience."
}
]
},
{
"intendedAudience": "Utviklere, tech-leads, arkitekter og alle som er interessert i mikrotjenester",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Mikrotjenester. For omtrent ti år siden ble vi for alvor kjent med konseptet som tok oss med storm. Det var spennende, hipt, moderne og gøy. \nTilhengerene ble flere og flere, de samlet seg, grupperte seg, marsjerte taktfast, fant frem fakler og høygafler. Revolusjonen var nært forestående.\n\"Død over monolitten! Lenge leve Mikrotjenester!\"\n\nTiden var endelig kommet, monolittene skulle slaktes. Monolitt-forkjemperne skulle  brennes. \nEn ny verden skulle formes.\n\nI denne praten legger vi vekk de rosefargede brillene, går back to basics og tar en pragmatisk titt på mikrotjenester. Når skal vi bruke dem? Når skal vi ikke? Hvor store skal de være? Og har monolitten utspilt sin rolle?",
"title": "Og så skulle alle lage mikrotjenester",
"id": "fb2193e4-9516-47a7-8741-b84eb3242d4d",
"sessionId": "fb2193e4-9516-47a7-8741-b84eb3242d4d",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Erik Dahl",
"twitter": "dahlsdata",
"bio": "Erik er java-programmer / techlead / arkitekt med et stort hjerte for arkitektur. Til daglig jobber han som uavhengig freelance-konsulent på offentlige og private prosjekter i Oslo-regionen. Han er alltid på utkikk etter nye perspektiver, ny forbedret forståelse og elsker å ha diskusjoner om arkitektur. "
}
]
},
{
"intendedAudience": "Ingen erfaring kreves. Det er en fordel om du har jobbet med en database før, enten en relasjon-, vektor- eller grafdatabase. Ingen dyp kunnskap om PostgreSQL eller andre databasemotorer er forventet.",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Populariteten til vektordatabaser har skutt i været det siste året, takket være LLMer og Retrieval Augmented Prompting. Databasene Qdrant, Pinecone og Milvus har sett dagens lys, og blitt verdsatt til flere hundre millioner dollar. I 2014 så vi den samme trenden, med grafdatabasenes inntog. Neo4J ble populært, og \"alle\" skulle nå modellere dataene sine som en graf fremfor tabeller. Enda lenger tilbake i 2009 var det MongoDB som stod for mye av hypen, med sin enkle dokumentmodell og JavaScript-liknende spørrespråk. Gjennom alle disse trendperiodene er det likevel noe som har bestått: PostgreSQL, som ble lansert allerede i 1996. Verdens mest populære open-source database lever i beste velgående, og har utviklet seg i takt med tiden. \n\nTrenger man egentlig alle disse andre databasene? Bli med på et dypdykk i verdenen av muligheter innen PostgreSQL og finn ut hvordan du kan erstatte store deler av din teknologistack med et par enkle spørringer.",
"title": "Alt du trenger er PostgreSQL",
"id": "ee9c247a-226c-4453-8e67-9890bf8a0f3b",
"sessionId": "ee9c247a-226c-4453-8e67-9890bf8a0f3b",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Vemund M Santi",
"twitter": "",
"bio": "Vemund er en fullstack-utvikler med en lidenskap for databaser og SQL. Han har erfaring fra både store enterprise-bedrifter og knøttsmå startuper han har vært med på å bygge. Med denne bakgrunnen har han fått et unikt innblikk i ulike måter å benytte seg av verdens mest populære relasjonsdatabase, PostgreSQL, og strukket strikken for hvor mye man faktisk kan benytte denne databasemotoren til."
}
]
},
{
"intendedAudience": "Anyone interested in reflections on learning.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "A lot of people in the IT industry get themselves certifications and do courses, but not so many do extra degrees. In the midst of the pandemic when the world was shut down I decided to go back to school. A few years later I had an extra degree.\n\nIn this lightning talk, I’ll give my reflections on my extra degree and what they can tell us about learning, inside and outside of formal degree programs.\n\nThey tell you that at university you learn how to learn. This is not necessarily so, but it may be so. And I’ll tell you how it may be so.",
"title": "Boost Your Development: Should You Get Another Degree?",
"id": "daa26792-ced3-4bd7-9acd-6d193739d2d8",
"sessionId": "daa26792-ced3-4bd7-9acd-6d193739d2d8",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jørn K Baltzersen",
"twitter": "@jkbaltzersen",
"bio": "Jorn K. Baltzersen is an experienced developer through more than a couple of decades. He holds an M.Sc. in engineering and computer science from the previous century and an M.Sc. (summa cum laude) in blockchain and digital currency (2023) from the University of Nicosia. He is a Principal Solution Consultant at Tietoevry."
}
]
},
{
"intendedAudience": "Alle som er opptatt av at teamene skal være innovative og få rom til å gjøre en best mulig jobb. Alle som er opptatt av at vi skal kaste bort minst mulig penger på løsninger som ikke brukerne vil ha. ",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Bestiller du output fra teamet så senker du innovasjonsgraden til teamet betraktelig. Du senker også teamets mulighet til å lykkes med å lage noe som brukerne vil ha. Derfor forteller jeg i min PhD-avhandling at man i digital produktutvikling må gi teamet resultatautonomi, og vil forklare dette i denne lyntalen. ",
"title": "Støyreduksjon (outcome) eller støyskjerm (output)?",
"id": "d8658b3b-15d2-4ca7-8128-dd92acb02874",
"sessionId": "d8658b3b-15d2-4ca7-8128-dd92acb02874",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Kristin Wulff",
"twitter": "",
"bio": "Har jobbet med prosessforbedring i IT-bransjen i mange år, som leder og konsulent. Tok en PhD fra 2018-2023 på organisasjonsdesign og spesifikt på hva slags autonomi digitale produktutviklingsteam trenger. "
}
]
},
{
"intendedAudience": "Alle som bygger programvare og ønsker å gjøre det på mest mulig givende og effektivt vis.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Forskning viser at \"Continuous Integration\" og \"Continuous Delivery\" gir økt hastighet, økt kvalitet og økt trivsel for oss som lager programvare. Men hvordan? Jo, med \"Trunk based development\", \"feature toggles\", automatiserte bygg, og par/mob programmering i stedet for å kaste pull requests på hverandre. Hvordan får vi til dette i praksis? Det skal jeg fortelle deg i dette foredraget, med praktiske eksempler.",
"title": "Slik leverer du kontinuerlig",
"id": "944da7b7-9c3f-414a-8368-e0d21be9aba3",
"sessionId": "944da7b7-9c3f-414a-8368-e0d21be9aba3",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Christian Johansen",
"twitter": "cjno",
"bio": "Jeg har bygget webløsninger i over 20 år, og jobber nå i et lite team hos Mattilsynet der vi er to utviklere. Jeg har committa til master/main og bygget rett til prod i over 10 år og har bred erfaring med å finne gode løsninger og få ting levert uten så altfor mye seremoni. Jeg skriver jevnlig om utvikling på https://parenteser.mattilsynet.io"
}
]
},
{
"intendedAudience": "Java developers who are interested in how the language, APIs, and runtime evolve - either because they're about to use the newest versions or because they just want to know what's happening.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Java 21 was an explosive release, but 22 and 23 are no slouches either:\n\n* from unnamed patterns to string templates\n* from the foreign-function and memory API to Stream gatherers and the class-file API\n* from a simpler main to launching multi-source-file programs\n\nThere are plenty of improvements to the language, API, and VM to discuss - whether new, improved, or finalized. So let's go over them!",
"title": "Java 23 - Better Language, Better APIs, Better Runtime",
"id": "9b81c061-10e5-4f0f-97c0-e15a62e759cc",
"sessionId": "9b81c061-10e5-4f0f-97c0-e15a62e759cc",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Nicolai Parlog",
"twitter": "nipafx",
"bio": "Nicolai (aka nipafx) is a Java enthusiast focused on language features, core APIs, and runtime evolution with a passion for learning and sharing. He does that mostly at conferences and in his biweekly Inside Java Newscast, but also occasionally in live streams, articles, and books - more on all that on [nipafx.dev]. He's a Java Developer Advocate at Oracle and otherwise best known for his haircut.\n\n[nipafx.dev]: https://nipafx.dev"
}
]
},
{
"intendedAudience": "Passer for alle. Du trenger ikke være utvikler eller ha nostalgibrillene på for å få glede av dette foredraget. Er du interessert i hvordan gamle datamaskiner fungerer? Eller lurer på hvordan man laget spill før? Lurer du på hva demoscenen er? Kom og lær!",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Commodore Amiga var verdens beste hjemmedatamaskin på slutten av 80-tallet, og et stykke inn i 90-tallet, i følge de troende. Ricki var en av dem. Amiga var frøet som startet karrieren til mange spillutviklere, musikere og film-produsenter. Den hadde god grafikk, fantastiske spill, og man kunne også gjøre leksene på den, hvis man ville. \n\nDe siste par årene har Ricki funnet tilbake til Amigaen, etter noen år med spillutvikling på C64. Ricki vil fortelle om sine opplevelser med Amiga på tidlig 90-tallet, historien til Amiga, og hvordan det er å lage spill til Amiga idag.\n\nDu får et innblikk og i verktøy og teknikker i moderne retro utvikling, og hvordan komme i gang.",
"title": "Retro-nerding med Commodore Amiga",
"id": "7fad2630-74d1-4275-a2d0-aeed0cc9f409",
"sessionId": "7fad2630-74d1-4275-a2d0-aeed0cc9f409",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Ricki Sickenger",
"twitter": "@bag_of_hats",
"bio": "Ricki Sickenger har 10 års erfaring fra spillbransjen, og ble konsulent i 2009. I 2012 var han en av gründerne av Sonat Consulting Bergen, der han fremdeles jobber som utvikler og tech lead. På fritiden utvikler han spill på gamle plattformer som C64 og Amiga."
}
]
},
{
"intendedAudience": "Are any Java developers interested in Quarkus. Spring Boot developers investigating dev mode.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "If you are a Java developer, chances are you’ve already heard about Quarkus. You may have heard about the developer joy that Quarkus provides. But what exactly is it? How does it enhance your day-to-day tasks? What benefits and shortcuts does it offer for your work? These are the questions we will address in this talk.\n\nDuring this live-coded session, we will create a new Quarkus project and demonstrate how it can expedite the development of Java microservices by leveraging its multitude of out-of-the-box features. The session will be interactive, so if there is something specific you want to see, we will gladly demonstrate it. Our primary focus, however, will be showcasing typical user application development scenarios, including database integration, remote connections using both blocking and asynchronous APIs, and implementing security measures. Naturally, we cannot overlook the crucial aspects of packaging applications into containers and deploying them to the cloud.\n\nQuarkus is a framework that places developer productivity as a top priority, fostering a sense of joy throughout the development process. You will witness the remarkable things we can achieve with Quarkus within the given time. By the end of the session, you will truly grasp the essence of Quarkus' development joy.",
"title": "A day in the life of a Quarkus developer",
"id": "21bf0c09-c247-489f-9fa3-bbae1cb1e8f3",
"sessionId": "21bf0c09-c247-489f-9fa3-bbae1cb1e8f3",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Martin Stefanko",
"twitter": "xstefank",
"bio": "Principal software engineer at Red Hat, BrnoJUG leader, author Quarkus in Action book, MicroProfile committer, working on Red Hat middleware technologies like Quarkus, SmallRye, Wildfly, JBoss middleware (RESTEasy, Weld, …​), programming and microservices enthusiast."
}
]
},
{
"intendedAudience": "Anyone involved in software development can get something from this talk. Participants will benefit from a new perspective on their work. It will also satisfy any nerdy interest in aviation and break up a day of more specific/technical sessions.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "What can aviation teach us about software? More specifically, what can aviation disasters teach us?\n\nA lot actually.\n\nIn the first edition of this talk, I focused on the human factor of aviation disasters. An important element of both software engineering and aviation. But that is just the tip of the iceberg.\n\nIn this talk, I will go deeper into the technical oddities of some of the most famous aviation disasters. Buckle up for more case studies, more geeky stuff, and yeah, also some more human stuff.\n\nWe will cover\n- Most common causes of aviation disasters and how that has changed over time\n- Redundancy in systems\n- Deadly UX\n- Project failures and (wrong) incentives\n\nDespite the subject matter, this talk is not doom and gloom. It is a practical look at the methods and insights that almost 100 years of investigating commercial aviation disasters can teach us as software engineers.",
"title": "Mayday Mark 2! More Software Lessons From Aviation Disasters.",
"id": "0b40a4c8-5a56-4798-a930-75f40797b9f5",
"sessionId": "0b40a4c8-5a56-4798-a930-75f40797b9f5",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Adele Carpenter",
"twitter": "iam_carpenter",
"bio": "\nAdele is a software engineer at Trifork Amsterdam (and aviation geek in her spare time), where she is working on backend systems for the educational sector. Most of her work day is spent in the JVM/Spring ecosystems. Adele got the coding bug later in life but since then has been making up for lost time, going from command line noob to employed software engineer in just one year. Her experiences both in and out of tech have given her a unique perspective on the art of programming together with humans, which she hopes is useful to other humans who program with humans."
}
]
},
{
"intendedAudience": "People interested in building their careers and understanding why contributing to communities is a powerful process that benefits contributors, employers, and the community. Remember, everyone benefits when everyone contributes!",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "The world has changed — everything is fast. You’re busy updating your technical skills and leveraging the latest tools, but are you using the most efficient learning techniques to improve yourself? Are you building your own network in a systematic way by contributing to FOSS communities?\nThere are massive benefits to contributing to Open Source projects, especially Java as one of the largest and most innovative software development communities in the world. But remember that although communities are fun and valuable they can also be fragile and change radically. They need continual nurturing so they can support opportunities for contributors.\n\nIn this session, we’ll explore the skills necessary to learn new things deeply, offer contributions of value to communities, and build our own networks to leverage markets. The result is that we benefit, the community benefits, and our employers benefit. These three levels represent critical reciprocal relationships that need careful consideration.\n\nWe'll touch on concepts from network science, neuroscience, education, and history to help promote innovation and learning. We’ll also cover the who, what, where, when, why, and how of contributing to FOSS communities, highlighting Java specifically and the Japan Java User Group. And finally, I’ll share some career success stories of my own and also some of my catastrophic failures. I've photographed thousands of developers and interviewed hundreds of them for videos, streams, and podcasts, so we surely have many images to show and lessons to learn as we become inspired to thrive in this crazy fast world.",
"title": "Developers — Contribute to FOSS Communities Now!",
"id": "b0dfb362-91b2-4d19-aa0b-4ec26097af40",
"sessionId": "b0dfb362-91b2-4d19-aa0b-4ec26097af40",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jim Grisanzio",
"twitter": "@jimgris",
"bio": "I’ve managed projects in software, biotech, publishing, and construction. I’ve navigated multiple economic and political systems, sparred with several unions, survived a few near-death experiences, and overcame serious medical limitations. I’m lucky to be alive and thankful to be working.\n\nI ran my own excavating and real estate development business, and I was also a mechanic and a truck driver. After everything crashed I picked up the pieces, went back to school, and eventually became a publicist, an editor, and a writer. I’ve worked with the global news media and local, state, and federal government officials; I’ve interviewed hundreds of engineers, scientists, and clinicians; and I’ve produced thousands of articles, photos, videos, and podcasts. In recent years I’ve been building FOSS communities at Sun and Oracle, managing developer events globally, and delivering my own community sessions at conferences.\n\nCurrently, I'm the host of the Duke's Corner Podcast on Oracle's Java Developer Relations team and I profile Java developers.\n\nFull Bio: https://jimgrisanzio.wordpress.com/"
}
]
},
{
"intendedAudience": "Hvis du er interessert i å lære mer om hendelsesdrevne systemer og hvordan vi jobber i NAV er denne workshopen for deg! Vi anbefaler at du har noe kjennskap til Python og/eller programmering, men du trenger ikke være en ekspert.\n",
"length": "240",
"format": "workshop",
"language": "no",
"abstract": "I NAV utvikler vi moderne systemer i hendelsesdrevet mikrotjenestearkitektur som skal løse fremtidens arbeids- og velferdsbehov. Nå vil vi lære dere mer om hvordan vi bygger applikasjoner og hvordan hendelsesdrevne systemer kan brukes til å løse komplekse utfordringer.\n\nVi kommer til å presentere viktige prinsipper bak hendelsesdrevne systemer før vi kjører i gang med et “gamified” opplegg hvor dere koder sammen to og to. Gjennom kurset skal dere utvikle en applikasjon som kobles mot Kafka for å motta utfordringer deres applikasjon må håndtere. Dette blir en morsom workshop med litt utfordring og mye moro!\n\nTania Holst, Emil Elton Nilsen, Øydis Kind Refsum, Helene Arnesen og Kyrre Havik er alle utviklere som jobber innenfor forskjellige produktområder i NAV. De har forskjellige erfaringer fra både teknologier og kunnskap, og utvikler dette kurset for å selv bli bedre på hendelsesdrevne systemer og programmering.",
"title": "Hendelsesdrevet utvikling med Leesah Game",
"workshopPrerequisites": "Før du kommer til kurset:\n\n* Installer en valgfri IDE\n* Installér Python 3.10 (eller nyere) – https://www.python.org/downloads/\n* Ta gjerne en titt på https://kafka.apache.org/\n\nTa med pc/mac til kurset",
"id": "003748cd-a212-493d-bdad-83d27209d05f",
"sessionId": "003748cd-a212-493d-bdad-83d27209d05f",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Øydis Kind Refsum",
"twitter": "",
"bio": ""
},
{
"name": "Kyrre Havik",
"twitter": "",
"bio": ""
},
{
"name": "Emil Elton Nilsen",
"twitter": "",
"bio": ""
}
]
},
{
"intendedAudience": "Who I believe will benefit from this workshop:\nSoftware Developers/Engineers, Data Engineers, Solutions Engineers/Architects, AI Engineers, and Technical Decision Makers\n\nHow will the participants benefit from attending:\n* Learn the fundamental building blocks about building GenAI/ChatGPT apps\n  - LLMs\n  - Agents\n  - Chains\n  - Memory\n  - Model I/O\n* Build a basic ChatGPT app that interacts with LLMs\n",
"length": "240",
"format": "workshop",
"language": "en",
"abstract": "With ChatGPT taking center stage since the beginning of 2023, developers who have not had a chance to work with any forms of Artificial Intelligence or Machine Learning systems may find themselves either intrigued by the “maze” of new terminologies, or some may be eager to learn more, while perhaps a smaller group may not actually want to get themselves into a territory that’s unknown to them. \n\nThis workshop is catered for developers from all backgrounds (Java, Javascript, Python, etc), as we start by having a quick introduction to GenAI, ChatGPT, and all of those new terminologies around generative AI.  Then we’ll dive right into the hands-on part, about how we can construct a ChatGPT-based app quickly, using state-of-the-art tools such as PgVector, which provides vector extension to the popular open source Postgres.\n\nHands-on lab will cover:\n- Vector Search with PgVector\n- LLM providers and APIs\n- Integrating with ChatGPT models\n- Generating embeddings\n- Prompt engineering\n- Building generative AI applications\n",
"title": "Demystifying GenAI: Building a ChatGPT App with Spring, LangChain4J, and Vector Store",
"workshopPrerequisites": "* Java 8 or above installed\n* OpenAI API Key ready for use",
"id": "492896b6-5307-482d-801f-62d4afac5873",
"sessionId": "492896b6-5307-482d-801f-62d4afac5873",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Mary Grygleski",
"twitter": "@mgrygles",
"bio": "Mary is a Java Champion, and an experienced, passionate Developer Advocate.  She has serviced companies as an advocate, such as IBM and DataStax in topic areas that include, most recently, GenAI, Streaming systems, Open source, Java, Cloud, and Distributed Messaging systems..  She started as an engineer in Unix/C, then transitioned to Java around 2000 and has never looked back since then.   She is an active tech community builder outside of her day job, and currently the President of the Chicago Java Users Group (CJUG), as well as the Chicago Chapter Co-Lead for AICamp."
}
]
},
{
"intendedAudience": "Anyone who wants to see how cool CSS has gotten recently 🤓",
"length": "10",
"format": "lightning-talk",
"language": "en",
"abstract": "How often do you take the time to read about all the cool new stuff that's been added to CSS in the last few years? I did exactly that, so you don't have to, and I want to share the awesome features I found with you!\n\nDid you know that changing the styling of an element based on the width of its parent is actually really easy? And how about custom colors for native input elements or a powerful way to organize selector specificity? It's all just a few lines of CSS away!\n\nJoin me in this lightning talk as we take a quick look at 7 CSS features that range from relatively new to cutting edge. We of course also cover how these can be used in practice without throwing older browsers to the wolves.",
"title": "Exploring Features in Modern CSS",
"id": "34839cbc-fa7c-4450-b0ae-cd13f43dd86b",
"sessionId": "34839cbc-fa7c-4450-b0ae-cd13f43dd86b",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jakob Endrestad Kielland",
"twitter": "@itzjacki",
"bio": "Jakob works as a developer and consultant in Variant Trondheim, and has been a proper nerd his entire life. Right now he is building Norway's student platform of the future together with a very skilled group of people, and focuses on the frontend.\n\nIn addition to a solid interest for technology, Jakob has a passion for teaching. He loves the challenge of communicating something complex in a simple and engaging way, something which should shine through in his talk!"
}
]
},
{
"intendedAudience": "Developers interested in learning more about how to get into hacking and pentesting.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Have you ever been hacked or wondered how people figure out how to compromise systems? What tools do they use? What skill sets are involved? And how can you and your team gain knowledge on better securing your systems and service?\nLet us venture on a journey into the wonderful world that is web exploitation and learn how to use gamified safe environments to build skillsets to help us write better and safer software. Because once you learn how to hack, you never go back. This session is all about exploiting web tech, so no binary voodoo or super low level skills required.",
"title": "How hacking works - Web edition",
"id": "c0e267b2-59ec-4f68-9e0d-82417c4694c2",
"sessionId": "c0e267b2-59ec-4f68-9e0d-82417c4694c2",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Espen Sande-Larsen",
"twitter": "",
"bio": "Making rocks think since 1986...\n\nEspen is a developer and architect with over 25 years of experience. He has worked on everything from embedded systems, electronics and games to large scale cloud applications. He is as full-stack as they come. Espen started coding at age six on a C64, and has been cranking out software ever since.\n\nHe served as the principal architect, tech lead and lead security\nengineer on the compensation scheme for Covid-19 suffering\nbusinesses released by the Norwegian government.\n\nEspen was the Senior VP of Technology Exploration for DNB in San Fransisco and is currently doing technology and security research in DNB NewTechLab."
}
]
},
{
"intendedAudience": "Alle deltakere på JavaZone på tvers av alle kjønn og roller. Du vil få konkrete tips som vil gjøre deg bedre rustet til å skape et bredere mangfold i ditt arbeidsmiljø.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Hvor er damene? Vi har sett på kvinneandelen i tekniske roller i et av Norges største IT-miljøer, NAV IT. Spesielt offentlig sektor burde være best i klassen, men tallene er ikke oppmuntrende. Bli med og utforsk IT-verdenens usynlige barrierer og hva vi kan gjøre for å rive de ned og skape et mer inkluderende arbeidsliv. Vi vil dele personlige erfaringer som belyser utfordringene vi står ovenfor i 2024, samtidig som vi ser på forskning og statistikk på området. \n\nVi ser på hvor tidlig vi introduserer kjønnsroller, utforsker viktigheten av tekniske rollemodeller og ser på behovet for endring i hvordan vi snakker om utviklerrollen. Hvordan kan vi skape ringvirkninger ved hjelp av økt oppmerksomhet rundt ubevisste bias og sørge for at ingen kvinner på JavaZone trenger å bli spurt om de jobber i HR! \n\nDenne talken gir deg en mulighet til å høre direkte fra kvinner i tech, og du får med deg fem konkrete tips for å skape et arbeidsmiljø vi alle vil være en del av. Fokuset ligger på kjønnsdimensjonen, men tipsene er i stor grad overførbare for å øke inkludering av andre underrepresenterte grupper, noe som igjen vil føre til økt trivsel for oss alle! \n",
"title": "Hvor er damene?",
"id": "74ae82c0-cb34-4dcb-a35b-28fe9ea42bb6",
"sessionId": "74ae82c0-cb34-4dcb-a35b-28fe9ea42bb6",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Caroline Winther",
"twitter": "",
"bio": "Caroline jobber i dag som data engineer i NAV IT, men har siden ingeniørutdannelsen tatt veien via analytiker-roller i både kraftbransjen og i folketellingen på New Zealand. Siden hun kom inn i IT-verdenen i 2020, har hun tenkt mye hvor kult det egentlig er å jobbe i IT, men også blitt nysgjerrig på hvorfor det er så få damer her. Caroline har jobbet med å samle inn data for å belyse nettopp dette og gjøre organisasjonen bedre rustet til å øke mangfoldet! "
},
{
"name": "Aurora Christine Hofman",
"twitter": "",
"bio": "Aurora jobber som data scientist og gledesspreder i NAV IT. I det siste har hun også fått prøve seg som utvikler og storkoser seg med det! Når hun ikke er med teamet er sjansen stor for at hun er et eller annet sted og presenterer eller har dratt med seg intetanende kollegaer på spaserturer i nærområdet. Aurora er brennende engasjert for å øke kvinneandelen i tekniske roller i NAV og øke forståelsen for hvorfor dette er essensielt for å lage gode løsninger for Norges befolkning.  "
}
]
},
{
"intendedAudience": "This presentation is aimed at developers (both novice and experienced) and technology enthusiasts looking to enhance their demo presentations by integrating storytelling as an effective tool. The goal is to inspire and guide participants to create more engaging and persuasive presentations that appeal to emotions and establish a deeper connection with the audience.\n\nParticipants will learn how to thoroughly prepare for their presentation, both technically and in terms of content, enabling them to maximize the impact of storytelling and avoid common mistakes that can weaken the message. This talk is ideal for developers who want to stand out from the crowd and make an impact with their demo presentations.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "As developers we sometimes have to conduct demos, and we all know they can be a bit dull. But demo presentations aren't going away, so why not add some ingredients to make the story shine?\n\nBecause demo presentations aren't just about showing what we can do. They're an opportunity to convey a story, to draw in those watching into a narrative that evokes emotions and inspires action. Whether it's customers, colleagues, or partners.\n\nAs a developer, I've conducted many demo presentations, and I want to use this talk to be inspire others to use communication techniques that not only explain how the solution works but also capture the interest of the listeners. Because that might be the most important thing.",
"title": "How to Use Storytelling in Demo Presentations",
"id": "3334d800-83d4-41cb-96c0-9094b3afe360",
"sessionId": "3334d800-83d4-41cb-96c0-9094b3afe360",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Elise Johnsen",
"twitter": "",
"bio": "I am a senior consultant at Netcompany with experience in system development and customer collaboration. Over the past years, I’ve been a developer in the DigiBarnevern project, where we are developing a new case management system for child welfare services in selected municipalities in Norway. Throughout this project, I have engaged closely with our clients, primarily social workers within the child welfare sector. My experience with demo presentations stems from the need to communicate complex technical solutions in a simple and understandable way for non-technical users. In such a context, it is crucial to focus on the functionality of the solution and how it effectively addresses existing problems. As a technology advisor, I am aware of the importance of oral communication and the ability to convey technical concepts in an accessible manner."
}
]
},
{
"intendedAudience": "Denne presentasjonen passer for utviklere på alle nivå som er interessert i en effektiv måte å bryte ned større oppgaver til flere mindre, men konstruktive, endringsforslag.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "I forbindelse med moderniseringen av Felles studentsystem og Samordna opptak, opplevde vi en betydelig oppskalering av antall team og utviklere. En uheldig konsekvens av denne utvidelsen var en vesentlig økning i tidkrevende og frustrerende merge-konflikter. Etter litt research fant vi ut at dette er et vanlig problem som kan løses ved å implementere Trunk Based Development.\n\nDet er lett å forstå at det vil oppstå færre og enklere merge-konflikter hvis en branch bare eksisterer i en dag eller to. Men å forstå hvordan man utfører en større endring under dette regimet er en større utfordring, spesielt når ingen av oss har gjort det før.\n\nNettstedet https://trunkbaseddevelopment.com/ har vist seg å være en utmerket ressurs, men det var først da jeg kom over boken \"The Mikado Method\" av Ola Ellnestam og Daniel Brolund at ting virkelig begynte å falle på plass.\n\nDenne boken introduserer Mikado-metoden, en teknikk som hjelper deg å utføre store, komplekse endringer på en trygg og forsvarlig måte. Metoden er spesielt nyttig når man arbeider med Trunk Based Development, da den gir en strukturert tilnærming til hvordan du stykker opp arbeidet til flere endringer som kan merges fortløpende.\n\nKom og lære mer om hva Mikado-metoden handler om, hvordan den fungerer, og hvordan den kan hjelpe deg og ditt team å håndtere store endringer mer effektivt. Jeg tør påstå at etter bare 10 minutter så har du lært nok til å prøve på egenhånd.\n",
"title": "Hva har egentlig Mikado med programmering å gjøre?",
"id": "5b1b94fe-29cd-4a9c-bd52-1719c96b34e8",
"sessionId": "5b1b94fe-29cd-4a9c-bd52-1719c96b34e8",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Alf Lervåg",
"twitter": "",
"bio": "Velkommen til denne lyntalen om Mikado-metoden. Jeg er Alf Lervåg, en kodende\narkitekt i produktområdet Studieadministrasjon i Sikt. Jeg har en lidenskap for\nåpen kildekode og tilgjengeliggjøring av offentlige APIer, og jeg trives best\nnår jeg finner elegante løsninger som gjør komplekse problemer enkle å forstå.\nJeg har lite tålmodighet for dårlig datakvalitet, ineffektivitet og unødvendig\nkompliserte løsninger. Sistnevnte gjelder forøvrig ikke for min egen datamaskin. Her kjører jeg nemlig NixOS med Emacs som window manager."
}
]
},
{
"intendedAudience": "Developers, developers, developers, developers (and other tech minded people)",
"length": "240",
"format": "workshop",
"language": "en",
"abstract": "OpenTelemetry might be the biggest thing that has happened in the OpenSource ecosystem since Kubernetes. With a much steeper contribution graph, and the recent General Availability of the specification, OpenTelemetry is now ready for prime time with more and more observability vendors supporting the standard.\n\nGetting insights from your applications is absolutely vital for modern software development teams that want to release new functionality with confidence. The process of programming your application to give this insight is often called instrumentation and we often refer to the three pillars of observability; logs, metrics and tracing.\n\nThe instrumentation itself can be a hassle and will take some time depending on the size and of the application and how modern the framework and libraries are, but it is manageable. The real value comes when you instrument enough of the applications within a team or a business domain. \n\nThis also brings us to the challenge – instrumentation technology that works across different languages and frameworks without having to rewrite those applications from scratch. Different vendors have been providing this as their secret sauce if you accept to be cemented into their walled garden and lock this vital information in their monitoring system. Until now with OpenTelemetry! The universal telemetry toolkit for all your observability needs. \n\nOpenTelemetry is a graduated CNCF project with first release in 2019 after OpenCensus and OpenTracing decided to merge. It has support for all major programming languages (Java, .NET, Go, Python, ++) and more and frameworks such as (Spring ++) has built in support for OpenTelemetry. All major monitoring tools and platforms (such as the Grafana stack) are contributing and supporting OpenTelemetry in one way or another.\n\nIn this workshop, you will learn how to get a modern observability stack up and running with the open source monitoring platform from Grafana – the LGTM stack (Loki, Grafana, Temp and Mimir). You will also learn how to instrument different applications with OpenTelemetry SDK and agent, to gain insight into the application’s performance. ",
"title": "Observability with OpenTelemetry: From Idea to Insight",
"workshopPrerequisites": "* Laptop with Docker\n* Intellij/Visual Studio Code",
"id": "d1a9a205-4330-4920-bea9-01b6ad342a99",
"sessionId": "d1a9a205-4330-4920-bea9-01b6ad342a99",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Øyvind Randa",
"twitter": "",
"bio": "Technology enthusiast and Leader for Google Developer Group Bergen. Working as Solution Architect for NextGenTel."
},
{
"name": "Hans Kristian Flaatten",
"twitter": "https://www.linkedin.com/in/hansflaatten/",
"bio": "Dad to three kids, loves open source and being outdoor. Platform developer in NAV with responsebility for the observability stack for the NAIS-platform."
}
]
},
{
"intendedAudience": "Alle",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "I dette foredraget snakker jeg om hvordan TV 2 bruker AI for å levere høydepunkter fra Eliteserien i produksjon. Foredraget tar for seg bakgrunnen til prosjektet, utfordringer møtt underveis, den tekniske løsningen, arkitektur og sluttresultatet.\n\nForedraget passer godt for de som enten er interesserte i hvordan AI kan anvendes i produksjon, er interessert i sport eller bare er nysgjerrig. \n\n\n\n\n\n",
"title": "Hvordan TV 2 bruker AI for å levere høydepunkter fra Eliteserien i (nesten) Sanntid",
"id": "40bca3d4-f94c-4b7e-985e-d85f90f02f11",
"sessionId": "40bca3d4-f94c-4b7e-985e-d85f90f02f11",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Casper Feng",
"twitter": "",
"bio": "Casper er en utvikler som trives i alle deler av stacken, og har erfaring fra flere bransjer."
}
]
},
{
"intendedAudience": "Any developer wanting to stay up to date on whats happening on the frontend side of things. As a web dev, they will get the tools needed to start developing with React Server Components. Frontend and React experience is ideal.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "React Server Components are a new feature that lets you render React components on the server and stream them to the client. This enables you to build rich user interfaces with minimal client-side code, while still preserving interactivity and performance.\n\nIn this talk, you will learn how React Server Components work, how to use them in your web apps, and what benefits they offer over traditional approaches. You will also see some real-world examples of React Server Components in action and get some tips and best practices for adopting them in your projects.",
"title": "React Server Components: A New Way to Build Fast and Interactive Web Apps",
"id": "15a66f1c-91ca-481d-9d9c-ba4630ea9249",
"sessionId": "15a66f1c-91ca-481d-9d9c-ba4630ea9249",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Aurora Scharff",
"twitter": "https://twitter.com/aurorascharff",
"bio": "Aurora Scharff is a full-stack cloud developer from Norway. She is skilled in multiple technologies and domains, such as React, Angular, .NET, and Azure, and has recently gained a great interest in web development specifically. She holds a bachelor's degree in Robotics and Intelligent Systems, with knowledge of engineering mathematics, robotics, algorithms, data structures, and computer architecture. After working for a couple of years at a financial technology startup, she continues developing as a consultant at Inmeta."
}
]
},
{
"intendedAudience": "Key takeaways:\n - Why architecture failed and was effectively killed\n - Why agile has been a bit of a disappointment\n - Some ideas about what needs to be done if we want to do better",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Twenty years ago, Architecture was murdered in its sleep by Agile, as part of the assassination plot against the arch-enemy Waterfall. Fast forward to today, and Agile isn't looking too healthy either: reduced to hollow-eyed cargo culting and pointless rituals, wading through the debris of methodology pop culture, or bloated to a monstrous parody of itself in order to scale. But death aside, they are both still among us, and moaning for brains. Why?\n\nOne reason might be that the problems that Architecture and Agile set out to fix have not been solved. Or they need continuous solving, and in the absence of new life to tackle them, we animate our dead. What are these problems, and how do we see them today?\n\nHave we learned anything from the death of these two grand attempts at fixing software development, and their refusal to lie down even in the face of death? Is there still something that can be found in the black hearts of the undead that can be of use in addressing the problems we face? Should Agile and Architecture be revived or put out of their misery for good? Will we be able to? What new kind of life do these two half-lives point towards? What acts of bravery will be required of us going forward?",
"title": "Agile and Architecture: a meeting of the undead",
"id": "a96ef1b7-68fb-4c50-ba09-e8014c287bda",
"sessionId": "a96ef1b7-68fb-4c50-ba09-e8014c287bda",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Einar W. Høst",
"twitter": "@einarwh",
"bio": "Einar W. Høst is a socio-technical advisor at the Norwegian Labour and Welfare Administration. He enjoys collaborative modelling, API design and computer programming. Over the past ten years, he has done talks on a variety of topics, including hypermedia, resiliency, recursive art and lambda calculus. He has a PhD in Computer Science from the University of Oslo."
}
]
},
{
"intendedAudience": "Anyone who wants to know more about the theoratical side of monads. It helps if there is a bit of coding experience since we will learn mostly by giving examples and look at what aspect the examples all share. This way people can learn though pattern recognition instead of theoretical knowledge.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Functional programming terminology can be quite intimidating, but it does not have to be.\nLearning this terminology actually helped me in becoming a better problem solver, and therefor better at programming (in any language).\nIn this talk we will go into what all these fancy words like monad, monoid, functor etc. actually mean.\nWe will go into a bit of theory (but not as far as category theory), and there will be a strong focus on practical everyday examples.\nAfter this talk you will see that you have been using monads all along, know why the Java Optional is not a monad, and your abstract thinking skills will level up.",
"title": "Monads explained",
"id": "25d7aeb9-6d82-406a-a4c0-a5f946c9a0ef",
"sessionId": "25d7aeb9-6d82-406a-a4c0-a5f946c9a0ef",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Ties van de Ven",
"twitter": "@ties_ven",
"bio": "Ties is a software engineer with passion for concepts, sofware engineering fundamentals and helping others. He combines these passions by doing public speaking, volenteer work for organisations like Devoxx4kids and codingcoach and working as a Software Quality Expert at Alliander.\n"
}
]
},
{
"intendedAudience": "The entire team: Developers, designers, tester and more. No previous knowledge is required, but any experience with web development and accessibility is an advantage.",
"length": "120",
"format": "workshop",
"language": "en",
"abstract": "One thing we rarely have enough of when working on web development is time. There is always some new feature that is needed, a bug to be fixed or some pressing matter that requires you to speed up the time to release. Unfortunately, this can come at the expense of accessibility. While I maintain that ensuring accessibility is just another part of development, just like design, JavaScript coding, testing or security, reality is that sometimes we are not given enough time to check the product before releasing it to the users.\n\nAs such, we need to make sure we spend the time we have as efficiently as possible. To help us do that, I have come up with my very own Accessibility Testing Pyramid. To be clear, that is an entirely un-scientific method. Not founded in research, exposed to peer review, etc. Just based on more than a decade of experience working on accessibility as a frontend developer.\n\nHowever, I do think it has a lot of value, especially because it helps you prioritise which tests provides best value for the time you have available. It also lets you get started with the minimum amount of tools and provides a lot of testing that anyone on your team can perform, regardless of whether they are a developer, designer, tester, subject matter expert, product owner or something else. I want the entire team to have a stake in the accessibility of the product, and this method helps with that.\n\nAttending this workshop will teach you cheap and quick methods for testing for accessibility, as well as what you should test for with each method. These are highly practical tips that you should be able to use as soon as you are back to work. Combined, the methods also provide a quite thorough accessibility test of your product.",
"title": "Accessibility testing for the entire team",
"workshopPrerequisites": "Participants needs to bring a computer, but nothing more.",
"id": "909123ff-a497-4ab5-8a08-c67928d95137",
"sessionId": "909123ff-a497-4ab5-8a08-c67928d95137",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Vegard Haugstvedt",
"twitter": "it_vegard",
"bio": "An experienced frontend developer who has worked with accessibility for over a decade, Vegard started is working for NAV's design system team, Aksel. There, he gets to work in the cross-section between frontend development, design and accessibility, just like he likes it!\n\nHe is a recurring speaker at several Norwegian conferences, primarily trying to spread the good word about accessibility, to help others learn how they can make their websites work for everyone. His career goal is to make the web a more accessible space."
}
]
},
{
"intendedAudience": "Everyone, but especially architects and management roles. Patterns is a universal concept that is mostly talked about at the lower levels of an organisation, but has the highest consequences when misapplied by upper management. \n\nNo prior experience is needed, but experience with microservices, agile, or digitalisation is beneficial.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Architecture, at every level, is about finding the right balance between capabilities and costs to maximise value. Whether we're discussing strategic, organisational, or system structures, we often rely on established patterns to guide quality. However, while the core literature may highlight the numerous benefits of these patterns, Magnus aims to shed light on the often-overlooked aspect: patterns come with costs. They represent trade-offs that have been effective in specific scenarios for certain companies. In this talk, discover how to critically analyse and adeptly apply patterns to enhance the architecture of your software, organization, or entire company, with three example patterns: Microservices, Agile, Digitalisation. Understanding, not familiarity, allows you to choose the right pattern, and the right parts of the pattern, to apply to your challenges.",
"title": "The dark side of patterns",
"id": "9f16c779-79c9-42ab-8a87-59f908207291",
"sessionId": "9f16c779-79c9-42ab-8a87-59f908207291",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Magnus Elden",
"twitter": "",
"bio": "Magnus is a seasoned architect that brings a unique blend of expertise to the table, with an extensive background that spans from hands-on development to strategic oversight as a Chief Technology Officer. His holistic approach to architecture allows him to integrate diverse experiences, offering fresh insights on traditional subjects while shedding light on overlooked challenges."
}
]
},
{
"intendedAudience": "Anyone interested in learning more about machine learning. No coding experience needed, the code and environment is ready to go in the cloud.",
"length": "120",
"format": "workshop",
"language": "en",
"abstract": "At Norkart we aim to develop the best AI models for automatic mapping of objects from aerial imagery. With a wealth of already labeled objects, such as buildings, we find ourselves in a somewhat unique position - we are able to train precise Machine Learning models without the need of manually annotating data.\n\nIn this workshop you will be guided through the process of collecting training data, training machine learning models to identify objects in aerial imagery and validating those models on new regions the model hasn't seen yet. No coding experience needed. The code and environment is set up in the cloud, just bring your laptop.\n\nJoin us for an inspiring session about Machine Learning in the geospatial domain - maybe you'll learn a thing or two as well!",
"title": "GeoAI - From Pixels to Patterns: Image Segmentation from Aerial Imagery",
"workshopPrerequisites": "No preparations needed",
"id": "66395699-71d6-4859-86e4-3ed1429ee1cb",
"sessionId": "66395699-71d6-4859-86e4-3ed1429ee1cb",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Malte Loller-Andersen",
"twitter": "@realMalte",
"bio": "Malte is a passionate machine learning engineer working for Norkart, operating in the geospatial domain. Through various projects for big Norwegian companies, he has gathered knowledge about data engineering, machine learning and MLOps.\n\nIn addition to diving deep into the technicalities, Malte enjoys sharing knowledge and holding talks at various conferences. Since starting his career he has held workshops and talks at several international conferences, and he will continue doing exactly that until a global pandemic stops him. If a global pandemic even can stop him!"
},
{
"name": "Mathilde Ørstavik",
"twitter": "",
"bio": "Mathilde is the head of AI at Norkart. She has broad experience in applied AI on geospatial data, in particular in semantic segmentation from aerial high-resolution data. She enjoys sharing her vision and inspiring audiences at the intersection of geospatial data and artificial intelligence."
}
]
},
{
"intendedAudience": "The session is primarily aimed at software engineers and architects without need for particular previous experience.\n\nThe desired takeaways of this session:\n- Lessons learned for integrating (very) old software systems\n- pitfalls and solutions for related problems such as encryption/SSL, distributed systems and transactions, mismatching staging concepts, testing, load/performance + non-functional requirements\n- strategies for planning and implementation of legacy integration\n- thoughts on future-proof systems, documentation and organizations (is future-proofing an illusion?)\n- smart ways of integrating legacy systems in my project/work environment\n\nI would imagine that working with a system this old should be out of the ordinary and interesting even for seasoned industry veterans.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Over the past years we have rebuilt the sales platform of Deutsche Bahn with over 100 microservices, 400 people in over 30 teams and thereby (re-)created one of the biggest e-commerce platforms in Germany.\n\nDuring this process we felt increasingly like Marty McFly because in order to get early user feedback we began by integrating the \"Elektronische Platzbuchungsanlage\" (EPA), a software system for seat reservations that went into production in 1983(!). With system integration being a core activity in enterprise software engineering and architecture, and legacy systems in particular being a common occurrence in the everyday work of many software engineers we would like to share some in-the-trenches experiences of the problems we faced. Let's embark on a journey through the past beginning with poorly accessible documentation and experts threatened by extinction, over generational leaps in technology for example regarding interfaces, unclear non-functional requirements up to mismatching concepts with respect to staging and testing and the pitfalls of distributed systems and transactions. We will reflect on the difficult and painful solutions and discoveries we made and smarter ways of doing a legacy integration like this.\n\nUltimately, we are all building legacy systems of tomorrow, so let us share our thoughts about \"future-proof\" software, documentation and organizations. After all: \"The only real mistake is the one from which we learn nothing.\"",
"title": "Back to the future - how we integrated a 40 year old software system",
"id": "8beb375f-fc35-4461-8b97-3578276b9f88",
"sessionId": "8beb375f-fc35-4461-8b97-3578276b9f88",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Lukas Pradel",
"twitter": "@lukaspradel",
"bio": "Lukas Pradel is a Software Engineer at DB Fernverkehr AG. He likes to spend the time he saves by automating everything on riding his motorcycle."
}
]
},
{
"intendedAudience": "Anyone interested in knowing more about how computer chips are created and how Java is involved in the process.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "We all use computer chips such as processors, memory and sensors in our daily lives. But how are they created? How did the chip creation process evolve and what future changes can we expect?\n\nThis session explains how computer chips are physically created by some of the most advanced machines on the planet. Did you know that these chips, nowadays, can contain more than one hundred million transistors per square millimeter?\n\nJava software is used everywhere, also in the process of chip manufacturing. In my project at ASML we’re working on a relatively new analytics platform which is used to process the data from the machines. The application then visualizes the results in order to find issues or improvement areas. This information is used to change the configuration parameters of the physical machine in order to create more and better chips. I will explain, on a high level, how our applications look like and which Java technologies we use.\n",
"title": "How sand and Java are used to create the world’s most powerful chips",
"id": "57d1a518-26c7-4afc-abcf-a4fc9528d97b",
"sessionId": "57d1a518-26c7-4afc-abcf-a4fc9528d97b",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Johan Janssen",
"twitter": "@johanjanssen42",
"bio": "Johan is working as software architect at ASML. He has been working for various demanding companies where rapidly delivering high-quality software was very important. Johan regularly writes articles for InfoQ and Java magazines, mainly around Java. He presented more than 90 conference sessions in 24 countries at conferences such as JavaOne, GOTO, Devoxx, JavaZone, J-Fall, J-Spring, Jfokus and JavaLand. Johan received the JavaOne Rock Star, Oracle Code One Star and Oracle ACE Pro awards."
}
]
},
{
"intendedAudience": "Utviklere som vil samarbeide bedre med hverandre og andre som vil ha et annet argument for å skrive tester.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Et av de andre teammedlemmene er blitt syk, men oppgaven hen satt på har høy prioritet.\n\nDu sjekker ut branchen i git og prøver å forstå hva hen har tenkt for å løse problemet.\n\nHvordan skal du få oversikt over hva personen har gjort og hva som gjenstår?\n\nI løpet av denne lyntalen vil du se at å skrive tester tidlig i utviklingen kan hjelpe den neste personen som ser på koden til å forstå hvordan du har tenkt.",
"title": "Test tidlig for å utvikle sammen",
"id": "f3753e32-8c4f-4328-8928-d928404ac438",
"sessionId": "f3753e32-8c4f-4328-8928-d928404ac438",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Peter Stensby Norstein",
"twitter": "",
"bio": "Peter er medeier av Kantega AS i Oslo der han jobber som systemutvikler.\n\nHan er lidenskapelig opptatt av utvikleropplevelse, automasjon og bærekraftige samarbeidsformer."
}
]
},
{
"intendedAudience": "Developers and others interested in technology and the origins of the universe. \n\nBonus: If you´re like me and thinks that CERN is damned cool, this should be a good fit. ",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Handling thousands of petabytes of data produced annually by the Large Hadron Collider at CERN in the experiment ALICE, requires immense computing resources. To support the upgraded LHC detectors a new grid middleware, the Java ALICE Environment (JAliEn) was developed. Millions of CPU cores spread across over 60 computing centers worldwide that work together to provide the computational resources required to process data from physics  experiments. \n\nJoin me for a short sneak peak behind the curtain to see what goes on at CERN and being part of a research project of this scale.  I will talk about how I collaborated with experts at CERN to solve a database locking issue causing havoc in production.\n\nThe project is A Large Ion Collider Experiment (ALICE) which is focused on researching quark gluon plasma. A state of matter that existed shortly after the Big Bang. In other words, quark gluon plasma is the glue that appear to hold quarks and gluons together. ",
"title": "Optimizing job management on the LHC Computing Grid.",
"id": "9cb40bd0-aaa6-4599-9f36-62ba13b2c927",
"sessionId": "9cb40bd0-aaa6-4599-9f36-62ba13b2c927",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jørn-Are Flaten",
"twitter": "",
"bio": "My name is Jørn-Are Flaten and have been working with technology for the past 8 years and the last 5 as a developer. I´m writing a Master's thesis in Software Engineering on the side. I currently work at NAV IT and my Master's thesis is related to a CERN project through Høgskulen på Vestlandet(HVL) in Bergen. I think software development, the universe and space is super interesting and I´m thrilled to be able to work with this project and would like to talk about it. "
}
]
},
{
"intendedAudience": "Alle deltagere i et team og ledere av team.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Hva kjennetegner autonome team som lykkes? Hva er de beste smidige praksisene? Hvor stort skal et team være? Hvordan kan smidige team ta i bruk OKR? Hva skjer når du flytter en senior med stort nettverk fra et team til et annet? Hvordan bruker suksessfulle team Generativ AI?\n\nVi har de siste 10 årene gjort forskning på team hos virksomheter som Sparebank1, Entur, Oslo kommune, Spotify, Ericsson, NAV, Storebrand, Det Norske Veritas, Finn, DnB, m.fl. \n\nEn forsker og en praktiker presenterer de viktigste faktabaserte suksesskriteriene for å lykkes med smidige team og produktutvikling.",
"title": "10 resultater fra 10 år med forskning på team",
"id": "7b58c0a3-3a45-469b-9039-131b17b7c6bd",
"sessionId": "7b58c0a3-3a45-469b-9039-131b17b7c6bd",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jan Henrik Gundelsby",
"twitter": "janhenrik",
"bio": "Jan Henrik har over 25 års erfaring som utvikler, arkitekt og teknologileder i store prosjekter og programmer. Han har jobbet med store strategiske satsninger på digitalt førstevalg og digital transformasjon, og har vært sentral i å bygge DevOps-kultur, mikrotjenestearkitektur og produktorganisasjon hos store virksomheter. Jobber tett med forskningsmiljøer rundt autonome team og digitale plattformer. Jan Henrik var tidligere CTO i Knowit, og er nå leder for forskning og utvikling, der han jobber med å tette gapet mellom akademia og praksis - med fokus på digitalisering, storskala smidig og plattformtankesett. "
},
{
"name": "Nils Brede Moe",
"twitter": "nilsbm",
"bio": "Sjefforsker på SINTEF Digital med godt humør.  Han forsker på virtuelt arbeid, hjemmekontor, prosessforbedring, autonome team og global systemutvikling. Han jobber tett med mange globalt firma innen bransjene energi, telekom, transport og finans. I tillegg jobber han med flere konsulentfirma, programvarehus og offentlig sektor i Norge. Han har også en forskningsstilling ved Blekinge Institute of Technology i Sverige og er fast spaltist i e24."
}
]
},
{
"intendedAudience": "This talk will attract anyone who works in a cross-functional team. (Product owners, developers, designers, testers). Also, managers and team leaders will find the story inspirational. \n\nCross domain problem is very common in organisations which runs several cross-functional teams. However, very little improvement is done in this field. Understanding the Hosting Day will encourage the attendees to apply Hosting Day in their organisations.\n",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Cross-functional teams easily fall into an isolated room, especially in large scale organisations. The developers know little about business domain, people, and technology stack in other teams. At Fremtind, we have been running Hosting Day activities among front-end developers to target this common problem. Hosting Day is a half-day activity where front-end developers from two product teams get together and introduce their products to each other. Besides, they also solve their tasks with developers from other teams in pair programming sessions. \n\nThis talk will firstly identify the cross domain challenges of developers and it will also share the experiences of Hosting Day. I will also touch upon how we managed to spread this new practice across the organisation and made it become a part of Fremtind's organisational culture.\n",
"title": "How does Fremtind solve cross domain problem of its developers?",
"id": "34551b81-0fad-4090-b6d8-bdb507f50ff9",
"sessionId": "34551b81-0fad-4090-b6d8-bdb507f50ff9",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Sercan Leylek",
"twitter": "",
"bio": "Front-end developer at Fremtind. Delivers programming articles via his blog website and coding tutorial videos. Sercan Leylek has published several sci-fi and fantasy novels in Norway and Turkey."
}
]
},
{
"intendedAudience": "Alle som er interessert i webteknologi og IT-historie. Utviklere, teknologer, designere, UX.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Bli med på en reise tilbake i tid og opplev Norges aller første nettsted fra 1993. En tid da nyheter på nett, fildeling av musikk og videoklipp fra kommende Nintendo-spill framsto som revolusjonerende!\n\nMultiTorg ble publisert våren 1993. Nettstedet for lengst er borte, men Nasjonalbiblioteket har rekonstruert det, basert på en 30 år gammel kopi av webtjeneren. Gjennom metodisk og tverrfaglig arbeid har vi gjenopplivet en viktig del av norsk IT-historie og kulturhistorie. \n\nHva kan vi lære av MultiTorgs enkelhet og fokus på innhold? Og hvordan ble nøkkelpersonene i MultiTorg sentrale i den internasjonale utviklingen av webteknologi?\n\nVi tar deg med tilbake til webmediets fødsel, og lar deg oppleve en glemt bit av norsk og internasjonal internetthistorie.",
"title": "REBOOT: En reise tilbake til Norges første nettsted",
"id": "5f408bac-1233-4f93-9045-e16208547ed5",
"sessionId": "5f408bac-1233-4f93-9045-e16208547ed5",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jon Tønnessen",
"twitter": "@nettarkivet",
"bio": "Jon Tønnessen er historiker, med bakgrunn fra webdesign og medieproduksjon. I dag jobber han som webarkeolog ved Nasjonalbibliotekets nettarkiv, og er en ettertraktet formidler."
},
{
"name": "Thomas Langvann",
"twitter": "@nettarkivet",
"bio": "Thomas Langvann er produkteier i Nasjonalbibliotekets nettarkiv. Han har bakgrunn fra FAST, Telenor, ..."
}
]
},
{
"intendedAudience": "My talk is designed for Java developers, security professionals, and software architects. Attendees will benefit from understanding how Rust's safety features can enhance Java applications' security and performance. The presentation will be particularly valuable for those with experience in Java development and an interest in security and software architecture. Basic familiarity with Rust would be beneficial but is not required. This session aims to provide practical insights and methodologies for integrating Rust into Java environments, fostering a new perspective on secure software development.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "This presentation delves into the innovative integration of Rust's safety features within Java environments, aiming to enhance security and reliability in software development. It targets Java developers, security specialists, and software architects interested in exploring advanced security methodologies. The session will outline the process of incorporating Rust into Java projects, emphasizing the benefits of Rust's compile-time safety and zero-cost abstractions. We will discuss practical strategies for interlanguage communication, ensuring seamless operation between Java and Rust components. Case studies will be presented to demonstrate the effectiveness of this hybrid approach in real-world applications, highlighting significant improvements in security and performance. By the end of this talk, attendees will acquire valuable insights into leveraging Rust's robust safety mechanisms in Java applications, a strategy that not only boosts security but also fosters innovation in multi-language software development.",
"title": "Enhancing Java Security with Rust: A Hybrid Approach for Robust Software Development",
"id": "99bf5d46-d634-4c41-8ec2-4646ab8c84e2",
"sessionId": "99bf5d46-d634-4c41-8ec2-4646ab8c84e2",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Dr Mohammadreza Ashouri, PhD",
"twitter": "",
"bio": "Mohammadreza (Mo) Aahouri, PhD, is a computer scientist specializing in Java and JVM ecosystems, focusing on integrating Rust and LLVM technologies. His expertise extends to improving JVM security and functionality during his tenure at SAP, where he played a pivotal role in enhancing system security and performance through innovative Rust integrations. With a PhD in Software Security from the University of Potsdam, Mo combines academic rigor with practical experience, significantly contributing to JVM security enhancements and Rust-based solutions in enterprise environments. His work at SAP is notable for its impact on secure and efficient software development, leveraging his deep understanding of both Rust and JVM technologies."
}
]
},
{
"intendedAudience": "Developer, Architect, Platform Engineer",
"length": "240",
"format": "workshop",
"language": "en",
"abstract": "During this workshop we will teach how to create Roblox games integrated with a Java / Quarkus backend to manage teams, quiz, coding challenges and also cool integrations with IoT sensors, musical instruments using Fast Fourier Transform with Java / Processing.org and also camera with AI to detect objects in the real players.\n\nWe will explain the basics of Roblox development and show detailed architecture and challenges we have integrating Roblox with AWS Cloud. All the code is available to anyone that want to create your own game, demos for booths and amazing integration between virtual world and real world and vice-versa.\n\nAll the participant will create:\n\nA Roblox game with quiz and coding challenge A Quarkus backend to manage quiz / teams / other data in a noSQL database Deployment backend in AWS Lambda with AWS DynamoDB and Amazon Codewhisperer Optional: Test FFT integration with processing.org Optional: use camera with AI to recognize objects in real players (if the player is wearing a hat, then the avatar inside the game wear a hat) AWS Temporary Accounts will be provided - \"all you can eat\" accounts for 12 hours.",
"title": "Hands-on-lab: Creating Game with Roblox, Quarkus, IoT devices, musical instruments and AI",
"workshopPrerequisites": "The participants need to bring their own laptops then we'll provide the workshop environment on AWS. No requirements to create am AWS account at all.",
"id": "9a54b377-90ae-47bf-ae85-4339fbedc12d",
"sessionId": "9a54b377-90ae-47bf-ae85-4339fbedc12d",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Daniel Oh",
"twitter": "@danieloh30",
"bio": "Daniel Oh is Java Champion and Senior Principal Developer Advocate at Red Hat to evangelize developers for building cloud-native apps and serverless ob Kubernetes ecosystems. He's also contributing to various cloud open-source projects and ecosystems as a CNCF ambassador for accelerating DevOps adoption in enterprises. He's speaking at lots of technical seminars, workshops, and meetups to elaborate on new emerging technologies for enterprise developers & DevOps teams."
},
{
"name": "Kevin Azijn",
"twitter": "",
"bio": "Kevin Azijn is a Senior Solutions Architect and Head of Solutions Architecture in Public Sector Benelux at Amazon Web Services (AWS). He started as a Java Developer more than 15 years ago, rolled into mobile development, DevOps and managing teams before joining AWS in 2019."
}
]
},
{
"intendedAudience": "Everyone who is interested in data and how to treat facts, and bring some sanity into the predictive madness of AI!",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "A Knowledge Graph works as a unifying logical language for data, equally interpretable by both humans and machines, not depended on natural language. \nIn a world with huge amount of information, knowledge becomes valuable. This talk will put the spotlight on an immensely under communicated technology within the AI umbrella: Knowledge Graphs! \nIt holds properties and relations between these entities, including the meaning of what is. A knowledge graph holds semantics — the meaning of things— in your domain, and the facts and logic of your world. \nGraphs are all around us. They are all about reflecting reality in the data itself. It lets data flourish in its natural way of patterns and connectivity, and not forcing information into tables.\n\nLet us explore the wonders of Semantic Knowledge Graphs together!",
"title": "Knowledge Graphs for Dummies!",
"id": "27dae678-79ea-412f-af37-d7f40455db4b",
"sessionId": "27dae678-79ea-412f-af37-d7f40455db4b",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Veronika Heimsbakk",
"twitter": "veronikaheim",
"bio": "Veronika is a dedicated and enthusiastic outreach of the wonders of semantic knowledge graphs. And an advocate for bringing the knowledge of AI as more than just machine learning to the world. Recently awarded among Norway’s Top 50 Women in Tech. "
}
]
},
{
"intendedAudience": "Utviklere, produktledere, designere, leder utenfor produktteam, jurister",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "\"Compliance\" gjør ikke at folk hopper av entusiasme. \n\nSmidig produktutvikling handler om å løse problemer gjennom å bryte de ned i små deler og håndtere risiko underveis. Compliance går gjerne mer på siden, der det enten gjøres i forkant eller samles opp og gjøres \"etterpå\".\n\nMen compliance er viktig! For oss handler det om at vi kun bruker dine opplysninger til det vi skal. At det er tydelig hvordan vi kommet frem til et vilkår slik at det er mulig å ettergå det.\n\nKan vi få compliance til å gå mer hånd i hånd med det vi gjør i det daglige? Vi har ikke fasiten, men har lyst til å dele våre refleksjoner og erfaringer rundt det å ivareta compliance på en måte som gjør at vi kan levere *bedre* produkter til folk.",
"title": "Compliance og produktutvikling -- hånd i hånd!",
"id": "c956c7cc-639f-41fc-a05a-3e02c2a3e00c",
"sessionId": "c956c7cc-639f-41fc-a05a-3e02c2a3e00c",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Louis Dieffenthaler",
"twitter": "",
"bio": "Louis er produktleder for compliance-løsninger og dataplattformen i NAV. Hans motivasjon er å lage gode produkter til folk i Norge slik at folk får ytelsene og hjelpen de skal ha raskt og riktig.\n\nSpunnet ut fra utfordringer med å bruker målinger til å ta bedre valg i produktutvikling, har han lyst til å gjøre det lettere for team å forstå når det de gjør er \"godt nok\" -- og hva teamene kan gjøre for å komme seg innenfor igjen."
}
]
},
{
"intendedAudience": "Utviklere og ledere som vil endre interne prosesser, få ting gjort, og se innovasjonen spire. Ingen spesiell kompetanse kreves.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Hva skal til for å få bruk av KI inn i den hverdagslige produktutviklingen? Det er lett å sette opp slides med høy buzzwordfaktor, men verre er det å endre eksisterende prosesser og videreutvikle produkter som fungerer ved bruk av ny teknologi. Denne presentasjonen gir deg en praktisk innføring i hvordan du kan få til dette, fortalt både fra utviklerperspektiv og lederperspektiv. Hva er forutsetningene for innovasjon, på team- og applikasjonsnivå og på virksomhetsnivå? Mads og Vilde forteller fra hvert sitt perspektiv hvordan du kan være lur og få ting gjort.",
"title": "Innovation for Dummies - en praktisk guide til KI i produktutvikling",
"id": "80279b72-4f87-45c7-8d42-b613bdceee06",
"sessionId": "80279b72-4f87-45c7-8d42-b613bdceee06",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Mads Steenfeldt-Foss",
"twitter": "",
"bio": "Mads Steenfeldt-Foss is a developer turned product manager at Schibsted Marketplaces in the Real Estate vertical. With a background in full-stack development at Finn, Mads combines technical expertise with a passion for AI, prompting his way to success."
},
{
"name": "Vilde Opsal",
"twitter": "",
"bio": "Vilde Opsal is the Principal Engineer for Real Estate in Schibsted Nordic Marketplaces. She studied Symbolic Systems at Stanford University before developing applications for game developer Wooga and streaming platform SoundCloud in Berlin. The last few years, Vilde returned to her roots in two ways - relocating to her native Norway and diving back in to systems thinking through architecture and engineering leadership. She's passionate about storytelling, analogies, and learning new things."
}
]
},
{
"intendedAudience": "Utviklere (backend, frontend) i alle typer programmeringsspråk,team ledere,produkteiere",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Våren 2022 hadde vi problemer i SpareBank 1 Utvikling. Noen av våre mest sentrale APIer hadde kommet i en tilstand der vi ikke klarte å forvalte dem skikkelig. Høy teknisk gjeld. Høy test-gjeld. For mange kokker hadde laget mye søl. Vi trodde selv vi hadde “jobbet riktig”, men nå trengte vi hjelp. Da ringte vi Anders Sveen. Han fortalte om Test Double fra Martin Fowler, og en teknikk som heter Fakes. Dette skulle vise seg å totalt endre måten vi skrev kode på.\n\nDette er en historie om hvordan vi i SpareBank 1 Utvikling gjennom årene alltid har jaktet rask feedback-loop når vi koder, tester og prodsetter løsningene våre. Vi forteller hvordan vi mener bruk av Fakes i dag har bidratt til raskere feedback-loop, men også pushet et mer domenedrevet design med heksagonal arkitektur(ports and adapters). \nOg vi har fullstendig fjernet lange ende-til-ende tester. \n\nÅ levere flere ganger til produksjon om dagen har blitt en vane, og en avhengighet. For store endringer gjør oss nervøse, og vi vil ikke gå tilbake. Om man går noen år tilbake så kunne man ofte høre noe sånt som “det går ikke i en bank”, men i SB1 Utvikling har vi gjort det. Det er til og med mye bedre enn alternativet.\n\nAsgaut og Anders har diskutert i det uendelige om kontinuerlige leveranser, TDD og fakes. Dette er en oppsummering av det de har kommet frem til, og implementert i av noen av teamene i SpareBank 1 Utvikling.\n",
"title": "Bruk av Fakes for domenedrevet design og rask feedback-loop i SpareBank 1 Utvikling",
"id": "66a525c4-6e7b-4b62-a1fc-528ed3434196",
"sessionId": "66a525c4-6e7b-4b62-a1fc-528ed3434196",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Asgaut Mjølne",
"twitter": "",
"bio": "Utvikler i SpareBank 1 Utvikling DA. Backend-utvikler som elsker svensk mörkrost kaffi og koding sammen med andre. 18 års erfaring med utvikling og parprogrammering fra bla FINN, Bouvet, TietoEVRY, Telia, egen startup, SpareBank1 Utvikling."
},
{
"name": "Anders Sveen",
"twitter": "",
"bio": "Anders har jobbet med utvikling i 20 år. I løpet av disse årene har han hatt mange forskjellige roller, men alltid søkt mot utvikling og koding. Mange år som konsulent, men de siste 5 årene som lead developer hos start-upene Porterbuddy og ZTL Payments. Han er brennende opptatt av enkel og effektiv utvikling av systemer for å løse reelle forretningsproblemer. Kompleksiteten i tech stacken må ned. Dette har gjort at han opp igjennom årene har jobbet mye med DevOps, TDD og kontinuerlige leveranser. Han jobber nå som selvstendig teknisk coach med fokus på kontinuerlige leveranser.\n"
}
]
},
{
"intendedAudience": "Alle utviklere og teknologer er i målgruppen. Målet er å gjøre utviklere kjent med en teknologi som garantert kommer til å berøre dem om kort tid, både som utvikler og privatperson. Utviklere skal kunne gå fra presentasjonen med nok kunnskap til å bli nysgjerrig på teknologien og vite hvor de skal begynne for å ta den i bruk allerede nå.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Vi står på trappene av en ID-revolusjon i Europa. EU vil å gi alle borgere tilgang på en \"Digital lommebok\", som kan brukes til elektronisk identifisering, signering og lagring av verifiserte digitale dokumenter. Lommeboken bygger på en teknologi som kalles \"Desentralisert ID\", og i denne presentasjonen dykker vi inn i grøten for å se på hva som ligger bak teknologien, hva som får den til å tikke, og hvordan du kan ta den i bruk allerede nå! ",
"title": "Desentralisert ID - teknologien bak EU's neste supersatsing",
"id": "bd5e1c0a-4ff1-4e1a-8626-62c4f5cadc28",
"sessionId": "bd5e1c0a-4ff1-4e1a-8626-62c4f5cadc28",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Eirik Ølberg",
"twitter": "",
"bio": "Eirik jobber med Desentralisert ID og digitale lommebøker som en del av en større satsning i Kantega. Her er han en del av et tverrfaglig team som har som mål å være Norges fremste eksperter på denne teknologien. Som konsulent arbeider Eirik med Desentralisert ID i form av utvikling av løsninger, og rådgivning for strategi og posisjonering, og ikke minst undervisning og formidling for å senke terskelen for å komme i gang med pilotering av fremtidens digitale lommebøker allerede i dag."
}
]
},
{
"intendedAudience": "Workshoppen er midt i blinken for alle som jobber med å skape digitale tjenester og produkter. Uansett om du skriver kode, bruker mye post-its, leder folk, styrer pengesekken, pixel-pusher eller er en slags potet.",
"length": "120",
"format": "workshop",
"language": "no",
"abstract": "Har du noen gang tatt i bruk en skikkelig bra tjeneste, men så klarer du ikke helt sette fingeren på hva det er som gjør den bra? Eller brukt en utrolig dårlig tjeneste, hvor ting åpenbart ikke er gjort helt «etter boka»?\n\nVi bruker alle normer og konvensjoner for å utforme digitale løsninger, både bevisst og ubevisst. Ikke alle vet at det finnes en samling av disse, bedre kjent som UX Laws eller UX lover, som mannen Jon Yablonski har samlet i boka «Laws of UX». Tiltross for et formelt navn, er ikke disse UX lovene lovfestet, men heller en slags samling av etablerte og uoffisielle normer og konvensjoner.\n\nI denne workshoppen gir vi deg en innføring i UX lovene – før alt er lov! For her blir det konkurranse om å lage den dårligste brukeropplevelsen på kort tid, ved å være bevisst på hvilke UX lover som brytes. Vi lover en gøy workshop litt utenom det vanlige, med materiell du får med deg hjem og morsomme erfaringer i baggasjen. Mulig du også stikker av med en premie, hvem vet?",
"title": "Alt er lov! La oss bryte noen lover",
"workshopPrerequisites": "Ingen forkunnskaper eller forberedelser er nødvendig for delta på workshoppen.",
"id": "0ca39e20-0a26-4eda-8898-c8100c506b37",
"sessionId": "0ca39e20-0a26-4eda-8898-c8100c506b37",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Helene Storstrøm",
"twitter": "",
"bio": "Helene er designer hos Kantega i Oslo og jobber for tiden med Posten sin app. Hun er utdannet designer med både bachelorgrad og fagbrev, i tillegg har hun også et år med læringspsykologi i bagasjen. Dette gir henne et unikt perspektiv som designer. For med dette kan hun kombinere atferd, design og teknologi når hun jobber med løsninger for å finne ut hva som er best for brukeren. Etter jobb finner du henne ofte ute på treninger med hunden, som hun også konkurrerer aktivt med."
},
{
"name": "Elen Haksø",
"twitter": "",
"bio": "Elen er designer hos Kantega i Oslo, og jobber for tiden med bedriftsnettbanken til Sparebank 1. Hun har en master i interaksjonsdesign fra Norges teknisk-naturvitenskapelige universitet, og en bachelor i mediedesign fra Høyskolen i Volda og Griffith University. Hun har også studert UX Design hos Noroff. Etter jobb finner du henne ofte med kamera i hånda, eller på tur i skogen med hunden Ted og hesten Rosie."
}
]
},
{
"intendedAudience": "My talk is for Software Developers, Tech Leads, and Managers. Whether you're coding daily, leading a team, or overseeing projects, my insights on leveraging Error Prone will be helpful. \n\nError Prone offers the flexibility to create custom rules, proving invaluable in addressing various coding challenges. Whether it is updating deprecated APIs, handling common issues, refining code style, transitioning between libraries, or reducing technical debt, these rules can streamline the development process, saving time and manual effort.\n\nBesides demonstrating and explaining the tools, we’ll dive into how this tool can become a core part of the development workflow.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Are you tired of constantly fixing the same bugs and anti-patterns in your codebase? At Picnic, we've found a solution that not only resolves bugs once and for all, but also leads to a more consistent and high-quality codebase. Enter Error Prone: a tool that automates large-scale refactorings in your Java codebase. As a compiler plugin, it is capable of automatically suggesting and applying fixes at scale.\n\nFor years, Picnic has been using Error Prone to streamline our development process. In this talk, we will provide a comprehensive demonstration of Error Prone's capabilities, as well as offer practical guidance on how to set it up for your own team. Additionally, we will share our experiences and learnings, including creating and enabling our own set of custom rules. These are now open-sourced in Picnic's Error Prone Support repository.\n\nCome and learn how you can use Error Prone to streamline your development process as well!",
"title": "Say goodbye to bugs and anti-patterns with Error Prone",
"id": "35cc30a9-d68a-4c4c-948b-cd25e58636d9",
"sessionId": "35cc30a9-d68a-4c4c-948b-cd25e58636d9",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Rick Ossendrijver",
"twitter": "rick_ossies",
"bio": "Rick Ossendrijver is a Software Engineer at Picnic, a fast-growing online supermarket. He is part of the Java Platform team, which supports over 300 engineers within the company. Rick is a committer and enthusiast of the Error Prone project. Moreover, he is passionate about improving software quality through static analysis and automation, and works on Picnic's open-source Error Prone Support project."
}
]
},
{
"intendedAudience": "Folk som er nysgjerrige på utvikling av AR eller VR applikasjoner. \n\n\nVi vet at AI er framtiden, men har du tenkt på hvor den skal bo?\nI AR og VR headset så klart! Denne presentasjonen vil gjøre deg klar for framtiden.\n",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "\"AR og VR er fremtiden!\", spås det, men hvordan i alle dager lager man en AR eller VR applikasjon! \n\nI løpet av denne presentasjonen kommer du til å innse hvor enkelt AR/VR utvikling er, og at du kan begynne allerede i dag.\nJeg tar deg gjennom hvordan du kan lage AR- og VR-applikasjoner i spillmotoren Unity, og du vil lære alt du trenger for å komme i gang.\n\nDette er en gylden mulighet til å slenge deg med på en av de største teknologiske flodbølgende i nyere tid! \n\nBli med!\n",
"title": "Bli AR/VR utvikler på 45 min!",
"id": "a019ef7e-9a49-4562-a06f-ed63b0178436",
"sessionId": "a019ef7e-9a49-4562-a06f-ed63b0178436",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Kristoffer Håkon Håkonsen",
"twitter": "",
"bio": "Kristoffer ble ferdig utdannet i 2022 og har jobber som fulltack Utvikler i SpareBank 1 Utvikling siden. Han jobber med XR utvikling på fritiden og leder en faggruppe innen XR utvikling på fagdagene på torsdager. "
}
]
},
{
"intendedAudience": "This session is for all Java developers who are interested in learning about what it takes to make Java fast, really fast.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Your mission, should you decide to accept it, is the following: aggregate temperature values from a CSV file and group them by weather station name. There’s only one caveat: the file has one 1,000,000,000 rows!\n\nThis is the task of the \"One Billion Row Challenge\" which went viral within the Java community earlier this year. Come and join me for this talk where I’ll dive into some of the tricks employed by the fastest solutions for processing the challenge’s 13 GB input file within less than two seconds. Parallelization and efficient memory access, optimized parsing routines using SIMD and SWAR, as well as custom map implementations are just some of the topics which we are going to discuss.\n\nI will also share some of the personal experiences and learnings which I made while running this challenge for and with the community.\n",
"title": "1️⃣🐝🏎️ 1BRC–Nerd Sniping the Java Community",
"id": "3bf29139-5af9-4dee-b01d-228d053d2091",
"sessionId": "3bf29139-5af9-4dee-b01d-228d053d2091",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Gunnar Morling",
"twitter": "@gunnarmorling",
"bio": "Gunnar Morling is a software engineer and open-source enthusiast by heart, currently working at Decodable on real-time ETL based on Apache Flink. In his prior role as a software engineer at Red Hat, he led the Debezium project, a distributed platform for change data capture. He is a Java Champion and has founded multiple open source projects such as JfrUnit, kcctl, and MapStruct. Gunnar is an avid blogger (morling.dev) and has spoken at various conferences like QCon, Java One, and Devoxx. He lives in Hamburg, Germany."
}
]
},
{
"intendedAudience": "This talk is great for both beginners as well as more experienced engineers. I'll be talking about multiple things that are usually less discussed about vector search, however, I'll keep the topic simple and engaging, so that everyone can follow.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "It's fascinating how vector search boosted the usage of contextual search across numerous applications.\n\nThe underlying idea is quite straightforward. For example, let's take a movie recommendation system. The idea is first to represent each movie in our catalog as a vector, a numerical representation of a piece of text. Next we also convert the search phrase into a vector. Having done that we enter a whole new realm — a multidimensional space where these vectors replace the original text values. Now through some mathematical techniques, we can determine which movie representations are closest to each other and to our search phrase!\n\nBut how do we create such vector representations? We need an AI model trained on vast amounts of data to recognize patterns and effectively convert text phrases into vectors. We also need proper tools to run the model and do the inferences.\n\nAll of this and more you’ll learn in this session. We'll try out different data solutions - ClickHouse, OpenSearch, PGVector and others. We'll also explore different models that are available depending on your language preference and programming skills. Or, if you don’t want to run the model locally, what APIs you can use to do the inference for free.\n\nPlenty of demos and a bit of coding for each of the options. This session will be useful for anyone who is intrigued by contextual search and usage of AI, but might find themselves overwhelmed by the complexities to get started.",
"title": "Contextual search with vector search: exploring your options with open source tools",
"id": "134e0f01-d50f-4714-90a1-f3c5f403ff5c",
"sessionId": "134e0f01-d50f-4714-90a1-f3c5f403ff5c",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Olena Kutsenko",
"twitter": "https://twitter.com/OlenaKutsenko",
"bio": "Olena is passionate about data and its applications, sustainable software development, and teamwork. With a background in computer science, she's led teams and developed mission-critical applications at Nokia, HERE Technologies, and AWS. Currently, she works at Aiven where she supports developers and customers in using open-source data technologies such as Apache Kafka, ClickHouse, and OpenSearch. She is also a AWS Community Builder, a Confluent Catalyst, a volunteer teacher at non-profit tech school and an international public speaker, who regularly present at conferences around the world."
}
]
},
{
"intendedAudience": "Utviklere som ikke alltid klare å få til best practise selv om man har kompetanse og vilje. Her kan du få noen tips til hvordan du kan håndtere en ikke-optimal arbeidshverdag.  Kanskje kan du også få noen tips på veien for å gå fra worst practise til mediocre practise og dermed unngå et par katastrofer. ",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Det er enkelt å gjøre ting riktig. Du følger bare oppskriften for Best Practise så blir koden sikker, med 80% testdekning, skalerbar, driftssikker, har høy ytelse og future proof for alt av kommende endringer. Hvis du har det slik  på jobb så er dette ikke lyntalen for deg. Har du derimot litt utfordringer med å få gjort alt etter “boka” så kan det være greit å følge med. Vi skal se litt på forskjellige grunner til at ting kommer skjevt ut slik at du i hvert fall kan være forberedt og kanskje kan gjøre noe med det. ",
"title": "Er det så jævlig vanskelig å gjøre ting riktig?",
"id": "167b0d02-34e5-4fff-8a35-255c0c35b561",
"sessionId": "167b0d02-34e5-4fff-8a35-255c0c35b561",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Morten Udnæs",
"twitter": "",
"bio": "Morten jobber som TechLead i Fremtind Forsikring. Etter mer enn 30 år som utvikler lurer han fremdeles på om man blir klokere av erfaring eller bare skadet."
}
]
},
{
"intendedAudience": "Java developers",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Application and Infrastructure configuration is usually an easy place to make catastrophic mistakes.  The new Pkl configuration language brings a safer and more maintainable approach to defining and managing configuration.  It works with Kubernetes, Spring applications, and really anywhere that you have Yaml or other unstructured configuration.  This talk will introduce Pkl and teach you how to use it in your Java applications and infrastructure.",
"title": "Pkl: Safe and Maintainable Config for Java Apps and Infrastructure",
"id": "255b334b-29f6-49ef-9de9-667c32e60b38",
"sessionId": "255b334b-29f6-49ef-9de9-667c32e60b38",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "James Ward",
"twitter": "_JamesWard",
"bio": "Professional software developer since 1997, with much of that time spent helping developers build software that doesn't suck. A Typed Pure Functional Programming zealot who often compromises on his ideals to just get stuff done. Currently a Developer Advocate for AWS."
}
]
},
{
"intendedAudience": "Any Java developer with a faire experience on concurrent programming and reactive programming, aware of the benefits and problems this programming paradigm are bringing. \nThe main takeaway is a good understanding of the patterns that virtual threads and structured concurrency are bringing, and how they compare to reactive programming, including from the performance perspective. ",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Java 21 was released about a year ago, and among all the features that this version brings, Loom virtual threads are probably the most exciting and promising one. One of the promise was to enable the \"simple thread-per-request style to scale with near-optimal hardware utilization\", something that could only be achieved by reactive style programming. How can virtual threads achieve this kind of performance? Can virtual threads make the asynchronous programming model obsolete? Is this model going to disappear? These are the questions we cover in this presentation. Virtual threads are cheap to create, to a point where you can have as many as you need. It allows for a new API, Structured Concurrency, that brings a new asychronous programming model, simpler than the reactive programming model. The last element you need to create complete applications are Scoped Values, a replacement of Thread local variables, that we also cover.",
"title": "Are Virtual Threads Going to Make Reactive Programming Irrelevant?",
"id": "648e3ca3-0d4d-4f3a-9d98-174803d1689e",
"sessionId": "648e3ca3-0d4d-4f3a-9d98-174803d1689e",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "José Paumard",
"twitter": "@JosePaumard",
"bio": "José works as Java Developer Advocate at Oracle. PhD in applied maths and computer science, assistant professor at the University Sorbonne Paris Nord for 25 years, he is a Java Champion Alumni and JavaOne Rockstar. He is a member of the french Paris Java User Group, has been a co-organizer of the conference Devoxx France, and is a disorganizer of JChateau, an unconference held in the Chateau of the Loire Valley. He works on the dev.java documentation and community website, publishes the JEP Café, a monthly video cast on YouTube, and maintains a french YouTube channel with more than 80 hours of Java courses. He is also a Pluralsight author in the Java space.\nI selected 60mn because I like to have more time, including for questions, but this talk is fine in 45mn."
}
]
},
{
"intendedAudience": "Developers",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Verdikjede angrep ser ut til å ha kommet for å bli. For å ha en forhåpning om å komme seg helskinnet gjennom de neste rundene med verdikjedeangrep vil det være nyttig å vite hvilken programvare, og hvilke versjoner som inkluderes i din programvare. For å få og beholde kontroll på dette vil en S-BOM (software bill of materials) kunne hjelpe. I denne lyntalen skal vi se på hvordan du også enkelt kan generere og inkludere en S-BOM i din neste release og alle releaser etter ved bruk av både kjente og kjære verktøy (og noen utvidelser).",
"title": "Lag deg en S-Bom på 10 minutt",
"id": "ca583f36-6cc6-476b-9b2a-0b8e7bc1e3b4",
"sessionId": "ca583f36-6cc6-476b-9b2a-0b8e7bc1e3b4",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Kent Inge Fagerland Simonsen",
"twitter": "",
"bio": "Kent Inge har vært programvareutvikler i snart 20 år. I løpet av den tiden har han vært opptatt av sikkerheten (og usikkerheten) i datamaskinsystemer og ble smertelig klar over at det kunne være lurt å vite hvilke moduler man til enhvertid kjørte rundt samme tid som den såkallede log4shell sårbarheten herjet som værst. Etter det har han vært med på et internt prosjekt for å utforske mulige verktøy for S-BOM'er og S-BOM generering."
}
]
},
{
"intendedAudience": "Utviklere og alle andre som liker å programmere. Foredraget krever ingen forkunnskaper i spesifikke programmeringsspråk.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Hvert år i desember samles hundretusener av utviklere for å delta i Advent of Code (adventofcode.com), en kodeadventskalender som byr på en ny kodeoppgave hver dag frem til jul. I mitt foredrag vil jeg dele hvordan jeg tok utfordringen et skritt videre ved å løse disse oppgavene med et nytt programmeringsspråk hver dag. Fra de populære språkene som Java og Python, til de mer obskure som Brainfuck og LOLCODE, vil jeg gi et innblikk i hvert språks særegenheter, styrker og svakheter.\n\nMålet med foredraget er å gi et innblikk i programmeringsspråkenes mangfold og hvordan de kan berike vår forståelse av kode og problemløsning. Ved å dele mine erfaringer, håper jeg å inspirere andre til å utforske og lære seg nye programmeringsspråk.",
"title": "Advent of programming languages - en lynrask innføring i 25 programmeringsspråk",
"id": "3a11953f-8955-49ce-a9df-32be00f8ee26",
"sessionId": "3a11953f-8955-49ce-a9df-32be00f8ee26",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Hogne Jørgensen",
"twitter": "",
"bio": "Techlead i SpareBank 1 Utvikling DA. Elsker å lære nye ting. Har deltatt i Advent of Code siden starten i 2015 og har løst alle oppgavene de siste 4 årene."
}
]
},
{
"intendedAudience": "Alle",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Kontinuerlige leveranser har etablert seg som en forventning for å oppnå velfungerende, fremragende team. Praksisen påvirker både utviklingshastighet og utviklings- og produktkvalitet. Samtidig har kontinuerlige leveranser innvirkning på vår egen og teamets velvære. Hva skal til for å oppnå kontinuerlige leveranser, hva skjer når smerten for å deploye er høy og hvordan påvirker velvære arbeidet vårt?\n\nDenne talken ser nærmere på to capabilities fra forskningsprogrammet dora.dev; well-being og continous delivery. Vi går gjennom de begge, samt går inn på hvordan de påvirker hverandre. Vi ser på hva velvære er i kontekst av utvikling og hvordan det vi kan måle det. Med både personlige erfaringer og erfaringer fra NAV IT ser vi på hvordan velvære og kontinuerlige leveranser kan øke kvalitet og hastighet for deg og ditt team.",
"title": "Kontinuerlige leveveranser og velvære med DevOps-capabilities",
"id": "67548ebd-21cb-495f-a21a-5432e95757d4",
"sessionId": "67548ebd-21cb-495f-a21a-5432e95757d4",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Line Moseng",
"twitter": "@linemos",
"bio": "Line er utvikler og tech lead i NAV. Hun har holdt foredrag om både nasjonalt og internasjonalt om mangfold og inkludering i IT-bransjen og brenner for og jobber aktivt med å skape en mer inkluderende bransje for alle."
}
]
},
{
"intendedAudience": "Folk som lager viktige ting på internett",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Vi lager nettbank, og gjør alt vi kan for å gjøre den sikker. Likevel var vi overbevist om at vi hadde sårbarheter vi ikke hadde funnet ennå. Det stemte!\n\nLa oss snakke om hva SpareBank 1 lærte av å innføre finnerlønn for sårbarheter, og hvorfor du er sprø om du ikke gjør det samme. Hva gjør bug bounties unikt, og hvordan reddet det rumpa til foredragsholderen?",
"title": "Sett skuddpremie på sårbarhetene dine",
"id": "70e744ed-8eba-49ef-a650-98d54fd4e4b5",
"sessionId": "70e744ed-8eba-49ef-a650-98d54fd4e4b5",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jon Are Rakvåg",
"twitter": "",
"bio": "Sikkerhetsgartner i SpareBank 1 Utvikling. Driver SpareBank 1 Utviklings bug bounty-program. Finner som regel flere sårbarheter enn han forårsaker selv."
}
]
},
{
"intendedAudience": "Intermediate audience.\n\nWe will explore how to test distributed and monolithic systems effectively, easing the effort needed to write and maintain tests. ",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Testing in modern software development is, and has always been, a \nchallenge for the industry. Everyone claims to want to do more of it but often finds themselves falling short. Some find testing to be the most tedious part of their job.\n\nTesting has become more complicated in recent years due to the popularity of microservices and other distributed architectures. However, the way we tested it mostly stayed the same.\n\nThis talk will explore practical techniques, tools, and more to make testing easier and more approachable, to test more with less. Some of the things we will look at are TestContainers, contract testing and testable code and architectures.",
"title": "Modern testing - Test more and better with less friction",
"id": "d521c0f1-9e2a-40ff-b288-2c60df85c947",
"sessionId": "d521c0f1-9e2a-40ff-b288-2c60df85c947",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "John Mikael Lindbakk",
"twitter": "",
"bio": "John is a tech lead/architect with 10+ years of experience with a passion for developer processes. His experience lies in scalable distributed systems working within various of high-profile industries such as banking, healthcare and insurance.\n\nHe is currently a consultant at bspoke, where he also is the JVM subject manager (fagansvarlig)."
}
]
},
{
"intendedAudience": "Utviklere",
"length": "240",
"format": "workshop",
"language": "no",
"abstract": "Har du noen gang splittet en streng med data på en delimiter, for å så måtte splitte hvert element igjen \nmed en annen delimiter, da er denne workshoppen for deg.\n\nVi bruker parsere hver dag, men ofte er vi ikke klar over at vi gjør dette. Vi ønsker i denne workshoppen\nå sette lys på en teknikk som kalles Parser kombinatorer. \n\nVi kommer til å bruke Scala 3 med implementasjonen cats-parse. Vi vil også gi deg noen ressurser som du kan bruke\nfor å implementere dine egne parsere.\n\nDenne workshoppen kommer til å bestå av et foredrag, pluss oppgaveløsning.",
"title": "Parser Combinators - En parser som spiser parsere",
"workshopPrerequisites": "- Java 21\n- Intellij IDEA med Scala plugin eller Visual Studio Code med Metals plugin\n- [scala-cli](https://scala-cli.virtuslab.org/)\n- Ønske om å lære",
"id": "5ad28847-cdc1-483a-b045-388e56895f88",
"sessionId": "5ad28847-cdc1-483a-b045-388e56895f88",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Erlend Hamnaberg",
"twitter": "https://snabelen.no/@hamnis",
"bio": "Utvikler og konsultent hos Arktekk.\nHar 20 års erfaring som åpen kildekode utvikler. (http4s, http4s-netty, http4s-grpc, immutable-json, )\nHar jobbet med Scala siden 2011. Begynte å leke med det i 2009.\n\n\nhttps://snabelen.no/@hamnis"
},
{
"name": "Eirik Meland",
"twitter": "",
"bio": "Utvikler og konsultent hos Arktekk.\nAlltid på utkikk etter en anledning til å skrive en parser.\nGodt over gjennomsnittet interessert i tastaturer.\n\nHar jobbet med Scala siden 2013. \n"
}
]
},
{
"intendedAudience": "This talk is accessible to everybody, as it won't be too technical. We'll share some experiences from what we did, and provide you with some tips and tricks on how you can do the same in your organisation.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Sometimes deadlines are made up – and sometimes you don't really have a choice but to ship in time. This is a story of the latter.\n\nIn the fall of 2023, a motley crew of engineers, designers, editors and product owners got to rewrite two gigantic websites from an old web platform to a modern stack based on Sanity and Remix. This talk will dive into our mission, what challenges we faced, and how we launched both incrementally and successfully well within our deadline.",
"title": "Replatforming Giants: How we modernized two huge websites in 3 months",
"id": "6faffebe-5ed3-41d0-ac56-fc7ffb520fc8",
"sessionId": "6faffebe-5ed3-41d0-ac56-fc7ffb520fc8",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Sandra Lekve",
"twitter": "",
"bio": "Sandra is a fullstack developer, with one year of experience building neat products for clients."
},
{
"name": "Kristofer Giltvedt Selbekk",
"twitter": "selbekk",
"bio": "Kristofer is a seasoned speaker, fullstack developer and frontend enthusiast, with 11 years of experience building cool stuff."
}
]
},
{
"intendedAudience": "Most developers are struggling with interconnecting different applications or services during their local development. This talk should help them get some ideas on how to ease that pain. ",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "You’ve all been there, haven’t you? You are developing an application that needs to connect to something else in order to function properly, yet you can’t (or don’t want to) run that “something else” on your local developer laptop. What are some of the tools and technologies that could help in this situation?\n\nIn this talk, you will learn about some potential solutions you can use to help tackle some of these challenges by either running services locally in a more efficient way or by interacting with external dependencies/systems remotely.  \n\nYou should come away from this session with some new ideas on how to work with distributed applications using technologies such as Docker Compose, Podman Desktop, TestContainers, Quarkus Dev Services, Remote Development, Skupper, and Eclipse JKube.",
"title": "Taming Kubernetes: Streamlining Inner-loop Development for Distributed Systems",
"id": "5c9394e0-0309-412d-863f-106fc88949e3",
"sessionId": "5c9394e0-0309-412d-863f-106fc88949e3",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Kevin Dubois",
"twitter": "@kevindubois",
"bio": "Kevin is a Java Champion, software engineer, author and international speaker with a passion for open source, Java, and cloud native development & deployment practices. He currently works as developer advocate at Red Hat where he gets to enjoy working with open source projects and improving the developer experience. \nKevin is actively involved in open source communities, contributing to projects such as Quarkus, Knative, Apache Camel, and Podman (Desktop). He is also a member of the Belgian CNCF and the Belgian Java User Group.\n"
}
]
},
{
"intendedAudience": "Alle utviklere og alle andre som er interessert i å utvikle egne organisasjoner og seg selv",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "IT-industrien i Norge er i sterk endring, og kommer ikke til å være den samme om noen få år. Etter mange år med tydelig mangel på utviklere og utvikler-kompetanse, har videregående skolene begynt å utdanne lærlinger i IT-utviklingfaget. Dette vil påvirke industrien, ikke bare i forhold til fagmiljøer, innovasjon og kultur, men det vil også sette forventninger til hvordan organisasjoner bør endre seg.\n\nI lyntalen skal vi snakke om hvordan lærlinger endrer spillet i offentlig IT-utvikling. Bli med en lærling og hans mentor på en dykk lærlingenes verden i en stor og samfunnskritisk organisasjon. Vi vil avdekke hvordan lærlinger ikke bare lærer fra boken, men også bidrar med friske perspektiver til feltet, og hvordan deres mentorer formgir fremtidens IT-eksperter. \n\nEtter presentasjonen, vil du se verdien lærlinger bringer til organisasjoner, sett gjennom øynene på både mentor og lærling. Du vil også forstå bedre hvilke krav og forventninger som er nødvendig å ta vare på for å ta inn lærlinger i egen bedrift eller organisasjon.\n",
"title": "Perspektiver på IT-Utvikling lærlinger i offentlig sektor",
"id": "bcc32bc9-d544-4674-8a7c-3d3334c5716c",
"sessionId": "bcc32bc9-d544-4674-8a7c-3d3334c5716c",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Lucas William Bateson",
"twitter": "",
"bio": "Lucas is a 17 year old developer who has had a keen interest in programming and computers throughout his life. He has a special interest for AI, databases, OOP and DSL's. Lucas first started his programming journey in primary school, and his interest has grown exponentially from there. He is about to start his journey as a trainee to complete his education as a software developer."
},
{
"name": "Dervis Mansuroglu",
"twitter": "dervismn",
"bio": "Dervis er en utvikler og leder med 15 års erfaring fra IT-bransjen fra både privat og offentlig sektor. Han jobber i dag som leder for en gruppe utviklere i NAV IT. På fritiden er Dervis leder for JavaBin, den norske Java-brukerforeningen, og er engasjert i flere non-profit initiativer i det norske fagmiljøet. Dervis koder aktivt i fritiden, og er spesielt interessert i både funksjonelle og objektorienterte programmeringsspråk. Han er aktiv foredragsholder på norske og internasjonale konferanser."
}
]
},
{
"intendedAudience": "This talk has something for everyone.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Embark on a captivating journey through the vibrant tapestry of computer programming as a realm of boundless creativity and expression. From the LSD-fueled hacking subculture in the 1960s San Francisco Bay Area to the dynamic web gallery of today, discover how programming has evolved into a breathtaking art form.\n\nThis talk delves into the awe-inspiring stories of artists who, coming from fine arts backgrounds, stumbled upon the magic of computers in the 1970s. We'll unravel the mysteries of mathematical algorithms that have become the still lives and croquis nudes of computer art, adding a touch of beauty and emotional resonance to the digital canvas.\n\nRelive the rebellious 1980s, where amateur programmers ignited the demoscene movement, echoing the ethos of the street art. Explore the fusion of technology and rebellion, where code became a medium for expressing raw, unbridled creativity.\n\nIn the digital age, witness the internet transform into a global gallery, showcasing not only the final masterpieces but also the code itself as a form of art. Marvel at the unexpected instances where a computer printout of a well-known algorithm commands thousands at art auctions, blurring the lines between traditional and digital markets.\n\nWe'll unravel the artistic potential of mathematical algorithms, exploring how they breathe life into digital works. Moreover, we'll showcase instances where the code itself is the masterpiece, challenging conventional perceptions of art.\n\nBe prepared for a visually engaging experience supported by stunning imagery and beautiful code, aiming to inspire you to dive into the rich world of computerized art. Gain insights into the frameworks and tools that empower you to channel your creativity through code. Join us in embracing the fusion of technology and imagination, where the possibilities for artistic expression are as limitless as lines of code on a screen.\n\nMaybe you'll even have an answer to the everlasting question of \"What is art?\".",
"title": "The History of Computer Art",
"id": "6bccdf91-bb6a-4ed1-a78e-31603492a339",
"sessionId": "6bccdf91-bb6a-4ed1-a78e-31603492a339",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Anders Norås",
"twitter": "",
"bio": "Originally educated in arts and design, Anders has spent the last twenty years writing code.\n\nHe has given numerous talks and keynotes at conferences such as JavaZone, NDC, J-Fall, Øredev and many more. Have given 100+ conference talks to a variety of audiences including media, design and hardcore computer science. Known for an energetic and highly engaging presentations."
}
]
},
{
"intendedAudience": "Both beginners & seasoned professionals in data streaming and microservices development and operations.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Today, when you think about building event-driven and real-time applications, the words that come to you spontaneously are probably: RabbitMQ, ActiveMQ, or Kafka. These are the solutions that dominate this landscape. But have you ever heard of Apache Pulsar?\n\nAfter a brief presentation of the fundamental concepts of messaging, you'll discover the Apache Pulsar features that enable you to build amazing event-driven applications.\nYou'll learn the following:\n- how Apache Pulsar architecture differs from other brokers\n- how it enables scaling processing power & data independently, quickly, and with no hassle\n- how it guarantees high durability of messages across nodes and different data centers\n- how it covers the use cases of both RabbitMQ & Kafka while involving a single broker\n- how to integrate Pulsar with your existing application portfolio\n- and more",
"title": "Apache Pulsar: Finally an Alternative to Kafka?",
"id": "49a79606-e850-4e89-80ce-7b570c61b721",
"sessionId": "49a79606-e850-4e89-80ce-7b570c61b721",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Julien Jakubowski",
"twitter": "jak78",
"bio": "Julien Jakubowski is a Developer Advocate at StreamNative with over 20+ years of experience as a developer, staff engineer, and consultant. He has built several complex systems with distributed, scalable, and event-driven architecture for various industrial sectors such as retail, finance, and manufacturing.\n\nJulien delivers talks at conferences on software engineering, specializing in the Java community: Devoxx, VoxxedDays, Berlin Buzzwords, Java User Groups, and Google Developer Groups, among others.\n\nJulien is located in France. He's also one of the founders and leaders of the Ch'ti JUG - Java User Group of Lille, France."
}
]
},
{
"intendedAudience": "Anyone who is interested in ways to increase their productivity by making mundane project setup tasks more convenient. It will be interesting from developer to platform engineer of any experience. You could be a Maven novice or  a Maven Guru. No prior experience is required, thought some Maven knowledge can be useful.\n\nBy attending the participants achieve a new tool in their tool belt to automate these mundane tasks which will result in less error prone project creation. They will gain valuables insights in my experience of using these archetypes on my projects and what to look out for when trying it yourself.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "If you've ever utilized tools or websites to kickstart your application development, such as Spring Initializr, Micronaut Launch, Quarkus starter or even the classic copy-paste-rename-method. You're likely familiar with the convenience it offers in avoiding standard project setups. As microservices have gained traction, repetitive setup tasks are one of the many items on our lists as developers. But what if there was a way to elevate this process further? What if I told you, we can do better. Even without AI.\n\nThis talk dives into the workings of Maven Archetypes, highlighting their strengths and addressing common pitfalls. Learn how to leverage this plugin to streamline development workflows, adhere to architectural decisions, and boost productivity. \nWhether you're new to Maven or a seasoned developer, discover practical insights to optimize your development process and deliver high-quality software efficiently.",
"title": "Maven Marvels: Project Generation at Warp Speed",
"id": "3dc8c301-b05e-4c37-aa61-db0563c01404",
"sessionId": "3dc8c301-b05e-4c37-aa61-db0563c01404",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Giovanni van der Schelde",
"twitter": "https://twitter.com/gvdschelde",
"bio": "An adventurous developer and trainer that enjoys trying out new things. Sharing knowledge and learning from each other is one of the best ways to improve ourselves and others. Oh, and I love to do outdoor activities, good food and play all sort of games."
}
]
},
{
"intendedAudience": "Beginners",
"length": "120",
"format": "workshop",
"language": "en",
"abstract": "Distributed batch processing can be quite a challenge in Java. How do you manage task coordination and data consistency? What to do with the complexities of concurrency?\n\nThis workshop offers a practical exploration of JobRunr, an open-source Java library recommended by ThoughtWorks. It leverages your existing infrastructure and facilitates efficient job scheduling using only a Java 8 lambda. The workshop is designed to provide you with a thorough understanding of JobRunr's capabilities in a real-world application context.\n\nThroughout the session, you will learn how to integrate JobRunr into Java applications, emphasising its use for distributed batch job processing. The lab will cover the setup and configuration of JobRunr, demonstrating its adaptability across various Java frameworks.\n\nKey takeaways from this workshop include:\n- Fundamentals of JobRunr for distributed batch job scheduling in Java.\n- Practical steps for integrating JobRunr into Java projects, including Spring Boot and Quarkus.\n- Techniques for optimizing distributed job execution.\n- Approaches to manage and monitor background tasks in distributed systems.",
"title": "Distributed Batch Processing using only a Java lambda: a hands-on JobRunr workshop",
"workshopPrerequisites": "There are no prerequisites for this workshop except a laptop.",
"id": "58694ad7-0eeb-4faa-9a5a-20d8e1c35920",
"sessionId": "58694ad7-0eeb-4faa-9a5a-20d8e1c35920",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Ronald Dehuysser",
"twitter": "rdehuyss",
"bio": "Meet Ronald Dehuysser, a full-time open-source developer from Belgium and creator of JobRunr. Seamlessly integrating with Spring Boot, Micronaut, and Quarkus, JobRunr is Ronald's solution for efficient distributed background job processing.\n\nWhen he's not working on open-source software, he is probably either rock climbing or sipping a fine Belgian Duvel beer."
}
]
},
{
"intendedAudience": "No experience is required. I will show Python code for the application, which will be understandable for any developer. ",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Trend waves come and go, but should you ride them? Let me take you on a practical walkthrough of how we innovate with generative AI through validated learning.\nData-driven decision making is fundamental to the finance business. However, decision makers are often not proficient in the tools we use for data extraction, and rely on technical resources to run data extractions and report back on the results. Such tasks add a significant work load on technical resources who much rather would focus on more complex tasks, so what if we could automate it away using AI?\nIn this talk, I will walk you through our process of innovation with generative AI to create chatbots that aim to increase the efficiency of internal processes. You will meet \"Eglev\", our AI analyst designed to empower non-technical personnel in data extraction tasks to better inform decision making, while allowing our data scientist to focus their time on more interesting and complex tasks. Learn how we turned Eglev from idea to production with over 91% accuracy in only 6 weeks from both a practical coding perspective, as well as the method behind working as a lean startup in a large (and less lean) organisation.",
"title": "Pivot or Persevere: Should you ride that (AI) trend wave?",
"id": "46f60322-ecb0-413c-8919-4885d741abb2",
"sessionId": "46f60322-ecb0-413c-8919-4885d741abb2",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Alexandra Diem",
"twitter": "",
"bio": "I'm a former academic turned data scientist with a passion for data mesh architectures.\n\n🔬 Background in applied mathematics and statistics, adept at leveraging data-driven insights to solve complex problems. Experienced in diverse domains spanning the private and public sectors.\n\n🧠 Made significant contributions to research in physiological modeling, successfully debunking a leading biomedical hypothesis on Alzheimer's disease during my PhD. Developed innovative approaches to quantify blood supply to the heart.\n\n💡 Solution-oriented thinker with a track record of efficiently tackling challenging problems and adapting to novel scenarios.\n\n⚙️ Expertise: Team Leadership • Data Platform • MLOps • Data Mesh • Artificial Intelligence (AI)\n\nIn my spare time, you'll find me exploring the great outdoors—whether it's pedaling through scenic landscapes on a bike or riding down the slopes on a pair of skis."
}
]
},
{
"intendedAudience": "Alle kan få mye verdi fra denne talken",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Progress baren er en designkomponent som viser brukeren hvor langt maskinen har kommet med en gitt oppgave. Høres jo ganske enkelt ut eller? Men hvorfor klarer da ingen å lage en perfekt progress bar? Hvordan kan noe som ser så enkelt ut være så komplekst? \n\nI denne talken skal jeg presentere problemstillinger som omfatter progress baren som kanskje er DET mest krevende designkomponenten å designe og implementere. Nettopp fordi den krever ressurser fra både frontend og backend, men rører andre fagfelt som UX og psykologi. I tillegg, vil jeg komme med en egen erfaring som konsulent om hvordan jeg håndterte denne problemstillingen hos en kunde.",
"title": "Maskinen lyver til deg!",
"id": "d51aa204-144a-40a4-8b6d-a2b215649116",
"sessionId": "d51aa204-144a-40a4-8b6d-a2b215649116",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Uy Tran",
"twitter": "",
"bio": "Uy er en utvikler med fokus på frontend. Han har jobbet i bransjen i 6 år og har vært storfan av Javazone siden studiene og kommer derfor tilbake i år. Utenom frontend, har han jobbet med Java i backend og en del DevOps med konfigurering av pipelines og deploy rutiner. "
}
]
},
{
"intendedAudience": "Everyone",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Starting a new project, launching a new platform, releasing a new workflow. There is a plethora of documents out there suggesting best practices, considerations, patterns and possible solutions for how and when to start a project or process. Interestingly, however, there is not enough focus on retiring outdated and costly legacy applications in a way that meets business and compliance needs.\n\nConducting an assessment based on factors such as the value of the application to the business, the cost of retiring it, and the potential savings that can be achieved is the first step at the project level. What else can we do at application development and even more so at operations?\n\nIn this session we will discuss what challenges every project should consider when thinking about the entire software application lifecycle.",
"title": "Killing me softly..",
"id": "9ea5eb29-4592-4637-a36e-335c068543d2",
"sessionId": "9ea5eb29-4592-4637-a36e-335c068543d2",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Ixchel Ruiz",
"twitter": "@ixchelruiz",
"bio": "Ix-chel Ruiz has developed software application & tools since 2000. Her research interests include Java, dynamic languages, client-side technologies and testing. Java Champion, Oracle ACE pro, Testcontainers Community Champion, CDF Ambassador, Hackergarten enthusiast, Open Source advocate, public speaker and mentor."
}
]
},
{
"intendedAudience": "Java programmers, new or experienced with GenAI.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "A run-through of building, and operating in production, java-built Generative AI apps using the new Spring-AI framework.\n\nIt is super easy today to build AI Agents of all types, and you can do it in Java.  We'll start here and evolve the building blocks that lead to the concept an AI Capable Data Platform in your stack. \n\nMore…\nWriting GenAI apps is incredibly simple, often with only a few lines of code, introducing a whole new world of possibilities and commoditising many forms of data science, but presents serious challenges in the broader DevOps and capacity planning contexts. \n\nThe big picture for Systems Designs is now brand new, there are significant paradigm shifts from Product Ideation, to engineering and Operations. \n\nIdeation with GenAI requires fresh thinking.  Product Managers are typically overly excited with the possibilities, and you'll see this excitement at any of your local AI meetup groups.  The problem here is product folks are often thinking too grand, where the ROI wins are often way simple mundane ideas that were technically infeasible or costly before.\n\nProgramming with GenAI requires us to stop thinking in classical mathematical terms.  There is no right or wrong answers to the output, and many things no longer can be deterministic, or even probabilistically, correct.  Expectations on the correctness and quality of the output is a trap here. Building small ideas that involved closed-loop systems are a great place to start. Post-processing caching and validation of LLM results is another entry level requirement, in building out a production capable Evaluations framework.\n\nOperations in production is difficult. There are lessons from ML platforms to take. Mick can share production experience on Observability, CD/CI, Capacity Planning, Quality+Performance metrics, and A/B and Regression Testing.\n\nGenAI requires a fresh mindset: from product ideation, to programming and not being not stuck in classical mathematical thinking, to addressing all the production concerns in a brand new systems designs world.\n\nI hope you'll join me in sharing how to make GenAI java smooth and elegant.",
"title": "Production Generative AI apps – building an AI Brain in Spring-AI",
"id": "aa4b0909-1555-4afb-b5e2-a97a4b07f84e",
"sessionId": "aa4b0909-1555-4afb-b5e2-a97a4b07f84e",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Mick Semb Wever",
"twitter": "mck_sw",
"bio": "Mick is an Apache Cassandra committer and previous PMC Chair of multiple Apache projects.  An avid open source contributor where ever the itch scratches, recently writing the Spring-AI Vector store for Cassandra, DSE and Astra DB.\n\nApache Cassandra has proven itself to be the most relevant and performant Vector database of all thanks to the innovation gone into the JVector library.\n\nAs an engineer on DataStax database internals, Mick has worked closely with the largest companies around the world building their GenAI apps and building out their AI-capable data platforms."
}
]
},
{
"intendedAudience": "Anyone interested in telemetry and observability for cloud-native Java apps.\nThis session will very briefly cover logs and metrics (for any audience members who may not be overly familiar with these), but will primarily focus on distributed tracing. Attendees will gain insight into how these work, how Open Telemetry helps to provide a standard format for this and how they can make use of this within their own applications through MicroProfile Telemetry. There will be a live coding demo within this session where I will demo how to add distributed tracing to a cloud-native Java application using MicroProfile Telemetry through different implementations (i.e. automatic, manual and agent).\nAudience members can have a go with this coding demo afterwards through our OSS interactive Open Liberty guides, enabling audience members to reinforce this learning, see the source code and further investigate how they could add this to their own applications.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Our cloud-native environments are more complex than ever before! So how can we ensure that the applications we’re deploying to them are behaving as we intended them to? This is where effective observability is crucial. It enables us to monitor our applications in real-time and analyse and diagnose their behaviour in the cloud. However, until recently, we were lacking the standardization to ensure our observability solutions were applicable across different platforms and technologies. In this session, we’ll delve into what effective observability really means, exploring open source technologies and specifications, like OpenTelemetry, that can help us to achieve this while ensuring our applications remain flexible and portable. ",
"title": "Through the looking glass: Effective observability for cloud native applications",
"id": "ba84e481-b4ed-46ba-86df-e6d267d707fb",
"sessionId": "ba84e481-b4ed-46ba-86df-e6d267d707fb",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Grace Jansen",
"twitter": "@gracejansen27",
"bio": "Grace is a Java Champion and Developer Advocate at IBM, working with Open Liberty, MicroProfile and Cloud Technologies. She has been with IBM since graduating from Exeter University with a Degree in Biology. Grace enjoys bringing a varied perspective to her projects and using her knowledge of biological systems to simplify complex software patterns and architectures. As a developer advocate, Grace builds POC’s, demos and sample applications, and writes guides and tutorials. She is a regular presenter at international technology conferences and has authored a book on reactive systems. Grace also has a keen passion for encouraging more women into STEM and especially Technology careers."
}
]
},
{
"intendedAudience": "1. Java developers interested in GraalVM Native Image technology.\n2. Developers or DevOps engineers who are looking for ways to monitor non-JVM applications.\n3. Individuals interested in application performance monitoring and telemetry.\n4. Users of OpenTelemetry or those interested in learning about it.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "GraalVM Native Image is an excellent technology for reducing the startup time and memory footprint of Java applications, offering significant benefits to businesses. It is now becoming a popular choice for the creation of Java-based microservices. Though GraalVM Native Image applications are not executed on a standard JVM, several Java profilers, such as JDK Flight Recorders, assist users in diagnosing application issues as they arise. From an end-to-end monitoring perspective, however, how can we proceed? Fortunately, we have a powerful tool in our arsenal – OpenTelemetry!\n\nIn this session, I will demonstrate how to monitor GraalVM Native Image applications with OpenTelemetry. I'll also share valuable insights from real-world scenarios. Attendees will walk away from this session with the confidence to leverage OpenTelemetry effectively for monitoring GraalVM Native Image applications.",
"title": "End-to-end Monitoring of GraalVM Native Image applications with OpenTelemetry",
"id": "2f66e46b-3990-4e61-a8e5-41d062bb2532",
"sessionId": "2f66e46b-3990-4e61-a8e5-41d062bb2532",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Akihiro Nishikawa",
"twitter": "logico_jp",
"bio": "Working for Microsoft as a Cloud Solution Architect, based in Japan. Conference speaker at not only local conferences but also outside Japan. Expertise in application development with Java and application integration, including API management and EDI. Also, a board member of JJUG (Japan Java Users Group)."
}
]
},
{
"intendedAudience": "Teammedlemmer, teamledere, alle som er opptatt av godt samspill i team",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Lytting spiller en sentral rolle i å skape og opprettholde velfungerende utviklingsteam. Lytting er mye mer enn bare å høre, det handler om å forstå, respektere og verdsette andres perspektiv og synspunkter. Etter presentasjonen skal tilhøreren ha en forståelse av viktigheten av god lytting, og kjenne til verktøy for å ta det i bruk i eget utviklingsteam. \n\nVi kommer til å presentere lyttestigen og nivåene i den, et konkret verktøy for å fremme lytting i utviklingsteam. Vi vil også fortelle om konsekvensene som manglende lytting kan ha i et utviklingsteam. \n\nAvslutningsvis ønsker vi å utfordre publikum til å implementere lyttende praksiser i sine egne utviklingsteam.  ",
"title": "Lytting som nøkkel til effektive utviklingsteam",
"id": "38ff6c84-5b86-4ee9-88ba-5025fcc46b58",
"sessionId": "38ff6c84-5b86-4ee9-88ba-5025fcc46b58",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Hilde Nielsen",
"twitter": "",
"bio": "Hilde jobber som avdelingsleder i Kantega. Hun har lang fartstid både fra IT-prosjekter og med ledelse, og har klare ledelsesfilosofier og ledestjerner."
},
{
"name": "Stine-Aileen Strand",
"twitter": "",
"bio": "Stine-Aileen jobber som avdelingsleder i Kantega. Hun har lang fartstid både fra IT-prosjekter og med ledelse, og har klare ledelsesfilosofier og ledestjerner."
}
]
},
{
"intendedAudience": "Managers, team leads, developers, anyone who works in a product team that could benefit from our lessons learned the hard way. No experience is required.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Our development team was steadily delivering according to expectations, until unexpected external events forced several key resources to leave on short notice. Our team´s accumulated years of experience within our own portfolio would be reduced from 32 to 8 years in a matter of three months. News about this got to us in late May. Summer holidays were fast approaching. The upcoming seasons looked all but worry free. Why did it happen? How did we handle the challenges? Did we manage to keep the boat afloat? This talk contains food for thought and hopefully some good advice for teams that aim to be robust and sustainable in a fast-paced industry.",
"title": "A Nightmare on Efm Street: Building a Team in the Aftermath",
"id": "8677d392-69c3-4225-9958-57b2a65762f7",
"sessionId": "8677d392-69c3-4225-9958-57b2a65762f7",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Johannes Molland",
"twitter": "",
"bio": "Senior developer at Digitaliseringsdirektoratet, currently acting as team leader for the eFormidling team. Also a herder of cows and sheep, and a dedicated husband and father who loves spending time in the outdoors with his half-Japanese family."
},
{
"name": "Lars Bårdgard Åstveit",
"twitter": "",
"bio": "Lars is a senior developer at Digitaliseringsdirektoratet, still trying to learn what his job is actually about. Lars is a charismatic presenter, and has experience presenting at several conferences, including Sunndalskonferansen. Lars also wants to own cows."
}
]
},
{
"intendedAudience": "Any developer working in a staticly typed programming language should be able to benefit from this talk. The technique is not commonly known, but also not particularly hard to grasp.\n\nThe technique allows the developer to use the type system to enforce a contract without any runtime overhead.",
"length": "10",
"format": "lightning-talk",
"language": "en",
"abstract": "There are times where it would be helpful to restrict operations on types based on where an object comes from, or what the state of the object is. Examples include which database table an integer id came from, whether a Map of strings represents validated values or if a particular branch of your code has read or write access to a file.\n\nThe obvious solution is to create dedicated types for each state, but this requires a bit of boilerplate and extra runtime overhead.\n\nIn this talk we'll look at a technique to enforce the constraints we want, with a minimum amount of code and no runtime overhead.",
"title": "Phantom Types in Java",
"id": "a03c35b1-f5aa-4b07-b5ba-5857beae672b",
"sessionId": "a03c35b1-f5aa-4b07-b5ba-5857beae672b",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Robin Heggelund Hansen",
"twitter": "",
"bio": "Robin is a consultant at Kodemaker and the creator of the Gren programming language. He enjoys programming languages in general, and is always looking for a way to improve the way we develop software today."
}
]
},
{
"intendedAudience": "Utviklere og alle som liker et post-mortem",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "4. juni 1996 skjer verdens (hittil) dyreste programvarefeil. Etter mange suksessfulle oppskytninger med Ariane 4 raketten er ESA – Europas svar på NASA – endelig klar for å ta i bruk sin nye og moderne rakett, Ariane 5. Raketten har tatt mange år å utvikle, men nå er den endelig klar for sin første ferdsel. I 37 sekunder går reisen feilfritt, helt til én enkelt kodelinje skaper katastrofale problemer.\n\nForedraget gir en lett innføring i romraketter: dens historie fra tidlig fransk sci-fi, til tyske adelsmenn med behov for fart og spenning. \nVi ser på hvordan disse menneskene formet romkappløpet i det 20. århundre og hvordan romfarten igjen spilte en kritisk rolle i utformingen av moderne datamaskiner og programmeringsspråk.\n\nTil slutt tar vi for oss selve buggen og dens eksplosive konsekvenser på den heldigvis ubemannede raketten. Kan vi i 2024 lære noe av en bug som skjedde for 28 år siden?",
"title": "Verdens (hittil) dyreste programvarefeil",
"id": "b494492e-e0ac-4875-8b48-9676afe57d97",
"sessionId": "b494492e-e0ac-4875-8b48-9676afe57d97",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Håvard Opheim",
"twitter": "",
"bio": "Jeg er en utvikler med forkjærlighet for hvordan mennesker samhandler med digitale systemer. Jeg har jobbet som utvikler siden 2020, har livslang erfaring som nerd, og jobber for tiden hos Capra Consulting. De siste årene har jeg hengt meg veldig opp i hvorfor det tilsynelatende er så vanskelig å lage gode digitale systemer, selv hvor mye tid og penger vi måtte bruke på det. Sammen med gode venner produserer jeg podcasten \"EDB er dyrt og vanskelig\": en samling av hendelser gjennom tidene hvor IT og systemene rundt har sviktet oss, og hva vi kan lære av det. "
}
]
},
{
"intendedAudience": "Dette er en presentasjon for alle på teamet med og uten fotballinteresse. Vi kommer innom tema fra prosess og metodikk, ux og design samt litt av det tekniske som ligger under. Men uten å dykke dypere enn at alle kan henge med.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Hva skjer egentlig på innsiden av VAR-rommet? I år ble video-assistert dømming (VAR) innført i Eliteserien. Under generalprøvene to måneder før seriestart oppdaget Norges Fotballforbund at de trengte en løsning for å kommunisere mellom VAR-dommerne og medieproduksjonen.\n\nVi går gjennom prosjektet fra avspark 3. februar til seriestart i Eliteserien 10. april når systemet måtte være live. Akkurat som dommerne selv måtte vi ta viktige avgjørelser under press, og med begrenset tid til å revurdere valgene. Hvilke konsekvenser dette fikk for prosess og teknologivalg kommer til å være hovedfokuset vårt. Vi ser på hvilke (dommer)avgjørelser tatt i kampens hete som var gode og ikke, hvordan alt holdt på å rakne i overtiden og ekstraomgangene vi måtte gjennom for å bli klare til cupfinalefesten.",
"title": "Fra null til cupgull - VAR på 2 måneder",
"id": "77505b89-e5cc-46d0-abb0-f54223866aa1",
"sessionId": "77505b89-e5cc-46d0-abb0-f54223866aa1",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Brage Breivik",
"twitter": "",
"bio": "Brage er den på laget som aldri klarer å holde seg i posisjon men virrer over hele banen, men som likevel bidrar til et godt resultat. UX, frontend, backend - alt er gøy og må prøves, noen ganger med godt resultat"
},
{
"name": "Jørgen Langemyr",
"twitter": "",
"bio": "Jørgen trives best litt bak på banen der han kan drive med sine backend og database-ting, men når kampbildet krever det så bidrar han også fremover på banen."
}
]
},
{
"intendedAudience": "I would expect this session to be useful for those who have tried some things with GraalVM, or are considering using it in production, but have concerns or questions, more specifically around production-readiness and library availability -- this session will address those",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "In the past few years GraalVM got widely adopted by the Java ecosystem, and even more more since Native Image becoming officially supported by many frameworks. Now it's hard to imagine starting a new project, especially for the cloud, without at least considering going native. However, taking a new technology to production requires research and preparation. What is the best way to build and deploy such native executables? Once deployed, how can I monitor them? Can I test them as I do with regular Java applications? What if startup is less important, how do I optimize for peak performance and latency? How do I use 3rd-party libraries? In this session, we'll go through all those aspects and illustrate them with demos.",
"title": "Going AOT: Everything you need to know about GraalVM for Java applications",
"id": "dd3f9bec-9f81-4fbe-b19a-a74335d8950f",
"sessionId": "dd3f9bec-9f81-4fbe-b19a-a74335d8950f",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Alina Yurenko",
"twitter": "@alina_yurenko",
"bio": "Alina is a developer advocate for GraalVM at Oracle. Loves both programming & natural languages, compilers, and open source."
}
]
},
{
"intendedAudience": "Alle som er nysgjerrige på Staff+ engineer karrierespor",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Tenker du ofte at problemet du sliter med ikke lar seg løse av kode alene? \n\nSelv om det er mest gøy å kode, vil du heller løse rotårsaken til problemet. Og det er kanskje en dysfunksjon i teamet, i arkitekturen eller til og med i hele organisasjonen. Kanskje du egentlig vil bli teknologiprinsipal?\n\nMen hva jobber en teknologiprinispal egentlig med? Sånn helt konkret? Og hvordan blir man det, og hvem kan egentlig bli det? Det er det vi skal snakke om i dag. Vi vil snakke om spennet i rollen, fra koding i team til bygging av kompetansemiljøer og virksomhetsarkitektur.  \n\nVi har vært teknologiprinisipaler i NAV siden før NAV ble kult. I takt med at NAV har endret seg, har rollen vår også endret seg. Vi vil dele, åpent og ærlig hvordan vi har løst rollen da og nå. Vi vil også dele eksempler på teknologiprinsipaloppgaver fra andre andre steder, både nasjonalt og internasjonalt. \n\nBasert på vår egen erfaring og samtaler med andre med tilsvarende roller vil vi komme med våre aller beste tips for kunne ta en slik rolle. Hvilken teknisk erfaring bør du bygge og hvilke mellommenneskelige egenskaper bør du søke? ",
"title": "Hva er en teknologiprinsipal, og hvordan kan du bli en i tre enkle steg!",
"id": "79a91a12-e582-4a5a-8cd8-d5d73a5c8734",
"sessionId": "79a91a12-e582-4a5a-8cd8-d5d73a5c8734",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Truls Jørgensen",
"twitter": "trulsjor",
"bio": "Truls er principal engineer i NAV. Vil skrive endringsdyktig software, og må derfor også jobbe med å bygge en organisasjon som muliggjør dette."
},
{
"name": "Audun Fauchald Strand",
"twitter": "audunstrand",
"bio": "Audun er principal engineer i NAV. Dermed får han oppfylt drømmen sin om å få kode og bestemme samtidig. Han blir glad av utviklingsfart og tullete navn, og elsker å høre om når ting gikk galt."
}
]
},
{
"intendedAudience": "Anyone that enjoys something different. A little bit of experience in coding is probably good especially in C like languages like Java.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "I want to introduce you to the beauty of shader coding. Harness the Teraflops on the GPU to produce beautiful and complex patterns. Get some real-life usage of your old high school math. I show how to use the powerful shader IDE kodelife to do basic shapes using distance fields and then spicing them up with various transforms. I hope to incorporate some suggestions from the audience. Finally let's publish what we do on shadertoy for the world the marvel at.\n\nExample: https://www.shadertoy.com/view/DsXcWn",
"title": "Literally beautiful code",
"id": "af1d7f5f-dbf9-4db0-a220-101931a92fb7",
"sessionId": "af1d7f5f-dbf9-4db0-a220-101931a92fb7",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Mårten Rånge",
"twitter": "@range_marten",
"bio": "I want to introduce you to the beauty of shader coding. Harness the Teraflops on the GPU to produce beautiful and complex patterns. Get some real-life usage of your old high school math. I show how to use the powerful shader IDE kodelife to do basic shapes using distance fields and then spicing them up with various transforms. I hope to incorporate some suggestions from the audience. Finally let's publish what we do on shadertoy for the world the marvel at.\n\nExample: https://www.shadertoy.com/view/DsXcWn"
}
]
},
{
"intendedAudience": "Basically anyone can attend who has some experiences building web applications.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Spring is typically only used for JSON API backend development, while the web frontend is built with a JavaScript framework.\n\nhtmx enables us to create interactive web applications with server-side rendered templates without JavaScript.\n\nIn this talk you will get an introduction to htmx and server-side rendering with Spring.\n\nYou will learn to use patterns you know and love from building backends to create a full-stack application with reusable components.",
"title": "Building server-side web applications with htmx and Spring",
"id": "28528536-99c2-4bb9-a539-e225b62d6c56",
"sessionId": "28528536-99c2-4bb9-a539-e225b62d6c56",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Thomas Schühly",
"twitter": "tschuehly",
"bio": "My server-side rendering journey started as a developer trying to make life easier while developing my first product in my free time.\nCreating Spring ViewComponent enabled me to be the youngest Speaker at the largest European Spring conference while also enabling me to build awesome software full-time with my open-source library.\nI'm discovering new ways to build enterprise web applications on the Spring Framework for one of the largest Austrian infrastructure providers."
}
]
},
{
"intendedAudience": "Java developers, devops, security experts, ops teams, in short everyone who wants to learn about eBPF.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "eBPF is buzzing all over the cloud native world, as the cutting-edge technology reshaping the way we understand performance, security, and observability within kernel space. Java, with its recent strides in modernization and optimization, from enhancing startup times to facilitating native execution and advancing machine learning applications, stands at the cusp of this transformative era.\n\n\nJoin us in this journey, where we will embark on an ambitious challenge to write and build a high-throughput firewall leveraging the combined power of eBPF and Java. We'll start with a deep dive into eBPF's capabilities for kernel-level packet manipulation, then transition to how Java's latest advancements, particularly through Project Panama, enable seamless native code invocation and interoperability. Our focus will then converge to a hands-on demonstration of building a simple firewall using eBPF and Java, integrating kernel-level operations with high-level programming for real-time performance enhancements. \n\nAttendees will gain practical insights into deploying eBPF programs from Java using the hello-ebpf library, managing packet flows efficiently, and implementing firewall rules with precision, leveraging the strengths of both worlds.",
"title": "Building a Lightning Fast Firewall with Java & eBPF",
"id": "60571529-7dcf-4613-9441-361cdb50545e",
"sessionId": "60571529-7dcf-4613-9441-361cdb50545e",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Johannes Bechberger",
"twitter": "@parttimen3rd",
"bio": "Johannes Bechberger is a JVM developer working on profilers and their underlying technology in the SapMachine team at SAP. This includes improvements to async-profiler and its ecosystem, a website to view the different JFR event types, and improvements to the FirefoxProfiler, making it usable in the Java world. He started at SAP in 2022 after two years of research studies at the KIT in Java security analyses. His work today comprises many open-source contributions and his blog, where he regularly writes on in-depth profiling and debugging topics. He also works on hello-ebpf, the first eBPF library for Java.\nSince 2023, he's been touring Europe's Java User Groups and conferences, like JavaZone and Devoxx Belgium, to speak on various topics."
},
{
"name": " Mohammed Aboullaite",
"twitter": "@laytoun",
"bio": "Mohammed is a community catalyst, a true open-source believer who has contributed to many open-source projects. Mohammed has extensive hands-on, cross-industry experience in designing, building and evolving distributed applications at scale. He's one of Google developer experts in cloud, and work @Spotify as Sr Backend engineer."
}
]
},
{
"intendedAudience": "Mid+ engineers up to architects, inclusive?\nPeople making changes to organisations, scaling and reorganising should enjoy this.\nPeople who have fixed a number of broken organisation will say \"yeah\" or \"I know that already\".",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Forget about the spine-chilling tales of managing large-scale systems. It doesn't have to be a daunting task. We're here to advocate for battle's proven simplicity with a pinch of fun. We'll slice through the Gordian knot of complexity, juggle scalability patterns while uncovering their dark sides, and turn time modelling into a time-travel adventure.\n\nNo sale of silver bullets here; instead, we arm you with practical solutions and actionable strategies based on real-world examples and the dark sides of rapid-scaling problems.\n\nJoin us to transform your approach to evolving system architecture, leaving you with insights immediately applicable to your work.",
"title": "Architecture Uncomplicated: tools for simplified large scale systems",
"id": "98f88938-3109-47fd-ab70-88415d74ae11",
"sessionId": "98f88938-3109-47fd-ab70-88415d74ae11",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Andrzej Grzesik",
"twitter": "ags313",
"bio": "ags likes distributed systems in all shapes and form. Coding since the age of 8, loves simplicity and continuous delivery. While he has written in many languages, he favours the JVM. Since „most software problems are people problems”, he stirs communities, organizes and speaks at conferences (proud to be a JavaONE Rockstar!). He is passionate about all things data, because science! In his spare time… cycling, photography and books. And he is a Java Champion!"
},
{
"name": "Wojtek Ptak",
"twitter": "",
"bio": "Wojtek works as Head of Product Engineering and Engineering Executive at Revolut. Before, he worked as CTO for several companies, provided consulting, training, and assisted in building various data collecting, analytics, and applied ML/AI solutions, including Big Data implementations, data stream processing systems, and data insight projects. He worked with multiple Forbes 500 brands in the US, UK, and the Netherlands, including The Coca-Cola Company, the American Bankers Association, Macy's, Bloomingdales, Heineken, Saks 5th Avenue, BP, Boots, Polo Ralph Lauren, Porsche, HSBC, and others."
}
]
},
{
"intendedAudience": "This is for anyone associated with a team, either as a manager, developer, team lead, UX, test, dev-ops. It's also relevant for students or people who have just joined the IT industry. Both experienced people and inexperienced people will get value from this talk. ",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "We’ve all been the newcomer; we don’t know if something is vitally important or a minor detail. When is it OK to ask for help? When do you become “productive”? More knowledge is always better, right?\n\nBeing a newcomer is an asset! This talk is about what newcomers have to offer, and how we can work with our newest colleagues, and create a better working place at the same time. \n\nAre you experienced? Inexperinced? Are you a manager or a member of a team? This talk is for you. ",
"title": "Is your team complete without inexperience?",
"id": "237e03e8-c55b-4ee8-9c83-ddedeb139dfd",
"sessionId": "237e03e8-c55b-4ee8-9c83-ddedeb139dfd",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Siv Midtun Hollup",
"twitter": "",
"bio": "Siv loves creating software, preferably together with other people. She works both as a developer, mentor and team lead, and has been teaching and mentoring for over 20 years. Her passion for teaching and outreach led her to help found the Nerdschool meetup, help create technical onboarding courses and teach fundamental software engineering skills at the University of Bergen. She is actively engaged in teaching newcomers to our field about craftsmanship of developing software and also spends time encouraging girls to choose tech and IT as a career path. "
}
]
},
{
"intendedAudience": "Medium/Highly experienced Java devs, anyone who works with Java (or with Kotlin to some extent). The most value would be for people struggling with writing some boilerplate code or who want to do some compile time checks that other libraries don't provide.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "I've crafted an annotation processor at work, improving our efficiency. It tackled our issue by automatically generating boilerplate code, a task no other tool could handle in that case. Dealing with boilerplate is a common challenge that many could encounter. You may be facing it now.\n\nI enjoyed writing the annotation processor and consider it a very educational experience. It was the time I had found interest in the annotation processing.\n\nDespite annotation processing being a feature of Java since 1.6, I've observed that it remains somewhat esoteric to the Java community and has only recently started gaining attention. For example, Micronaut and Quarkus, new players in the framework market, utilize it extensively, from Dependency Injection to Aspect-Oriented Programming.\n\nGiven this, I'm eager to introduce it to you so you can incorporate it into your toolbox.",
"title": "Annotation Processor: Solved my problem and can solve yours!",
"id": "8a274848-6f83-472d-897e-984664172bc5",
"sessionId": "8a274848-6f83-472d-897e-984664172bc5",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Jacek Dubikowski",
"twitter": "",
"bio": "Senior Software Engineer at Virtuslab with 7+ years of experience in Java and Kotlin. Interested in a vast number of IT-related topics. However, he would love to learn Haskell and Rust. Becoming a team leader is his dream.\n\nTo learn more about annotation processing, he created an educational framework that supports essential DI, @Transactional and creating controllers by using an annotation processor, which can be found on GitHub in the build-your-own-framework public repository with readme describing everything step by step."
}
]
},
{
"intendedAudience": "Alle som har en interesse av å forstå utfordringene som ligger i å bruke virksomhetens data på tvers og vanskeligheten av å kjøre maskinlærte modeller i produksjon for å skape reell verdi for virksomheten. Alt fra ledere som ønsker å bedre forstå til system utviklere og data scientists som ønsker å høre mer om hvordan disse to verdenen nå krasjer sammen på godt og vondt.",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Er det galskap å se for seg en fremtid hvor systemutviklere støtter data scientists i produksjonen av dataprodukter? Dataprodukter som kan benyttes i risikovurderinger understøttet av alt fra enkle til komplekse maskinlærte modeller. Et paradigmeskifte for å smelte to miljøer sammen for endelig å kunne smidig og effektivt levere og forvalte KI i produksjon. Vi ønsker å fortelle om en slik reise og hva vi har lært underveis, samt hvor vi står og hvor vanskelig det er og har vært. Er vi på riktig vei - eller er det vi holder på med galskap?",
"title": "Hvordan en av Skatteetatens tilnærminger til dataprodukter og maskinlæring i produksjon enten er genial eller galskap!",
"id": "b009c654-539a-4378-97b0-a13d95fd5754",
"sessionId": "b009c654-539a-4378-97b0-a13d95fd5754",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Sigve Albretsen",
"twitter": "albretsen",
"bio": "Sigve Albretsen is a digital transformation catalyst at Diforma with a Master of Technology and Computer Science from Norwegian University of Science and Technology (NTNU) and an Executive Master of Technology Management (MTM) from \nMassachusetts Institute of Technology - MIT Sloan School of Management, Norwegian School of Economics (NHH) and Norwegian University of Science and Technology (NTNU)"
},
{
"name": "Per Harald Barkost",
"twitter": "",
"bio": "Per Harald Barkost is a data scientist at Skatteetaten with a Master of Science in Physics from UiT - The Arctic University of Norway."
}
]
},
{
"intendedAudience": "This talk is relevant for developers of all backgrounds, as our primary focus is on architectural principles rather than specific programming languages and tools.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "On a beautiful summer day in 2020, Entur’s entire sales system went down, leaving all of Norway unable to purchase train tickets. But did we let the Norwegian people travel for free on that fateful day? Perhaps a few, but not all! Those who managed to board a train still had to pay for tickets via the app used by train conductors.\n\nSo how do you go about developing a robust application which can withstand downtime in the backend systems, and act as a backup in case of critical failures? How do you build something that survives without internet in 2024?\n\nA potential solution to this challenge is offline-first architecture, a design principle which assumes the lack of internet connectivity as the default state in software development.\n\nDuring this presentation, we’ll share our insights from building a complex ticketing application based on offline-first architecture, discuss the challenges encountered along the way, and highlight some of the solutions we’ve discovered.",
"title": "Offline-first Architecture - How to survive without internet?",
"id": "b302ad74-9495-4313-b7c5-528c7fc369bc",
"sessionId": "b302ad74-9495-4313-b7c5-528c7fc369bc",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Ahlam Aatif",
"twitter": "",
"bio": "Ahlam is a software developer currently working for team ombord at Entur. Other than making sure that people are fined when they're caught without a valid ticket (I'm terribly sorry), she enjoys bouldering during her freetime (even though she's kinda scared of heights).  "
},
{
"name": "Zi Liu",
"twitter": "",
"bio": "Zi worked as a software developer and team lead for Team Ombord at Entur, and has as a result worked extensively with building an offline-first application for the Norwegian public transportation system. Team Ombord is part of the reason you have to (sometimes) pay fines when you forget to buy tickets on public transportation...even when there is no internet. Even when the backend systems are down. "
}
]
},
{
"intendedAudience": "Denne presentasjonen krever ingen forkunnskaper og er ment for utviklere og andre som er interessert i kunstig intelligens, backendsystemer, automasjon og kodebasen i et stort norsk selskap. Presentasjonen vil fokusere på hvordan man kan lage \"llm-agenter\" som utfører oppgaver et menneske vanligvis ville gjort. ",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Kan man integrere en Open Source Large Language Model med systemmeldinger fra en IBMi Power maskin ved hjelp av AI-agenter som forstår feilmeldinger, går inn i relevant program, foreslår en løsning i kode og varsler riktig utvikler på Teams med denne informasjonen?\n\nHer kan du få se og lære om hvordan vi i Fremtind jobber med å integrerer kunstig intelligens i våre backendsystemer for å hjelpe oss med å utføre kjernekritiske oppgaver, raskere. Systemet til Fremtind håndterer en årlig omsetning på NOK 20 mrd. ",
"title": "Hvordan LLM-agenter løser kjernekritiske oppgaver hos Fremtind",
"id": "c7c9f32d-d79f-4e32-8b4d-0ce17bbcf984",
"sessionId": "c7c9f32d-d79f-4e32-8b4d-0ce17bbcf984",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Torstein Frogner Knutson",
"twitter": "",
"bio": "Torstein Frogner Knutson er en selvlært utvikler og jobber i Fremtind med kjernesystemet. Før han ble utvikler startet og drev han Påfuglen - Norges største kunstnerkollektiv på Grünerløkka. Mulighetene kunstig intelligens tilbyr er i hans øyne grenseløse og har mange lavthengende frukter. Bli med på denne spennende reisen i hvordan man integrerer en lokal AI med et stort backendsystem - uten forkunnskaper.  "
}
]
},
{
"intendedAudience": "Developers who want less manual hassle when deploying to production.",
"length": "10",
"format": "lightning-talk",
"language": "en",
"abstract": "When deploying your application, you may run into manual hassle for compliance reasons. Many organisations use Jira or similar systems to record workflows and changes made to production. That doesn't need to slow you down! Learn how BankID sped up deployments and reduced manual tasks by delegating the job of creating Jira issues and pushing them through the workflow to our CI/CD setup with GitHub Actions. Integrating with Slack as well kept everybody in the loop. While this use case focuses on Jira, Slack, and GHA, our approach is applicable to many other setups.",
"title": "Cutting Red Tape with GitHub Actions",
"id": "b8797000-6f68-4485-bed6-d3fae80d4993",
"sessionId": "b8797000-6f68-4485-bed6-d3fae80d4993",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Markus Krüger",
"twitter": "markusbk",
"bio": "Markus Krüger is a developer at BankID BankAxept AS. He has worked as a developer, architect, and tech lead for over 20 years, and has been running systems on various cloud platforms since 2011. He has held several presentations at JavaZone and other venues previously, on various subjects such as performance testing, scheduling, scaling systems, and cloud architectures. He likes making large things go fast."
}
]
},
{
"intendedAudience": "Dette foredraget er rettet mot av utviklere og teknologi-entusiaster som ønsker å utforske eller er interessert i webanimasjon og hvordan vi har laget Origo sin Oslo-loader med SMIL ✨. https://punkt.oslo.kommune.no/latest/komponenter/loader/",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Bli med å utforsk hva du kan gjøre med SVG sin native animasjonsteknologi, SMIL! 😄🌸\nTruet ikke Google med å deprecate SMIL i 2016 - hvordan lever SMIL enda? Hvorfor skal man gidde å bruke SMIL fremfor andre alternativer? 🕵🏻‍♀️\n\nI dette foredraget skal gi den en introdiksjon til SMIL animering av SVG elementer, når det er hensiktsmessig å bruke, hvordan SMIL kan brukes - og hvordan vi har brukt det til å lage en gøyal loader i Origo sitt designsystem 🌊",
"title": "SVG og SMIL-animasjon 🦋✨",
"id": "e5be9db5-81c4-47b0-b5b0-b2dd798a71fb",
"sessionId": "e5be9db5-81c4-47b0-b5b0-b2dd798a71fb",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "My Thao Nguyen",
"twitter": "ItWasntMy",
"bio": "Til det daglige, jobber My som utvikler i Oslo Origo,og mer spesifikt med designsystemet Punkt 💙 \nFormelt sett har My har en informatikk utdannelse fra NTNU, og har en 5,5 år erfaring som konsulent (Bekk og Dfind) før hun til slutt har slått seg til ro i Oslo Origo 🏡.  My brenner for å brukerorienterte og tilgjengelige løsninger, og er spesielt glad i samarbeid med designere og visuell grafisk moro 🤪. \nEllers er hun glad i karaoke, gaming, og kattene sine - Nala og Luffy 🐈"
}
]
},
{
"intendedAudience": "Utviklar",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Rotårsaka til nedetid er ca 8 av 10 gonger ein ekstern avhengigheit utanfor vår kontroll som utviklar. Dette kan vera nettverk, database, cache eller api.\n\nKorleis som utviklar enkelt og fort lage metrics frå exception som fortel om kvar rotårsaka er. Korleis utnytte exceptions godt. Definere kategoriar av feil som gir overblikk (database, remote cache, eksternt api, internt api) over heile løysinga di eller visualiseringer som gir innblikk i korleis kvar enkelt applikasjon har det mot omverda.\n\nDømer frå verkelegheita utan og med desse metricane ved nedtid i ID-porten. ID-porten er i dag bygd opp av rundt 15-20 små applikasjonar med eit par større unntak.\n\nHands-on med visning av java-kode og grafana visualiseringer.\n\nTeknologi: Spring Boot,  Micrometer, Prometheus, Grafana og muligens Alerts i Kubernetes.\n\nBonus: Helsesjekk som metrics i Spring Boot?",
"title": "ID-porten gjekk ned! Kvar er feilen!?",
"id": "009acff4-011c-4898-8150-31ee1835ec23",
"sessionId": "009acff4-011c-4898-8150-31ee1835ec23",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Randi Øyri",
"twitter": "",
"bio": "Randi jobbar med ID-porten i Digdir og har vore utviklar i snart fleire ti-år. Har også jobba mykje i grensesnittet mot ops og gledar seg over at me endeleg nærmar oss ei verd der ein enkelt kan observere og få feedback frå produksjon utan mykje jobb."
}
]
},
{
"intendedAudience": "Arkitekter og utviklere ",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "En elefant av gangen, bit for bit. Hadde det bare vært så enkelt… \nCargoNet er et togselskap og et terminalselskap og har i over 20 år basert driften på et felles, egenutviklet IT-system som nå må erstattes. CargoNet ønsker å innføre et nytt hyllevare-system for booking. I tillegg innfører BaneNor et nytt IT-system på alle godsterminalene. I praksis betyr dette at tilnærmet hele systemporteføljen skal byttes ut omtrent samtidig. \n\nDet blir konkrete eksempler og hvilke erfaringer vi har med et konkret pattern eller tilnærming. \n\nUnngå big bang - strangler pattern og alternativer \n\nUbiquitous language i kode og i hodet \n\nPatterns for endringsdyktighet (systemarkitektur og i kode)",
"title": "Hvordan spise tre elefanter på en gang?",
"id": "94c884b4-ddaa-49aa-abfd-f1990a266b91",
"sessionId": "94c884b4-ddaa-49aa-abfd-f1990a266b91",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Erik Drolshammer ",
"twitter": "",
"bio": "Teknisk arkitekt \n17 års erfaring\nJobbet for Vy de siste 5 årene "
}
]
},
{
"intendedAudience": "Dette er relevant for utviklere som arbeider med mikrotjenester. Både erfarne utviklere som kanskje allerede bruker fellesbiblioteker, og de som er nye innen dette konseptet og ønsker å lære mer før de eventuelt vurdere å implementere det i sitt eget arbeid. Ingen spesielle forhåndskunnskaper er nødvendig.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Bruken av mikrotjeneste-arkitektur har skapt en debatt rundt bruken av fellesbiblioteker. Er det egentlig effektivt, eller bare en annen form for kodeduplikasjon? Gjennom eksempler og personlige refleksjoner, vil jeg i denne presentasjonen vise hvordan velutformede fellesbiblioteker kan være et verktøy for effektiv utvikling. Fra å håndtere konsistens til å fremme samarbeid, vil jeg gjennomgå best practices og mulige fallgruver. Kom og hør hvordan riktig bruk av fellesbiblioteker kan være med å effektivisere deres utviklingsteam og legge til rette for fremtidig suksess.",
"title": "Fellesbiblioteker og mikrotjenester, effektivitet eller kodeduplikasjon?",
"id": "8b8824df-7ea7-4b08-87fa-fa388609bf52",
"sessionId": "8b8824df-7ea7-4b08-87fa-fa388609bf52",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Magne Kjellesvik",
"twitter": "",
"bio": "Magne, utvikler i Oslo. Jobber for tiden med en portefølje som består av mange mikrotjenester. "
}
]
},
{
"intendedAudience": "Developers who are curious about passwordless authentication or authentication in general. No prerequisite knowledge required.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Learn how we at BankID implemented Passkeys in order to offer fast, secure, phishing-resistant passwordless authentication, directly from the browser using WebAuthn over platform-native, hardware-backed FIDO2 authenticators, without having to handle or store biometric data. The talk also includes the necessary technical know-how needed to implement your own passwordless authentication solution using Passkeys and WebAuthn.",
"title": "Passwordless BankID with Passkeys and WebAuthn",
"id": "eadedaea-a72f-438f-94cd-de438b4fb295",
"sessionId": "eadedaea-a72f-438f-94cd-de438b4fb295",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Tarald Riise",
"twitter": "",
"bio": "Tarald is a dedicated full stack developer who's been working for BankID with FIDO2 and Passkeys since 2021. With a background in cryptography and secure development, he's passionate about offering the next level of authentication as widely as possible."
}
]
},
{
"intendedAudience": "I think people who have a fair amount of experience working in the data ecosystem will benefit the most, since this talk gives them a perspective on the work they're giving this part of their lives to. However, any intellectually curious developer should enjoy this talk.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Data professionals and software developers of every stripe are moving rapidly into the world of real time, streaming data. There's no question that this is our future, and that its promises are already being realized, but how did we get here? Is real-time really the natural evolution of batch processing? When did we start thinking of what we call \"data\" itself, and has has that influenced the way we build systems how have our data management tools evolved from the earliest days?\n\nWe'll start with a frustrated Belgian astronomer who went on a quest to compute the statistics of the \"average man.\" Then we'll follow automated computing technology as it proceeds from the Difference Engine to tabulating machines to the first digital computers to mainframes to data warehouses to Hadoop to Kafka to Apache Pinot. As we tell the stories of the main technologies along the way, we'll look at how developments in data storage, information management, and computation all conspired to give us not just just the data stack we have now, but the very ideas we use to think about data, put it to use, and move its associated technologies forward.",
"title": "A Brief History of Data",
"id": "3597c820-90d7-453c-aa41-0942046d89a8",
"sessionId": "3597c820-90d7-453c-aa41-0942046d89a8",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Tim Berglund",
"twitter": "tlberglund",
"bio": "Tim is a teacher, author, and technology leader with StarTree, where he serves as the VP of Developer Relations, working with his team to make Apache Pinot and real-time analytics accessible to all developers. He is a regular speaker at conferences and a presence on YouTube explaining complex technology topics in an accessible way. He tweets as @tlberglund, and lives with his wife and stepdaughter in Mountain View, CA, USA. He has three grown children and four grandchildren."
}
]
},
{
"intendedAudience": "Utviklere som ønsker å komme igang med å bruke enkle Maskinlæringsfunksjoner.\nMange kodeksempler i workshoppen, så deltakere bør kunne forstå enkle programmer i forskjellige programmeringsspråk.\n",
"length": "240",
"format": "workshop",
"language": "no",
"abstract": "Denne presentasjonen er for de som kjører elektrisk, vurderer å kjøre elektrisk, eller er på jakt etter argumenter for å ikke kjøpe en elektrisk bil.\nIkke interesert i elektriske biler ? men hvis du er interessert i å komme igang med maskinlæring på en enkel måte, eller du har lyst til å programmere litt på en liten Microcontroller – er denne workshoppen for deg.  \nVi vil sette opp en ESP32 med temperatursensor, (som du får med deg hjem etterpå), programmerer litt Python hvor vi bruker JSON/REST for å koble opp mot bakenforliggende tjenester.\nVi vil  benytte gratis skytjenester og/eller  fritt tilgjengelig programvare for on-premiss til å analysere og predikere med enkel maskinlæring (generalized linear model (GLM) og Support Vecotor Machine (SVM), og hvilke som gir best nøyaktighet), og vi vil se på hva AutoML kan hjelpe oss med.\nTil slutt vil vi bygge en enkel applikasjon for å presentere analysene med plattformens Low Code verktøy.\n\n",
"title": "Tre myter om Elektriske Biler – Analysert med gratis Cloud Tjenester, ESP32 Microcontroller  JSON/REST og Maskinlæring",
"workshopPrerequisites": "Deltaker må stille med PC/MAC og en USB kabel med USB C, for å koble til Microcontroller, vi kommer med microcontrollere og sensorer, som dere får med hjem etterpå.",
"id": "02b8d1cc-ce11-464e-8c7d-171f6e9f4316",
"sessionId": "02b8d1cc-ce11-464e-8c7d-171f6e9f4316",
"conferenceId": "ad82e461-9444-40a4-a9d5-cc4885f9107a",
"speakers": [
{
"name": "Frode Pedersen",
"twitter": "",
"bio": "More than 35 years experience in Technical Sales Support : Oracle database, Middelware, Linux etc.\n\nAttended Java Zone multiple times, also involved with planning and worked on the Oracle Stand."
},
{
"name": "Inge Os",
"twitter": "",
"bio": "More than 35 years experience in Technical Sales Support : Oracle database, Middelware, Linux, Security, Java.\n\nAttended Java Zone multiple times, also worked on the Oracle Stand."
}
]
}
]
}
    "#.to_string()
}