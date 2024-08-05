// Mock data for the program to not massively spam the API
pub fn program() -> String {
    r#"
    {
"sessions": [
{
"intendedAudience": "Utviklere, arkitekter, produkteiere, designere, ledere og andre som har en interesse for modernisering og transformering av eksisterende kodebaser",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "NAV sin pensjonsløsning, Pesys, har i mange år vært blant Norges største og mest komplekse kodebaser. Opprinnelig skrevet med teknologi, som i dag, ville fått utviklere til å gråte. Vi skal i dette foredraget fortelle hvordan vi har modernisert det grafiske brukergrensesnittet. Vi skal rett og slett fortelle deg en ærlig historie om hvordan vår erfaring med dette var. Vi kommer til å snakke om den moderne retningen som premissgiver, hvor vi måtte ta tak i både menneskelige og tekniske utfordringer. På veien skal vi også prate om hvordan vi organiserte teamene og folkene, og fortelle hvordan vi tok tak i arkitekturen og det sosiotekniske aspektet av det. Du skal på den måten lære hvorfor vi mener dette økte selvsikkerheten og eierskapet til alle involverte.  Videre deler vi hvordan vi har løst de tekniske utfordringene omkring implementasjon; integrasjon og sikkerhet ved å blant annet bruke api gateways og micro-frontends. Etter å ha hørt foredraget vil du være i stand til å bedre ta tak i modernisering av legacy-kodebase i din organisasjon.",
"title": "Moderne brukergrensesnitt i en legacy kodebase",
"room": "Room 7",
"startTime": "2023-09-06T10:20",
"endTime": "2023-09-06T11:20",
"video": "861629688",
"startTimeZulu": "2023-09-06T08:20:00Z",
"endTimeZulu": "2023-09-06T09:20:00Z",
"id": "18aaf407-ff02-4f49-b11d-29877a9de906",
"sessionId": "18aaf407-ff02-4f49-b11d-29877a9de906",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T10:20",
"startSlotZulu": "2023-09-06T08:20:00Z",
"speakers": [
{
"name": "Per Christian Moen",
"twitter": "",
"bio": "Har over 20 års erfaring med utvikling i både prosjekt og forvaltning. Lidenskapelig opptatt av alt som har med programvareutvikling å gjøre. Brenner for tilegne og dele kunnskap, forbedre prosesser og kodebaser, hjelpe team, med mer. Stikker nesa borti i alt, gjerne det andre ikke ønsker å ta tak i. \n \nHar siden 2020 jobbet med å modernisere, Pesys, som er pensjonsløsningen til NAV. Her har han hatt en ledende rolle i transformasjonen fra hovedleveranser til kontinuerlige leveranser, han har vært initiativtaker for micro frontends og er deltakende i flere andre moderniseringstiltak."
},
{
"name": "Tordbjørn Wang Eriksen",
"twitter": "",
"bio": "Tordbjørn har 10 års erfaring som utvikler og er lidenskapelig opptatt av teknologi, programvareutvikling og kompetansedeling. Han har jobbet både som in-house utvikler og konsulent innen et bredt spekter av domener i både privat og offentlig sektor. Tordbjørn søkte seg til NAV IT og har jobbet med Pesys siden 2022. Hans største motivasjon er å kunne være med og bidra til moderniseringen av Pesys og samfunnsoppdraget som er gitt. Utenfor jobb er han med i Fredrikstad Tech Community og jobber for at utviklere skal ha et felles møtepunkt for å dele kompetanse uavhengig av tilhørighet.  "
}
]
},
{
"intendedAudience": "Software developers who want to gain a better understanding of software licensing.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "In the fast-paced world of software development, it is critical for developers to have a clear understanding of the legal landscape surrounding the software that they develop. This presentation will look into the different branches of software licenses and open source. In this context, the presentation will discuss common terms of commercial agreements and what pitfalls to avoid. Finally, this presentation will look into the OpenJDK project and its formal organization. Attendees will gain a solid understanding of the legal and technical considerations of choosing licensed software, enabling them to make informed decisions about their projects.",
"title": "A software developer's guide to licenses and other legalities",
"room": "Room 1",
"startTime": "2023-09-06T13:00",
"endTime": "2023-09-06T14:00",
"video": "861665089",
"startTimeZulu": "2023-09-06T11:00:00Z",
"endTimeZulu": "2023-09-06T12:00:00Z",
"id": "37cdf4dd-4f9a-4d93-ad9f-eb4994cb2f52",
"sessionId": "37cdf4dd-4f9a-4d93-ad9f-eb4994cb2f52",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T13:00",
"startSlotZulu": "2023-09-06T11:00:00Z",
"speakers": [
{
"name": "Rafael Winterhalter",
"twitter": "rafaelcodes",
"bio": "Rafael works as a software consultant in Oslo, Norway. He is a proponent of static typing and a JVM enthusiast with particular interest in code instrumentation, concurrency and functional programming. Rafael blogs about software development, regularly presents at conferences and was pronounced a JavaOne Rock Star. When coding outside of his work place, he contributes to a wide range of open source projects and often works on Byte Buddy, a library for simple runtime code generation for the Java virtual machine. For his work, Rafael received a Duke's Choice award, an Oracle groundbreaker award and was elected a Java Champion."
}
]
},
{
"intendedAudience": "Session is targeted towards experienced developers, consultants or DevOps folks willing to integrate enterprise search in their organization or projects.\n",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Elasticsearch is nowdays a preferred solution for enterprise search. You may be aware that not all of its features are available for free but did you know there is a solid open source alternative created by Amazon called OpenSearch ? And in fact this free alternative is based on a fork of Elasticsearch ? There are however certain pitfalls if you prefer to go the OpenSearch route. In this session we will make a comparison between traditional Elasticsearch and OpenSearch to understand is going completely free and open source a save path.",
"title": "Free enterprise search with OpenSearch",
"room": "Room 5",
"startTime": "2023-09-07T09:00",
"endTime": "2023-09-07T09:45",
"video": "861949288",
"startTimeZulu": "2023-09-07T07:00:00Z",
"endTimeZulu": "2023-09-07T07:45:00Z",
"id": "3ba51b2d-c986-4678-8b74-c40257576cb6",
"sessionId": "3ba51b2d-c986-4678-8b74-c40257576cb6",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T09:00",
"startSlotZulu": "2023-09-07T07:00:00Z",
"speakers": [
{
"name": "Martin Toshev",
"twitter": "@martin_fmi",
"bio": "Martin is a solution architect and IT consultant conducting professional trainings for novice and experienced developers. He is a Java enthusiast and one of the leads of the Bulgarian Java User group (BG JUG). \n"
}
]
},
{
"intendedAudience": "Java Developers that are interested in understanding more about all the different OpenJDK distributions",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "OpenJDK with it’s Java Virtual Machine is great but there is not only one flavour but many. There is Oracle OpenJDK, Eclipse Temurin, IBM Semeru, Amazon Corretto, Azul Zulu, Alibaba Dragonwell, Huawei Bi Sheng, Tencent Kona and many more. Did you ever ask yourself which one is better, faster, free or something similar? Or do you want to know where the differences are in those distributions, well then this session might bring some answers to your questions. It will give you an idea about what the JVM is and will cover all the available distributions not only of OpenJDK but also of GraalVM and will try to explain the differences and features of the available distributions. It will also try to give you an idea what JVM to use for specific use cases.",
"title": "Welcome to the Jungle - A safari through the JVM landscape",
"room": "Room 7",
"startTime": "2023-09-06T13:00",
"endTime": "2023-09-06T14:00",
"video": "861645054",
"startTimeZulu": "2023-09-06T11:00:00Z",
"endTimeZulu": "2023-09-06T12:00:00Z",
"id": "88d67125-288c-499c-85af-e0b6bab2316f",
"sessionId": "88d67125-288c-499c-85af-e0b6bab2316f",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T13:00",
"startSlotZulu": "2023-09-06T11:00:00Z",
"speakers": [
{
"name": "Gerrit Grunwald",
"twitter": "@hansolo_",
"bio": "Gerrit Grunwald is a software engineer that loves coding for around 40 years already. He is a true believer in open source and has participated in popular projects like JFXtras.org as well as his own projects (TilesFX, Medusa, Enzo, SteelSeries Swing, SteelSeries Canvas, JDKMon). \nGerrit blogs regularly at http://harmonic-code.org, he is an active member of the Java community, where he founded and leads the Java User Group Münster (Germany), he is a JavaOne rockstar and a Java Champion. He is a speaker at conferences and user groups internationally and writes for several magazines."
}
]
},
{
"intendedAudience": "Most of the developers could improve by learning a bit (more) about tests. If you're working with microservices (or big but connected services) you need to know about contract tests, and how to use them for evolving the system without breaking it. \n\nThe experience of working with microservices will help to appreciate the problem better, but even junior developers will find this session inspiring and educational! ",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Verifying behaviors of the cloud-native applications and ensuring that all of the services in the system work correctly together is both crucial and challenging. Manually maintaining environments to test the correctness of the entire system is undevops-like and fragile.\n\nLuckily, modern tools can help you to build automated, reliable test pipelines, and in this session, we explore how using Spring Cloud Contract and Testcontainers together can improve your testing and deployment processes.\n\nSpring Cloud Contract is an implementation of Consumer-Driven Contracts, an approach that provides a way to easily describe and verify APIs, at the same time allowing building API backward compatibility verification into the deployment process.\n\nTestcontainers lets developers programmatically build test environments consisting of real services running in lightweight and disposable containers. It turns the process of integration testing into a seamless, unit-test-like experience.\n\nIn this presentation, we’ll show how contract and integration tests complement each other and explore one of the most natural and reliable approaches to service evolution with contract testing. We’ll discuss why in Spring Cloud Contract, we’ve decided to switch to using Testcontainers as the solution for Kafka and AMQP messaging verification and demonstrate practical use-cases and code examples of how to set up both types of tests in your applications and deployment pipelines.",
"title": "Stop breaking stuff all the time!",
"room": "Room 6",
"startTime": "2023-09-07T15:40",
"endTime": "2023-09-07T16:40",
"video": "862083847",
"startTimeZulu": "2023-09-07T13:40:00Z",
"endTimeZulu": "2023-09-07T14:40:00Z",
"id": "3a06f863-c1ee-4d15-be2e-3cc53ae473a9",
"sessionId": "3a06f863-c1ee-4d15-be2e-3cc53ae473a9",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T15:40",
"startSlotZulu": "2023-09-07T13:40:00Z",
"speakers": [
{
"name": "Oleg Šelajev",
"twitter": "shelajev",
"bio": "Oleg Šelajev is a developer advocate at AtomicJar working on making integration tests with Testcontainers better for everyone in the community. VirtualJUG leader. In 2017 became a Java Champion."
},
{
"name": "Olga Maciaszek-Sharma",
"twitter": "olga_maciaszek",
"bio": "Olga Maciaszek-Sharma is a Senior Software Engineer in the Spring Cloud Team at VMware, where she works primarily on Spring Cloud LoadBalancer, Spring Cloud Contract (of which she was the first user and one of the first contributors), Spring Cloud OpenFeign and Spring Cloud Netflix projects. She also works on native image support for Spring Cloud projects and contributes to RSocket Broker projects from time to time. She programs mostly using Java and Groovy."
}
]
},
{
"intendedAudience": "Data Engineers, analytic engineers, data practitioners",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "At Br Karlsen we make, buy and sell fish of all kinds, here are our experience in deploying the modern data stack , built on Airbyte, dbt , BigQuery and Apache Superset. Why we chose the components we did, and what our experience has been. When did we choose the managed version vs the open source version of a product? ",
"title": "Building a fishy cloud datawarehouse in 2023",
"room": "Room 3",
"startTime": "2023-09-07T15:40",
"endTime": "2023-09-07T16:25",
"video": "862067862",
"startTimeZulu": "2023-09-07T13:40:00Z",
"endTimeZulu": "2023-09-07T14:25:00Z",
"id": "6f9c99a8-2aab-4473-a848-34b4492acb8f",
"sessionId": "6f9c99a8-2aab-4473-a848-34b4492acb8f",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T15:40",
"startSlotZulu": "2023-09-07T13:40:00Z",
"speakers": [
{
"name": "Magnus Fagertun",
"twitter": "db_magnus",
"bio": "With a long background in databases and data, Magnus went from Google in Oslo to the small island Husøy on Senja up far north to work with fish, aquaculture and data at Brødrene Karlsen. He's currently building an analytics environment in the Cloud. "
}
]
},
{
"intendedAudience": "No experience is required, but interest in / knowledge about DevOps is beneficial.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "What does a team of developers do in a BI and Analytics environment? The use of data analysis within a modern technology stack is at the core of a successful insurance business. Gjensidige, Norway's largest insurance company, has recently begun it's journey of moving all data operations from an on-prem data warehouse solution into the cloud. With almost 30 individual analyst teams in the mix, this task is no mean feat. Everything we do is data-driven with the goal to provide the best possible customer experience: from pricing to marketing and sales, as well as direct customer service, and claims handling. To succeed in this journey, we utilise the power of software best practices and DevOps culture: Product thinking, end-to-end responsibility, cross-team collaboration, and early problem solving. In this talk I will show how developers are a crucial resource to our success in publishing and maintaining high quality data products into our data platform by both growing and rooting DevOps culture and software best practices within our analyst teams.",
"title": "DevOps revolutionised software development. It's time to revolutionise data.",
"room": "Room 3",
"startTime": "2023-09-06T15:40",
"endTime": "2023-09-06T16:25",
"video": "861721829",
"startTimeZulu": "2023-09-06T13:40:00Z",
"endTimeZulu": "2023-09-06T14:25:00Z",
"id": "036d16a0-59e0-4001-b7b3-08d30cf01a6a",
"sessionId": "036d16a0-59e0-4001-b7b3-08d30cf01a6a",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T15:40",
"startSlotZulu": "2023-09-06T13:40:00Z",
"speakers": [
{
"name": "Alexandra Diem",
"twitter": "",
"bio": "Former academic turned data scientist turned data mesh frontrunner. \n\nAlexandra has a background in applied mathematics and statistics, and holds a PhD in mathematical and computational modelling of physiological processes. As an academic Alexandra championed reproducible and reusable science, recognising and utilising the benefits of version control and containerised development early on. Following her academic career, Alexandra worked as both a software development and data science consultant in several domains in both the private and public sector. Her solution-oriented approach allowed her to recognise the potential of DevOps best practices within data science, becoming a frontrunner in implementing DevOps culture within the data world. Today, Alexandra leads team Cloud Analytics and MLOps at Gjensidige, Norway’s largest insurance company. The team’s mission is to empower analysts by facilitating self-service analytics, encouraging cross-team collaboration, and promoting software best practices.\n\nIn my spare time you'll find me either on a bike or a pair of skis."
}
]
},
{
"intendedAudience": "The talk does not require any prior knowledge, but some points will be lost to non-technical participants.\n\nDevelopers, Tech Leads and Architects will get the most out of the talk, but the content is adapted to a varied audience so managers of technology businesses would also benefit. ",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "You might have heard that the advent of Quantum Computers heralds the end of commonly used cryptography, but what does that mean, and why does it concern you?\n\nIn this talk, I will help answer some of these questions. You will learn what a Quantum Computer is, why they are such a bad thing for cryptography, and what you should do about it.",
"title": "Post-Quantum Cryptography 101 - aka. The end of the world as we know it",
"room": "Room 1",
"startTime": "2023-09-07T13:00",
"endTime": "2023-09-07T13:45",
"video": "862026553",
"startTimeZulu": "2023-09-07T11:00:00Z",
"endTimeZulu": "2023-09-07T11:45:00Z",
"id": "83e6d221-af8f-4c57-9d5f-81a6aff39a4e",
"sessionId": "83e6d221-af8f-4c57-9d5f-81a6aff39a4e",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T13:00",
"startSlotZulu": "2023-09-07T11:00:00Z",
"speakers": [
{
"name": "Stian Svedenborg",
"twitter": "@SSvedenborg",
"bio": "Stian is a security enthusiast with a passion for cryptography. He graduated from NTNU in 2014 specializing in cryptography and spent a number of years as a developer. He has entered the eID space as the Security Architect for BankID."
}
]
},
{
"intendedAudience": "Anyone interested in the leading-edge features of recent Java releases would get a good overview from this talk. Anyone interested in Ruby on the JVM would see how we are making use of these features. Intermediate experience is probably best, but nothing I show would be too difficult for a beginner to understand.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "JRuby brought Ruby to the JVM years ago, and since then we have pushed JVM developers for the features we need: dynamic invocation, virtual threads, native function calls. Now in 2023, we have nearly all of these features available! This talk will survey key features of the modern JVM: invokedynamic to optimize dynamic code, Loom for true userland threading, and Panama for native operations. We'll show how JRuby builds on these features to support Ruby, and how they will eventually help all JVM users, regardless of your chosen language.",
"title": "Ruby on the Modern JVM Using Loom, Panama, and More",
"room": "Room 4",
"startTime": "2023-09-07T09:00",
"endTime": "2023-09-07T10:00",
"video": "861951420",
"startTimeZulu": "2023-09-07T07:00:00Z",
"endTimeZulu": "2023-09-07T08:00:00Z",
"id": "a10ed0ff-e95c-4663-b9c6-74ed54abf7a4",
"sessionId": "a10ed0ff-e95c-4663-b9c6-74ed54abf7a4",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T09:00",
"startSlotZulu": "2023-09-07T07:00:00Z",
"speakers": [
{
"name": "Charles Oliver Nutter",
"twitter": "headius",
"bio": "Charles has worked at Red Hat for the past decade as a JRuby core developer and JVM language advocate. He believes the JVM is the best general-purpose platform for language development, and has spent the last 20 years working with JVM developers to add features that benefit all languages."
}
]
},
{
"intendedAudience": "The expected audience are frontend developers wanting to learn about Svelte and SvelteKit. The talk will start with an empty slate and build up a full web application from start to finish, developing an API as development progress. ",
"length": "240",
"format": "workshop",
"language": "en",
"abstract": "Single page apps and the library frameworks that supports them have transformed the web and moved the post regarding what the users expects from a web application. \n\nIn all their glory, however, they do come with some drawbacks. For starters they mostly have a steep learning curve, forcing the developer to invest a significant amount of time to get acquainted with their library of choice.\n\nThe nature of SPAs mean that the size of the application grows along the apps’ functionality, while often forcing the developers into a complex programming regime. \n\nIn between SPAs and traditional web apps we find Svelte, occupying a new segment - transitional apps. As Svelte is a compiler and not a library, it gets around some of the constraints of the SPA libraries, being able to build extremely fast websites with a small footprint - as svelte can include only the parts you actually use instead of the whole library. This also means that the feature set offered with Svelte can grow without impacting the size and complexity of your own app. \n\nThe development process is easy to get started with, while retaining the power needed to build complex apps that have the features a user have come to expect from a web app in 2021. \n\nIn this workshop you will learn to build a complete website with SvelteKit, complete with backend requests, API development, server side rendering, and the reactivity you have come to expect from a modern web app. ",
"title": "Making the web Svelter (tm) with SvelteKit",
"workshopPrerequisites": "Some tools should be installed before attending the workshop: \n- Git\n- NPM\n- Node (a recent version)",
"room": "Workshop E",
"startTime": "2023-09-05T09:00",
"endTime": "2023-09-05T13:00",
"registerLoc": "https://moosehead.javazone.no/#/register/making_the_web_svelter_tm_with_sveltekit",
"startTimeZulu": "2023-09-05T07:00:00Z",
"endTimeZulu": "2023-09-05T11:00:00Z",
"id": "12606bcd-4e08-44e1-820a-658f3be4efbd",
"sessionId": "12606bcd-4e08-44e1-820a-658f3be4efbd",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T09:00",
"startSlotZulu": "2023-09-05T07:00:00Z",
"speakers": [
{
"name": "Joachim Haagen Skeie",
"twitter": "joachimhs",
"bio": "Joachim Haagen Skeie is a full stack developer, and author of Ember.js in Action (Manning Publications), have been involved with javaBin and Teknologihuset during the mid 2010s, and run a makerspace for kids in Oslo up until the pandemic hit. Joachims current role is as a tech lead at NorgesGruppen Data, from august 2023 as a senior consultant/tech lead at Experis. "
}
]
},
{
"intendedAudience": "Intermediate engineers, doing events.\nThose who have not ground their teeth building event-based systems - to see what are errors, why are they errors, and how to build around.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Some say: Use events, events are great, all will be great and rosy. Then reality bites, and.. let's talk about strangers you'd rather avoid - mistakes to make when building using events. Things change, evolve, are not and overdelivered, old events start taking a lot of space, and someone is already using the name you wanted?! Expect: problems, symptoms, consequences, and some 'get-out-of' cards. Come for the experience others gained first-hand, so you know what to avoid.",
"title": "Avoiding mistakes with events, one event at a time",
"room": "Room 6",
"startTime": "2023-09-07T09:00",
"endTime": "2023-09-07T10:00",
"video": "861948180",
"startTimeZulu": "2023-09-07T07:00:00Z",
"endTimeZulu": "2023-09-07T08:00:00Z",
"id": "1306f8e9-af1b-4e2c-9ba3-7c4771b6aba1",
"sessionId": "1306f8e9-af1b-4e2c-9ba3-7c4771b6aba1",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T09:00",
"startSlotZulu": "2023-09-07T07:00:00Z",
"speakers": [
{
"name": "Andrzej Grzesik",
"twitter": "ags313",
"bio": "ags likes distributed systems in all shapes and form. Coding since the age of 8, loves simplicity and continuous delivery. While he has written in many languages, he favours the JVM. Since 'most software problems are people problems”, he stirs communities, organizes and speaks at conferences. And he is a Java Champion!"
},
{
"name": "Wojtek Ptak",
"twitter": "wjptak",
"bio": "Wojtek works as Head of Product Engineering and Engineering Executive at Revolut. Before, he worked as CTO for several companies, provided consulting, training, and assisted in building various data collecting, analytics, and applied ML/AI solutions, including Big Data implementations, data stream processing systems, and data insight projects.\n\nHe worked with multiple Forbes 500 brands in the US, UK, and the Netherlands, including The Coca-Cola Company, the American Bankers Association, Macy’s, Bloomingdales, Heineken, Saks 5th Avenue, BP, Boots, Polo Ralph Lauren, Porsche, HSBC, and others."
}
]
},
{
"intendedAudience": "The workshop is intended for any developer who have basic knowledge of SQL, but want to understand how to improve their queries, or are just curious as to how a relational database works.",
"length": "120",
"format": "workshop",
"language": "no",
"abstract": "Bad SQL statements can ruin both performance and user experience. And in some cases, they can be quite hard to fix. Luckily most relational databases have a tool we can use to analyse the execution of queries, and understand where the problem lies.\n\nIn this workshop we will be taking a look at Postgres' implementation of this functionality, called Explain. We'll use Postgres as it is a popular open-source alternative, but the principles you'll learn are transferrable to most other relational databases.\n\nWe will talk about how to interpret the execution plans and get to know some of the most common operations. Underway, we'll be testing the theory on a supplied dataset to see how it works in practice. After completion, you should be able to read and analyse most queries.\n\nThe workshop is intended for any developer who have basic knowledge of SQL, but want to understand how to improve their queries, or are just curious as to how a relational database works.",
"title": "Explain Postgres Explain",
"workshopPrerequisites": "Laptop with Git and Docker installed.\n\nIf you have a preferred database client, it would be advisable to install it before the workshop. If not, a client will be supplied.",
"room": "Workshop E",
"startTime": "2023-09-05T15:45",
"endTime": "2023-09-05T17:45",
"registerLoc": "https://moosehead.javazone.no/#/register/explain_postgres_explain",
"startTimeZulu": "2023-09-05T13:45:00Z",
"endTimeZulu": "2023-09-05T15:45:00Z",
"id": "04831fc3-0b49-4517-93fc-ccbf98987c2b",
"sessionId": "04831fc3-0b49-4517-93fc-ccbf98987c2b",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T15:45",
"startSlotZulu": "2023-09-05T13:45:00Z",
"speakers": [
{
"name": "Jørgen Langemyr",
"twitter": "",
"bio": "Jørgen works as a senior consultant for Kantega, with previous experience from Kongsberg Gruppen. And he is passionately interested in all things backend."
}
]
},
{
"intendedAudience": "Foredraget er for alle, og det kreves ingen forkunnskaper for å forstå innholdet. Foredraget tar for seg en nisje problemstilling som de fleste kan lære fra.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "En redegjøring for hva CAPTCHA er og hvorfor dagens CAPTCHA løsninger ikke er tilstrekkelig universelt utformet. Dersom man ser på CAPTCHA som bruk av harde AI problemer for å oppnå sikkerhet, er det da konseptuelt mulig å oppnå universell utforming? Foredraget tar for seg dagens situasjon og mulige løsninger for fremtiden.",
"title": "Universelt utformet CAPTCHA - er det mulig?",
"room": "Room 5",
"startTime": "2023-09-07T14:20",
"endTime": "2023-09-07T15:05",
"video": "862045944",
"startTimeZulu": "2023-09-07T12:20:00Z",
"endTimeZulu": "2023-09-07T13:05:00Z",
"id": "f422a137-ccc2-4336-844f-eca42ac8a7a9",
"sessionId": "f422a137-ccc2-4336-844f-eca42ac8a7a9",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T14:20",
"startSlotZulu": "2023-09-07T12:20:00Z",
"speakers": [
{
"name": "Levi Sørum",
"twitter": "",
"bio": "Levi er en dev-ops og fullstack utvikler hos Kraftlauget som også interesserer seg for inkludering i utvikling og universell utforming.\n\nGjennom sin rolle som teknisk ansvarlig i den frivillige skeiv-samiske organisasjonen Garmeres har han møtt på flere nisje-problemstillinger i forsøk på å utvikle løsninger som er inkluderende for kryss-minoriteter."
}
]
},
{
"intendedAudience": "The expected audience for this talk would be Java developers who are interested in learning about the new concurrency model and how to take advantage of virtual threads and structured concurrency in their applications. They should have some basic knowledge of Java and multithreading, but don't necessarily need to be experts in the field.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Java 20 will preview one of the most anticipated features, virtual threads, and structured concurrency. It rethinks the way multithreading works in Java. It was born out of the idea that reactive Java is too complicated and blocking threads has to be cheap. Resulting in the new virtual threads that we run thousands of! \n\nThe aim of virtual threads and structured concurrency is to have high-throughput lightweight threads and new programming models on the Java platform. \n\nIn this talk, I will demonstrate virtual threads and how to create and manage them using structured concurrency. Furthermore, I'll provide some practical advice to avoid pitfalls when you start using virtual threads in your application.   ",
"title": "Introduction and pitfalls of Java's new concurrency model",
"startTime": "2023-09-07T15:40",
"endTime": "2023-09-07T16:25",
"room": "Room 5",
"video": "862083576",
"startTimeZulu": "2023-09-07T13:40:00Z",
"endTimeZulu": "2023-09-07T14:25:00Z",
"id": "bfbc0c41-fc9f-4392-89a1-be70ec389ac1",
"sessionId": "bfbc0c41-fc9f-4392-89a1-be70ec389ac1",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T15:40",
"startSlotZulu": "2023-09-07T13:40:00Z",
"speakers": [
{
"name": "David Vlijmincx",
"twitter": "@David_Vlijmincx",
"bio": "David is an experienced Java developer who has been working with the programming language since 2016. He is highly active in the Java community and shares his knowledge through his blog, where he writes about Java-related topics. In addition to his writing, he is also an international speaker who has given talks on Java at various events. Furthermore, he is also an author, writing a book about Jakarta EE 10"
}
]
},
{
"intendedAudience": "Every person that uses JVM as the daily driver and is also interested in managing infrastructure programatically, typically mixed devops teams members.\n",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Infrastructure-as-code is a constantly growing field where the need for reliability necessitated by operations meets with application developer knowledge and experience. JVM platform is well known for it's robustness and JVM languages are known for being awesome choices to build reliable software. In my talk I will introduce and compare 3 language SDKs - Java, Kotlin and Scala for Pulumi - a terraform alternative that allows you to use a full programming language to declare infrastructure. I will present what benefits they bring to the table and how you can leverage your knowledge to build scalable and maintainable platform for your team.",
"title": "Infrastructure as code done right with JVM",
"room": "Room 4",
"startTime": "2023-09-06T14:20",
"endTime": "2023-09-06T15:20",
"video": "861667150",
"startTimeZulu": "2023-09-06T12:20:00Z",
"endTimeZulu": "2023-09-06T13:20:00Z",
"id": "444fd903-4857-4591-b0c2-a824ccf5d3af",
"sessionId": "444fd903-4857-4591-b0c2-a824ccf5d3af",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T14:20",
"startSlotZulu": "2023-09-06T12:20:00Z",
"speakers": [
{
"name": "Łukasz Biały",
"twitter": "lukasz_bialy",
"bio": "Polyglot full-stack developer and functional programming enthusiast. PSE @ VirtusLab. Values quality over quantity. Permanent learner with a severe information dependency problem. Enjoys conversations about philosophy and all things related to mind's inner workings. Loves mountains, biking and hiking."
}
]
},
{
"intendedAudience": "Developers with any experience level can take something away from this talk. ",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Are you a software engineer? You may want to think twice before answering... Looking closely, our collective software development practices may not be so rigidly engineered as we'd like to think. And you know what? That's ok! Let's embrace the art of software development and see where this takes us. In this keynote, we'll look at software development as a craft combining art and engineering. What does this mean for coding, testing, architecture and design, and other important aspects of our work? And more importantly: what does this tell us about learning and growing as software developers? Our obsession with tools may cloud what software development is really about. Join this keynote to unlock your inner artist!",
"title": "The Art of Software Development",
"room": "Room 6",
"startTime": "2023-09-06T18:20",
"endTime": "2023-09-06T19:05",
"video": "861739113",
"startTimeZulu": "2023-09-06T16:20:00Z",
"endTimeZulu": "2023-09-06T17:05:00Z",
"id": "3c966990-920a-448e-bf78-0e6ae30119c0",
"sessionId": "3c966990-920a-448e-bf78-0e6ae30119c0",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T18:20",
"startSlotZulu": "2023-09-06T16:20:00Z",
"speakers": [
{
"name": "Sander Mak",
"twitter": "@Sander_Mak",
"bio": "Sander is part of the tech leadership team at Picnic —the Dutch online grocery scale-up— building Java-based systems at scale. He also is a Java Champion and author of the O'Reilly book 'Java 9 Modularity' (see javamodularity.com). As an avid conference speaker, Sander loves sharing knowledge, and also does so as a Pluralsight instructor."
}
]
},
{
"intendedAudience": "Alle som jobber med utvikling av software!",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "\"Softwarekrisen\" ble i industriens barndom brukt om utfordringene med å utvikle nyttige og effektive dataprogrammer på tilmålt tid. Nå, 50 år senere, er det mye som tyder på at software er i ferd med å kollapse under sin egen vekt: Det lages for mye software og for dårlig software. Vi som industri er sjanseløse på å forvalte den. Det finnes ingen magiske løsninger, men denne problemstillingen må få konsekvenser for /hva/ vi lager og /hvordan/ vi lager det.",
"title": "Software kollapser under sin egen vekt",
"room": "Room 7",
"startTime": "2023-09-07T14:20",
"endTime": "2023-09-07T15:05",
"video": "862044885",
"startTimeZulu": "2023-09-07T12:20:00Z",
"endTimeZulu": "2023-09-07T13:05:00Z",
"id": "a49db1ec-cda5-402a-8674-398206352871",
"sessionId": "a49db1ec-cda5-402a-8674-398206352871",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T14:20",
"startSlotZulu": "2023-09-07T12:20:00Z",
"speakers": [
{
"name": "Trond Arve Wasskog",
"twitter": "ilmyggo",
"bio": "Trond Arve har vært utvikler og arkitekt i Bekk, og er nå Principal i NAV IKT. Han har jobbet med forvaltning og modernisering hos store norske virksomheter, og er stadig overrasket over at ting henger sammen."
}
]
},
{
"intendedAudience": "Nerds, Developers, Decision Makers, Project Manager",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "What can Artificial Intelligence achieve today? In this lecture, we will show numerous live examples to point out whether the term is an overhyped buzzword or the key technology of our digital future.\n\nThe TNG Innovation Hacking Team has been working on numerous AI projects in the field of computer vision and natural language processing for years. Thomas Endres, Martin Förtsch and Jonas Mayer take you on a journey through the world of Artificial Intelligence and its architecture. Based on various innovation projects, the speakers will illuminate the basic structure of the underlying neural networks, but also their limitations.\n\nA firework of live demonstrations and showcases completes the entertaining experience. There will be neural networks creating art, deep fakes in real time, an NLP chatbot, and an AI that can generate social media comments.",
"title": "The future of AI is now - Transformations in Vision and Language Processing",
"room": "Room 3",
"startTime": "2023-09-07T09:00",
"endTime": "2023-09-07T10:00",
"video": "861949568",
"startTimeZulu": "2023-09-07T07:00:00Z",
"endTimeZulu": "2023-09-07T08:00:00Z",
"id": "4cb1c2d9-0c3c-49cf-8037-5936b285da98",
"sessionId": "4cb1c2d9-0c3c-49cf-8037-5936b285da98",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T09:00",
"startSlotZulu": "2023-09-07T07:00:00Z",
"speakers": [
{
"name": "Thomas Endres",
"twitter": "originalone1984",
"bio": "In his role as a Partner for TNG Technology Consulting in Munich, Thomas Endres works as an IT consultant. Besides his normal work for the company and the customers he is creating various prototypes - like a telepresence robotics system with which you can see reality through the eyes of a robot, or an Augmented Reality AI that shows the world from the perspective of an artist. He is working on various applications in the fields of AR/VR, AI and gesture control, putting them to use e.g. in autonomous or gesture controlled drones. But he is also involved in other Open Source projects written in Java, C# and all kinds of JavaScript languages.\n\nThomas studied IT at the TU Munich and is passionate about software development and all the other aspects of technology. As an Intel Software Innovator and Black Belt, he is promoting new technologies like AI, AR/VR and robotics around the world. For this he received amongst others a JavaOne Rockstar award."
},
{
"name": "Martin Förtsch",
"twitter": "MartinFoertsch",
"bio": "Martin Förtsch is an IT-consultant of TNG Technology Consulting GmbH based in Unterföhring near Munich who studied computer sciences. Work wise his focus areas are Agile Development (mainly) in Java, Search Engine Technologies, Information Retrieval and Databases.\n\nAs an Intel Software Innovator and Intel Black Belt Software Developer he is strongly involved in the development of open-source software for gesture control with 3D-cameras like e.g. Intel RealSense and has built an Augmented Reality wearable prototype device with his team based on this technology.\n\nFurthermore, he gives many talks at national and international conferences about Artificial Intelligence, Internet of Things, 3D-camera technologies, Augmented Reality and Test Driven Development as well. He was awarded with the Oracle JavaOne Rockstar."
},
{
"name": "Jonas Mayer",
"bio": "Jonas Mayer is a Senior Consultant at TNG Technology Consulting. As Head of Innovation Hacking, his main focus lies on creating innovative showcases and prototypes in soft- and hardware. Since 2018 he's been working on numerous projects ranging from market-leading Realtime Deepfakes, an LLM Shitposting AI, all the way to autonomous drone racing. As a keynote speaker, he has been talking about the Innovation Hacking projects at over a hundred conferences all across IT and Tech. Prior to joining TNG, Jonas studied Informatics: Games Engineering at TU Munich. More information can be found at innovation-hacking.com."
}
]
},
{
"intendedAudience": "The expected audience for this conference talk are Java developers who are interested in deploying their applications to the cloud and optimizing the performance of their Java applications. Specifically, the talk will be beneficial to Java developers who are concerned about JVM startup time and want to learn how to improve it. Additionally, the talk will be relevant to developers who use scale-to-zero policies for clusters, as it will discuss how checkpoint and restore can help improve startup times while working with unprivileged/root-less modes in containers.This talk is applicable to seasoned veterans and newcomers to the industry.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Fast JVM Startup or Dynamic JVM Capabilities, why not both?\n\nStatically compiled applications can significantly improve startup time, but they may not be suitable for existing applications that rely on the dynamic functionalities of a JVM. However, with checkpoint and restore, you can now enjoy the lightning-fast startup time of native images while keeping all the capabilities of a JVM.\n\nJoin us in this session to learn how checkpoint and restore can revolutionize the way you approach JVM startup. Discover how this approach can help you improve your Java application's startup performance, while maintaining its dynamic features. We'll explore the challenges and solutions in implementing checkpoint and restore with unprivileged/root-less modes in containers, and how you can overcome these hurdles to improve your application's startup.\n\n\n",
"title": "Lightning-Fast JVM Startup without compromise",
"room": "Room 6",
"startTime": "2023-09-06T11:40",
"endTime": "2023-09-06T12:25",
"video": "861612487",
"startTimeZulu": "2023-09-06T09:40:00Z",
"endTimeZulu": "2023-09-06T10:25:00Z",
"id": "46936190-72ed-4f7b-a674-3f7a5826f328",
"sessionId": "46936190-72ed-4f7b-a674-3f7a5826f328",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T11:40",
"startSlotZulu": "2023-09-06T09:40:00Z",
"speakers": [
{
"name": "Tobi Ajila",
"twitter": "@tobi_ajila",
"bio": "Tobi is a Java Runtime developer for the OpenJ9 VM team in Ottawa, Canada. In the past he has worked on Interpreter optimizations, JVMTI enhancments, Modularity, JSR 335 and more. Currently, his main focus is on investigating checkpoint/restore technology at the JVM level. and Project Panama where he collaborates with other developers in the Valhalla expert group."
}
]
},
{
"intendedAudience": "Java developers looking for an introduction to Micronaut and GraalVM",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "In this talk, Micronaut creator Graeme Rocher will demonstrate how to build fast efficient Cloud Native applications with Micronaut 4 and GraalVM. Taking advantage of the latest features of the Micronaut 4 and GraalVM, Graeme will use live coding demonstrations to showcase the simplifications offered by Micronaut and efficient deployment options with GraalVM.",
"title": "GraalVM Cloud Native applications with Micronaut 4",
"room": "Room 5",
"startTime": "2023-09-06T14:20",
"endTime": "2023-09-06T15:05",
"video": "861666089",
"startTimeZulu": "2023-09-06T12:20:00Z",
"endTimeZulu": "2023-09-06T13:05:00Z",
"id": "033d0201-1063-4694-9e0e-e7201160b41a",
"sessionId": "033d0201-1063-4694-9e0e-e7201160b41a",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T14:20",
"startSlotZulu": "2023-09-06T12:20:00Z",
"speakers": [
{
"name": "Graeme Rocher",
"twitter": "graemerocher",
"bio": "Graeme Rocher is the creator of several popular Open Source projects including Grails (https://grails.org) and Micronaut (https://Micronaut.io) and co-author of \"The Definitive Guide to Grails\" (Apress - http://apress.com/book/view/1590599950). Graeme currently works as an Architect at Oracle. Graeme is a member of the Java Champions (https://twitter.com/java_champions) and 2018 was awarded the Groundbreaker award by Oracle (https://developer.oracle.com/groundbreakers/) for his work on Open Source."
}
]
},
{
"intendedAudience": "Utviklere, sikkerhetsfolk, og personer som er interessert i hvordan utvikle software som kunden definitivt ikke vil at du skal produksjonssette!",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Datasystemer blir bare mer viktige og mer integrerte i livene våre. De siste tiårene har gjort livene våre digitale og nå følger også industrien vi tar for gitt etter. vann, kraft, mat og all industriell aktivitet går mot å digitaliseres og kobles på nett. Det gir økt effektivitet og innsikt i de viktigste bærebjelkene i samfunnet, men det åpner også for nye sårbarheter.\n\nHar du hørt navn som Industroyer, NoPeyta eller Sandworm eller annen skadevare og lurt på hvordan de egentlig fungerer? Hvordan går man frem når man skal bygge software for å bryte seg inn, få fotfeste i et system og oppnå målet, ødeleggelse?\n\nI denne presentasjonen skal vi se på hvordan malware fungerer ved å utvikle vår egen command and control (C2) applikasjon. Med funksjonalitet for å generere payloads for å bryte seg inn i et system, unngå deteksjon og tilslutt ringe hjem for å motta nye instrukser. Ved å forstå hvordan hacking software fungerer, og hvilke mål de prøver å oppnå, kan vi bedre forsvare oss mot det.",
"title": "Hvordan bygge software for å ødelegge",
"room": "Room 6",
"startTime": "2023-09-06T13:00",
"endTime": "2023-09-06T13:45",
"video": "861642841",
"startTimeZulu": "2023-09-06T11:00:00Z",
"endTimeZulu": "2023-09-06T11:45:00Z",
"id": "a98ce483-fd28-4ab0-8834-92089f01618b",
"sessionId": "a98ce483-fd28-4ab0-8834-92089f01618b",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T13:00",
"startSlotZulu": "2023-09-06T11:00:00Z",
"speakers": [
{
"name": "Sondre Halvorsen",
"twitter": "",
"bio": "Sondre Halvorsen, utvikler i Omny AS (https://omnysecurity.com/). Vårt oppdrag hos Omny er å beskytte samfunnets mest kritiske infrastruktur fra den stadig økende kompleksiteten til cyber og fysiske trusler.\n\n"
}
]
},
{
"intendedAudience": "Åpen for alle. Relevant både for utviklere, designere, prosjektledere - og kanskje spesielt for de som er foreldre i tillegg. Vi håper at flere blir inspirert til å satse på sine hobbyprosjekt!",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Hva har en utvikler, en designer, en lege og en influenser til felles? - Vi er alle mødre! Og vi er ganske lei av manglende og dårlige verktøy som liksom skal organisere familielivet. Såååå - vi har bestemt oss for å lage en app selv. Vi ønsker å lage et verktøy som hjelper småbarnsfamilier til å fjerne litt av kaoset i hverdagen. Vi ønsker at hverdagsoppgavene skal fordeles likt mellom partnere og at det tredje skiftet ikke kun er forbeholdt mødre. Dette er historien om hvordan det har gått hittil - med innblikk i prosess, teknologivalg og brukertesting.",
"title": "Historien om fire mammaer som lagde en app",
"room": "Room 1",
"startTime": "2023-09-06T15:40",
"endTime": "2023-09-06T16:25",
"video": "861712117",
"startTimeZulu": "2023-09-06T13:40:00Z",
"endTimeZulu": "2023-09-06T14:25:00Z",
"id": "442d4982-f852-416b-95a3-eb4eeac280f5",
"sessionId": "442d4982-f852-416b-95a3-eb4eeac280f5",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T15:40",
"startSlotZulu": "2023-09-06T13:40:00Z",
"speakers": [
{
"name": "Ellen Wagnild-Antonsen",
"twitter": "",
"bio": "Ellen Wagnild-Antonsen jobber som tjenestedesigner i Variant Trondheim. Ellen er utdannet sivilingeniør i Industriell design og har i tillegg en bachelorgrad i markedsføringskommunikasjon. Hun elsker å jobbe i team for å utvikle kreative løsninger til komplekse problem. Ellen er også en mamma, entreprenør og feminist."
},
{
"name": "Sarah Serussi",
"twitter": "",
"bio": "Sarah jobber i Variant Trondheim som utvikler. Hun er utdannet sivilingeniør i data og foretrekker frontend-utvikling. I Variant er hun mangfoldsansvarlig og ellers er hun mamma, entreprenør og feminist. "
}
]
},
{
"intendedAudience": "Alle. Er du interessert i hvordan C64 fungerer, eller lurer på hva assembly er for noe? Har du lyst å vite hvordan man laget spill på 80-tallet?",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Utvikling har de siste årene blitt redusert til Jira-oppgaver, forutsigbare prosesser, og høynivå-språk. La oss ta et langt steg tilbake og se på hvordan man utvikler spill på en av meste solgte hjemmedatamaskinene noensinne.\n\nI dette foredraget vil Ricki fortelle om sin reise inn i moderne spillutvikling på den over 40 år gamle Commodore 64 maskinen.",
"title": "Moderne Retro-utvikling på Commodore 64",
"room": "Room 7",
"startTime": "2023-09-06T18:20",
"endTime": "2023-09-06T19:20",
"video": "861737315",
"startTimeZulu": "2023-09-06T16:20:00Z",
"endTimeZulu": "2023-09-06T17:20:00Z",
"id": "4c0f1d7a-1866-4931-9423-82ae00961ce9",
"sessionId": "4c0f1d7a-1866-4931-9423-82ae00961ce9",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T18:20",
"startSlotZulu": "2023-09-06T16:20:00Z",
"speakers": [
{
"name": "Ricki Sickenger",
"twitter": "@bag_of_hats",
"bio": "Ricki Sickenger har 10 års erfaring fra spillbransjen, og ble konsulent i 2009. I 2012 var han en av gründerne av Sonat Consulting Bergen, der han fremdeles jobber som utvikler og agile lead. På fritiden utvikler han spill på gamle plattformer som C64 og Amiga.\n"
}
]
},
{
"intendedAudience": "Utviklere som interesserer seg for byggverktøy, og/eller irriterer seg over mange PRer for oppdateringer av avhengigheter og plutselige breaking changes når du endelig har tid til å oppdatere den interne pakken som du ligger bakpå.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Historien om hvordan noen av teamene i Fremtind gikk fra polyrepo, Jenkins og Maven over til Bazel og monorepo. Nå bygges applikasjoner i både Java, Kotlin, Javascript og Typescript (fortsatt plass til flere!) med samme byggverktøy, fra samme repository og med samme workflows. Lær mer om hvorfor vi valgte Bazel og hva som skiller det fra andre byggsystemer, utfordringer underveis og hvordan vi løste dem og veien videre for oss.",
"title": "Hjelp, vi bygger med Bazel!",
"room": "Room 2",
"startTime": "2023-09-07T13:30",
"endTime": "2023-09-07T13:50",
"video": "862029410",
"startTimeZulu": "2023-09-07T11:30:00Z",
"endTimeZulu": "2023-09-07T11:50:00Z",
"id": "ed28bc79-5aed-475d-8f86-9737a2262402",
"sessionId": "ed28bc79-5aed-475d-8f86-9737a2262402",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T13:00",
"startSlotZulu": "2023-09-07T11:00:00Z",
"speakers": [
{
"name": "Knut Eirik Leira Hjelle",
"twitter": "",
"bio": "Knut Eirik Leira Hjelle er utvikler i Fremtind med 14 års erfaring innen utvikling i hele stacken, med alt fra restaurantløsninger, online dating til forsikringløsninger og infrastruktur. Det har foregått i en rekke forskjellige språk, og mange byggverktøy og CI/CD-løsninger. Når Knut Eirik ikke programmerer, finner du han på konserter, på pub eller i skogen på en stisykkel."
}
]
},
{
"intendedAudience": "For anyone interested in what data the government has and shares. No prior experience with government data needed.",
"length": "10",
"format": "lightning-talk",
"language": "en",
"abstract": "What data is being shared by the government and what is not (yet) shared?\n\nThere’s a lot of data available from the Norwegian government, from company registry to weather data. You can find most of what’s currently shared via the the national data-portals data.norge.no and geonorge.no. But what data is not yet shared?\nWe recently created an overview on data.norge.no of who is sharing (or not) and of known requests for data to be shared. We’ll give you an overview of the status, and tips for finding data/APIs and how you can request data which is not yet available.",
"title": "Mapping what data the norwegian government has yet to share",
"room": "Room 2",
"startTime": "2023-09-06T14:40",
"endTime": "2023-09-06T14:50",
"video": "861699562",
"startTimeZulu": "2023-09-06T12:40:00Z",
"endTimeZulu": "2023-09-06T12:50:00Z",
"id": "0f5ceec5-89f7-4d9b-93d5-4aea120d354b",
"sessionId": "0f5ceec5-89f7-4d9b-93d5-4aea120d354b",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T14:20",
"startSlotZulu": "2023-09-06T12:20:00Z",
"speakers": [
{
"name": "Livar Bergheim",
"twitter": "livarb",
"bio": "Livar has been working with sharing of government data for many years, and is currently a data hunter at the Norwegian Digitalisation Agency (Digdir)"
}
]
},
{
"intendedAudience": "Rastløse utviklere (og deres ledere) som ønsker fornyet pågangsmot, inspirasjon og mulighet til å skape noe nytt.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Ansatte bør få regelmessig fri fra daglige oppgaver til å oppdage nye evner og gjøre uvanlige ting. Men, hvordan gjør du dette internt og hvordan overbeviser du sjefen om å gi deg tid til det? Du vil få høre om hvordan Fremtind kjører halvårlige skaperdager for alle ansatte hvor det lages teknologiske løsninger på samfunnsproblemer på tre dager.",
"title": "Laserkuttere + forsikringsselskap = Lykkeligere utviklere",
"room": "Room 2",
"startTime": "2023-09-07T13:00",
"endTime": "2023-09-07T13:20",
"video": "862030016",
"startTimeZulu": "2023-09-07T11:00:00Z",
"endTimeZulu": "2023-09-07T11:20:00Z",
"id": "fe59b93f-23ba-48d2-b095-1cc89939c5cf",
"sessionId": "fe59b93f-23ba-48d2-b095-1cc89939c5cf",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T13:00",
"startSlotZulu": "2023-09-07T11:00:00Z",
"speakers": [
{
"name": "Glenn Brownlee",
"twitter": "",
"bio": "Med bakgrunn som utvikler, jobber Glenn nå som engineering manager i Schibsted Accelerator hos FINN. Der er han en av tech-hodene som er med på å sparke i gang de ferskeste markedsplassene i Norden."
}
]
},
{
"intendedAudience": "People who use networking tools and would like to understand what happens when they get a cryptic error message involving an IPv6 address.\nPeople who are curious about how IPv6 differs from IPv4.",
"length": "10",
"format": "lightning-talk",
"language": "en",
"abstract": "IPv6 has slowly started to appear and people may be using it without knowing it,... and be baffled when a DNS name resolves to an IPv6 address instead of an IPv4 address and e.g. ping suddenly can't route.\n\nThis lightning talk aims to help people quickly troubleshoot and work around IPv6 problems they stumble over, explain why there may seemingly be an IPv6 address on your NIC but you still can't route, how to read and understand the cryptic and long hex strings that are IPv6 addresses, what it means if you have multiple IPv6 addresses on a NIC and more.\n\nAlso a super-brief history lesson and why NAT sidelined IPv6 for so many years, as well as why IPv6 is making a comeback. And about what IPv6 can do that NAT'ed IPv4 can't.\n\nMuch will be done in a live demo from a GNU/linux bash command line.",
"title": "A lightning fast practical intro to IPv6",
"startTime": "2023-09-06T13:20",
"endTime": "2023-09-06T13:30",
"room": "Room 2",
"video": "861674793",
"startTimeZulu": "2023-09-06T11:20:00Z",
"endTimeZulu": "2023-09-06T11:30:00Z",
"id": "b4b5a624-a38f-414a-873a-225da6e4e698",
"sessionId": "b4b5a624-a38f-414a-873a-225da6e4e698",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T13:00",
"startSlotZulu": "2023-09-06T11:00:00Z",
"speakers": [
{
"name": "Steinar Bang",
"twitter": "@steinarba",
"bio": "Certified IPv6 newbie\n(The certifcation is real: I had to get certified to be allowed to post at https://forums.he.net )"
}
]
},
{
"intendedAudience": "Anyone interested in quantum computing with any role, such as developers, testers, managers, etc.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Quantum Computing (QC) is maturing to become a reality as a large amount of money worldwide is being poured into building quantum computers with varying technologies, such as superconducting, optics-based, and ion-trapping. Indeed, having functional, powerful, and error-free quantum computers together with relevant development platforms is essential. However, the breakthrough applications promised by QC, e.g., in medicine, healthcare, finance, and simulations discovery, finance, are empowered with quantum software. Therefore, there is a need for methods for efficient and intuitive development of quantum software for quantum computers to build QC applications. This lighting talk will start with a brief introduction to quantum computing, including the state of current quantum computers, QC platforms, programming languages, and development platforms. Next, the talk will dig deep into the current practice of programming quantum computers as low-level circuits. Subsequently, the talk will provide ideas on how the current practice can be improved to enable the masses to program quantum computers as opposed to the current practice where only specialists program quantum computers. In addition, the talk will present challenges in testing QC applications to ensure their correctness. Finally, the talk will provide a teaser of quantum software testing tools developed by our team. Such tools are the first in the world to automate the testing of quantum software.",
"title": "Quantum Computing - an army of Schrödinger's cats is heading our way",
"room": "Room 2",
"startTime": "2023-09-06T09:40",
"endTime": "2023-09-06T10:00",
"video": "861596110",
"startTimeZulu": "2023-09-06T07:40:00Z",
"endTimeZulu": "2023-09-06T08:00:00Z",
"id": "51500138-ad34-45e4-a6cc-19c586c79708",
"sessionId": "51500138-ad34-45e4-a6cc-19c586c79708",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T09:00",
"startSlotZulu": "2023-09-06T07:00:00Z",
"speakers": [
{
"name": "Noah Hegerland Oldfield",
"twitter": "@OldfieldHNoah",
"bio": "Noah Hegerland Oldfield is a Ph.D student at Simula Research Laboratory, Norway. Under the supervision of Shaukat Ali, he researches novel methods for testing quantum circuits by drawing upon his previous background in theoretical physics from the University of Oslo. His current research aims at exploiting quantum mechanical properties of quantum programs to perform more efficient and effective error discovery in ways not possible with classical programs. He has a passion for communicating science and particularly quantum computing in the hopes of increasing the general body of knowledge in the computer science community in preparation for the coming 3rd quantum revolution."
}
]
},
{
"intendedAudience": "Developers of any experience level.  Some Java experience and familiarity with Maven or containers is helpful but not required.",
"length": "240",
"format": "workshop",
"language": "en",
"abstract": "Java is a great programming language, however 'traditional' Java isn't so great to work with when it comes to modern, Cloud Native development. Quarkus is a (fairly) new Java stack that addresses issues such as the typical slow startup time and rather large memory usage that hinder the adoption of Java in container and/or Serverless workloads. Quarkus is not just useful for optimizing resource usage though. There is also a big focus on improving the developer experience.\n\nIn this session we'll take a look at how Quarkus is very easy to work with and allows developers to work with containers and external dependencies such as databases, Kafka clusters, Kubernetes etc without being experts in any of these technologies.  Not to mention that there is no need to manually recompile/redeploy to see any of these changes. \n\nAfter this session, the audience should come away with inspiration to build modern Cloud Native applications with Java and Quarkus, and have fun doing so!\n",
"title": "Quarkus Deep Dive",
"workshopPrerequisites": "We'll go through setup/signup instructions at the beginning of the lab. \n\nWindows users: You may need WSL2 and/or a VM with Linux or containerized environment for some of the advanced commands. If you do not have WSL enabled please install Podman Desktop (https://podman-desktop.io/docs/Installation/windows-install) or Docker Desktop beforehand.  \n\nLinux or MacOS users should be good to go. ",
"room": "Workshop A",
"startTime": "2023-09-05T09:00",
"endTime": "2023-09-05T13:00",
"registerLoc": "https://moosehead.javazone.no/#/register/quarkus_deep_dive",
"startTimeZulu": "2023-09-05T07:00:00Z",
"endTimeZulu": "2023-09-05T11:00:00Z",
"id": "ec146183-6412-4751-a484-bd09e3bfaade",
"sessionId": "ec146183-6412-4751-a484-bd09e3bfaade",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T09:00",
"startSlotZulu": "2023-09-05T07:00:00Z",
"speakers": [
{
"name": "Kevin Dubois",
"twitter": "@kevindubois",
"bio": "Kevin is a software engineer and developer advocate at Red Hat who is on a mission to supercharge developer joy and productivity using Open Source as the guiding light. He is a frequent conference speaker, talking mostly about Java, Quarkus and Cloud Native Development and Deployment practices. \nKevin previously worked as a (Lead) Software Engineer at a variety of organizations ranging from small startups to large US enterprises and even the Belgian public sector. \nIn his free time you can find Kevin somewhere in the wild hiking, gravel biking, snowboarding down mountains or packrafting (up and) down WW rivers."
}
]
},
{
"intendedAudience": "Spring Boot application developers ",
"length": "120",
"format": "workshop",
"language": "en",
"abstract": "Platform teams define Golden Paths to specify the standard way to create, build and deploy an application or library. These are usually applied using templates, such as Cookiecutter. This provides autonomy to stream aligned teams to customize them.\n\nAre you faced with upgrading dozens of services to Spring Boot 3? Want an easier way to apply all that you’ve seen at work? In this workshop we’ll explore automation to get your old Spring projects onto the new Spring Boot 3.1. And we’re not just changing version numbers; we’re actually changing code to adopt breaking changes.\n\nCurrently, there are several solutions to automatically upgrade the dependencies. However, in order to take advantage of the Spring API functionalities, it is necessary to also upgrade the source code, which usually conflicts with product priorities.\n\nIn this workshop, we will explore tooling to migrate to Spring Boot 3.1 and beyond, centered around OpenRewrite. Topics include OpenRewrite itself, writing migration recipes, Spring Tool Suite and Spring Boot Migrator. We’ll explore challenges and limitations along the way, such that you can migrate on your own.\n",
"title": "Spring Boot 3 is here. where are you?",
"workshopPrerequisites": "Intellij\n\nGradle\n\nJDK 17 and JDK 8\n\nMaven \n\nGitHub account\n\nGit",
"room": "Workshop A",
"startTime": "2023-09-05T13:30",
"endTime": "2023-09-05T15:30",
"registerLoc": "https://moosehead.javazone.no/#/register/spring_boot_3_is_here_where_are_you",
"startTimeZulu": "2023-09-05T11:30:00Z",
"endTimeZulu": "2023-09-05T13:30:00Z",
"id": "106f0d9f-3d6b-4131-be6b-36612dd29f97",
"sessionId": "106f0d9f-3d6b-4131-be6b-36612dd29f97",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T13:30",
"startSlotZulu": "2023-09-05T11:30:00Z",
"speakers": [
{
"name": "Raquel Pau",
"twitter": "@raquelpau",
"bio": "Raquel Pau is Engineering Manager at Moderne, which automates software refactoring at scale. She has extensive experience building developer tools (e.g walkmod) to improve the developer productivity. Previously, she worked as a growth product manager at CloudBees and Athenean, designing a new generation of developer tools. She also worked as a software engineer at Schibsted designing developer workflows to reduce the lead time for more than 1000 developers. She has also completed an EMBA at ESADE Business School.\n"
},
{
"name": "Tim te Beek",
"twitter": "",
"bio": "Tim te Beek is a staff software engineer at Moderne, which automates software refactoring at scale. He has extensive experience contributing to and presenting on Open Source software within the Java ecosystem. Previously he worked as a consultant specializing in migration engineering and developer productivity.\n"
}
]
},
{
"intendedAudience": "Any level / role should get something out of this workshop; some coding experience will help. \nThe workshop will provide a code example and some exercises to look at this code from looking at the structure of the code, to trying to summarize it's purpose. Attendees can learn ways to approach unfamiliar code, to think about what they notice and why, and learn from each other's perspective.",
"length": "120",
"format": "workshop",
"language": "en",
"abstract": "As developers, we spend a lot of time learning to write code, while spending little to no time learning to read code. Meanwhile, we often spend more time reading code than actually writing it. Shouldn't we be spending at least the same amount of time and effort improving this skill? Deliberate practice can help us get better at reading code. Learning how to better read and understand code, can in turn teach us what makes code readable. This might even help us to write code that is easier to read.\nIn this workshop we will practice our code reading skills by reading an unfamiliar piece of code (possibly in an unfamiliar language), using structured exercises. Participants will practice reading code, and take away knowledge about how they can continue to improve this important skill.\n",
"title": "Code reading workshop",
"workshopPrerequisites": "This workshop does not have any technical prerequisites.\nI will provide a printed version of code (1 A4 paper) which they can annotate on paper. \nIf desired, participants can bring a device with internet access to annotate the code digitally. This works only if the venue has wifi for participants)",
"room": "Workshop E",
"startTime": "2023-09-05T13:30",
"endTime": "2023-09-05T15:30",
"registerLoc": "https://moosehead.javazone.no/#/register/code_reading_workshop",
"startTimeZulu": "2023-09-05T11:30:00Z",
"endTimeZulu": "2023-09-05T13:30:00Z",
"id": "19170259-bb96-495e-851f-49b73f8e3fb3",
"sessionId": "19170259-bb96-495e-851f-49b73f8e3fb3",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T13:30",
"startSlotZulu": "2023-09-05T11:30:00Z",
"speakers": [
{
"name": "Marit van Dijk",
"twitter": "@MaritvanDijk77",
"bio": "Marit van Dijk is a software developer with 20 years of experience in different roles and companies. She loves building awesome software with amazing people and has contributed to open-source projects like Cucumber and various other projects. She enjoys learning new things as well as sharing knowledge on programming, test automation, Cucumber/BDD, and software engineering. She speaks at international conferences, in webinars, and on podcasts, occasionally writes blog posts, and contributed to the book \"97 Things Every Java Programmer Should Know\" (O'Reilly Media)."
}
]
},
{
"intendedAudience": "Denne workshopen er myntet på utviklere med lite eller ingen erfaring med Figma. Deltakere vil få innblikk i det grunnleggende i Figma, som de vil kunne bruke i arbeidet sitt, og for å samarbeide bedre med designere på teamet sitt.",
"length": "240",
"format": "workshop",
"language": "no",
"abstract": "Har du noen gang hatt lyst til å bli bedre kjent med Figma? Denne workshoppen er myntet på utviklere som har lite eller ingen kjennskap til verktøyet, men som gjerne vil bli bedre kjent med det. Du blir kjent med frames, components og prototyping for å nevne noe. I løpet av workshopen vil jeg også dele av mine erfaringer og fortelle om noen av mine snarveier til design med Figma. Vi vil jobbe i et figma-brett jeg har laget – hver deltaker får sin egen kopi som kan brukes som referanse etter konferansen.\n\nEtter workshoppen vil du være i stand til å lage skisser og prototyper, og du vil ha et innblikk i økosystemet til Figma.\n\nFigma er et verktøy for grensesnittdesign, der utviklere og designere kan samarbeide på designprosjekter i sanntid. Figma brukes av de fleste virksomheter som utvikler programvare, og omtales ofte som “Google Docs for design”.\n\nTa med egen PC. Jeg anbefaler å laste ned Figma på forhånd, men om nødvending kan det også brukes i nettleseren.",
"title": "Første flytur i Figma",
"workshopPrerequisites": "Ta med egen pc. Jeg anbefaler dog deltakere å laste ned Figma og lage en konto på forhånd. ",
"room": "Workshop C",
"startTime": "2023-09-05T09:00",
"endTime": "2023-09-05T13:00",
"registerLoc": "https://moosehead.javazone.no/#/register/forste_flytur_i_figma",
"startTimeZulu": "2023-09-05T07:00:00Z",
"endTimeZulu": "2023-09-05T11:00:00Z",
"id": "692e1e48-e67b-43cd-a71d-28112d13101a",
"sessionId": "692e1e48-e67b-43cd-a71d-28112d13101a",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T09:00",
"startSlotZulu": "2023-09-05T07:00:00Z",
"speakers": [
{
"name": "Andreas Wisle",
"twitter": "",
"bio": "Andreas er en dyktig designer, med høy kompetanse, spesielt rundt Universell utforming. Han er en god kommunikator, og liker å dele kunnskap med andre, i både formelle og uformelle settinger."
},
{
"name": "Vegard Hesselberg",
"twitter": "",
"bio": "Hei! Jeg heter Vegard Hesselberg og er født og oppvokst i Oslo. Jeg er utdannet på NTNU Trondheim, med bachelor i Informatikk og mastergrad i Interaksjonsdesign. Denne kombinasjonen hjelper meg stort i mitt arbeid, der jeg brenner for samarbeidet og samhandlingen mellom utvikler og designer.\n\nJeg er en relativt nyutdannet designer som begynte i Knowit høsten 2021. Jeg har tidligere holdt foredrag på interne seminarer om Figma og Innsiktssamling. Jeg har brukt Figma til designarbeid siden 2018 og har god kontroll over verktøyet."
}
]
},
{
"intendedAudience": "This talk is aimed at developers without any Rust experience.\nYou should have at least a little bit of programming experience (no matter the language)",
"length": "240",
"format": "workshop",
"language": "en",
"abstract": "Learn the basics of the Rust programming language by implementing the world's most iconic cellular automaton: Conway's Game of Life.\n\nThe Rust programming language is a modern high-level language that offers memory-safety and thread-safety guarantees, great tooling and extremely high performance.\nIt has repeatedly been voted the most loved programming language on Stack Overflow's developer survey and has been added as an officially supported language in Linux kernel development.\nIf you want to get in on the fun or just see what this language is all about, this workshop is for you!\n\nIn this workshop we will take a look at how to write Rust programs and how to build them using cargo.\nWe will explore Rust's approach to object-oriented programming and tackle some of its more unique features like the borrow-checker.\nThe goal is to get a good understanding of the basics of Rust and get some hands-on experience in using it.",
"title": "Learn Rust with Conway's Game of Life",
"workshopPrerequisites": "Bring a laptop with the following installed:\n - git\n - a code editor of your choice\nAnd preferably already installed:\n - rust tool-chain for your architecture",
"room": "Workshop D",
"startTime": "2023-09-05T09:00",
"endTime": "2023-09-05T13:00",
"registerLoc": "https://moosehead.javazone.no/#/register/learn_rust_with_conways_game_of_life",
"startTimeZulu": "2023-09-05T07:00:00Z",
"endTimeZulu": "2023-09-05T11:00:00Z",
"id": "fd163d3b-32da-4c99-9015-50e4ecfc4706",
"sessionId": "fd163d3b-32da-4c99-9015-50e4ecfc4706",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T09:00",
"startSlotZulu": "2023-09-05T07:00:00Z",
"speakers": [
{
"name": "Julian Philipp Merlin Ertel",
"twitter": "",
"bio": "Julian works as a senior consultant for Miles Bergen.\nHis main competence is C++ development, but over the years he has gathered experience in a variety of different languages and technologies.\nAs a developer he has mainly worked in the domains of video processing and seismic data acquisition.\nWhen given the option Julian's language of choice is Rust."
}
]
},
{
"intendedAudience": "Primarily directed at frontend developers.\n\nParticipants will learn how to add different levels of accessibility testing to their projects, and which tools work best for different types of errors.\n\nSome previous experience with frontend (React/NodeJS) development is beneficial.",
"length": "120",
"format": "workshop",
"language": "en",
"abstract": "In modern software development, with short sprints or even continuous deployment of code, time for manual testing can often be scarce. If we want to properly test for accessibility, we need to make every effort to automate what can be automated, so we can spend our time where it is most valuable.\n\nBy attending this workshop, you will get hands-on experience with different tools to test for accessibility. We will cover linting, unit testing and ui-/integration testing using Eslint, React Testing Library, Cypress and Axe. You will also learn about which types of tests you can't automate, and be introduced to some tools to aid your manual testing.\n\nThis workshop is best suited for developers and technical testers, or others who know how to run a Node.js application. In order to get the most out of the time at the workshop, it is beneficial to pre-install Git and Node beforehand.",
"title": "Continuous accessibility testing",
"workshopPrerequisites": "Participants should bring a laptop with Node, Git and their choice of IDE pre-installed.",
"room": "Workshop C",
"startTime": "2023-09-05T15:45",
"endTime": "2023-09-05T17:45",
"registerLoc": "https://moosehead.javazone.no/#/register/continuous_accessibility_testing",
"startTimeZulu": "2023-09-05T13:45:00Z",
"endTimeZulu": "2023-09-05T15:45:00Z",
"id": "605331d0-940a-4acf-8d01-5e84b062511f",
"sessionId": "605331d0-940a-4acf-8d01-5e84b062511f",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T15:45",
"startSlotZulu": "2023-09-05T13:45:00Z",
"speakers": [
{
"name": "Vegard Haugstvedt",
"twitter": "@it_vegard",
"bio": "An experienced frontend developer who has worked with accessibility for almost a decade, Vegard started working for NAV's design system team, Aksel, this year. There, he gets to work in the cross-section between frontend development, design and accessibility, just like he likes it!\n\nHe is a recurring speaker at several Norwegian conferences, primarily trying to spread the good word about accessibility, to help others learn how they can make their websites work for everyone. His career goal is to make the web a more accessible space."
}
]
},
{
"intendedAudience": "nothing specific other than being a Java developer",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Java 17 announced the depreciation of the Security Manager (which is ok since hardly anyone used it) but that doesn’t mean the JVM leaves you vulnerable.  There are many design features in the JVM and the JDK that are there to help keep your application safe from harm.  \n\nIn this session, we’ll walk through these points - from compiler, to bytecode to runtime and give you a refresher on how to get the best from these features.  We’ll also look at new things in the works, compile-to-native consequences and even some off-the-wall “it’s just an idea”  thoughts about how to make the JVM an even more secure environment.  ",
"title": "Hidden security features of the JVM - everything you didn’t know and more ",
"room": "Room 4",
"startTime": "2023-09-06T11:40",
"endTime": "2023-09-06T12:40",
"video": "861611326",
"startTimeZulu": "2023-09-06T09:40:00Z",
"endTimeZulu": "2023-09-06T10:40:00Z",
"id": "243609e5-4340-49bd-b2dc-9db5f6689146",
"sessionId": "243609e5-4340-49bd-b2dc-9db5f6689146",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T11:40",
"startSlotZulu": "2023-09-06T09:40:00Z",
"speakers": [
{
"name": "Steve Poole",
"twitter": "@spoole167",
"bio": "Developer Advocate, Security Champion, DevOps practitioner (whatever that means) Long time  Java developer, leader and evangelist. I’ve been working on  Java SDKs and JVMs since Java was less than 1.  JavaOne Rockstar,  JSR leader and representation, Committer on  open source projects including ones at Apache, Eclipse and OpenJDK.  A seasoned speaker and regular presenter at international conferences on technical and software engineering topics."
}
]
},
{
"intendedAudience": "Alle utviklere",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Som utviklere og teknologer opplever vi stadig mange forskjellig situasjoner på både jobb, hos kunder, og i livene våre. Vi opplever oppturer og nedturer både alene og sammen med andre mennesker. Noen ganger kan vi le og smile av det, andre ganger lar vi tiden gå i stillhet. Noen situasjoner er derimot spesielle; de setter oss i en posisjon der vi må velge en retning, en reaksjon eller et svar. Dette er en historie om mentoring, en historie om ledelse, og en historie om det å bry seg om folk. I lyntalen skal jeg dele litt om min erfaring som leder for utviklere, og hvorfor stillhet ikke alltid er det rette svaret. \n\nTalken handler om det å vokse som utvikler eller ta på seg lederansvar, og som en effekt av å vokse er det avgjørende å klare å håndtere den økte mengden med ansvar. Det vil være situasjoner når du vil føle deg alene, situasjoner der teamet er splittet, situasjoner der du bevitner alt fra urettferdighet, mobbing og dårlig holdninger\n\nØnsker du å vokse i din rolle må du kunne håndtere og prate om slike situasjoner. Når avstanden til dialog er mye lenger enn vanlig, er dialog fremdeles det beste verktøyet du har. Når dialog ikke virker, er fremdeles dialog det beste du har. Når heller ikke dette virker, må ansvarlighet og integritet bli til ord, for at endringer skal bli til handlinger.",
"title": "Fra utvikler til leder: En erfaringshistorie om ansvarlighet og integritet",
"startTime": "2023-09-07T10:20",
"endTime": "2023-09-07T10:40",
"room": "Room 2",
"video": "861983350",
"startTimeZulu": "2023-09-07T08:20:00Z",
"endTimeZulu": "2023-09-07T08:40:00Z",
"id": "28a1d79d-fdbd-482d-8e95-a7f2dd1cd6e2",
"sessionId": "28a1d79d-fdbd-482d-8e95-a7f2dd1cd6e2",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T10:20",
"startSlotZulu": "2023-09-07T08:20:00Z",
"speakers": [
{
"name": "Dervis Mansuroglu",
"twitter": "dervismn",
"bio": "Dervis is an experienced Java-developer, currently working for the Norwegian Labour and Welfare Administration. He is passionate about programming languages, functional programming and algorithms. Dervis is a Java Champion and the leader of the Norwegian JUG JavaBin (Dukes Choice Award winner in 2019). Dervis has spoken at several international conferences as well as being a regular speaker at local meetups in Norway."
}
]
},
{
"intendedAudience": "Engineers, architects and directors, that are making systems design choices on domain-driven low latency data platforms that can serve data to consumers to all types: transactional (application), search, analytics, and ML and soft AI.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "The Accord Consensus Protocol, providing global leaderless single-network-round-trip consensus using commodity clocks.\n\nResearch from University of Michigan & Apple Inc. introduces ACID-compliant, strict serialisable transactions possible globally at any scale, at high throughput, with low latency.\n\nMove over Paxos, ePaxos, Raft, Janus, Calvin, all now outdated consensus protocols, slow and fault intolerant.\n\nWe'll explain why your database must be leaderless, scalable & fast. The notion of copying data to analytics platforms is dead weight in the age of Data Mesh & Soft-AI. Your database can no longer be a single-process monolith from ages decades old if you are to be David against the Goliath of Big Tech. \n\nMick will explain hard problems in computer science solving elastic scalable ACID-compliant low-latency databases, explaining trade-offs from sharding data, shadow paging, sagas, to choosing eventual consistency & isolation consistency from the database. ",
"title": "The Accord Consensus Protocol, how to do ACID transactions at scale",
"room": "Room 1",
"startTime": "2023-09-07T10:20",
"endTime": "2023-09-07T11:20",
"video": "861981051",
"startTimeZulu": "2023-09-07T08:20:00Z",
"endTimeZulu": "2023-09-07T09:20:00Z",
"id": "c0ebbd09-9ff0-4307-891b-49f59e79c3e6",
"sessionId": "c0ebbd09-9ff0-4307-891b-49f59e79c3e6",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T10:20",
"startSlotZulu": "2023-09-07T08:20:00Z",
"speakers": [
{
"name": "Mick Semb Wever",
"twitter": "mck_sw",
"bio": "A 11-year Cassandra veteran, a member of the Cassandra Project Management Committee (PMC), this year's PMC Chair, and an Apache Member. Among the top 15 committers to the project. \n \nIn Norway I have worked at FINN.no, Sesam.no, and Bokklubben.\n\nBi-cultural with homes in both Australia and Norway.\n\nCurrently working at DataStax, as a Principal Architect and Technical Lead for Open Source. Our Open Source solutions: Apache Cassandra, Apache Pulsar, K8ssandra, Stargate, Astra, and Kaskada, provides real-time data platforms for AI-Driven Applications to the world's biggest clusters and global enterprises.\n\nI love engineering and coding, but nothing beats being helping others become successful. "
}
]
},
{
"intendedAudience": "Beginner to Intermediate. Some familiarity with the JVM/Java/Clojure is ideal.",
"length": "120",
"format": "workshop",
"language": "en",
"abstract": "Babashka is a native Clojure interpreter for scripting with fast startup. Its main goal is to leverage Clojure in places where you would be using scripting otherwise. By the end of the workshop you will know Babashka, it's ecosystem, and know when and how to use it. This is a great way to leverage our JVM experience in a scripting context and this workshop provides a practical way to learn and use it.",
"title": "Elegant and Maintainable scripting with Clojure and Babashka",
"workshopPrerequisites": "- Have the ability to pull repositories from GitHub\n- https://github.com/babashka/babashka#installation\n- https://clojure.org/guides/install_clojure\n- Use one of the popular supported editors like Neovim, VSCode, Emacs, IntelliJ etc.\n- Have a look at https://practical.li/clojure/clojure-editors/",
"room": "Workshop C",
"startTime": "2023-09-05T13:30",
"endTime": "2023-09-05T15:30",
"registerLoc": "https://moosehead.javazone.no/#/register/elegant_and_maintainable_scripting_with_clojure_and_babashka",
"startTimeZulu": "2023-09-05T11:30:00Z",
"endTimeZulu": "2023-09-05T13:30:00Z",
"id": "19a5cab3-7afd-4dc1-b60a-bea8562d3186",
"sessionId": "19a5cab3-7afd-4dc1-b60a-bea8562d3186",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T13:30",
"startSlotZulu": "2023-09-05T11:30:00Z",
"speakers": [
{
"name": "Anupriya Johari",
"twitter": "anupriyajo",
"bio": "I am software engineer by profession and a knowledge hungry researcher by heart. Over the years spending a lot of time in the industry and in and around the JVM, I can bring in the enterprise experiences and the need to have a robust scripting foundation and specially when its generally nowhere near what we are used to from the JVM."
},
{
"name": "Rahul De",
"twitter": "lispyclouds",
"bio": "I'm an SRE and developer dedicated to making simpler tools to bridge the gap between Dev and Ops. As one of the maintainers of the Babashka project and a die hard Clojure fan, I am fascinated to use its power and simplicity to make infra simple too; a place where it is rarely seen but can be of immense value. I love to organise and attend meetups and conferences around increasing diversity in tech, functional programming and food."
}
]
},
{
"intendedAudience": "Any level",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "If Log4Shell, Spring4Shell, etc. have taught us anything, it's that we need to keep our dependencies up to date. But updating our applications can take a lot of time. How do we stay on top of that, while also continuing to deliver business value?\nLuckily, there are plenty of tools that can help us with this, from package managers to bots that can automatically create changes on our repositories. Let's go over some of the different options, so we can make informed choices about what's best for us in a particular situation.",
"title": "Keep your dependencies in check",
"room": "Room 5",
"startTime": "2023-09-06T17:00",
"endTime": "2023-09-06T17:45",
"video": "861716570",
"startTimeZulu": "2023-09-06T15:00:00Z",
"endTimeZulu": "2023-09-06T15:45:00Z",
"id": "b8e3b5e1-8313-409b-8f6f-037a402c97a2",
"sessionId": "b8e3b5e1-8313-409b-8f6f-037a402c97a2",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T17:00",
"startSlotZulu": "2023-09-06T15:00:00Z",
"speakers": [
{
"name": "Marit van Dijk",
"twitter": "@MaritvanDijk77",
"bio": "Marit van Dijk is a software developer with 20 years of experience in different roles and companies. She loves building awesome software with amazing people and has contributed to open-source projects like Cucumber and various other projects. She enjoys learning new things as well as sharing knowledge on programming, test automation, Cucumber/BDD, and software engineering. She speaks at international conferences, in webinars, and on podcasts, occasionally writes blog posts, and contributed to the book \"97 Things Every Java Programmer Should Know\" (O'Reilly Media)."
}
]
},
{
"intendedAudience": "Anyone interested in the latest developments in AI",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "With ChatGPT taking center stage since the beginning of 2023, developers who have not had a chance to work with any forms of Artificial Intelligence or Machine Learning systems may find themselves either intrigued by the “maze” of new terminologies, or some may be eager to learn more, while perhaps a smaller group may not actually want to get themselves into a territory that's unknown to them.\n\nThe truth is that, whether we like it or not, we have all been “thrust” into this new era of computing. Instead of procrastinating, let's start by learning about Generative AI specifically with this presentation. We will go over the history and evolution of AI and ML, then look at how it has evolved to where it is today. We will touch upon as many new concepts that have popped up in the last 6-9 months, which include: Generative AI (GenAI), ChatGPT, Large Language Models (LLMs), Natural Language Processing (NLP), Vector DB, and the growing importance of Vector Search. We will also point out the new operational concerns when it comes to managing the life-cycle of a machine learning environment. We will then look at a demo on how Vector Search is being done behind the scenes. We will discuss the benefits of this new wave of technology as well as the challenges that it brings to the industry and the marketplace.",
"title": "Enter the Brave New World of GenAI with Vector Search",
"room": "Room 3",
"startTime": "2023-09-06T13:00",
"endTime": "2023-09-06T14:00",
"video": "861681521",
"startTimeZulu": "2023-09-06T11:00:00Z",
"endTimeZulu": "2023-09-06T12:00:00Z",
"id": "e87fe57f-b55e-4864-b5cd-482e50b21ab9",
"sessionId": "e87fe57f-b55e-4864-b5cd-482e50b21ab9",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T13:00",
"startSlotZulu": "2023-09-06T11:00:00Z",
"speakers": [
{
"name": "Mary Grygleski",
"twitter": "mgrygles",
"bio": "Streaming Developer Advocate at DataStax, Java Champion, President of Chicago-JUG\n\nMary is a Java Champion and a passionate Senior Developer Advocate at DataStax, a leading data management company that champions Open Source software and specializes in Big Data, DB-as-a-service, Streaming, and Cloud-Native systems. She spent 3.5 years as a very effective advocate at IBM, focusing on Java, Jakarta EE, OpenJ9, Open Source, Cloud, and Distributed Systems. She transitioned from Unix/C to Java around 2000 and has never looked back since then. She considers herself a polyglot and loves to continue learning new and better ways to solve real-life problems. She is an active tech community builder outside of her day job, and currently the President of the Chicago Java Users Group (CJUG), as well as a co-organizer for several IBM-sponsored meetup groups in the Greater Chicago area."
}
]
},
{
"intendedAudience": "Developers/DevOps engineers interested in CI/CD automation with GitHub Actions",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "It's been said that automating oneself out of a job is one of the best qualities of a software engineer. When it comes to building and releasing software that statement becomes more relevant than ever. GitHub is a popular choice for hosting code and since a few years ago offers a native CI/CD solution: GitHub Actions. They are a powerful tool that's worth adding to your development toolbox but getting started might be a bit daunting. In this session we'll cover the basics of working with GitHub Actions and their structure, moving into more advanced topics such as inputs/outputs, job dependencies, reusable workflows, composite actions, and secrets.",
"title": "Lights, Camera, GitHub Actions!",
"room": "Room 7",
"startTime": "2023-09-07T13:00",
"endTime": "2023-09-07T14:00",
"video": "862027491",
"startTimeZulu": "2023-09-07T11:00:00Z",
"endTimeZulu": "2023-09-07T12:00:00Z",
"id": "6dbf2d97-e4bc-4dbb-8546-60948be7c3e1",
"sessionId": "6dbf2d97-e4bc-4dbb-8546-60948be7c3e1",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T13:00",
"startSlotZulu": "2023-09-07T11:00:00Z",
"speakers": [
{
"name": "Andres Almiray",
"twitter": "aalmiray",
"bio": "Andres is a Java/Groovy developer and a Java Champion with more than 2 decades of experience in software design and development. Andres is a true believer in Open Source."
},
{
"name": "Ixchel Ruiz",
"twitter": "",
"bio": "Ixchel Ruiz has developed software applications and tools since 2000. Her research interests include Java, dynamic languages, client-side technologies, DevSecOps, and testing. Ixchel travels around the world (sometimes virtually) sharing knowledge. "
}
]
},
{
"intendedAudience": "Developers, managers, business strategists and anyone who enjoys a good rant.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "There's a good chance that you - yes you - are currently working on making a \"new platform\" of some kind. I have lost count of how many \"new platforms\" I've been asked to help out making. \nWhy are we always working on these \"new platforms\"? Why is there always something so fundamentally wrong with the old ones that we need a whole new platform? Do they end up delivering the value they promise? Value for whom?  The users of the software? The business owners? The project managers? The developers? Who are we making these platforms for? \nLet's get to the bottom of this, and figure out who's problems we should be solving, and who's money we should be saving and how we can make these platforms worth their investment. ",
"title": "Who needs a platform?",
"room": "Room 6",
"startTime": "2023-09-06T09:00",
"endTime": "2023-09-06T09:45",
"video": "861598414",
"startTimeZulu": "2023-09-06T07:00:00Z",
"endTimeZulu": "2023-09-06T07:45:00Z",
"id": "ba595f56-5e3f-437d-b1f2-85bcb90f4cf0",
"sessionId": "ba595f56-5e3f-437d-b1f2-85bcb90f4cf0",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T09:00",
"startSlotZulu": "2023-09-06T07:00:00Z",
"speakers": [
{
"name": "Christin Gorman",
"twitter": "@christingorman",
"bio": "Christin is an experienced public speaker who has worked as a developer for more than 20 years.  She is currently working as a contractor/consultant at Kodemaker.  \nHer enthusiasm for software development is catching and her talks are generally seen as both entertaining and thought provoking."
}
]
},
{
"intendedAudience": "Intermediate audience\nattendee will learn the characteristics of each JVM GC, how to choose and how to tune depending on their workload/applications",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "OpenJDK brings GCs with different characteristics and not always easy to understand. Those GCs allows the JVM to adapt to different workloads in terms of latency or throughput. I will explain how to tame those beasts and how to take advantage of them to improve your applications and resources.",
"title": "Mastering GC: tame the beast and make it your best ally",
"room": "Room 1",
"startTime": "2023-09-06T09:00",
"endTime": "2023-09-06T10:00",
"video": "861586790",
"startTimeZulu": "2023-09-06T07:00:00Z",
"endTimeZulu": "2023-09-06T08:00:00Z",
"id": "ae187257-582d-4041-aba7-52ca4ff13d93",
"sessionId": "ae187257-582d-4041-aba7-52ca4ff13d93",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T09:00",
"startSlotZulu": "2023-09-06T07:00:00Z",
"speakers": [
{
"name": "Jean-Philippe Bempel",
"twitter": "@jpbempel",
"bio": "Developer and Java Champion passionate by performance, runtimes (JVM, CLR) and Mechanical Sympathy supporter, Jean-Philippe has more than 8 years experience in low latency trading systems. After He brings his expertise on the JVM at Criteo in order to optimize resources on thousand node clusters, He is now at Datadog to evaluate & improve both profiler & tracer agent. He is also committer on the OpenJDK's project JDK Mission Control."
}
]
},
{
"intendedAudience": "Developers or designers interested in Spatial Computing, Virtual Reality, Augmented Reality, Mixed Reality, XR and Metaverse concepts.\nNo required experience.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "What is the state of Mixed Reality in 2023, and how did we get here? \n\nStarting with an accelerated history of Virtual Reality and Augmented Reality, we will look at the current state of the art of Mixed Reality in terms of hardware and software, and have a look forward to the near future to see what is just around the corner for this relatively new medium.\n\nAs Apple (Apple Vision Pro), Meta (Meta Quest 3), and Microsoft (HoloLens 2) carve out their own respective visions for the future of Mixed Reality, it's never been a more important time to stop and take a look at how far the technology has come, and where it might take us in the future. ",
"title": "Mixed Reality check",
"room": "Room 2",
"startTime": "2023-09-06T09:00",
"endTime": "2023-09-06T09:20",
"video": "861590477",
"startTimeZulu": "2023-09-06T07:00:00Z",
"endTimeZulu": "2023-09-06T07:20:00Z",
"id": "4f42bd4a-4905-4a7f-8551-f5ad25034698",
"sessionId": "4f42bd4a-4905-4a7f-8551-f5ad25034698",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T09:00",
"startSlotZulu": "2023-09-06T07:00:00Z",
"speakers": [
{
"name": "Scott Leaman",
"twitter": "",
"bio": "Scott Leaman is one of Europe's foremost experts in Mixed Reality (MR) consulting, a Microsoft MVP for Mixed Reality, and the Mixed Reality Lead for Sopra Steria Norge. He has been using VR/AR/MR to bring value to the enterprise domain for many years, based on a background of 18 years IT consulting experience in Australia, America and Europe. He has been building software for the Microsoft HoloLens since 2015, and has long experience with development, design and project management for Mixed Reality projects. "
}
]
},
{
"intendedAudience": "Er du interessert i maskinlæring og kunstig intelligens? Liker du å grave deg ned i ny teknologi og test ut nye løsninger? Bli med og lær litt av vår reise der vi tester ut en objektgjennkjenningsmodell fra \"The TensorFlow Model Garden\" på Vegvesenets webkamerabilder",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Bli med på en liten reise sammen med Statens vegvesen der vi har prøvd ut en maskinlæringsmodell for å telle kjøretøy. Vi ser på de praktise utfordringene som dukker opp når en skal prøve å lage en publikumstjeneste av dette. Hvordan ser teknologien ut under panseret og hvilke problemer må løses for å kunne gi verdi til publikum. Hvor robuste tjenester kan en lage med denne teknologien?",
"title": "Å telle kjøretøy i webkamerabilder med maskinlæring, hvor vanskelig kan det egentlig være?",
"room": "Room 3",
"startTime": "2023-09-06T10:20",
"endTime": "2023-09-06T11:05",
"video": "861594919",
"startTimeZulu": "2023-09-06T08:20:00Z",
"endTimeZulu": "2023-09-06T09:05:00Z",
"id": "a64780e5-3009-4597-9010-d841d7c6c58f",
"sessionId": "a64780e5-3009-4597-9010-d841d7c6c58f",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T10:20",
"startSlotZulu": "2023-09-06T08:20:00Z",
"speakers": [
{
"name": "Hans Petter H. Bauhr",
"twitter": "",
"bio": "Jeg er en senior utvikler og har jobbet med programvareutvikling i 22 år. Jeg har jobbet i mange roller og flere ulike domener, men jeg trives best når jeg kan være med å utvikle nye løsninger, lære ny teknologi og å få bryne meg på nye og vanskelige ting ;-)\n\nI de siste årene har det blitt mye Kubernetes (verktøystøtte-team på intern skyplatform), API-er (backend i java) og maskinlæring (objektgjenkjenning)\n\nJeg bor i Solbergelva i Drammen og har bodd her på \"Østlandet\"  de siste 22 årene. Jeg er født og oppvokst på Nordmøre og migrerte fra Masterstudiet i Kybernetikk på NTNU til Kongsberg (Kongsberg Defence & Aerospace). Deretter ble det 7 år i Oslo som konsulent i Ergogroup, Acando og Steria. I de 9 siste årene har jeg vært i Statens vegvesen i Drammen (som nå er det nye hovedsete for IT-divisjonen i Vegvesenet :-) )"
}
]
},
{
"intendedAudience": "Alle som jobber med digitalisering",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Desentralisert identitet er et paradigmskifte innen elektronisk identitet. Den fjerner de sentraliserte aspektene ved dagens ID-løsninger og gir brukeren i full kontroll over sin egen identitet. Desentralisert identitet løser dagens problemer med ID-tyveri, persondata på avveie og manglende brukerkontroll. Samtidig åpner det for helt nye anvendelser av eID og skaper nye forretningsmodeller. I dette foredraget ser vi på de sentrale konseptene innen desentralisert identitet og digitale lommebøker. Jeg snakker om hvordan vi kan ta i bruk digitale lommebøker og teknologien som gjør det mulig.",
"title": "Desentralisert identitet - elektronisk ID gjort riktig",
"startTime": "2023-09-06T09:00",
"endTime": "2023-09-06T10:00",
"room": "Room 5",
"video": "861593997",
"startTimeZulu": "2023-09-06T07:00:00Z",
"endTimeZulu": "2023-09-06T08:00:00Z",
"id": "335e1a55-b8a7-4fd7-ae64-5e3ebc331ae6",
"sessionId": "335e1a55-b8a7-4fd7-ae64-5e3ebc331ae6",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T09:00",
"startSlotZulu": "2023-09-06T07:00:00Z",
"speakers": [
{
"name": "Erling Skard",
"twitter": "",
"bio": "Erling Skard er seniorkonsulent og medeier i Kantega. Han er en erfaren utvikler og arkitekt og en ekspert på elektronisk identitet. Erling er ansvarlig for Kantegas strategi for digitale lommebøker"
}
]
},
{
"intendedAudience": "The talk does not assume any experience in any particular technology or paradigm. In fact, there are tales of people being introduced to (functional) programming through live coding music, so really any background is welcome.\n\nPeople interested in playing with technology, creative computation, music composition, functional programming, or anything in between will likely be the ones benefiting most from this talk.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "The talk will introduce the audience to live coding music in Haskell through the TidalCycles framework. The goal is to demonstrate how functional programming can be applied to make music in untraditional ways, and to introduce the perspective that music-making is a model for understanding function composition. The talk will also briefly touch on the Algorave movement, events where people dance to music generated on the fly by algorithms by performers on stage.\n\nThe talk aims to motivate that learning through playing is valuable, and that there are interesting connections between concepts and patterns in the musical domain and in the more traditional software development domain.",
"title": "Music can be functions too: an introduction to live-coding in Haskell",
"room": "Room 2",
"startTime": "2023-09-06T09:20",
"endTime": "2023-09-06T09:40",
"video": "861593988",
"startTimeZulu": "2023-09-06T07:20:00Z",
"endTimeZulu": "2023-09-06T07:40:00Z",
"id": "a697c07e-62b6-47d4-abd4-4af43a8750e0",
"sessionId": "a697c07e-62b6-47d4-abd4-4af43a8750e0",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T09:00",
"startSlotZulu": "2023-09-06T07:00:00Z",
"speakers": [
{
"name": "Ulrik Antoniussen Halmøy",
"twitter": "",
"bio": "Ulrik is a software developer, currently working on algorithmic journalism in NTB. He has a background in music, and enjoys working with code in a variety of  mediums."
}
]
},
{
"intendedAudience": "Everyone",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "To make sense of the world, we rely on our brains' capability to form fictions that we call \"categories\" of things and experiences. This capability is both automatic and hidden: we can't avoid doing it, yet we don't know exactly how we do it. We know that differences and similarities play a role, but how? When we try to be more deliberate about the process, for instance because we want to write software based on our categories, we call it modelling. In the process, we tend to replace our intuitive but ill-defined common-sense categories with more precise technical categories. But precision comes at a cost. In this talk, we'll look at different perspectives on categorization, see that nothing remains the same for long, and that edge cases are just regular cases that got unlucky.",
"title": "Modelling vs Reality",
"room": "Room 7",
"startTime": "2023-09-06T14:20",
"endTime": "2023-09-06T15:20",
"video": "861668497",
"startTimeZulu": "2023-09-06T12:20:00Z",
"endTimeZulu": "2023-09-06T13:20:00Z",
"id": "149df097-f90d-45b7-9601-0fa02a541719",
"sessionId": "149df097-f90d-45b7-9601-0fa02a541719",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T14:20",
"startSlotZulu": "2023-09-06T12:20:00Z",
"speakers": [
{
"name": "Einar W. Høst",
"twitter": "@einarwh",
"bio": "Einar W. Høst has been a software developer for a long time. He enjoys collaborative modelling, API design and computer programming. He is working as a socio-technical facilitator at the Norwegian Labour and Welfare Administration."
}
]
},
{
"intendedAudience": "Anyone who works with frontend web programming should benefit from this talk. Not a lot of experience is required, but well seasoned developers should also find the ideas in the talk of interest. Participants will be introduced to some new ways to approach component-driven UI development, and learn to enumerate the pros and cons of different approaches.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "React put component-driven UIs on the agenda a decade ago. Since then my approach to web development has become increasingly more data-driven to the point where my UI code is almost entirely stateless. I will show you how to successfully build data-driven components and what they're good at, and we'll explore the possibilities that open up when our UIs stop being so darn clever.",
"title": "Stateless, Data-driven UIs",
"room": "Room 5",
"startTime": "2023-09-06T10:20",
"endTime": "2023-09-06T11:05",
"video": "861600197",
"startTimeZulu": "2023-09-06T08:20:00Z",
"endTimeZulu": "2023-09-06T09:05:00Z",
"id": "85f23370-440f-42b5-bf50-4cb811fef44d",
"sessionId": "85f23370-440f-42b5-bf50-4cb811fef44d",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T10:20",
"startSlotZulu": "2023-09-06T08:20:00Z",
"speakers": [
{
"name": "Christian Johansen",
"twitter": "cjno",
"bio": "I am a jack-of-all trades programmer with more than 20 years of experience. I've done web development since pure CSS table-less designs were cutting edge, I've written my share of JavaScript (and a book about testing it), Ruby, and for the past 10 years, Clojure and ClojureScript. I've done infrastructure, backends, integrations, web frontends and native apps. I love it all, and I particularly enjoy working across the entire stack."
}
]
},
{
"intendedAudience": "Developers and architects.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Norwegian Air Shuttle ASA is Norway's largest airline, and the fourth largest low-cost carrier in Europe. Every year millions of customers travel with Norwegian Air Shuttle. This ambitious company has been exploring innovative payment system for years to boost revenues and improve customer experiences. \n\nJoin this talk to learn how we built custom settlement system to settle millions payment transactions of Norwegian Air Shuttle customers and how we achieve scalability and reliability by using AWS serverless technologies and which challenges serverless technologies help to solve and at the same time how it enables future possibilities for data engineering and advanced analysis.",
"title": "How We Settle Millions Of Payment Transactions At Norwegian Air",
"room": "Room 2",
"startTime": "2023-09-06T13:00",
"endTime": "2023-09-06T13:20",
"video": "861676504",
"startTimeZulu": "2023-09-06T11:00:00Z",
"endTimeZulu": "2023-09-06T11:20:00Z",
"id": "d74c4264-9ac6-4b79-9801-0eec65ea7a5b",
"sessionId": "d74c4264-9ac6-4b79-9801-0eec65ea7a5b",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T13:00",
"startSlotZulu": "2023-09-06T11:00:00Z",
"speakers": [
{
"name": "Dapeng Han",
"twitter": "",
"bio": "Dapeng Han is a senior consultant from Webstep. He is mainly working with system development, but also has interest in AI/machine learning, cloud and finance domain knowledge. He has been working with different domains in finance industry, e.g. pension, payment, settlement, reconciliation. Apart from work, he likes playing innebandy and power lifting."
}
]
},
{
"intendedAudience": "people who like building server-side applications with java",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": " Bootiful Spring Boot 3\nSpring Framework 6 and Spring Boot 3 are here, and you know what that means. New baselines and new possibilities! Spring Framework implies a Java 17 and Jakarta EE baseline and offers new support for building GraalVM-native images and a compile-time component model in the new Spring AOT engine. It also offers a new observability layer, declarative HTTP and RSocket clients, preliminary Project Loom and CRaC support, Problem-Details support, and so much more. Join me, Spring Developer Advocate Josh Long (@starbuxman), and we'll look at next-gen Spring.\n\n",
"title": "Bootiful Spring Boot 3",
"room": "Room 6",
"startTime": "2023-09-06T14:20",
"endTime": "2023-09-06T15:20",
"video": "861664924",
"startTimeZulu": "2023-09-06T12:20:00Z",
"endTimeZulu": "2023-09-06T13:20:00Z",
"id": "334dd20b-e11b-4981-8a7b-b02886b35ade",
"sessionId": "334dd20b-e11b-4981-8a7b-b02886b35ade",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T14:20",
"startSlotZulu": "2023-09-06T12:20:00Z",
"speakers": [
{
"name": "Josh Long",
"twitter": "starbuxman",
"bio": "Josh (@starbuxman) has been the first Spring Developer Advocate since 2010. Josh is a Java Champion, author of 6 books (including \"Reactive Spring\") and numerous best-selling video training (including \"Building Microservices with Spring Boot Livelessons\" with Spring Boot co-founder Phil Webb), and an open-source contributor (Spring Boot, Spring Integration, Spring Cloud, Activiti and Vaadin, etc), a Youtuber (\"Coffee + Software with Josh Long\" as well as my Spring Tips series ), and a podcaster (\"A Bootiful Podcast\")."
}
]
},
{
"intendedAudience": "Everyone interested in understanding how the field of AI probably will develop, in a broader perspective :-)",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Evolusjonsteoriske innsikter har vært sentrale i utviklingen av kunstig intelligens. Men kan evolusjonsteori hjelpe oss å tenke litt klarere på hva som kommer til å skje rundt kunstig intelligens i årene fremover?\nEvolusjonsbiologen kommer med fem overraskende spådommer. ",
"title": "Evolusjonsbiologens fem spådommer om kunstig intelligens",
"room": "Room 3",
"startTime": "2023-09-06T11:40",
"endTime": "2023-09-06T12:40",
"video": "861653811",
"startTimeZulu": "2023-09-06T09:40:00Z",
"endTimeZulu": "2023-09-06T10:40:00Z",
"id": "a1bb624e-017d-4bd1-933c-6081e4fe32ab",
"sessionId": "a1bb624e-017d-4bd1-933c-6081e4fe32ab",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T11:40",
"startSlotZulu": "2023-09-06T09:40:00Z",
"speakers": [
{
"name": "Jens Andreas Huseby",
"twitter": "",
"bio": "Jens Andreas Huseby jobber i Bekk Consulting med digital produkt- og forretningsutvikling. Han er utdannet evolusjonsbiolog og har deltatt som ekspert i flere programmer på TV og radio; blant annet i Trygdekontoret, Brille, Ekko og Gift ved første blikk. Han skrev om \"evolusjonsteknologi\" (genetiske algoritmer) i 2005 og har fulgt med på modningen av kunstig intelligens siden 1995."
}
]
},
{
"intendedAudience": "Talk is very relevant for developers, data engineers and architects. All are welcome.\nThe audience will learn many unique and special aspects of cloud native architectures including best of class design principles and design patterns.\n",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "In my session, I will take the audience on a special and unique journey across the Cloud native architectural landscape.\nI will introduce them to the design principles underlying cloud native architectures.\nI will also tell them about the various design patterns that are generally used.\n",
"title": "Mastering the Skies: Navigating Cloud Native Architecture with Design Principles and Patterns",
"room": "Room 2",
"startTime": "2023-09-06T12:10",
"endTime": "2023-09-06T12:30",
"video": "861649905",
"startTimeZulu": "2023-09-06T10:10:00Z",
"endTimeZulu": "2023-09-06T10:30:00Z",
"id": "3e5d4f56-196c-4bb5-9b9f-cc03347a1b9c",
"sessionId": "3e5d4f56-196c-4bb5-9b9f-cc03347a1b9c",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T11:40",
"startSlotZulu": "2023-09-06T09:40:00Z",
"speakers": [
{
"name": "Soumitra Bhattacharya",
"twitter": "",
"bio": "Soumitra Bhattacharya is Head of Software Architecture at Capra Consulting AS.\nHe is also their AWS and Kotlin community lead.\nHe has a passion for internet, architecture, quantum computing and AWS and helps customers worldwide on AWS related projects.\nHe loves to share his expertise with people.\nDuring his stellar career he has worked for companies like Opera Software, Onecall and Cxense.\nHobbies include reading and meditation.\n"
}
]
},
{
"intendedAudience": "Just about all Java developers use the collections framework, so anyone with minimal experience of Java programming should benefit from this talk.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "The classes of the Java Collections Framework are arguably the most heavily used data structures on the planet. In their 25 year lifetime, they’ve become part of the everyday vocabulary of millions of programmers. But, actually, how good are they? And how have they changed, along with the changing priorities of the language?\n\nTo answer these questions, this session will preview some of the material in the upcoming second edition of Java Generics and Collections: the new SequencedXxx interfaces, use cases for the unmodifiable collections of Java 9, best practices for using the framework, and a design retrospective distilling the experience of a quarter-century’s use of the Framework and comparing it with alternatives like Eclipse Collections and Guava. You should leave this session with a better understanding of the design forces that shaped the Framework and an improved understanding of how to get the very best out of it.",
"title": "Return to Planet Collections",
"room": "Room 4",
"startTime": "2023-09-06T13:00",
"endTime": "2023-09-06T14:00",
"video": "861643448",
"startTimeZulu": "2023-09-06T11:00:00Z",
"endTimeZulu": "2023-09-06T12:00:00Z",
"id": "7841a7ec-2f37-49d2-8321-b7559055226c",
"sessionId": "7841a7ec-2f37-49d2-8321-b7559055226c",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T13:00",
"startSlotZulu": "2023-09-06T11:00:00Z",
"speakers": [
{
"name": "Maurice Naftalin",
"twitter": "@mauricenaftalin",
"bio": "Five decades working in IT:  developer, designer, architect, manager, teacher, and author. Working with Java since 1.0. Author, Mastering Lambdas; co-author, Java Generics and Collections (2e in preparation). Disorganises the unconference JAlba. Java Champion, Oracle Ace Pro. Speaks at conferences a lot.\n"
}
]
},
{
"intendedAudience": "Developers working with Elasticsearch or any other full-text search engine. The topic could be of interest to both beginners interested in performance tuning and to mid-level or senior engineers trying to optimize their systems.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Elasticsearch is a full-text search engine rather than your typical database. This means its query performance can be surprising at times. In this talk I will discuss what happens when Elastic handles a query, and from this I will derive general rules about what kinds of queries Elastic can handle efficiently and with which it will struggle. Since the underlying principles are very similar, these hints should mostly be applicable to Solr or any other full-text search engine just as well as to Elasticsearch.",
"title": "Elasticsearch Performance",
"room": "Room 5",
"startTime": "2023-09-06T13:00",
"endTime": "2023-09-06T14:00",
"video": "861641786",
"startTimeZulu": "2023-09-06T11:00:00Z",
"endTimeZulu": "2023-09-06T12:00:00Z",
"id": "4bfde3bb-3c1e-479a-9f19-e0fdd6fcebb7",
"sessionId": "4bfde3bb-3c1e-479a-9f19-e0fdd6fcebb7",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T13:00",
"startSlotZulu": "2023-09-06T11:00:00Z",
"speakers": [
{
"name": "Michał Kosmulski",
"twitter": "mkosmul",
"bio": "Software Engineer turned Team Leader, turned Software Engineer again. I work mostly with Kotlin, and I'm interested in distributed systems and high performance. Editor-in-chief of company tech blog at https://blog.allegro.tech/"
}
]
},
{
"intendedAudience": "Passer for alle, litt tekniske detaljer ",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Det siste året har interessen for NRK.no økt blant utenlandske stormakter. Selv om det er hyggelig å være populær kan det noen ganger bli litt for mye. I denne presentasjonen skal vi gå gjennom hvordan DDoS-angrep oppleves hos NRK.no og hvordan vi prøver å minske konsekvensene når angrepene skjer. Vi skal også se at ikke alle angrep er fiendtlige eller kommer utenifra.  ",
"title": "Ikke alle angrep er slemme: DDoS innenfra og utenfra ",
"room": "Room 1",
"startTime": "2023-09-06T11:40",
"endTime": "2023-09-06T12:25",
"video": "861641166",
"startTimeZulu": "2023-09-06T09:40:00Z",
"endTimeZulu": "2023-09-06T10:25:00Z",
"id": "75bcefdc-0f1c-45c7-a4e7-54c720f46c0f",
"sessionId": "75bcefdc-0f1c-45c7-a4e7-54c720f46c0f",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T11:40",
"startSlotZulu": "2023-09-06T09:40:00Z",
"speakers": [
{
"name": "Ingrid G. Guren",
"twitter": "",
"bio": "Ingrid er tech lead for Javascript på NRK.no, og har kjørt disaster recovery av forsida klokka tre om natta."
},
{
"name": "John Arne S. Pedersen",
"twitter": "",
"bio": "John Arne er tech lead for Java på NRK.no, og var på vakt da MGP gikk ned i 2020."
}
]
},
{
"intendedAudience": "This is for Java developers who want to learn how to monitor their applications with OpenTelemetry and Grafana.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "OpenTelemetry is a collection of standards and tools to make it easy to get metrics, distributed traces, and logs out of applications. For example, OpenTelemetry's Java agent will instrument Java applications out-of-the-box, with no code change required.\n\nThis talk shows how to use these signals for application monitoring. We will introduce Grafana's open source databases: Loki for logs, Tempo for traces, and Mimir for metrics. And we will show how to use Grafana to explore the telemetry data for an example application running on Kubernetes.\n\nGrafana and Prometheus metrics have been popular among platform engineers for monitoring Kubernetes clusters for a long time. \n\nthis talk will show how application developers can benefit from Grafana as well, using open standards like OpenTelemetry, and open source monitoring backends like Loki, Tempo, and Mimir.",
"title": "Application Monitoring with Grafana and OpenTelemetry",
"room": "Room 6",
"startTime": "2023-09-06T10:20",
"endTime": "2023-09-06T11:05",
"video": "861623888",
"startTimeZulu": "2023-09-06T08:20:00Z",
"endTimeZulu": "2023-09-06T09:05:00Z",
"id": "fa0469ab-f13c-417d-bf68-d7818ed02aa3",
"sessionId": "fa0469ab-f13c-417d-bf68-d7818ed02aa3",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T10:20",
"startSlotZulu": "2023-09-06T08:20:00Z",
"speakers": [
{
"name": "Fabian Stäber",
"twitter": "",
"bio": "Dr. Fabian Stäber is engineering manager and monitoring enthusiast at Grafana. He is a member of the Prometheus open source project, where he is maintainer of the Prometheus Java client library and the JMX exporter. At Grafana Fabian has his focus on application monitoring with OpenTelemetry."
}
]
},
{
"intendedAudience": "Dette er beregnet på systemarkitekter, ledende utviklere og andre som designer tekniske løsninger",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "I Tolletaten har vi mange interessenter på utsiden, og mange løsninger av ulike generasjoner på innsiden. Erfaring har vist oss at ulike partnere har ulike teknologipreferanser, ulik endringstakt og -vilje, og behov som ikke nødvendigvis går i takt med våre egne. Dermed har vi prøvd å få til et isolasjonslag mellom ulike teknologisfærer – eller et anti-corruption layer om man er oppvokst med domenedrevet design.",
"title": "Teknologisirkus med mange tilskuere",
"room": "Room 2",
"startTime": "2023-09-06T10:50",
"endTime": "2023-09-06T11:10",
"video": "861621233",
"startTimeZulu": "2023-09-06T08:50:00Z",
"endTimeZulu": "2023-09-06T09:10:00Z",
"id": "7017e9f6-74af-40e1-b966-7a8815e5ffe3",
"sessionId": "7017e9f6-74af-40e1-b966-7a8815e5ffe3",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T10:20",
"startSlotZulu": "2023-09-06T08:20:00Z",
"speakers": [
{
"name": "Lars Reed",
"twitter": "",
"bio": "Arkitekt med over 30 års erfaring i IT-bransjen, ikke rent få av dem i Tolletaten. Tidvis teknologisirkusdirektør og tidvis digital vaktmester."
}
]
},
{
"intendedAudience": "Dette foredraget passer for alle som er interessert i CI, CD eller pull requests. Om dere bruker Pull requests i dag på teamet deres, så er det kanskje spesielt interessant for å høre hvorfor dette ikke nødvendigvis gir bedre kodekvalitet. Og er du allerede litt skeptisk, så kanskje jeg kan hjelpe med noen argumenter for å jobbe på andre måter.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Pull requests brukes mer og mer, og har nå nærmest blitt standard i dagens utviklingsteam. De fleste sier også at de har Continuous Integration, samt at de har eller ønsker Continuous Delivery og Continuous Deployment. Pull Requests og Continuous Integration jobber mot hverandre og det er i praksis er umulig å ha begge deler. Jeg skal snakke om fordeler og ulemper det ene gir i forhold til det andre og hvorfor du bør ta et bevisst valg rundt hva som gir verdi i ditt team og ikke bare velge “begge deler” fordi det er \"Best Practice\". Jeg mener Pull Requests for de fleste team er en dårlig idé og at de fleste team bør velge andre måter å gjøre code reviews og sikre kvaliteten på kodebasen. ",
"title": "Pull Requests eller Continuous Integration, du kan bare velge én!",
"room": "Room 2",
"startTime": "2023-09-06T10:40",
"endTime": "2023-09-06T10:50",
"video": "861619774",
"startTimeZulu": "2023-09-06T08:40:00Z",
"endTimeZulu": "2023-09-06T08:50:00Z",
"id": "dac6f6ba-f66a-47af-96cc-0a2bac60b1f6",
"sessionId": "dac6f6ba-f66a-47af-96cc-0a2bac60b1f6",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T10:20",
"startSlotZulu": "2023-09-06T08:20:00Z",
"speakers": [
{
"name": "Gaute Magnussen",
"twitter": "@gautema",
"bio": "Gaute er en erfaren utvikler med mer enn 20 års erfaring med fullstack-utvikling. Han har jobbet med blant annet .NET, Elixir, Ruby, Javascript og Typescript i titalls prosjekter og sett mange former og variasjoner av team og smidige prosesser. Gaute jobber nå som CTO i Boitano."
}
]
},
{
"intendedAudience": "Utviklar, Ops",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Standardisert sporing på tvers av java applikasjonar med minimalt vedlikehold? Møt opp her.\nMe fortel kort om kva distribuert tracing er og w3c standarden for trace context med døme frå ny arkitektur i ID-porten. Me har skrive om ID-porten frå monolitt til microservice arkitektur og me måtte ha tracing. \nFokuset vil vera på korleis me teknisk løyste dette med OpenTelemetry og ulike hinder på vegen. \nDu vil høyre om OpenTelemetry java-agent og korleis ein applikasjon importerer denne via OpenTelemetry operator for Kubernetes. Her får du eit godt utgongspunkt for å ta i bruk tracing i dag, frå javakode til køyrande applikasjon i Kubernetes. ",
"title": "Distribuert tracing i ID-porten med OpenTelemetry",
"room": "Room 2",
"startTime": "2023-09-06T10:20",
"endTime": "2023-09-06T10:40",
"video": "861616555",
"startTimeZulu": "2023-09-06T08:20:00Z",
"endTimeZulu": "2023-09-06T08:40:00Z",
"id": "dafefe51-de36-490a-962e-e535a145de4f",
"sessionId": "dafefe51-de36-490a-962e-e535a145de4f",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T10:20",
"startSlotZulu": "2023-09-06T08:20:00Z",
"speakers": [
{
"name": "Randi Øyri",
"twitter": "",
"bio": "Randi jobbar med ID-porten i Digdir og har vore utviklar i snart fleire ti-år. Har drive med logging og sporing sporadisk med ulike verktøy og teknologiar i mange år, men vil eigentleg berre at det skal funke fint og vera enkelt å bruke!"
},
{
"name": "Bjørn-Erik Strand",
"twitter": "",
"bio": "Bjørn-Erik brenn for å sjå smilande utviklarar i friksjonslaus samhandling med teknologi og prosess. Har jobba med dette som mål i snart 10 år. Mest erfaring fra operations via Evry, men no litt tettare på utvikling via plattformteam i Digdir."
}
]
},
{
"intendedAudience": "Developer, Architect",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Developers have had extraordinary experiences and taken stunning capabilities while they were developing Java microservices from local to the cloud using Quarkus for the past 4 years. But there is more to come with their journey since we have just arrived in Quarkus 3 which provides good, better, and even excellent features and capabilities in terms of developer experience, performance, scalability, and cloud integration. Especially, Quarkus 3 simplifies asynchronous concurrent applications using virtual threads (Project Loom) for high scalability. \nIn this session, we take you through how Quarkus integrates Loom for developers to make concurrent applications easier, have cheaper memory, and have high performance using virtual threads. You can also learn about what’s new in Quarkus 3 such as JakartaEE 10, MicroProfile 6, Hibernate ORM 6, and more.",
"title": "Quarkus 3: The Road to Loom for Cheaper, Faster, and Easier Concurrent Applications",
"room": "Room 7",
"startTime": "2023-09-06T11:40",
"endTime": "2023-09-06T12:40",
"video": "861614422",
"startTimeZulu": "2023-09-06T09:40:00Z",
"endTimeZulu": "2023-09-06T10:40:00Z",
"id": "808094c6-0893-4d9d-8fe2-533c6442c9b8",
"sessionId": "808094c6-0893-4d9d-8fe2-533c6442c9b8",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T11:40",
"startSlotZulu": "2023-09-06T09:40:00Z",
"speakers": [
{
"name": "Daniel Oh",
"twitter": "danieloh30",
"bio": "Daniel Oh is a Java Champion and Senior Principal Developer Advocate at Red Hat. He works to evangelize building cloud-native microservices and serverless functions with cloud-native runtimes to developers. He also continues to contribute to various open-source cloud projects and ecosystems as a Cloud Native Computing Foundation (CNCF) ambassador for accelerating DevOps adoption in enterprises. Daniel also speaks at technical seminars, workshops, and meetups to elaborate on new emerging technologies for enterprise developers, SREs, platform engineers, and DevOps teams."
}
]
},
{
"intendedAudience": "You will learn from scratch how the Lightning Network works as an off-chain add-on to Bitcoin. No previous Bitcoin, blockchain or distributed ledger technology knowledge required.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "In January of 2009 the Bitcoin network saw the light of day. The Bitcoin network is many things, one of which is a challenger to the established money and payment system. However, the Bitcoin network has a capacity of approx. 7 transactions per second. In comparison, the VISA network claims to have a capacity of 65,000+ transactions per second.\n\nMany solutions to this low capacity problem have been proposed and tried: amendments to the Bitcoin protocol, launching of new blockchains, and off-chain solutions. The Lightning Network is an off-chain solution. It lets actors operate with transactions not all recorded on the blockchain.\n\nIn this talk we'll start off with the basics of the Bitcoin network. Then we'll explain how the Lightning Network is a solution to the scaling problem and how in detail the network works.",
"title": "Money at the Speed of Lightning",
"room": "Room 5",
"startTime": "2023-09-06T11:40",
"endTime": "2023-09-06T12:40",
"video": "861613193",
"startTimeZulu": "2023-09-06T09:40:00Z",
"endTimeZulu": "2023-09-06T10:40:00Z",
"id": "987aacd9-c334-4c56-9fb1-ee0b32e9dcae",
"sessionId": "987aacd9-c334-4c56-9fb1-ee0b32e9dcae",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T11:40",
"startSlotZulu": "2023-09-06T09:40:00Z",
"speakers": [
{
"name": "Jørn K Baltzersen",
"twitter": "@jkBaltzersen",
"bio": "Jorn K Baltzersen has been a professional developer for more than two decades. He is also an experienced speaker and trainer from conferences and other events in Norway and abroad. He recently graduated with the distinction of summa cum laude from the MSc program in blockchain and digital currency at the University of Nicosia. He is a Principal Solution Consultant at Tietoevry."
}
]
},
{
"intendedAudience": "Denne kosetimen passer for alle. Man trenger ikke kunne noe om programmering.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Bli med på en vill og underholdende reise inn i de mer betente delene av open source-miljøet! Etter en lang dag med faglige presentasjoner, kan du ta med deg noe leskende drikke og litt snacks for en saftig skravletime. Vi går gjennom noen av de mest juicy konfliktene som har oppstått blant utviklere i open source-verdenen. Historier om alt fra spektakulære GitHub-angrep, OCD-krangler om kodestil, \"shitstorms\" og drapstrusler, til opphetet diskusjon om tekanner. Men kan vi lære noe av alt det her? Jeg tror det. Kanskje.",
"title": "Tid for popcorn: de heteste dramaene i open-source-miljøet",
"room": "Room 4",
"startTime": "2023-09-06T18:20",
"endTime": "2023-09-06T19:05",
"video": "861734362",
"startTimeZulu": "2023-09-06T16:20:00Z",
"endTimeZulu": "2023-09-06T17:05:00Z",
"id": "d2c69806-66f2-4211-9950-9574348af14e",
"sessionId": "d2c69806-66f2-4211-9950-9574348af14e",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T18:20",
"startSlotZulu": "2023-09-06T16:20:00Z",
"speakers": [
{
"name": "Eirik Sletteberg",
"twitter": "eirikobo",
"bio": "Eirik er en allsidig utvikler som for tiden jobber frilans i sitt eget AS. Til vanlig er han en stort sett seriøs utvikler, som nå for tiden jobber mest med frontend. Gjennom karrieren har han vært borti alt fra React, TypeScript, JSP, Kotlin, Java, Linux, til Kubernetes og Docker. Han foretrekker indentering med fire mellomrom.\n\nNår han ikke skriver kode, jobber han ofte som musiker, og spiller konserter med klassisk eller eksperimentell musikk rundt omkring. Han spiller obo (derav Twitter-brukernavnet)."
}
]
},
{
"intendedAudience": "Denne presentasjonen treffer alle som bruker smarttelefon, og etter å ha sett og lagt merke til hvor mange som faktisk er suuuper avhengig uten å ville innrømme det selv, tror jeg at vi alle trenger en liten påminnelse på at livet skjer nå, over skjermen. Vi blir regelrett lurt av de store selskapene som lager produktene og systemene vi bruker hver dag, og vi vet det ikke selv en gang.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "I en verden hvor det meste skjer på digitale plattformer er det lett å bli avhengig av det vi bruker mest, nemlig smarttelefonene våre. Vi bruker timesvis hver dag på apper som gir hjernen vår litt ekstra dopamin, men er vi egentlig klar over skaden vi påfører oss selv? Trenger vi egentlig å vise til alle på sosiale medier at vi er på konsert? Eller at vi er ute og spiser middag på en dyr restaurant?\nJeg ønsker å gjøre dere oppmerksomme på hvor avhengige vi er, samt prøve å forklare hvorfor dette er skadelig for oss, hvordan det oppleves for barn og unge i dag, og hva de store selskapene har gjort for å lure oss inn i den fella vi sitter i i dag. Jeg skal forklare metoder og teknikker som du lett kan ta i bruk og du skal få høre mine 3 beste tips til hvordan dette kan gjøres i praksis.",
"title": "Legg vekk telefonen!",
"room": "Room 2",
"startTime": "2023-09-06T17:00",
"endTime": "2023-09-06T17:20",
"video": "861732616",
"startTimeZulu": "2023-09-06T15:00:00Z",
"endTimeZulu": "2023-09-06T15:20:00Z",
"id": "b98ceb1b-e015-45eb-b1de-3593bea3fff4",
"sessionId": "b98ceb1b-e015-45eb-b1de-3593bea3fff4",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T17:00",
"startSlotZulu": "2023-09-06T15:00:00Z",
"speakers": [
{
"name": "Tania Holst",
"twitter": "HolstTania",
"bio": "Tania har jobbet som frontendutvikler for interne systemer i NAV siden 2019, og elsker å bidra til at veilederne får en enklere hverdag ved å utforme gode systemer til dem. \nPå fritiden er hun opptatt med samboer, hund og trening, og prøver å holde seg oppdatert på alt fra Marvel, Star Wars og resten av det Disney eier, i tillegg til å lære seg italiensk som en unnskyldning for å reise til Italia for å spise pasta og pizza."
}
]
},
{
"intendedAudience": "Ingen forkunnskaper kreves. Du vil forhåpentligvis få lyst til å bygge ditt eget tastatur!",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Siden skrivemaskinens opprinnelse har designet preget tastaturene vi bruker selv i 2022. I denne presentasjonen vil jeg fortelle om hvorfor og hvordan jeg gikk vekk fra QWERTY og piltaster, og endte opp med å bygge mitt eget tastatur med bare 36 taster.",
"title": "Ergosplit og hersk: fra QWERTY til selvbygget tastatur",
"room": "Room 2",
"startTime": "2023-09-06T17:30",
"endTime": "2023-09-06T17:50",
"video": "861730271",
"startTimeZulu": "2023-09-06T15:30:00Z",
"endTimeZulu": "2023-09-06T15:50:00Z",
"id": "7577776c-c098-4699-9044-6e2367cd0802",
"sessionId": "7577776c-c098-4699-9044-6e2367cd0802",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T17:00",
"startSlotZulu": "2023-09-06T15:00:00Z",
"speakers": [
{
"name": "Mats Faugli",
"twitter": "",
"bio": "Erfaren utvikler med lidenskap for hjemmelagde mekaniske tastaturer, linux og progrock."
}
]
},
{
"intendedAudience": "Ledere som vil maksimere verdiskaping og glede i organisasjonen, utviklere og arkitekter som vil lære noe nytt, eller de som synes arkitektrollen er forvirrende og unødvendig kan tjene godt av å bli med. Det hjelper om man har erfaring fra industrien og kjenner til autonome team, accelerate boken, eller domain driven design konsepter. ",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Ta del i en innsiktsfull presentasjon med Magnus, der han utforsker IT-utviklingen i Norge og hvordan \"Arkitekt 1.0\" ble til. Han belyser også hvorfor denne rollen har endret seg og blitt presset ut over tid, i takt med at innovasjoner som autonome, tverrfaglige team har blitt mer fremtredende. Magnus viser til hvorfor slike team fungerer, men belyser samtidig hvilke utfordringer som følger dem. \"Arkitekt 2.0\" er en oppdatert arkitektrolle som er designet for å utnytte og styrke de fremskrittene vi nyter daglig. Lær hvordan din organisasjon kan implementere denne nye oppdateringen og gjøre jobben mer givende for alle mens man leverer enda mer verdi.",
"title": "Arkitekt 2.0 - En oppdatert arkitekt for en oppdatert verden",
"room": "Room 1",
"startTime": "2023-09-06T17:00",
"endTime": "2023-09-06T17:45",
"video": "861725688",
"startTimeZulu": "2023-09-06T15:00:00Z",
"endTimeZulu": "2023-09-06T15:45:00Z",
"id": "f2395670-fd16-4689-a644-6e4df50c4361",
"sessionId": "f2395670-fd16-4689-a644-6e4df50c4361",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T17:00",
"startSlotZulu": "2023-09-06T15:00:00Z",
"speakers": [
{
"name": "Magnus Elden",
"twitter": "",
"bio": "Fra irritasjon til inspirasjon, Magnus har gått en lang vei i sin arkitekturkarriere. I løpet av de siste fem årene har han dykket dypt inn i dyre og komplekse prosjekter, jobbet med sensitive data og ledet tverrfaglige, autonome team. Denne erfaringen har gitt ham et nytt perspektiv på hvordan en arkitekt i dagens raske teknologiske landskap kan videre styrke moderne team-orienterte organisasjoner. Magnus har utviklet en ny, fremtidsrettet forståelse for arkitektrollen, og ser stort potensiale i å kombinere gode team med innsiktsfulle arkitekter."
}
]
},
{
"intendedAudience": "This talk is meant to be intractive. The idea is to give the audience some ideas on what they should think about before committing to implementing a Service Mesh-based architecture. I wrote an article about this (https://medium.com/me/stats/post/1a44abdeea31) and also recorded some podcasts and videos on the same topics. This would be my first time adapting the content into a conference format.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Service Mesh is becoming a key component in the Cloud Native world. It allows teams to connect, secure, and observe complex microservices environments built on containers and container orchestration tools. But most Service Mesh tools are also very complex and require a lot of engineering overhead to deploy and maintain. In this talk, we will explore the considerations you have to take into account before you commit to a Service Mesh-based stack. I had the opportunity to help customers design around Istio (one of the most famous Mesh tools) and learned a lot through the years. This talk is a distilled experience of the learnings I had.\n",
"title": "You probably DON’T need a Service Mesh!",
"room": "Room 4",
"startTime": "2023-09-06T17:00",
"endTime": "2023-09-06T18:00",
"video": "861721087",
"startTimeZulu": "2023-09-06T15:00:00Z",
"endTimeZulu": "2023-09-06T16:00:00Z",
"id": "99e52eec-ccd5-4e1d-8bb2-e02ba713812a",
"sessionId": "99e52eec-ccd5-4e1d-8bb2-e02ba713812a",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T17:00",
"startSlotZulu": "2023-09-06T15:00:00Z",
"speakers": [
{
"name": "Abdel Sghiouar",
"twitter": "",
"bio": "Abdel Sghiouar is a senior Cloud Developer Advocate @Google Cloud. His focused areas are GKE/Kubernetes, Service Mesh and Serverless. Abdel started his career in datacenters and infrastructure in Morocco where he is originally. Before moving to Google's largest EU datacenter in Belgium. Then in Sweden he joined Google Cloud Professional Services and spent 5 years working with Google Cloud customers on architecting and designing large scale distributed systems before turning to advocacy and community work.\n"
}
]
},
{
"intendedAudience": "Developer, Architects who are new to Domain Driven Design (DDD)  or know little bit about DDD",
"length": "240",
"format": "workshop",
"language": "en",
"abstract": "This Hands-on workshop would help with learning Domain-Driven Design fundamentals and how to apply them for better microservices architecture. We will also cover how to decide on boundaries of microservices and how/when to split microservices using Domain-Driven Design.\n\nWhy Domain-Driven Design?\n\nDDD is very useful for designing scalable systems and platforms and the basis for designing better microservices architecture. On the data side, DDD is useful for building distributed data architectures including data mesh. DDD also helps with organizing a large team.\n\nThis one-day workshop will help you learn how to use domain-driven design to split domain and to design better microservice-style architectures. You’ll learn the DDD vocabulary and strategic patterns. Throughout the workshop, you’ll complete exercises in which the name of the domain-driven design concept is initially hidden until you code the problem statement, and then it will be revealed. Using simple coding use cases, you’ll learn DDD from the bottom-up to understand how current and existing code bases can be refactored to use DDD.\n\nYou will learn -\n\nThe relationship between DDD and microservices\n\nTactical and strategic DDD patterns\n\nDomain model as the primary and most important layer in three-tier/three-layer and hexagonal architecture\n\nDomain events, which are the basis for event-driven architecture\n\nAnd you will be able to -\n\nCome up with a microservice using the DDD aggregate concept\n\nDesign and evolve better boundaries for microservices using DDD\n\nUnderstand the domain layer and its importance in the overall success of a project\n\nApply DDD tactical patterns to existing code bases\n\nTopics Covered :\n\nThe what/why/when of domain-driven design; domain-driven design and friends\n\nWhat is a domain layer in the context of 3-tier/3-layer and hexagonal architecture?\n\nHands-on exercises- Problems 1 , 2 and showcase\n\nHands-on exercises- Problems 3 ,4, 5 and showcase\n\nDDD Concept 1\n\nHands-on exercises- Problems 6 and showcase\n\nDDD Concept 2\n\nHands-on exercises- Problems 7 and showcase\n\nDDD Concept 3\n\nHands-on exercises- Problems 8 and showcase\n\nDDD Concept 4\n\nHands-on exercises- Problems 9 & 10 and showcase\n\nDDD Concept 5\n\nRelationship to microservices architecture. Demo of microservice code using DDD. Covers when to split microservices. Sagas, microservices communication patterns.\n\nHands-on exercises- Problems 11 and showcase. Demo of microservice code\n\nDDD Concept 6\n\nRelationship to microservices architecture. Demo of microservice code using DDD\n\nDDD Concept 7 & 8\n\nRelationship to team topology,etc.\n\nModular monolith\n\nBrief on Event sourcing, CQRS,\n\nBrief on Event storming and Domain Storytelling - Technique to find bounded contexts.\n\nBrief on Data Mesh and its relation to Domain-Driven Design.",
"title": "Domain-Driven Design and microservices Hands-on workshop",
"workshopPrerequisites": "A computer set up with the IDE of your choice e.g IntelliJ IDEA community edition, VC Code or other. You can use any programming language for the workshop (Java, C#, Python, Ruby, Kotlin, Scala, Javascript, TypeScript, Go). Please create a repository for the language of your choice before joining the live event . You should preferably have 4+ years of software development experience (such as Java, C#, JavaScript, TypeScript, Kotlin, Scala, Ruby, Python, Go, etc.)\n\n( Please see sample repository for Java/gradle) \nhttps://github.com/ddd-workshop-org/ddd-sample-uc1\n\nPlease try to write code for the following four code problems before a workshop in the programming language of your choice. This workshop uses Object Oriented Paradigm for Domain Driven Design, so please use Object Oriented  programming.\n\nCode Problem 1\n Add a “Apple Pencil” to a Cart \n Note:  Please do not create a User class.  Please do not create ProductCategory, Variant, Colour, etc classes.\nSample implementation:\nhttps://github.com/ddd-workshop-org/ddd-sample-uc1\n\nCode Problem 2\nAdd a “Sony Wireless headphone” to a Cart\nhttps://github.com/ddd-workshop-org/ddd-sample-uc2\n\nCode Problem 3\nAdd 2 quantities of “Apple Pencil” to a Cart. \nhttps://github.com/ddd-workshop-org/ddd-sample-uc3\n\nCode Problem 4\nRemove already added Item “Apple Pencil” (with all its quantities) from Cart.\nhttps://github.com/ddd-workshop-org/ddd-sample-uc4",
"startTime": "2023-09-05T09:00",
"endTime": "2023-09-05T13:00",
"room": "Workshop B",
"registerLoc": "https://moosehead.javazone.no/#/register/domaindriven_design_and_microservices_handson_workshop",
"startTimeZulu": "2023-09-05T07:00:00Z",
"endTimeZulu": "2023-09-05T11:00:00Z",
"id": "b88ac7b7-4036-4609-9886-69480ae16064",
"sessionId": "b88ac7b7-4036-4609-9886-69480ae16064",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T09:00",
"startSlotZulu": "2023-09-05T07:00:00Z",
"speakers": [
{
"name": "Sandeep Jagtap",
"twitter": "sandeep_jagtap",
"bio": "Sandeep Jagtap is principal consultant for Thoughtworks India, where he has worked for 16 years. He’s a proponent, enthusiast, and practitioner of domain-driven design, with experience in building scalable systems using DDD, event sourcing and CQRS. During his 24 years of experience in the industry, he’s also served in roles such as developer, tech lead, tech principal, and architect. Please see LinkedIn profile for details https://www.linkedin.com/in/sandeepsjagtap/"
}
]
},
{
"intendedAudience": "Denne lyntalen er relevant for alle som bruker eller vurderer å bruke 1Password, og spesielt for de som jobber med hemmeligheter og/eller SSH-nøkler. Det kan også være relevant for brukere av andre passordbehandlere som har lignende funksjonalitet.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Lagrer du bare passord i 1Password? Det er på tide å utnytte verktøyet til det fulle! I denne presentasjonen viser jeg deg noen av de offisielle, men lite kjente, måtene 1Password kan hjelpe deg som utvikler.\n​\nMed praktiske eksempler viser jeg deg hvordan du kan bruke 1Password til å:\n​\n- dele SSH-nøkler mellom maskiner uten å måtte kopiere dem manuelt\n- låse opp SSH-nøkler med fingeravtrykk i stedet for passord\n- bruke hemmeligheter i kommandolinjen og konfigurasjonsfiler\n- dele hemmeligheter med teamet ditt\n​\nDisse teknikkene kan spare en enkelt utvikler for mye tid. Men når hele teamet benytter seg av dem, kan man spare enda mer – spesielt ved håndtering av hemmeligheter og onboarding av nye teammedlemmer. Ikke bare vil du jobbe mer effektivt, men også øke sikkerheten ved å bruke 1Password til å administrere sensitive data.\n​\nPresentasjonen passer for utviklere på alle erfaringsnivåer, uansett om man bruker Mac, Windows eller Linux. Bli med og lær hvordan du kan ta i bruk 1Password for å øke produktiviteten og sikkerheten i arbeidet ditt!",
"title": "Utnytt 1Password til det fulle som utvikler",
"room": "Room 2",
"startTime": "2023-09-06T15:40",
"endTime": "2023-09-06T16:00",
"video": "861720630",
"startTimeZulu": "2023-09-06T13:40:00Z",
"endTimeZulu": "2023-09-06T14:00:00Z",
"id": "ccbe6b82-d110-41af-8c1c-3e91d607fd6a",
"sessionId": "ccbe6b82-d110-41af-8c1c-3e91d607fd6a",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T15:40",
"startSlotZulu": "2023-09-06T13:40:00Z",
"speakers": [
{
"name": "Henrik Dæhli",
"twitter": "",
"bio": "Henrik er en erfaren konsulent med ekspertise innen utvikling. Han har en ekstra interesse innen sikkerhet. Henrik trives med å samarbeide tett med forretningssiden for å finne de beste løsningene og hans erfaring spenner fra små, til store samfunnskritiske systemer."
}
]
},
{
"intendedAudience": "Utviklere og team som er ansvarlige for utvikling og forvaltning av applikasjoner og løsninger som bruker npm-pakker som en del av løsningen, enten de brukes i byggetrinn eller i produksjon. ",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Forrest Gump sa engang: \"Livet er som npm install - du vet ikke alltid hva du får\". Neida. Men de siste årene har npm stadig oftere hatt utfordringer med sikkerhetshendelser som viser oss at det å ta i bruk npm-pakker kan være en risikosport! Hvordan kan vi som utviklere sikre oss? Hvordan kan vi skape større trygghet når vi installerer nye pakker eller oppgraderer eksisterende?\n\nTerje snakker om konkrete utfordringer, mekanismer, verktøy og rutiner i tillegg til god praksis som gjør at utviklere og team som bruker npm i applikasjonene sine kan sikre seg så godt det lar seg gjøre.",
"title": "NPM - lettvint, helt til det smeller!",
"room": "Room 2",
"startTime": "2023-09-06T16:10",
"endTime": "2023-09-06T16:30",
"video": "861718527",
"startTimeZulu": "2023-09-06T14:10:00Z",
"endTimeZulu": "2023-09-06T14:30:00Z",
"id": "5abca95f-3176-44e3-8e18-a014f5214e83",
"sessionId": "5abca95f-3176-44e3-8e18-a014f5214e83",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T15:40",
"startSlotZulu": "2023-09-06T13:40:00Z",
"speakers": [
{
"name": "Terje Karlsen",
"twitter": "",
"bio": "Terje har jobbet som utvikler med fokus på frontend og web-basert teknologi i 20 år. I den tiden har han sett både teknologi og metoder utvikle seg og selv opplevd hvor tidkrevende og usikkert håndtering av avhengigheter i store applikasjoner kan være."
}
]
},
{
"intendedAudience": "All front end developers or people interested in frontend development",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "The last couple of years, the frontend community has gone from spitting out single page applications with huge JavaScript bundles to yet again embracing the server model, with server side rendering, server components and static site generation. \n\nBut what's the right tool for you? What are the tradeoffs, both in complexity, user experience and developer experience? This talk will take you on a tour through history, the current landscape – and of course – give you the answer to what you should choose.",
"title": "The battle of the frontend frameworks",
"room": "Room 6",
"startTime": "2023-09-06T17:00",
"endTime": "2023-09-06T17:45",
"video": "861717284",
"startTimeZulu": "2023-09-06T15:00:00Z",
"endTimeZulu": "2023-09-06T15:45:00Z",
"id": "25478e42-4f96-4d63-9829-7a82746da67e",
"sessionId": "25478e42-4f96-4d63-9829-7a82746da67e",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T17:00",
"startSlotZulu": "2023-09-06T15:00:00Z",
"speakers": [
{
"name": "Kristofer Giltvedt Selbekk",
"twitter": "selbekk",
"bio": "Kristofer is an experienced frontend developer, React advocate and a father of two. He's been using lots of different frameworks through his 10 years of experience, and is looking forward to telling you a bit about his takeaways."
}
]
},
{
"intendedAudience": "Utviklere både med og uten erfaring",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "I tidligere Javazone iterasjoner har der vært enkelte innlegg som lot til å argumentere for at man burde være hyggelig og holde en kollegial tone når man forfatter en Pull Request (PR) review. Fordelene med dette er ting som at det kan bidra til bedre arbeidsmiljø og at kolleger blir tryggere på seg selv og hverandre. I denne lyntalen ønsker jeg å illustrere, via egen erfaring, at PR reviews oftere enn ikke er for hyggelige og at dette kan gå utover kvaliteten på en kodebase. Jeg vil gjøre dette ved å vise kontrasten mellom PR review og den akademiske peer review prosessen. Jeg vil vise at det er store likheter i formålet mellom disse to prosessene, men at resultatet kan divergere noe når det kommer til resultatet av disse prosessene. Videre vil jeg fremsette en hypotese om at hyggelig tone og korte tilbakemeldinger i PR utgjør en tapt mulighet for å lage enda bedre kode. Den vil jeg så underbygge med nøye utvalgte eksempler på både PR og akademisk peer reviews.",
"title": "Er våre Pull Request reviews altfor hyggelige?",
"room": "Room 2",
"startTime": "2023-09-06T16:00",
"endTime": "2023-09-06T16:10",
"video": "861717023",
"startTimeZulu": "2023-09-06T14:00:00Z",
"endTimeZulu": "2023-09-06T14:10:00Z",
"id": "e58dc48f-aee0-4349-b176-34b083f7b5bc",
"sessionId": "e58dc48f-aee0-4349-b176-34b083f7b5bc",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T15:40",
"startSlotZulu": "2023-09-06T13:40:00Z",
"speakers": [
{
"name": "Kent Inge Fagerland Simonsen",
"twitter": "",
"bio": "Kent Inge har 20 års erfaring som programvareutvikler og forsker innen samme emne. Han har dermed mottatt tilstrekkelig med både Pull Request og akademiske peer reviews til å kunne ane likheter og noen systematiske forskjeller."
}
]
},
{
"intendedAudience": "Developers familiar with Maven",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Apache Maven is an ubiquitous build tool in the Java ecosystem, some even claim it's the defacto standard build tool. Configuring Maven is deceptively simple, after all it's just a matter of writing XML, isn't it? Things look differently when the rubber meets the road. One must know the intricacies of the build lifecycle; how plugins, goals (mojos), and phases come together; rules for dependency resolution; configuration inheritance between parent - child POM files; enhancing the build with profiles; and more. These features may trip you over if the rules that govern them are unclear. We'll present a series of scenarios to test your knowledge on Maven rules. we guarantee you'll leave this session with a few bits of new information and better understanding of the Maven build tool.",
"title": "Maven Puzzlers",
"room": "Room 4",
"startTime": "2023-09-06T15:40",
"endTime": "2023-09-06T16:40",
"video": "861707277",
"startTimeZulu": "2023-09-06T13:40:00Z",
"endTimeZulu": "2023-09-06T14:40:00Z",
"id": "ca873dbe-7389-4ee7-a71f-7327860106c7",
"sessionId": "ca873dbe-7389-4ee7-a71f-7327860106c7",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T15:40",
"startSlotZulu": "2023-09-06T13:40:00Z",
"speakers": [
{
"name": "Andres Almiray",
"twitter": "@aalmiray",
"bio": "Andres is a Java/Groovy developer and a Java Champion with more than 2 decades of experience in software design and development. Andres is a true believer in Open Source."
},
{
"name": "Ixchel Ruiz",
"twitter": "@ixchelruiz",
"bio": "Ixchel Ruiz has developed software applications and tools since 2000. Her research interests include Java, dynamic languages, client-side technologies, DevSecOps, and testing.Ixchel travels around the world (sometimes virtually) sharing knowledge. "
}
]
},
{
"intendedAudience": "Java programmers working on web backend applications with or without the Spring framework.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Does Java backend development need to require lots of magic formulas and automatic behavior to be simple?\n\n20 years ago Java development (J2EE) was dominated by complex application servers and Enterprise Java Beans (EJBs) which were cumbersome to use. In response, the Spring framework was launched with the motto \"J2EE without EJBs\". But all cures become the new disease and even though it simplified the development experience at the time, as the Spring framework has become more mature and more popular, it has also become more complex and started obscuring the task.\n\nDeveloping Java backend solutions require you to understand issues of packaging and deployment, configuration, request routing, transactions and connection management and security. Popular frameworks seek to \"deal with this for you\", but unless you understand what the frameworks do, you will create hard to test, buggy and insecure applications anyway.\n\nThere are alternatives with fewer dependencies that don't really involve more coding or more learning, but that makes the resulting application easier to understand and the code easier to navigate.\n\nIn this talk, I will demonstrate how to code for the technologies I use to set up applications that let me develop with twice the speed of a Spring programmer. Along the way, we will organize and understand the parts that go into creating any Java web backend application with any framework on a containerized platform",
"title": "Java EE simplified without Spring",
"room": "Room 6",
"startTime": "2023-09-06T15:40",
"endTime": "2023-09-06T16:40",
"video": "861706245",
"startTimeZulu": "2023-09-06T13:40:00Z",
"endTimeZulu": "2023-09-06T14:40:00Z",
"id": "09a1ecb3-1a44-4c9c-a998-5e83909a5e45",
"sessionId": "09a1ecb3-1a44-4c9c-a998-5e83909a5e45",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T15:40",
"startSlotZulu": "2023-09-06T13:40:00Z",
"speakers": [
{
"name": "Johannes Brodwall",
"twitter": "jhannes",
"bio": "Johannes Brodwall is an experienced programmer and solution architect who was responsible for one of the first large scale adoptions of the Spring framework in the early 2000s. He has spent the last 20 years learning and adjusting to better alternatives that he so far has kept secret to get ahead of the competition."
}
]
},
{
"intendedAudience": "Alle som lytter til podkaster og som har kjent på utfordringene med å måtte forholde seg til mange lukkede plattformer. Det tekniske vil være mest interessant for utviklere.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "I 2021 begynte NRK å publisere podkastene sine én uke senere på andre plattformer enn egen app, og en del podkaster er nå kun tilgjengelig i appen. For at jeg selv og andre lyttere skal kunne høre på dem som før, publiserer jeg åpne RSS-feeds. Men hvorfor må jeg det? Og hva er en podkast egentlig?\n\nJeg viser frem løsningen jeg har satt opp for å automatisk oppdage og publisere podkaster, basert på NRKs åpne APIer. I tillegg vil jeg belyse noen av utfordringene med å begrense tilgjengeligheten til podkaster, hvorfor podkast er et fantastisk format, og hvorfor jeg fortsatt er ganske glad i NRK, tross alt.",
"title": "Hvorfor jeg må gjøre jobben til NRK",
"room": "Room 2",
"startTime": "2023-09-06T14:20",
"endTime": "2023-09-06T14:40",
"video": "861697003",
"startTimeZulu": "2023-09-06T12:20:00Z",
"endTimeZulu": "2023-09-06T12:40:00Z",
"id": "a0b0a006-6b4e-446a-b04c-658f256ab2fe",
"sessionId": "a0b0a006-6b4e-446a-b04c-658f256ab2fe",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T14:20",
"startSlotZulu": "2023-09-06T12:20:00Z",
"speakers": [
{
"name": "Sindre Lindstad",
"twitter": "@sindreli",
"bio": "Sindre Lindstad is a die-hard automation enthusiast. He's currently a Tech Lead at Entur AS, and once set a record as the fastest 60 meter runner in primary school."
}
]
},
{
"intendedAudience": "Alle som interesserer seg for data og hvordan skape kunnskap av dataene",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Politiet har en enormt viktig rolle i samfunnet vårt for å ivareta lov, orden og trygghet, og data går som en rød tråd gjennom alle deler av politiets arbeid som spenner seg fra de mest kritiske situasjonene, til etterforskning, cyberkrim og forebyggende arbeid. Politiet jobber til alle døgnets tider, hver eneste dag. Det genereres store mengder data som kan være vanskelig å få oversikt over og ofte er tilgang til oppdatert data avgjørende for å kunne løse politiets oppdrag. Bli med på denne talken og lær mer om hvordan Politiets IT-enhet jobber med å skape fremtidens IT-systemer og produkter for et politi der datadeling står sentralt, hvordan vi ivaretar sikkerhet i deling av data og hvordan vi skal jobbe med å forstå dataene våre bedre. Du vil også kunne lære mer om hvordan vi kan bygge ny kunnskap av data og vi skal vise aktuelle caser som du vil kunne dra nytte av. ",
"title": "Det datadrevne politiet",
"room": "Room 2",
"startTime": "2023-09-06T14:50",
"endTime": "2023-09-06T15:10",
"video": "861694952",
"startTimeZulu": "2023-09-06T12:50:00Z",
"endTimeZulu": "2023-09-06T13:10:00Z",
"id": "7c5fb42f-f015-4e18-9262-b532771b01df",
"sessionId": "7c5fb42f-f015-4e18-9262-b532771b01df",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T14:20",
"startSlotZulu": "2023-09-06T12:20:00Z",
"speakers": [
{
"name": "Audun Vindenes Egge",
"twitter": "",
"bio": "Audun bygger dataplattformer i politiet, og er opptatt av å bryte ned siloer, dele kunnskap og legge tilrette for fart, flyt og morro!"
},
{
"name": "Anthony Lærdahl",
"twitter": "",
"bio": "Anthony (Tony) jobber med data i politiet. Han har vært utvikler, arkitekt, prosjektleder og teamleder. Men nå er han mest opptatt av semantisk teknologi og modernisering av data- og informasjonsarkitekturen i politiet."
}
]
},
{
"intendedAudience": "This session is for anyone who is interested in the life of a digital nomad, whether you're already living the nomadic lifestyle, considering it, or just curious about how it all works. Participants can expect to gain insight in to what it actually means to be working remotely and traveling. ",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "I will share my experiences traveling and working as a nomadic software engineer across 30+ countries on 4 continents. As someone who went straight from a traditional office job to working for a startup while traveling the world, I learned a lot about the joys and challenges of being a digital nomad. I'll talk about the unique aspects of working remotely, the importance of flexibility and adaptability, and how to balance work and play while constantly on the move. My aim is to inspire others to try the nomadic lifestyle while also helping them understand whether it's the right fit for them.",
"title": "Breaking Free: The Joys and Challenges of Being a Digital Nomad Engineer",
"room": "Room 1",
"startTime": "2023-09-06T14:20",
"endTime": "2023-09-06T15:05",
"video": "861690439",
"startTimeZulu": "2023-09-06T12:20:00Z",
"endTimeZulu": "2023-09-06T13:05:00Z",
"id": "b20b29b0-0904-4572-a571-dffa666297c5",
"sessionId": "b20b29b0-0904-4572-a571-dffa666297c5",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T14:20",
"startSlotZulu": "2023-09-06T12:20:00Z",
"speakers": [
{
"name": "Nemanja Aksic",
"twitter": "",
"bio": "Nemanja is a digital nomad and software engineer working for SafetyWing, a Silicon Valley-based startup that operates with fully remote teams. Since becoming a nomad he's traveled to and worked from more than 30 countries across four continents and has experience working as a consultant in Oslo. With a passion for remote work and location independence, Nemanja is excited to share his practical insights and firsthand experience with the joys and challenges of being a digital nomad."
}
]
},
{
"intendedAudience": "Developers and systems architects. Main benefit is understanding that regular DNS can be a powerful and useful way to deal with service discovery, without having to add complicated infrastructure beyond a regular DNS server. Having experience with setting up microservices using other tools or services would be good.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "One of the key architectural decisions when building systems is how to deal with service discovery. In this talk I will present the option of using dynamic DNS techniques to register services during lifecycle phases, and then use DNS-aware clients to take advantage of this. In particular I will show how to use SRV records to enable client-side load-balancing and failover, and how this enables clients to be configured with logical DNS names, and only the physical mappings change in DNS to make the system as a whole come alive.\n",
"title": "Using dynamic DNS for service discovery",
"room": "Room 2",
"startTime": "2023-09-06T13:30",
"endTime": "2023-09-06T13:50",
"video": "861678772",
"startTimeZulu": "2023-09-06T11:30:00Z",
"endTimeZulu": "2023-09-06T11:50:00Z",
"id": "9a4299ba-f631-44b6-9668-49d8198b3a08",
"sessionId": "9a4299ba-f631-44b6-9668-49d8198b3a08",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T13:00",
"startSlotZulu": "2023-09-06T11:00:00Z",
"speakers": [
{
"name": "Rickard Öberg",
"twitter": "",
"bio": "I have been working on Java projects, products, and services since 1996. Founder and creator of several OpenSource Java projects, including XDoclet, WebWork (now Apache Struts 2), JBoss Application Server, the AOP Qi4j framework, and many other smaller libraries. \nMy focus is event based enterprise software and architectures, including integration between systems and services using REST based APIs. My general process is application of systems thinking in relating technology to business and customer needs, and understanding how various parts of a system functions together, including backends, frontends, metrics, and integration services."
}
]
},
{
"intendedAudience": "This talk is for experienced Java developers who want to get to start optimizing their Java applications: analyzing performance bottlenecks using open-source tools.\nThe participants will get an overview of profilers and their basic concepts, allowing them to add profilers to their toolbox.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Profilers help to analyze performance bottlenecks of your application - if you know which to use and how to work with them.\n\nThere are many open-source profilers, like async-profiler or JMC. This talk will give you insights into these tools, focusing on:\n\n- Understanding the basic concepts of profiling like flame graphs, ...\n- Usage of async-profiler and JMC\n- Advantages and disadvantages of the different tools\n\n",
"title": "Unleash the Power of Open Source Java Profilers",
"room": "Room 1",
"startTime": "2023-09-07T09:00",
"endTime": "2023-09-07T10:00",
"video": "861946709",
"startTimeZulu": "2023-09-07T07:00:00Z",
"endTimeZulu": "2023-09-07T08:00:00Z",
"id": "d69ca0c0-fbce-4a36-8f92-21b39456c0bd",
"sessionId": "d69ca0c0-fbce-4a36-8f92-21b39456c0bd",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T09:00",
"startSlotZulu": "2023-09-07T07:00:00Z",
"speakers": [
{
"name": "Johannes Bechberger",
"twitter": "parttimen3rd",
"bio": "Johannes Bechberger is a JVM developer working on profilers and their underlying technology in the SapMachine team at SAP. This includes improvements to async-profiler and its ecosystem, a website to view the different JFR event types, and improvements to the FirefoxProfiler, making it usable in the Java world. He started at SAP last year after almost 3 years of research studies at the KIT in the field of Java security analyses. His work today is comprised of contributing to various profiling-related open-source projects and fixing bugs in profiling APIs, working on his API in the JEP Candidate 435."
}
]
},
{
"intendedAudience": "No prior knowledge of video editing or programming needed! You'll probably enjoy the content a lot more if you have some experience with the command line. But the talk is meant to be lighthearted, give you some \"aha\"s and focus on the demystifying parts.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Most people, when asked to edit a video, use programs like Premiere Pro, Final Cut, or (😬) Windows Movie Maker. But that takes time. And clicking. Also, we're developers! We love the command line! If we could, we'd probably control our whole lives with iTerm2, kitty or the Visual Studio Code console.\n\nThe story of this talk started an afternoon when I was just trying to compress a video file I had lying around. Of course I wanted to do it from the command line. This ended up pulling me down into a rabbit hole of digital video, codecs and compression schemes. I'm now poking my head up to tell you all about it! Everything from how to edit video in your favourite shell and with your favourite coding language, to the inner workings of video and what makes it so clever. And just a little about how all of this is heavily used in the products that surround us every day. Live demos included.\n\nWhile being a complex topic, it's not as magic as you might think. Let's demystify video together!",
"title": "Video editing on the command line aka. what happens when you ask a programmer to edit your video",
"room": "Room 7",
"startTime": "2023-09-07T17:00",
"endTime": "2023-09-07T18:00",
"video": "862098964",
"startTimeZulu": "2023-09-07T15:00:00Z",
"endTimeZulu": "2023-09-07T16:00:00Z",
"id": "135e2753-1653-490d-af06-640f669e87d1",
"sessionId": "135e2753-1653-490d-af06-640f669e87d1",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T17:00",
"startSlotZulu": "2023-09-07T15:00:00Z",
"speakers": [
{
"name": "Benedicte Emilie Brækken",
"twitter": "benedebr",
"bio": "Benedicte lives for the command line! 👩‍💻 She's one of those people who still use mutt 😅 And for those of you in the know, that should be enough ethos. When COVID hit, she got super passionate about home office video conferencing gear 🎥, and spent most of her paycheck on DACs, microphones and cameras. This, among other things, sparked a deep passion for all things video, streaming, and recording, which she's looking forward to share with you all! 🎉"
}
]
},
{
"intendedAudience": "Everyone",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "In this talk we'll go on a journey through the history of music made with code. We'll meet the pioneers who invented computer music, visit classic video game soundtracks, learn how computer musicians made big sounds with tiny tech, how algorithms create never-ending soundtracks and the huge impact computer music has had on popular culture. Expect lots of nostalgia, vintage code on vintage computers and a musical experience of epic proportions.",
"title": "The History of Computer Music 2023 REMIX!",
"room": "Room 6",
"startTime": "2023-09-07T17:00",
"endTime": "2023-09-07T18:00",
"video": "862097159",
"startTimeZulu": "2023-09-07T15:00:00Z",
"endTimeZulu": "2023-09-07T16:00:00Z",
"id": "52ceebb6-3feb-4106-b8b9-0a220e7d5955",
"sessionId": "52ceebb6-3feb-4106-b8b9-0a220e7d5955",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T17:00",
"startSlotZulu": "2023-09-07T15:00:00Z",
"speakers": [
{
"name": "Anders Norås",
"twitter": "anoras",
"bio": "Director of Software Engineering (CTO) at Avanade\n\nOriginally educated in arts and design, Anders has spent the last twenty years writing code.\n\nHe has given numerous talks and keynotes at conferences such as JavaZone, NDC, J-Fall, Øredev and many more. Have given 100+ conference talks to a variety of audiences including media, design and hardcore computer science. Known for an energetic and highly engaging presentations."
}
]
},
{
"intendedAudience": "Foredraget passer for tekniske personer som kjenner til programmering. Foredraget handler om å fortelle hvordan en virtuell datmaskin klarer å kjøre et program, og hvordan man kan bruke Python til å løse en programmeringsoppgave med bruk av bl.a. boolean-operatoren XOR.\n\nFor de i publikum som er utviklere til vanlig vil foredraget forhåpentligvis inspirere til å ha det gøy med ferdighetene de allerede besitter, og å vise de at f.eks. kodeknekking er et av områdene kan kan begi seg ut på for å få nye utfordringer. Det kreves ikke at publikum trenger å forstå alt innholdet, målet er å demonstrere og inspirere til å lære mer på egen hånd.\n\nUtover en grunnleggende forståelse for programmering så trenger man ingen forkunnskaper, jeg vil forklare underveis konseptene som kodeoppgaven løser.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Hver påske legger PST (Politiets Sikkerhetstjeneste) ut en Capture the Flag-konkurranse (CFT), og i 2020 hadde de en ekstra morsom oppgave på 2. påskedag som jeg gjerne vil dele med deg.\n\nVi har foran oss et bilde med påskekyllingbetjenter, et merkelig Python-skript og en hel haug med emojier - hva kan dette være? Og forresten, hvordan starter og kjører et program på en datamaskin egentlig? \n\nBli med på litt påskenøttknekking og finn ut sammen med meg!",
"title": "Påskenøttknekking med emojier 🐇",
"room": "Room 4",
"startTime": "2023-09-07T17:00",
"endTime": "2023-09-07T17:45",
"video": "862095935",
"startTimeZulu": "2023-09-07T15:00:00Z",
"endTimeZulu": "2023-09-07T15:45:00Z",
"id": "9d67c296-85a2-4806-ac23-6f190f3e9bc2",
"sessionId": "9d67c296-85a2-4806-ac23-6f190f3e9bc2",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T17:00",
"startSlotZulu": "2023-09-07T15:00:00Z",
"speakers": [
{
"name": "Henrik Walker Moe",
"twitter": "",
"bio": "Henrik er sentral i sikkerhetsfagområdet i Bekk, og har over 17 års erfaring i bransjen. Når han finner noe han brenner for å dele med bransjen så kommer det gjerne i form av innlegg på en blogg, i et debattinnlegg eller fra talerstolen på konferanser.\n\nFaglig fokus de siste årene har vært på IT-sikkerhet, hvor Henrik bruker fritiden på å bl.a. være en \"White Hat\"-hacker som tester sikkerheten til fiktive nettløsninger i Capture-The-Flag oppgaver. I rollen som temaeier for Sikkerhet i Bekk jobber han for å skape et miljø rundt faglig engasjement, sikkerhetsbevissthet og bygge opp sikkerhetskompetansen hos kollegaer og kunder."
}
]
},
{
"intendedAudience": "This talk is intended for software developers to learn about how security reporting and research works to give a greater appreciation for this space and hopefully generate excitement to learn more. Developers will take away some simple practical steps to think about to write safer code and have safer team processes.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Ever wonder about the mindset of a hacker? What is a Zero day attack? When does the clock start ticking?\n\nAs cyber Attacks become an existential threat it’s critical that all software developers understand the role the CVE process plays in helping us keep our defenses strong - and where it can go wrong or be subverted.\n\nIn this session, we’ll cover how the CVE process works, explore the timelines of a few famous CVEs, and uncover the truth about ethical reporting. We'll then discuss the practical steps you can take as a developer to write safer software. From bug bounties and bad actors to unsung developer heroes and incredible researchers, it’s time to buckle up for a wild ride as we show you what CVEs are all about.",
"title": "CVE 101: A Developer's Guide to the World of Application Security",
"room": "Room 7",
"startTime": "2023-09-07T15:40",
"endTime": "2023-09-07T16:25",
"video": "862082493",
"startTimeZulu": "2023-09-07T13:40:00Z",
"endTimeZulu": "2023-09-07T14:25:00Z",
"id": "5cde4f58-77f2-4295-9cd5-00ee65c410f0",
"sessionId": "5cde4f58-77f2-4295-9cd5-00ee65c410f0",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T15:40",
"startSlotZulu": "2023-09-07T13:40:00Z",
"speakers": [
{
"name": "Theresa Mammarella",
"twitter": "t_mammarella",
"bio": "Theresa Mammarella is a software engineer at IBM, specializing in Eclipse OpenJ9 JVM and native image prototyping. She enjoys helping developers harness the full potential of their tools to create innovative solutions. Theresa actively contributes to the open-source community collaborating on various projects and is a regular conference speaker.\nWhen she's not coding, Theresa loves to spend her time volunteering with animal rescues and exploring the great outdoors, where she can often be found hiking, camping, or simply soaking up nature's beauty."
}
]
},
{
"intendedAudience": "This talk is designed for gamers who participate in team-based raids and want to improve their effectiveness and efficiency. The talk focuses on the importance of breaking down larger goals into smaller, more manageable tasks and provides practical tips on how to do so.\n\nDuring the presentation, attendees will learn how to organize a raid team, set up a schedule, and assign roles and responsibilities to team members. The talk will also include examples of tasks that might be included in a raid plan, such as gathering resources, scouting locations, and establishing communication protocols.\n\nBy the end of the talk, attendees will have a better understanding of how to approach raid goals and will be equipped with practical strategies for breaking down larger objectives into smaller, actionable tasks. The ultimate goal of the talk is to help gamers work more effectively as a team, improve their raid performance, and ultimately achieve success in their gaming endeavors.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Are you tired of endless project plans that seem to go nowhere? Do you feel like your team is struggling to hit its goals, no matter how much you plan ahead? You know I am talking about wow raids, right? Let's take a lesson from a project management framework: breaking down raid goals into smaller, actionable tasks.\n\nAs a gamer and developer, I have seen how effective it can be to think like a project manager when approaching complex group setup. Just like in a raid, you need to organize your team, set up a schedule, and assign roles and responsibilities. \n\nIn this 10-minute talk, I will share practical tips and examples from my experience raiding in world of warcraft, and how i used scrum methodologies to accomplish this. Whether you're leading a team of developers or coordinating to storm raszageth, by breaking down larger goals into smaller, more manageable tasks, you can make progress towards your objectives and build a sense of momentum that can carry you to success.\n",
"title": "Scrumming the WoW Raid: Boosting Efficiency and Fun",
"room": "Room 2",
"startTime": "2023-09-07T16:00",
"endTime": "2023-09-07T16:10",
"video": "862067344",
"startTimeZulu": "2023-09-07T14:00:00Z",
"endTimeZulu": "2023-09-07T14:10:00Z",
"id": "a4e42984-f085-49c1-9740-851669789921",
"sessionId": "a4e42984-f085-49c1-9740-851669789921",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T15:40",
"startSlotZulu": "2023-09-07T13:40:00Z",
"speakers": [
{
"name": "Malin Jakobsen",
"twitter": "",
"bio": "Front-end developer with a love for gaming and user experience working for Miles in Bergen"
}
]
},
{
"intendedAudience": "Everyone interested in new and disruptive technology. (or a good story) ",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "This talk will be about technology and life, about being passionate about something and make (or force) your company and maybe even your country to adapt. But where do you start when no one shares or even understand the thing you truly belive is the future for yourself, your company, your nation - heck, the very world itself?",
"title": "Quantumania - an unexpected journey",
"room": "Room 2",
"startTime": "2023-09-07T15:40",
"endTime": "2023-09-07T16:00",
"video": "862066474",
"startTimeZulu": "2023-09-07T13:40:00Z",
"endTimeZulu": "2023-09-07T14:00:00Z",
"id": "05d3a51e-1bd7-4df1-b884-aa42c0bdabfc",
"sessionId": "05d3a51e-1bd7-4df1-b884-aa42c0bdabfc",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T15:40",
"startSlotZulu": "2023-09-07T13:40:00Z",
"speakers": [
{
"name": "Andreas Ahlgren",
"twitter": "",
"bio": "Andreas is a multifaceted professional who combines his role as a manager with his expertise in frontend technology as well as demonstrating a fervent passion for Quantum Computing among other cutting-edge technologies. His interests extend far beyond the digital world, reflected in his collection of heavy-weight board games, numbering at least ten weighing over 10 kilograms each. An engaging speaker, Andreas relishes the opportunity to give talks, sharing his knowledge and passion. He's firmly anchored in the belief that stagnation equals decline, and with this mindset, he continually strives to stay abreast of the latest developments, whether it's in the realm of technological frameworks or the intricate lore of Warhammer 40K."
}
]
},
{
"intendedAudience": "Inspirational and entertaining session for programmers",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Advent of Code (AoC) is like a winter holiday party for programmers! Except instead of drinking gløgg and exchanging presents, participants solve daily programming puzzles that become increasingly difficult as the event progresses. It’s a joyful way to sharpen your coding skills, where the reward for each day is the satisfaction of solving the challenges with the language of your choice.\n\nDuring this talk, we’ll take a look at some funny and mind-blowing solutions to these puzzles from the AoC subreddit, as well as jokes and memes that were born from failures and focused endeavors to collect fifty \"stars\" and save Christmas. Get ready to be inspired by the creativity and perseverance of programmers who often intentionally overcomplicate things but still manage to find solutions. Let their stories reignite your passion for coding.\n\nThis talk is intended for programmers, nerds, and elves alike.",
"title": "AdventOfCode for the Brave and True!",
"room": "Room 2",
"startTime": "2023-09-07T16:10",
"endTime": "2023-09-07T16:30",
"video": "862065794",
"startTimeZulu": "2023-09-07T14:10:00Z",
"endTimeZulu": "2023-09-07T14:30:00Z",
"id": "e00614d6-55c2-45b0-a542-2de2d7e5e90a",
"sessionId": "e00614d6-55c2-45b0-a542-2de2d7e5e90a",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T15:40",
"startSlotZulu": "2023-09-07T13:40:00Z",
"speakers": [
{
"name": "Artur Zabeyvorota",
"twitter": "",
"bio": "As a passionate backend developer who loves a good challenge, I'm proud to have completed the \"Advent of Code\" in both 2020 and 2022 using Python and Kotlin."
}
]
},
{
"intendedAudience": "This presentation is for frontend developers, tech team leaders. It is a topic that covers a trend that group of people might find interesting to adopt for their own teams.\n\n\n\n",
"length": "10",
"format": "lightning-talk",
"language": "en",
"abstract": "Dealing with the limitations and headaches of using monolithic architectures in our web applications can be tiring. As developers, it's crucial that we're informed early in our careers about different paradigms that offer improved flexibility and faster deployment and development times! That's why you should join me to discover why micro frontends are the S#@T and why everyone should try them in their teams. Through examples, we'll demonstrate how this approach can help you break down your frontend into smaller, independent pieces and develop and deploy new features more quickly. \n",
"title": "Why micro frontends are the s#@t",
"room": "Room 2",
"startTime": "2023-09-06T12:00",
"endTime": "2023-09-06T12:10",
"video": "861652906",
"startTimeZulu": "2023-09-06T10:00:00Z",
"endTimeZulu": "2023-09-06T10:10:00Z",
"id": "b030dde9-783f-453f-8eca-0bb1c570946d",
"sessionId": "b030dde9-783f-453f-8eca-0bb1c570946d",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T11:40",
"startSlotZulu": "2023-09-06T09:40:00Z",
"speakers": [
{
"name": "George Harakis",
"twitter": "https://twitter.com/GHarakis",
"bio": "I am a full-stack Oslo based web developer (originally from Cyprus) with a focus on React and Java Spring Boot. I also hold a Bachelor's degree in Computer Science.\nAside from my professional work, I am passionate about teaching and sharing my knowledge with others. In my spare time, I create educational YouTube videos covering topics such as React and various aspects of web development. I find it incredibly rewarding to interact with people from all over the world.\n"
}
]
},
{
"intendedAudience": "Developers, data engineers, architects",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Analytics is arguably the oldest task of digital computers, and has remained an important part of business computing during its 80-year history. What could possibly be new in this space? A better way to serialize data? A better way to run distributed queries over a data lake? An exciting new integration technology? We need all these things urgently, but to be ready for the future, we something even more: we need analytics that can deliver results to users in real time, in the interaction layer, not to internal decision makers seconds or minutes later. We need what Apache Pinot delivers.\n\nPinot is a database optimized to serve analytical queries at extremely high levels of concurrency, at latencies measured in the tens of milliseconds, on data ingested from the streaming pipeline that underlies more and more of the systems we're building. It's an OLAP database, but one engineered from the ground up to power user-facing features, not executive-facing dashboards. In this talk, we'll look at how it's built, how to get data into it, how to query it, and the typical role it plays in a next-generation system exposing analytics-powered features to its users.",
"title": "Real-Time Analytics With Apache Pinot",
"room": "Room 6",
"startTime": "2023-09-07T11:40",
"endTime": "2023-09-07T12:40",
"video": "862005649",
"startTimeZulu": "2023-09-07T09:40:00Z",
"endTimeZulu": "2023-09-07T10:40:00Z",
"id": "d3342333-40da-4bbf-be1a-ca0ea98def73",
"sessionId": "d3342333-40da-4bbf-be1a-ca0ea98def73",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T11:40",
"startSlotZulu": "2023-09-07T09:40:00Z",
"speakers": [
{
"name": "Tim Berglund",
"twitter": "tlberglund",
"bio": "Tim is a teacher, author, and technology leader with StarTree, where he serves as the VP of Developer Relations. He is a regular speaker at conferences and a presence on YouTube explaining complex technology topics in an accessible way. He tweets as @tlberglund, blogs every few years at http://timberglund.com, and lives in Littleton, CO, USA. He has three grown children and two grandchildren, with a third on the way."
}
]
},
{
"intendedAudience": "This talk is geared towards folks (software engineers, architects, tech leads) who want to learn about processing changes in their data in real-time, using the open-source stack of Apache Kafka (for data streaming), Apache Flink (for stream processing), and Debezium (for log-based change data capture). Attendees should have some understanding of the fundamentals of Kafka.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Log-based change data capture (CDC) is a key component of the modern data streaming stack, used for data replication, feeding search indexes, low-latency data warehouse updates, and more.\n\nMerely taking data from A to B often isn't enough though; instead, change event streams, as for instance created using Debezium, may need to be filtered or routed based on event contents, multiple streams be joined, continuous queries be updated, etc. Enter Apache Flink: it lets you do stateful stream processing on change event feeds. Join us for this session and learn about\n\n* Implementing streaming queries on CDC events with the Flink data stream API and Flink SQL\n* Aggregating and enriching change data events\n* Different deployment options: Kafka Connect vs. Flink CDC\n\nIn a demo we'll put all these open-source components into action, showing how to set up a data streaming pipeline from your operational database to a live dashboard within minutes.\n\n",
"title": "Real-time Change Stream Processing with Apache Flink",
"room": "Room 3",
"startTime": "2023-09-07T14:20",
"endTime": "2023-09-07T15:20",
"video": "862061927",
"startTimeZulu": "2023-09-07T12:20:00Z",
"endTimeZulu": "2023-09-07T13:20:00Z",
"id": "355869fa-5aa0-43a7-abd2-7c5250e10bcd",
"sessionId": "355869fa-5aa0-43a7-abd2-7c5250e10bcd",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T14:20",
"startSlotZulu": "2023-09-07T12:20:00Z",
"speakers": [
{
"name": "Gunnar Morling",
"twitter": "@gunnarmorling",
"bio": "Gunnar Morling is a software engineer and open-source enthusiast by heart, currently working at Decodable on stream processing based on Apache Flink. In his prior role as a software engineer at Red Hat, he led the Debezium project, a distributed platform for change data capture. He is a Java Champion and has founded multiple open source projects such as JfrUnit, kcctl, and MapStruct. Gunnar is an avid blogger (morling.dev) and has spoken at a wide range of conferences like QCon, Java One, and Devoxx. He lives in Hamburg, Germany."
}
]
},
{
"intendedAudience": "De som er interessert i å få til kunstig intelligens i praksis, og høre om erfaringer fra Posten. Ingen forkunnskap nødvendig.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Digital transformasjon i et 375 år gammelt selskap er ikke bare enkelt. Posten har vokst fra levering med hest og kjerre til helautomatisk brevsortering og samlebåndsproduksjon av millioner av pakker hvert år. I foredraget vil jeg beskrive hvordan vi bruker data, maskinlæring og tverrfaglige team for å gjøre Posten mer datadrevet, og skissere noen av utfordringene man møter når man bruker datavitenskap i industriell skala.",
"title": "Fra hest og kjerre til digital transformasjon",
"startTime": "2023-09-07T14:50",
"endTime": "2023-09-07T15:10",
"room": "Room 2",
"video": "862046129",
"startTimeZulu": "2023-09-07T12:50:00Z",
"endTimeZulu": "2023-09-07T13:10:00Z",
"id": "423053a5-e577-4c07-8db3-e8d4aae36738",
"sessionId": "423053a5-e577-4c07-8db3-e8d4aae36738",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T14:20",
"startSlotZulu": "2023-09-07T12:20:00Z",
"speakers": [
{
"name": "Heidi Dahl",
"twitter": "",
"bio": "Heidi Dahl er Senior Data Scientist i Posten, og har 15 års erfaring som forsker i SINTEF. Hun startet og ledet Tekna Big Data i 5 år, og startet Women in Data Science Oslo som arrangerer sin 7. årskonferanse i år. Med en bakgrunn i matematisk modellering, brenner hun for å skape verdi med datavitenskap i praksis."
}
]
},
{
"intendedAudience": "Alle som er involvert i bruk eller utvikling av maskinlæringssystemer",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "\n\nEn maskinlæringsmodell blir aldri rettferdig. I alle fall ikke mer rettferdig en de data den er trent på. Og datasettene vi benytter foreslår som hovedregel at vi diskriminerer.\n\nSå kanskje vi rett og slett må akseptere at vi ruller ut ‘urettferdige’ maskinlæringssystemer? Men hvor mye skal vi tåle? Og hvordan beholder vi kontrollen, og sikrer at systemene våre holder seg under denne smertegrensen for diskriminering?",
"title": "Anta bias. Alltid",
"startTime": "2023-09-07T14:40",
"endTime": "2023-09-07T14:50",
"room": "Room 2",
"video": "862045805",
"startTimeZulu": "2023-09-07T12:40:00Z",
"endTimeZulu": "2023-09-07T12:50:00Z",
"id": "bb09b21f-4993-4a49-b821-6235adb121ac",
"sessionId": "bb09b21f-4993-4a49-b821-6235adb121ac",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T14:20",
"startSlotZulu": "2023-09-07T12:20:00Z",
"speakers": [
{
"name": "Nora Gjøen-Gjøsæter",
"twitter": "",
"bio": "Nora Gjøen-Gjøsæter er Data Scientist i Kantega, og brenner for ansvarlig AI. Hun har bakgrunn som statistiker, og erfaring med maskinlæring fra blant annet offentlig sektor, energisektoren, forsikringsbransjen og startups – i tillegg til en tidligere karriere som profesjonell fotballspiller."
}
]
},
{
"intendedAudience": "In this Live coding example, I will show the audience how to improve API quality & minimize the risk of breaking changes using Spring RestDocs & Spring Cloud Contract in Spring Boot 3.\n",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "This live code talk will show you how to use Spring RestDocs and Spring Cloud Contract to easily create and maintain API documentation and contract tests for your Spring Boot 3 applications. We will demonstrate how to use Spring RestDocs to automatically generate API documentation from your tests and use Spring Cloud Contract to ensure that your API adheres to the agreed-upon contracts. You will learn how to use these tools to improve the quality of your API documentation, increase confidence in changes made to your API, and minimize the risk of introducing breaking changes. With this talk, you'll be able to improve your API development process and create more robust, reliable, and maintainable APIs.",
"title": "Bootiful RESTful API Contracts and Restdocs",
"room": "Room 6",
"startTime": "2023-09-07T14:20",
"endTime": "2023-09-07T15:20",
"video": "862043663",
"startTimeZulu": "2023-09-07T12:20:00Z",
"endTimeZulu": "2023-09-07T13:20:00Z",
"id": "933b8f56-29f8-4ce0-8c6a-5cc00ff23927",
"sessionId": "933b8f56-29f8-4ce0-8c6a-5cc00ff23927",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T14:20",
"startSlotZulu": "2023-09-07T12:20:00Z",
"speakers": [
{
"name": "Mario Gray",
"twitter": "@mariogray",
"bio": "Mario is a principal technologist at VMware with more than 20 years of experience in software development and software architecture. He is co-author of Pro Spring Integration (Apress, 2011). He’s helped organizations large and small build service-based architectures in a number of different runtimes and platforms over the decades, but adopting Spring in 2004, and using it whenever possible since, was a no-brainer that culminated in joining the Spring team in 2017. As a Spring developer advocate, Mario loves to engage and inspire developers and businesses in the Pivotal ecosystem."
}
]
},
{
"intendedAudience": "The target audience are developers who have some understanding of Spring and some understanding of Kotlin",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "As a seasoned developer, you’re likely already familiar with Spring. But Kotlin can take your developer experience with Spring to the next level!\nJoin this webinar and learn how to:\n- Add new functionality to existing classes with Kotlin extension functions.\n- Use Kotlin bean definition DSL.\n- Better configure your application using lateinit.\n- Use sequences and default argument values to write more expressive code.\nBy the end of this talk, you’ll have a deeper understanding of the advanced Kotlin techniques available to you as a Spring developer and be able to use them effectively in your projects.\n",
"title": "Advanced Kotlin Techniques for Spring Developers",
"room": "Room 4",
"startTime": "2023-09-07T14:20",
"endTime": "2023-09-07T15:20",
"video": "862042771",
"startTimeZulu": "2023-09-07T12:20:00Z",
"endTimeZulu": "2023-09-07T13:20:00Z",
"id": "15876f1b-30a5-45c3-9f3c-b1f69f27b3ff",
"sessionId": "15876f1b-30a5-45c3-9f3c-b1f69f27b3ff",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T14:20",
"startSlotZulu": "2023-09-07T12:20:00Z",
"speakers": [
{
"name": "Pasha Finkelshteyn",
"twitter": "asm0di0",
"bio": "Pasha Finkelshteyn is a developer advocate for Kotlin at JetBrains. With over 14 years of experience in the IT industry, he has used Kotlin in production for various tasks, including backend development, since 2015. In addition to his expertise with Kotlin, Pasha also has experience in software quality assurance, system administration, and data engineering. He’s passionate about using technology to improve the world and is excited to share his knowledge and experience with Kotlin"
}
]
},
{
"intendedAudience": "In this session the attendees will have a chance to meet Apache Pulsar. What is it, and why should you use it? In the talk we're combining Pulsar with Kafka, to use the best of both Worlds.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Which streaming technology is right for me? Do I need to use Apache Pulsar or do I choose Apache Kafka?\n\nThat is mostly the question, but did you also knew that you can combine them? \n\nIn this session we will show how to use best of both worlds. Let's compare both architectures and Java Client Implementations and decide what's best for you!\n\nDo you pick the battle? Or are you making allies?\nIt's up to you!",
"title": "Best of Both Worlds: Apache Pulsar and Apache Kafka",
"room": "Room 1",
"startTime": "2023-09-07T14:20",
"endTime": "2023-09-07T15:05",
"video": "862042724",
"startTimeZulu": "2023-09-07T12:20:00Z",
"endTimeZulu": "2023-09-07T13:05:00Z",
"id": "4f17861c-bb56-4545-a690-fe4420d4ffd1",
"sessionId": "4f17861c-bb56-4545-a690-fe4420d4ffd1",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T14:20",
"startSlotZulu": "2023-09-07T12:20:00Z",
"speakers": [
{
"name": "Ko Turk",
"twitter": "@KoTurk77",
"bio": "Ko Turk is an experienced developer working for Blue4IT. He is focussing on Java, Kotlin and isn't afraid to code in Typescript. \n\nHe likes to write articles for the Dutch NLJUG JavaMagazine. Also he is regularly speaking at international conferences about Green Software Engineering,\n\nKafka, Micrometer and Kotlin. Because he doesn't like bull it slide presentations, you can find him (live) coding at stage.\n\nYou can also find him at the UtrechtJUG and love to have a chat!\n\nHe is always available at Twitter @KoTurk77"
},
{
"name": "Mary Grygleski",
"twitter": "@mgrygles",
"bio": "Streaming Developer Advocate at DataStax, Java Champion, President of Chicago-JUG\n\nMary is a Java Champion and a passionate Senior Developer Advocate at DataStax, a leading data management company that champions Open Source software and specializes in Big Data, DB-as-a-service, Streaming, and Cloud-Native systems. She spent 3.5 years as a very effective advocate at IBM, focusing on Java, Jakarta EE, OpenJ9, Open Source, Cloud, and Distributed Systems. She transitioned from Unix/C to Java around 2000 and has never looked back since then. She considers herself a polyglot and loves to continue learning new and better ways to solve real-life problems. She is an active tech community builder outside of her day job, and currently the President of the Chicago Java Users Group (CJUG), as well as a co-organizer for several IBM-sponsored meetup groups in the Greater Chicago area."
}
]
},
{
"intendedAudience": "Alle",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Det største tekniske systemet i NAV er et stormaskinsystem fra 70-tallet. NAV har jobbet i til sammen 17 år med å skru det av. Ikke bare har vi holdt på lenge, men vi har også med jevne mellomrom - ca hvert skuddår - fått ekstraordinære bevilgninger for å finansiere jobben. Nå er det siste prosjektet i planen ferdig, men systemet er fremdeles ikke skrudd av. Denne presentasjonen skal fortelle hvorfor - og også hvorfor vi likevel er stolte av den jobben som er gjort.  \n\nPå veien kommer vi innom statens prosjektmodell, konsulenter vs eierskap, arkitektur og kode, etterlevelse av lovverk, systemutvikling vs organisasjonsutvikling, smidig vs Smidig.",
"title": "Software er fortsatt politikk",
"startTime": "2023-09-07T11:40",
"endTime": "2023-09-07T12:40",
"room": "Room 4",
"video": "862007321",
"startTimeZulu": "2023-09-07T09:40:00Z",
"endTimeZulu": "2023-09-07T10:40:00Z",
"id": "81df4b91-2573-48b4-bbe4-091f28647e47",
"sessionId": "81df4b91-2573-48b4-bbe4-091f28647e47",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T11:40",
"startSlotZulu": "2023-09-07T09:40:00Z",
"speakers": [
{
"name": "Truls Jørgensen",
"twitter": "trulsjor",
"bio": "Truls er principal engineer i NAV. Vil skrive endringsdyktig software, og må derfor også jobbe med å bygge en organisasjon som muliggjør dette."
},
{
"name": "Audun Fauchald Strand",
"twitter": "audunstrand",
"bio": "Audun er principal engineer i NAV. Dermed får han oppfylt drømmen sin om å få kode og bestemme samtidig. Han blir glad av utviklingsfart og tullete navn, og elsker å høre om når ting gikk galt.  "
}
]
},
{
"intendedAudience": "This talk would benefit any Java developer who has some familiarity with Spring and Spring Boot. No previous knowledge of Quarkus is required.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "In this session, I will show concepts and conventions familiar to Spring developers and how those same concepts and conventions can be implemented in Quarkus, all while highlighting similarities and differences between them. Additionally, I will show similarities and differences in how testing is done, highlighting Quarkus Dev Services.\n\nThis session will be mostly code while minimizing the number of slides. I will introduce an existing Spring application with a full test suite and build a Quarkus equivalent version of it, live.",
"title": "Quarkus for Spring Developers",
"room": "Room 7",
"startTime": "2023-09-07T11:40",
"endTime": "2023-09-07T12:40",
"video": "862003843",
"startTimeZulu": "2023-09-07T09:40:00Z",
"endTimeZulu": "2023-09-07T10:40:00Z",
"id": "72a562c9-9cd9-43db-80e7-735ce5472de4",
"sessionId": "72a562c9-9cd9-43db-80e7-735ce5472de4",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T11:40",
"startSlotZulu": "2023-09-07T09:40:00Z",
"speakers": [
{
"name": "Eric Deandrea",
"twitter": "edeandrea",
"bio": "Eric Deandrea is a Senior Principal Developer Advocate at Red Hat, focusing on application development technologies. Eric has over 20 years of experience designing and building Java-based solutions and developer training programs for the financial services and insurance industries. He is also a contributor to various Open Source projects, including Quarkus and Spring. Eric recently put his Quarkus and Spring knowledge to use by publishing his first book, “Quarkus for Spring Developers.” He enjoys using DevOps-focused automation technologies to make life easy. Outside of work, Eric enjoys boating on the lakes of New Hampshire, ice hockey, and martial arts, in which he holds a black belt in Kempo Karate."
}
]
},
{
"intendedAudience": "Anyone with an interest in AI and that would like to learn about practical use cases, and the reality behind training your own models with your own data. ",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "At Norkart we aim to develop the best AI models for automatic mapping of objects from aerial imagery. With a wealth of already labeled objects, such as buildings, we find ourselves in a somewhat unique position - we have too much training data!\n\nI know what you’re thinking! There’s no such thing as too much training data. However, a large amount of irrelevant data can impede the development of a well-balanced training dataset. For example, when training a building-detection model, we need to be selective in the examples we use, focusing on a diverse range of building types rather than non-relevant data such as oceans, forests, and roads.\n\nSo how can we ensure an ideal selection of training data in order to get a model that is robust enough to analyze any part of Norway and recognize any sort of building?\n\nJoin us as we present our training rig where we dynamically select and produce training data while training and evaluating the model - to get the best AI for building detection",
"title": "GeoAi – How to handle the luxury of having too much training data",
"room": "Room 3",
"startTime": "2023-09-06T17:00",
"endTime": "2023-09-06T18:00",
"video": "861736155",
"startTimeZulu": "2023-09-06T15:00:00Z",
"endTimeZulu": "2023-09-06T16:00:00Z",
"id": "1a30b1fc-6243-4a1f-90c7-46ba9af75426",
"sessionId": "1a30b1fc-6243-4a1f-90c7-46ba9af75426",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T17:00",
"startSlotZulu": "2023-09-06T15:00:00Z",
"speakers": [
{
"name": "Mathilde Ørstavik",
"twitter": "",
"bio": "Mathilde is the head of geospatial AI at Norkart. She has a broad experience in applied Ai on geospatial data, in particular in semantic segmentation from aerial high-resolution data."
}
]
},
{
"intendedAudience": "Ledere, smidig coacher, arkitekter, systemtenkere, teamene.",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Begrepet \"sosioteknisk\" ser ut til å ha fått en renessanse den siste tiden, noe som er oppløftende med tanke på den positive effekten denne tilnærmingen har hatt for organisasjoner og deres ansatte verden rundt siden starten i de britiske kullgruvene på 50-tallet. For oss nordmenn er det enda mer relevant siden den mest betydningsfulle forskningen på denne nye måten å organisere arbeidet på ble gjort her i Norge på 60-tallet og ble til en av pilarene i norsk arbeidsliv. Spørsmålet er da hvor relevant dette er i IT, hvor vi allerede har smidig, DevOps, og andre arbeidsprosesser. Men hva med hele systemet, som organiseringen, tilpasningsdyktigheten og trivselen? \n\nI denne presentasjonen skal vi derfor se nærmere på hva sosioteknisk systemtenkning er, spesielt den grenen som kaller åpent systemteori. Vi skal både se på hvordan disse overlapper med smidig og hvor det er forskjeller. Tesen presentert her er at forskningen fra samfunnsvitenskapene med teknikkene og tilnærmingene brukt for å radikalt endre en organisasjons fra et byråkrati til et fullt deltagerne demokrati hvor alle bidrar produktivt er vel så relevant for oss i de \"digitale kullgruvene\" - antagelig enda mer.",
"title": "Sosioteknisk systemdesign for “digitale kullgruver”",
"room": "Room 7",
"startTime": "2023-09-07T10:20",
"endTime": "2023-09-07T11:20",
"video": "861989662",
"startTimeZulu": "2023-09-07T08:20:00Z",
"endTimeZulu": "2023-09-07T09:20:00Z",
"id": "e3c0c356-035f-495b-99c5-d781ab560d72",
"sessionId": "e3c0c356-035f-495b-99c5-d781ab560d72",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T10:20",
"startSlotZulu": "2023-09-07T08:20:00Z",
"speakers": [
{
"name": "Trond Hjorteland",
"twitter": "trondhjort",
"bio": "Trond er en IT-arkitekt og sosioteknisk utøver fra konsulentselskapet Scienta med mange års erfaring fra store, komplekse og forretningskritiske applikasjoner, primært som utvikler og arkitekt på mellomvare- og backendsystemer. Hans hovedinteresser er tjenesteorientering, domene-drevet design, hendelsesdrevet arkitektur og åpne sosiotekniske systemer, da benyttet i bransjer som telekom, media, TV-distribusjon og det offentlige. Mantra: Gode produkter blir til gjennom tett designsamarbeid."
}
]
},
{
"intendedAudience": "Denne lyntalen passer for de som har kjent på smerten i store alt-i-ett erstatningsprosjekter. Det finnes en bedre vei som gir kontroll og løpende feedback. Men det kommer til å være slitsomt til tider. \n\nForhåpentligvis får du med noen teknikker fra denne lyntalen som gjør det litt hyggeligere.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Kontinuerlige leveranser gjør det mulig å kunne endre ting gradvis og kontrollert. Men hva gjør du hvis du har en gigantisk monolitt med gammel teknologi? Må du ikke bare skrive om alt på én gang?\n\nJa og nei. Det finnes alltid en mulighet for å dele opp problemet. Men du må ta noen vanskelige valg under veis og kanskje gjøre ting du ikke ellers ville gjort.\n\nI denne lyntalen vil jeg fortelle om noen strategier, og et par ganger vi har måttet bytte ut motoren i fart.",
"title": "To kill a legacy system",
"room": "Room 2",
"startTime": "2023-09-07T12:00",
"endTime": "2023-09-07T12:10",
"video": "861989386",
"startTimeZulu": "2023-09-07T10:00:00Z",
"endTimeZulu": "2023-09-07T10:10:00Z",
"id": "c171355c-47ed-4dd0-9b92-d1b743ebbd31",
"sessionId": "c171355c-47ed-4dd0-9b92-d1b743ebbd31",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T11:40",
"startSlotZulu": "2023-09-07T09:40:00Z",
"speakers": [
{
"name": "Anders Sveen",
"twitter": "anderssv",
"bio": "Anders har lang erfaring som utvikler, arkitekt, team lead og drifter. Han ser alltid etter bedre måter å utvikle på, for å redusere risiko og maksimere verdi. Han snakker veldig gjerne om kontinuerlige leveranser, TDD og operations.\n\nEtter mange år i konsulentbransjen, har han de siste årene har lært utrolig mye av å gå rett i produksjon hos start-upene Porterbuddy og ZTL Payments.\n"
}
]
},
{
"intendedAudience": "Java programmers with experience in concurrent programming",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Project Loom, expected to be delivered in final form in Java 21, brings \"virtual\" threads to the Java platform. After an introduction to the concepts of virtual threads, structured concurrency, and extent locals, the talk focuses on the implications for programmers. Why should you care about Loom if you will never run a million concurrent tasks? How do you choose between virtual and platform threads? When blocking is cheap, is there still a benefit to reactive programming? Which parts of \"Concurrency in Practice\" are now outdated, and what is still relevant? What concurrent synchronization mechanisms and design patterns should you favor?  How do you debug and profile virtual threads? What pitfalls do you need to recognize when transforming your current code to take advantage of virtual threads? How do you structure new code? This pragmatic presentation provides a perspective beyond the \"what\" of Project Loom and into the \"why\" and \"how\". ",
"title": "Looming Changes in Java Concurrency",
"room": "Room 6",
"startTime": "2023-09-07T10:20",
"endTime": "2023-09-07T11:20",
"video": "861988939",
"startTimeZulu": "2023-09-07T08:20:00Z",
"endTimeZulu": "2023-09-07T09:20:00Z",
"id": "43753472-40fc-49ff-9bcd-2348b9193190",
"sessionId": "43753472-40fc-49ff-9bcd-2348b9193190",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T10:20",
"startSlotZulu": "2023-09-07T08:20:00Z",
"speakers": [
{
"name": "Cay Horstmann",
"twitter": "",
"bio": "Cay grew up in Northern Germany but scholarships lured him to earn a M.S. in computer science from Syracuse University and a Ph.D. in mathematics from the University of Michigan. He taught computer science at San Jose State University for almost thirty years and held visiting appointments at universities in Germany, Switzerland, Vietnam, and Macau. He was the CEO of a pre-internet software company, and VP and CTO of a dot com startup that went from three people in a tiny office to a public company. In his copious spare time he writes books, including the international best-seller Core Java, and develops online learning experiences for beginning and professional programmers. He has followed and written about Project Loom since its earliest releases."
}
]
},
{
"intendedAudience": "This talk is relevant and interesting for all audiences. No prior experience is needed.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "In this lightning talk I will show how you can be onboarded a complex data engineering project in 5 minutes.\nThen I will proceed in delivering the first pull request following al the standards of the project. All live!\nData Engineering consistency, speed and agility can be achieved by using a cloud development environments like Gitpod and GitHub Codespaces. Be inspired.",
"title": "Onboard a developer to your project in 5 minutes flat by using Gitpod.",
"startTime": "2023-09-07T12:10",
"endTime": "2023-09-07T12:30",
"room": "Room 2",
"video": "861988428",
"startTimeZulu": "2023-09-07T10:10:00Z",
"endTimeZulu": "2023-09-07T10:30:00Z",
"id": "03783e0e-aedd-4494-b77f-4d688899f985",
"sessionId": "03783e0e-aedd-4494-b77f-4d688899f985",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T11:40",
"startSlotZulu": "2023-09-07T09:40:00Z",
"speakers": [
{
"name": "Fredrik Hoem Grelland",
"twitter": "@grelland",
"bio": "I work as an Innovation advisor and have built several data platforms over the last few years designed for data engineering, data science and data mesh applications. I have focused on reducing the technology friction and helping analysts and other data savvy people onto code-oriented platforms in order to achieve greater agility, speed and quality of work."
}
]
},
{
"intendedAudience": "Anyone with an interest in GoLang and generics is sure to gain some helpful insights with this talk 😊",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Learn Golang Generics and become the talk of the town!\n\nThis talk covers everything you need to know to master Golang Generics, including writing generic functions and interfaces. I'll keep things interesting with practical examples, all explained in plain English so even your grandma can follow along (assuming she knows Go).\n\nWhether you're a beginner or a seasoned pro, join me to learn how to write more efficient, reusable, and impressive code in Go.",
"title": "GoLang Generics: Simplifying Programming Concepts So Well, Even Your Grandma Will Want to Code!",
"room": "Room 2",
"startTime": "2023-09-07T11:40",
"endTime": "2023-09-07T12:00",
"video": "861987672",
"startTimeZulu": "2023-09-07T09:40:00Z",
"endTimeZulu": "2023-09-07T10:00:00Z",
"id": "0d44df06-c0f3-459a-9e42-30b8822ea4c5",
"sessionId": "0d44df06-c0f3-459a-9e42-30b8822ea4c5",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T11:40",
"startSlotZulu": "2023-09-07T09:40:00Z",
"speakers": [
{
"name": "Meimona Hakim",
"twitter": "",
"bio": "Meimona Hakim, an avid programming enthusiast, has been coding for nearly 7 years and continues to explore the potential of the GoLang programming language. With a passion for sharing knowledge, she is excited to impart what she has learned to others."
}
]
},
{
"intendedAudience": "Java developers, tech-leads og arkitekter som jobber med objektorienterte språk. ",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Applikasjonsarkitektur. For mange et gammelt utdatert konsept som man forbinder med endeløse UML diagrammer og kompleksitet. Eller et konsept som bare senker all hastighet i utviklingen?\nEr du en av dem som grøsser ved tanken på applikasjonsarkitektur? Eller ikke skjønner poenget med den tradisjonelle lagdelte tre-lags-arkitekturen? Høres det ut som mye arbeid for lite nytte, og noe som driver kompleksiteten opp?\n\nGode nyheter. Du er ikke alene! \n\nGlem UML, glem tre-lags-arkitekturen og hiv ut arkitekten! I denne presentasjonen skal vi se nærmere på hvordan du som utvikler tar kontroll over applikasjonsarkitekturen på en enkel, ren og rett frem måte. ",
"title": "Hjelp! Applikasjonen vår er et eneste stort kaos",
"room": "Room 1",
"startTime": "2023-09-07T11:40",
"endTime": "2023-09-07T12:25",
"video": "861986610",
"startTimeZulu": "2023-09-07T09:40:00Z",
"endTimeZulu": "2023-09-07T10:25:00Z",
"id": "3572508a-4789-49ab-b07b-b3f49d9c5754",
"sessionId": "3572508a-4789-49ab-b07b-b3f49d9c5754",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T11:40",
"startSlotZulu": "2023-09-07T09:40:00Z",
"speakers": [
{
"name": "Erik Dahl",
"twitter": "dahlsdata",
"bio": "Erik er java-programmer / techlead / arkitekt med et stort hjerte for arkitektur. Til daglig jobber han som uavhengig freelance-konsulent på offentlige og private prosjekter i Oslo-regionen. Han er alltid på utkikk etter nye perspektiver, ny forbedret forståelse og elsker å ha diskusjoner om arkitektur. \n"
}
]
},
{
"intendedAudience": "Architects, developers, data handlers, people interested in the knowledge graph hype",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Put simply, knowledge graphs are a smart way of organizing data. They connect different pieces of information, making it easier to understand complex topics. Knowledge graphs are useful in a wide range of fields, including artificial intelligence, data management, and natural language processing. They can help improve search results, integrate data, and even uncover new insights. Additionally, knowledge graphs are valuable for decision-making, as they provide individuals and organizations with a more in-depth understanding of their information domain. So, if you're dealing with large amounts of complex (or easy) data, knowledge graphs could be a game-changer for you.",
"title": "What are knowledge graphs, and why should you care?",
"room": "Room 3",
"startTime": "2023-09-07T10:20",
"endTime": "2023-09-07T11:05",
"video": "861984781",
"startTimeZulu": "2023-09-07T08:20:00Z",
"endTimeZulu": "2023-09-07T09:05:00Z",
"id": "b133b301-524a-4f2f-b1da-25212658f802",
"sessionId": "b133b301-524a-4f2f-b1da-25212658f802",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T10:20",
"startSlotZulu": "2023-09-07T08:20:00Z",
"speakers": [
{
"name": "Veronika Heimsbakk",
"twitter": "veronikaheim",
"bio": "Veronika is a knowledge graph nerd at Capgemini, with 8+ years of experience in the field of knowledge graphs. She adores the wonders of logic and linked data, and helping clients create order in their chaotic data chaos."
},
{
"name": "Miriam Næss Jørstad",
"twitter": "",
"bio": "Miriam is a wizard in the field of natural language processing and computational linguistics. She sees the huge potential of combining linguistic madness together with semantics and knowledge graphs, to provide data solutions with high integrity for her clients."
}
]
},
{
"intendedAudience": "Any Java dev. ALthough me might get in a bit deeper to explain certain threads, it will be valuable to both Junior and Senior Java Devs.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "As a Java developer, you understand the importance of writing code that is functional, scalable, and maintainable. But how about secure code? Even the most seasoned developer can make common security mistakes that leave your code vulnerable to attack. In this session, we'll explore the most common and sometimes unknown security pitfalls made by Java developers and provide practical tips for avoiding them. We'll cover everything from input validation errors to injection to file overwrites and arbitrary code execution. We'll show real-world examples of insecure code and demonstrate how attackers exploit these vulnerabilities before showing you how to fix these code constructions. By understanding how these mistakes get exploited, you'll be better equipped to write secure, bulletproof code that can withstand attacks. Whether you're a junior developer just starting out or a seasoned pro looking to brush up on your skills, this session is a must-attend for anyone concerned with the security of their Java applications. Let’s start writing secure Java code and learn how to avoid security mistakes.",
"title": "Don't Get Burned! Secure Coding Essentials in Java to protect your application",
"room": "Room 4",
"startTime": "2023-09-07T10:20",
"endTime": "2023-09-07T11:20",
"video": "861986282",
"startTimeZulu": "2023-09-07T08:20:00Z",
"endTimeZulu": "2023-09-07T09:20:00Z",
"id": "1423c0cd-f75a-4cc4-aa4d-2112fead7b11",
"sessionId": "1423c0cd-f75a-4cc4-aa4d-2112fead7b11",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T10:20",
"startSlotZulu": "2023-09-07T08:20:00Z",
"speakers": [
{
"name": "Brian Vermeer",
"twitter": "BrianVerm",
"bio": "Brian is a Staff Developer Advocate for Snyk, Java Champion, and Software Engineer with over a decade of hands-on experience in creating and maintaining (web)applications. He is passionate about Java, (Pure) Functional Programming and Cybersecurity. Brian is a JUG leader for the Virtual JUG and the NLJUG. He also co-leads the DevSecCon community and is a community manager for Foojay. He is a regular international speaker on mostly Java-related conferences like JavaOne, Devnexus, Devoxx, Jfokus, JavaZone and many more. Besides all that, Brian is a military reserve for the Royal Netherlands Air Force and a Taekwondo Master / Teacher.\n\n"
}
]
},
{
"intendedAudience": "All,",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "Are you a leader or a team member and want to get the best from yourself and others, as well as keep everyone happy and safe at work? Then you don’t want to miss out on this!\nYou’ll get insight into how psychological safety is related to economic efficiency for organizations, as well as some great ideas that will benefit not only the team, but the entire company.\nI will share stories about psychological safety. Can you guess which ones are fake and which ones are based on real-life events?",
"title": "Is  psychological safety just a lie?",
"startTime": "2023-09-07T10:50",
"endTime": "2023-09-07T11:10",
"room": "Room 2",
"video": "861984071",
"startTimeZulu": "2023-09-07T08:50:00Z",
"endTimeZulu": "2023-09-07T09:10:00Z",
"id": "82428c41-353f-4d0f-900e-e2f3fa126fe2",
"sessionId": "82428c41-353f-4d0f-900e-e2f3fa126fe2",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T10:20",
"startSlotZulu": "2023-09-07T08:20:00Z",
"speakers": [
{
"name": "Malaz Alkoj",
"twitter": "",
"bio": "Full Stack developer with experience in both government and private sectors. Also both in Norway and other countries. Educated as Software engineer from BAATH university. "
}
]
},
{
"intendedAudience": "Alle og enhver. Både de som har knekt prod, de som er redde for at de en dag gjør det, og de som er redde for at kollegene deres en dag knekker prod.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Aleks opplevde å knekke prod i en av Norges største nettbanker. \nMed følelsen av å være en bankraner, var han forberedt på å miste jobben. \n\nI en stirrekamp med avgrunnen, i møtet med en utstrakt arm fra den siste personen han forventet skulle hjelpe ham. ",
"title": "Da jeg knakk prod og det nesten knakk meg",
"startTime": "2023-09-07T10:40",
"endTime": "2023-09-07T10:50",
"room": "Room 2",
"video": "861982940",
"startTimeZulu": "2023-09-07T08:40:00Z",
"endTimeZulu": "2023-09-07T08:50:00Z",
"id": "82da2120-2d56-43a7-984f-19c85a9935c3",
"sessionId": "82da2120-2d56-43a7-984f-19c85a9935c3",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T10:20",
"startSlotZulu": "2023-09-07T08:20:00Z",
"speakers": [
{
"name": "Aleks Gisvold",
"twitter": "gisvold",
"bio": "Aleks Gisvold er et smidighode, CDFO og teamleder i Aztek. \nHan brenner for smidige arbeidsmetoder, omtanke, samarbeid og diskgolf. Gjennom sine 10 år i bransjen har han opplevd større nedturer og større suksesser enn mange opplever i løpet av en karriere. Heldigvis er han glad i berg-og-dalbaner. Aleks er en aktiv kommentator i det norske utviklermiljøet, og tar ofte til ordet, særlig dersom det handler om å skape inklusjon."
}
]
},
{
"intendedAudience": "Utviklere og designere som ønsker å lære om hverandre",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Hva skjer når UX og utvikling går hånd i hånd? Er det som å se en giraff og en flodhest danse tango sammen? Eller kan det faktisk føre til lykkelige brukere og økt suksess for virksomheten? Hvordan kan vi jobbe sammen for å identifisere og løse problemer knyttet til brukeropplevelser, og hvordan kan dette føre til mer effektive produkter og tjenester? Bli med og lær om hvordan samarbeidet mellom UX og utviklere kan føre til en episk dansefest på kontoret!",
"title": "Når UX og utvikling forenes: Om å skape lykkelige og effektive opplevelser",
"startTime": "2023-09-07T09:30",
"endTime": "2023-09-07T09:50",
"room": "Room 2",
"video": "861948177",
"startTimeZulu": "2023-09-07T07:30:00Z",
"endTimeZulu": "2023-09-07T07:50:00Z",
"id": "b313a2a7-e84a-4464-a6c5-fa8830ab3959",
"sessionId": "b313a2a7-e84a-4464-a6c5-fa8830ab3959",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T09:00",
"startSlotZulu": "2023-09-07T07:00:00Z",
"speakers": [
{
"name": "Nadia Tokerud",
"twitter": "nakrissimo",
"bio": "Nadia er UX Lead i Buypass og har en lidenskap for å skape brukervennlige og helhetlige digitale opplevelser. Hun omtaler seg selv tidvis som en samarbeidsekstremist, og mener de beste løsningene kommer når man leker sammen på tvers av fagfelt."
}
]
},
{
"intendedAudience": "Alle som er redd for å bli avslørt som en bedrager. Og alle som jobber med de.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Bedragersyndromet er kanskje noe alle har kjent på en gang i livet, men hva er det og hvordan oppstår det egentlig? Jeg vil ta et lite dykk i ulike temaer som hvordan det ble oppdaget, hvilke trekk en bedrager kanskje har, ulike typer bedragere som finnes og litt til. Kan man gjøre noe for å føle seg mindre som en bedrager eller bli kvitt det? Dette skal alt krydres med egne erfaringer fra SpareBank 1 Utvikling.",
"title": "Det er en bedrager blant oss",
"room": "Room 5",
"startTime": "2023-09-07T11:40",
"endTime": "2023-09-07T12:25",
"video": "863082656",
"startTimeZulu": "2023-09-07T09:40:00Z",
"endTimeZulu": "2023-09-07T10:25:00Z",
"id": "2f46feed-c5e2-4032-a54f-819a792ac80e",
"sessionId": "2f46feed-c5e2-4032-a54f-819a792ac80e",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T11:40",
"startSlotZulu": "2023-09-07T09:40:00Z",
"speakers": [
{
"name": "Karianne Kristiansen",
"twitter": "",
"bio": "Karianne har jobbet som utvikler i SpareBank 1 Utvikling i litt over tre år. Selvom hun er ansatt som fullstackutvikler, er det frontend og UU hun finner mest engasjerende. I løpet av de siste årene har også psykisk helse på arbeidsplassen blitt et område hun ønsker å sette fokus på."
}
]
},
{
"intendedAudience": "Anyone interessert in doing anything Data Science related, Data Scientist, Leaders, Teamleads, Techleads, Data managers etc.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "AI har potensiale til å skape stor verdi for din bedrift, men mange data scientister har en akademisk bakgrunn som fokuserte mer på forskningsresultater enn på kodekvalitet og dokumentasjon.\nSom et resultat ser man ofte kode som er dårlig dokumentert, utestet og vanskelig å lese, noe som skaper teknisk gjeld og frustrasjon i data science-miljøene. Dette kan i verste fall føre til at bedriftene mister verdifulle data og kunder, samt at de bruker mye tid og ressurser på å rydde opp i kodeproblemer.\nHeldigvis kan data science lære av sine kollegaer innen programvareutvikling, der fokuset ligger på kodekvalitet, dokumentasjon, testing og å gjøre koden forståelig for andre.\nI denne talken vil du lære hvordan du kan unngå vanlige feil og jobbe på en forsvarlig måte med AI i din bedrift",
"title": "Data Science og Programvareutvikling: Når venner blir bestevenner",
"room": "Room 3",
"startTime": "2023-09-06T09:00",
"endTime": "2023-09-06T09:45",
"video": "861598350",
"startTimeZulu": "2023-09-06T07:00:00Z",
"endTimeZulu": "2023-09-06T07:45:00Z",
"id": "a6a11d02-5e53-4a65-af54-3433f0e0ee99",
"sessionId": "a6a11d02-5e53-4a65-af54-3433f0e0ee99",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T09:00",
"startSlotZulu": "2023-09-06T07:00:00Z",
"speakers": [
{
"name": "Alexander Amiri",
"twitter": "",
"bio": "Alexander Amiri is a data Scientist at Ruter with a degree in astrophysics and a keen interest in anything tech. He has previously worked at NAV as a data scientist and team lead and more recently worked in the financial world at Nordea as a data scientist. He has a broad technical background and is driven to create a better world by letting computers do most of the work."
}
]
},
{
"intendedAudience": "Any Java developer willing to follow the main projects currently been under development in the OpenJDK. ",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "The two years realese cadence of LTS versions gives the entire Java ecosystem the opportunity to accelerate. It also brings the new features Java has to offer faster, so that developers can improve applications readability, maintainability as well as development productivity. \nThis presentation covers three of the new features of the JDK: Amber, Loom and Valhalla. Amber is about bringing pattern matching to the Java language. It's currently added bit by bit: records, pattern matching for instanceof, for switch, and record pattern matching. Loom offers a new concurrent programming model. With Loom, you can write your code in a synchronous way, without any callback, and execute it asynchronously, with all the performances benefits you may expect. It is a preview feature of the JDK 19. Valhalla brings a new kind of objects to the language, so that you do not have to choose between performances and abstraction. Valhalla will add user defined primitive types to the Java language as well value types.",
"title": "From Java 17 to 21 and beyond: Loom, Amber and Valhalla",
"room": "Room 4",
"startTime": "2023-09-06T10:20",
"endTime": "2023-09-06T11:20",
"video": "861599116",
"startTimeZulu": "2023-09-06T08:20:00Z",
"endTimeZulu": "2023-09-06T09:20:00Z",
"id": "6e34ec29-e444-4851-bde1-da728f5d304e",
"sessionId": "6e34ec29-e444-4851-bde1-da728f5d304e",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T10:20",
"startSlotZulu": "2023-09-06T08:20:00Z",
"speakers": [
{
"name": "José Paumard",
"twitter": "JosePaumard",
"bio": "José works as Java Developer Advocate at Oracle. PhD in applied maths and computer science, assistant professor at the University Sorbonne Paris Nord for 25 years, he is a Java Champion Alumni and JavaOne Rockstar. He is a member of the french Paris Java User Group, has been a co-organizer of the conference Devoxx France, and is a disorganizer of JChateau, an unconference held in the Chateau of the Loire Valley. He works on the dev.java documentation and community website, publishes the JEP Café, a monthly video cast on YouTube, and maintains a french YouTube channel with more than 80 hours of Java courses. He is also a Pluralsight author in the Java space."
}
]
},
{
"intendedAudience": "This talk is suitable for anyone who is interested in system architecture, distributed databases, scalability, and optimization. As an attendee, you will learn numerous tricks for working with large-scale distributed databases and developing your own solutions on top of FoundationDB, such as: setting up tiered storage to enable both high write throughput and high read throughput, introducing sharding and versioning schemes, using custom compression, reducing your code's garbage collection footprint, and doing preaggregation with segment trees. No familiarity with any particular language is needed. Some familiarity with databases in general will be helpful.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Cognite's customers have trillions of historical data points from industrial sensors, and they are continuously ingesting new values into millions of time series in real time. This talk will explore the architecture of Cognite's custom timeseries database, showing how it is able to query both real time data and terabytes of historical data with low latency while still providing strong consistency and high availability. The talk takes place at a conceptual level (no code), and is structured as a series of challenges that we faced during development and the solution we arrived at for each challenge.",
"title": "Developing a Large-Scale Time Series Database",
"room": "Room 7",
"startTime": "2023-09-06T15:40",
"endTime": "2023-09-06T16:40",
"video": "861703795",
"startTimeZulu": "2023-09-06T13:40:00Z",
"endTimeZulu": "2023-09-06T14:40:00Z",
"id": "85eae038-49b5-4f32-83c6-0779d252d0e9",
"sessionId": "85eae038-49b5-4f32-83c6-0779d252d0e9",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T15:40",
"startSlotZulu": "2023-09-06T13:40:00Z",
"speakers": [
{
"name": "Åsmund Eldhuset",
"twitter": "@AasmundEldhuset",
"bio": "Åsmund Eldhuset is a geek of all trades who got a taste for teaching and public speaking when he TA'd the algorithms class at the Norwegian University of Science and Technology. After seven years in Silicon Valley, he is back home in Norway, and is now a Principal Software Engineer at Cognite."
}
]
},
{
"intendedAudience": "I expect this session to be attended by people who tried building a few simple applications with GraalVM, or heard some things about it, but not sure about how to approach it for production applications, where you need testing, libraries, monitoring, reflection. My plan is to show demos making each of those aspects work.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "You've probably heard of GraalVM, and how great it is for reducing startup time and memory consumption of Java applications, but do you know how to use it to the fullest in practice? In this hands-on session you'll see how to develop and test Native Image applications, optimize their performance, configure them to use popular libraries, monitor, and more!",
"title": "Supercharge your GraalVM applications in 5 steps",
"startTime": "2023-09-07T13:00",
"endTime": "2023-09-07T13:45",
"room": "Room 4",
"video": "862028728",
"startTimeZulu": "2023-09-07T11:00:00Z",
"endTimeZulu": "2023-09-07T11:45:00Z",
"id": "33cda76f-9a11-4b57-ac82-bdbcca533b9f",
"sessionId": "33cda76f-9a11-4b57-ac82-bdbcca533b9f",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T13:00",
"startSlotZulu": "2023-09-07T11:00:00Z",
"speakers": [
{
"name": "Alina Yurenko",
"twitter": "alina_yurenko",
"bio": "Alina is a developer advocate for GraalVM at Oracle Labs, a research & development organization at Oracle. A big believer in open source and communities, community organizer in the past. Love both programming & natural languages."
}
]
},
{
"intendedAudience": "Architects and developers who are considering moving into, out of, or across the cloud, and would like practical advice based on experience.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "More and more systems are moving into the cloud, with Amazon Web Services, Azure, and Google Cloud Platform as the leading providers. Should you, too, be in the cloud? If so, how should you design your systems to make the most of it? Which vendor is the right for you, and what are the differences?\n\nIn this presentation, you will get an overview of the essential services offered by the leading cloud providers and how to design your systems to make the most out of them. You will hear lessons learned from a decade of running systems in the cloud, and get pointers on whether you should be in the cloud at all, how to pick your cloud, and whether you should be using multiple clouds.",
"title": "Across the Clouds",
"room": "Room 6",
"startTime": "2023-09-07T13:00",
"endTime": "2023-09-07T14:00",
"video": "862025265",
"startTimeZulu": "2023-09-07T11:00:00Z",
"endTimeZulu": "2023-09-07T12:00:00Z",
"id": "06b3800c-9155-44a4-8cdb-35b2fc4df160",
"sessionId": "06b3800c-9155-44a4-8cdb-35b2fc4df160",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T13:00",
"startSlotZulu": "2023-09-07T11:00:00Z",
"speakers": [
{
"name": "Markus Krüger",
"twitter": "markusbk",
"bio": "Markus Krüger has worked as a developer, architect, and tech lead for over 20 years, and has been running systems on various cloud platforms since 2011. He has held several presentations at JavaZone and other venues previously, on various subjects such as performance testing, scheduling, scaling systems, and cloud architectures. He likes making large things go fast."
}
]
},
{
"intendedAudience": "everyone, you will learn about discrimination and get a proposal to fix it.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Marion and Dennie join forces to shed light on the issue of discrimination and its effects on individuals. Marion, a French immigrant residing in Norway, and Dennie, an individual on the autism spectrum without a paid job, both appear to be \"integrated\" into society. However, despite their seemingly \"normal\" appearances and situations, they face microaggressions on a daily basis. Dennie frequently hears the statement that anyone can have a paid job if they want, while Marion is told there is no gender gap, no culture privileges or glass ceiling. This serves as a reminder that discrimination can occur even to those who may not fit the traditional stereotype of a minority.\n\nThrough sharing their experiences, Marion and Dennie aim to raise awareness and promote empathy towards those who face discrimination. They also aim to teach people how to recognize and combat microaggressions, encouraging a more inclusive and equitable society for all.",
"title": "Embark on Your Diversity Allyship Journey  to End Discrimination in Tech",
"room": "Room 5",
"startTime": "2023-09-06T15:40",
"endTime": "2023-09-06T16:40",
"video": "861704867",
"startTimeZulu": "2023-09-06T13:40:00Z",
"endTimeZulu": "2023-09-06T14:40:00Z",
"id": "22cda0b8-7244-4748-8d1b-7ae034a1819b",
"sessionId": "22cda0b8-7244-4748-8d1b-7ae034a1819b",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T15:40",
"startSlotZulu": "2023-09-06T13:40:00Z",
"speakers": [
{
"name": "Marion Løken",
"twitter": "",
"bio": "Empathic digitalization leader. Helping organizations being digital and data-empowered with analytics and technology. Engaged to make our workplaces inclusive and safe."
},
{
"name": "Dennie Declercq",
"twitter": "@DennieDeclercq",
"bio": "Dennie is Microsoft MVP Developer Technologies and has experience in accessibility with Microsoft technologies. In daily life Dennie is president and developer at DDSoft, a nonprofit that connects IT to People who are less tech-savvy. Dennie invented technical solutions and systems to help people with disabilities to participate in their daily life. Thanks to his autism he's the right man at the right spot to contribute as a volunteer in function of people with disabilities."
}
]
},
{
"intendedAudience": "Presentasjonen er rettet mot alle som jobber med kode og/eller arkitektur, og som er opptatt av endringsdyktig software.",
"length": "60",
"format": "presentation",
"language": "no",
"abstract": "Regelmessig hører jeg utviklere uttrykke ønske om å lage nye microtjenester. Enten ved å implementere nye features som noe selvstendig, eller skille ut eksisterende funksjonalitet som noe eget.\n\nMen er dette alltid det rette valget, hvilke andre alternativer har en, og hva er prosessen for å komme frem til et valg? Jeg har nå begynt å spørre meg selv og andre; har microtjenester nå blitt den nye \"silver bullet\"?\n\nJeg ønsker at publikum skal stoppe opp litt, reflektere litt over hvordan arkitekturen ser ut hos dem, og i hvilken grad valgene deres tar de nærmere målet sitt eller bare et steg til siden (same-but-different). \n\nPåstand: Jeg opplever samtidig at vi snakker stadig mindre om arkitektur, samtidig som arkitektur bare blir viktigere i en microtjenesteverden.",
"title": "Er microtjenester den nye \"silver bullet\"?",
"room": "Room 4",
"startTime": "2023-09-07T15:40",
"endTime": "2023-09-07T16:40",
"video": "862082860",
"startTimeZulu": "2023-09-07T13:40:00Z",
"endTimeZulu": "2023-09-07T14:40:00Z",
"id": "c0e112d8-a229-4339-8f22-412f6d12fadb",
"sessionId": "c0e112d8-a229-4339-8f22-412f6d12fadb",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T15:40",
"startSlotZulu": "2023-09-07T13:40:00Z",
"speakers": [
{
"name": "Terje Heen",
"twitter": "tlheen",
"bio": "Utvikler hos NAV med 20+ års erfaring.\n\nJeg har i mer enn 10 år fokusert på design av applikasjonsarkitektur, med særlig fokus på package-by-feature fremfor package-by-layers. Dette er noe jeg fremdeles opplever som uvant/ukjent hos mange. De senere år har jeg også begynt å kombinere det mer aktivt med Domain-driven-design og Hexaortogonal arkitektur eller Ports and adapters. Jeg opplever at det gir en god separation-of-concerns, og god endringsevne."
}
]
},
{
"intendedAudience": "Enthusiastic developers and product managers which feel hindered daily in their attempt to make great software. ",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "The consultancy business and similar entities frequently use the term best practice - but let us sit back and analyse the major flaws in the idea and concept for a bit. Because it is utterly meaningless.   ",
"title": "Best practice -WTF!",
"room": "Room 2",
"startTime": "2023-09-06T17:20",
"endTime": "2023-09-06T17:30",
"video": "861729230",
"startTimeZulu": "2023-09-06T15:20:00Z",
"endTimeZulu": "2023-09-06T15:30:00Z",
"id": "c8f202c7-791d-405d-9ce9-f3e5d884e26d",
"sessionId": "c8f202c7-791d-405d-9ce9-f3e5d884e26d",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T17:00",
"startSlotZulu": "2023-09-06T15:00:00Z",
"speakers": [
{
"name": "Totto",
"twitter": "@javatotto",
"bio": "https://desktop.quadim.ai/profile/0fd4a1b7-5b1d-46c6-93bd-d8ef657f7f07"
}
]
},
{
"intendedAudience": "Ledere eller ikke-tekniske\nTeknologer på overordnet nivå\nTeknologer som jobber med tekniske detaljer\nFolk flest er opptatt av kulturbygging og cybersikkerhet",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "God sikkerhetskultur er utfordrende å bygge. Vi vil gjerne dele våre seire, nederlag og drømmer om reisen til god sikkerhetskultur i Oslo Origo, Oslo kommunes digitaliseringsetat.\nNoen reisetips vi deler i innlegget utover å ta med sikkerhetspass, penger og digitaliseringspiller er:\n* Hvordan skape engasjement for sikkerhets gjennom entusiasme, medvirkning og ikke tvang.\n* Hva skal til for å få utviklingsteam til å ansvar for sikkerheten i egen produktutvikling.\n* Hvordan skape en kultur som deler og lærer av feil.\n* Hva sikkerhetsteamets rolle er innen rådgivning, sikkerhetstesting og støtte.\n* Hvordan få til “Sikkerhetstesting as a Service”.\n* Hvilke forbedringspunkter har vi, og hvor vil vi være i fremtiden.",
"title": "From zero to hero: en reise i sikkerhetskultur for Oslo Origo",
"room": "Room 7",
"startTime": "2023-09-07T09:00",
"endTime": "2023-09-07T09:45",
"video": "861950296",
"startTimeZulu": "2023-09-07T07:00:00Z",
"endTimeZulu": "2023-09-07T07:45:00Z",
"id": "60ee8480-2ae4-487f-8d58-a511e0e4d753",
"sessionId": "60ee8480-2ae4-487f-8d58-a511e0e4d753",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T09:00",
"startSlotZulu": "2023-09-07T07:00:00Z",
"speakers": [
{
"name": "Rune Schumann",
"twitter": "@rune7mann",
"bio": "Rune Schumann er ansatt som principal security engineer i Oslo Origo, Oslo kommunes digitaliseringsetat. Rune har lang erfaring som sikkerhetsspesialist, rådgiver, virksomhetsarkitekt, løsningsarkitekt og systemutvikler både i offentlig og privat sektor. Den faglige interessen de siste 10 årene har dreid seg mer over mot informasjonssikkerhet og personvern, men med et holistisk syn på sikkerheten. Rune er opptatt av samspillet mellom teknologi, mennesker og organisasjon som forutsetninger til et godt informasjonssikkerhet og personvern."
},
{
"name": "Ingvild Løes Nilsson",
"twitter": "",
"bio": "Ingvild Løes Nilsson er seniorkonsulent i EY og har erfaring som sikkerhetstester og sikkerhetsrådgiver fra en rekke prosjekter. Ingvild har fra januar 2022 vært fast hos Origo som sikkerhetstester og -rådgiver. Hun er utdannet sivilingeniør i kommunikasjonsteknologi ved Norges Teknisk-Naturvitenskapelig Universitet (NTNU) i Trondheim. Ingvild har også sertifiseringene Offensive Security Certified Professional (OSCP), AWS Certified Solutions Architect Associate og ITIL Foundation."
}
]
},
{
"intendedAudience": "Intermediate Java experience recommended\n\nWe will get some exposure to OpenJDK development loop and make friends with Jtreg test harness\nThis session is useful for anyone considering to become a contributor on OpenJDK.",
"length": "120",
"format": "workshop",
"language": "en",
"abstract": "One of the recurrent themes from my other compiler workshops is “can you debug it ?” , so in an attempt at getting that sorted I simply ended up running and writing tests ! No spoilers, we will all discover together whether or not I got that debugger attached :) \n\nSo in my series of hacking on the Java compiler, we will focus this session on adding simple syntax updates and explore Jtreg the unit/regression test harness for OpenJDK.",
"title": "A little taste of testing the Java compiler",
"workshopPrerequisites": "Necessary Tools\n- GitHub account , I will give the audience access to Codespaces with ready environments so we spend more time on the code and less time on the local setup ceremonials\n\n- Codespaces can be used with different IDEs, but the main fallback is VSCode in the browser, so getting acquainted with that would be appreciated\n",
"room": "Workshop A",
"startTime": "2023-09-05T15:45",
"endTime": "2023-09-05T17:45",
"registerLoc": "https://moosehead.javazone.no/#/register/a_little_taste_of_testing_the_java_compiler",
"startTimeZulu": "2023-09-05T13:45:00Z",
"endTimeZulu": "2023-09-05T15:45:00Z",
"id": "d6afaf53-c57b-4f86-b186-e5ccc1a37011",
"sessionId": "d6afaf53-c57b-4f86-b186-e5ccc1a37011",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T15:45",
"startSlotZulu": "2023-09-05T13:45:00Z",
"speakers": [
{
"name": "Hasnae Rehioui",
"twitter": "@dviqueen",
"bio": "Staff Engineer at SafetyCulture, once upon a time maker of Confluence at Atlassian.\n\nHasnae is known as viqueen among her peers in Sydney, she is a Java architect by trade and a Maven connoisseur; yet with her 16 years of Java experience, Hasnae decided to keep life interesting by spicing things up a bit and switching to new tech stacks such as Node/Typescript and Golang."
}
]
},
{
"intendedAudience": "Passer alle utviklere uavhengig av erfaringsnivå, ingen forkunnskaper er nødvendig",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Hva er verdien av å tilgjengeliggjøre data i det øyeblikket de faktisk oppstår ?  Å motta viktig informasjon før det er for seint ? For Politiet kan dette i ytterste konsekvens utgjøre forskjellen mellom liv og død.\n\nVi, i Politiets IT-enhet, har fått et nytt verktøy i verktøykassa vår, og det\nheter Apache Kafka. Vi startet med en gammel monolitt, og forvandlet denne \ntil moderne løsning med bruk av hendelser og mikrotjenester. Fra starten av hadde vi mål om økt ytelse og skalerbarhet, men lite visste vi at det var på helt andre områder den store gevinsten egentlig skulle komme.\n\nOg det er nettopp dette foredraget skal handle om.\nOm hvordan vi bruker Kafka topics som \"samlebånd\" for enklere kunne dele data på tvers av applikasjoner og fagområder. Apache Kafka er i dag en kjent teknologi for mange, og har kanskje sitt største største bruksområde innenfor prosessering av store datamengder. I dette foredraget ser vi på alternative bruksområder, og fortelle om våre erfaringer med Kafka som et verktøy for å dele data på tvers av mikrotjenester.",
"title": "Catching Criminals with help of Apache Kafka",
"room": "Room 1",
"startTime": "2023-09-07T15:40",
"endTime": "2023-09-07T16:25",
"video": "862064425",
"startTimeZulu": "2023-09-07T13:40:00Z",
"endTimeZulu": "2023-09-07T14:25:00Z",
"id": "fdb32c4e-4fe8-41b0-a99c-ad00a9e6d03e",
"sessionId": "fdb32c4e-4fe8-41b0-a99c-ad00a9e6d03e",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T15:40",
"startSlotZulu": "2023-09-07T13:40:00Z",
"speakers": [
{
"name": "Rune Liseth Høivik",
"twitter": "",
"bio": "Rune er en lidenskapelig utvikler som elsker å løse komplekse problemer med enkle løsninger. Han er stolt utvikler i Politiets IT-enhet og motiveres av å forbedre og effektivisere det norske politiet. Rune har vært sentral i arbeidet med å ta i bruk Apache Kafka i Politiets IT-enhet og han gleder seg til å dele sine erfaringer med å utvikle løsninger basert på Kafka. Etter arbeidstid liker han best å være med familie og venner, brygge øl og løpe lange turer i østmarka."
}
]
},
{
"intendedAudience": "Utviklere som vil levere kode raskt, med høy kvalitet og samtidig med hvilepuls. Og ledere og team ledere som lurer på hvordan det fungerer i praksis",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Høsten 2021 bestemte SpareBank 1 Utvikling seg for å prøve en ny retning gjennom å kraftig øke hvor mye man jobber sammen i par. Dette har vist seg avgjørende for å oppnå økt fart, fokus og flyt. Samtidig har vi redusert antall feil og fått mer fornøyde utviklere. For å forstå effekten av parprogrammeringen har vi jobbet tett med forskere i SINTEF mens vi implementerte den nye strategien. \n\nI dette foredraget vil vi forklare  hvordan vi gikk frem for å endre kursen, hvordan vi testet ut kontinuerlig prodsetting med bittesmå endringer og hvordan antallet produksjonssettinger økte dramatisk. Du vil lære om de positive effektene vi opplevde på pull requests, testmiljøer, parprogrammering, kvalitet, fokus, flyt, psykologisk trygghet m.fl - uten at det var et mål i seg selv.\n\nEn viktig inspirasjonskilde er NAV som også har fokusert på parprogrammering og ikke bruker annet testmiljø enn produksjon. Nav sluttet med pull requests og argumenterer med at det  samtidig gav økt kvalitet og færre feil. Kunne vi få til det samme i en bank? I denne presentasjonen skal vi dele vår erfaring og forskningsbaserte funn med nettopp dette.\n\nI denne presentasjonen  forklarer vi hvorfor produksjonssetting av små endringer mens du utvikler, har blitt avgjørende hvis du skal gjøre  moderne utvikling. \n",
"title": "Hypereffektiv flyt med parprogrammering og kontinuerlige prodsettinger i SpareBank 1",
"room": "Room 7",
"startTime": "2023-09-06T09:00",
"endTime": "2023-09-06T09:45",
"video": "861601828",
"startTimeZulu": "2023-09-06T07:00:00Z",
"endTimeZulu": "2023-09-06T07:45:00Z",
"id": "a7d74140-a07c-4b82-8d45-a90beb9b08e4",
"sessionId": "a7d74140-a07c-4b82-8d45-a90beb9b08e4",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T09:00",
"startSlotZulu": "2023-09-06T07:00:00Z",
"speakers": [
{
"name": "Asgaut Mjølne",
"twitter": "",
"bio": "Utvikler i SpareBank 1 Utvikling. Elsker svensk mörkrost kaffi og koding sammen med andre. 17 års erfaring med utvikling og parprogrammering fra bla FINN, Bouvet, TietoEVRY, Telia, egen startup Cityhotels, Sparebank1 Utvikling"
},
{
"name": "Nils Brede Moe",
"twitter": "",
"bio": "Sjefforsker på SINTEF Digital med godt humør. \nHan forsker på virtuelt arbeid, hjemmekontor, prosessforbedring, autonome team og global systemutvikling. Han jobber tett med mange globalt firma innen bransjene energi, telekom, transport og finans. I tillegg jobber han med flere konsulentfirma, programvarehus og offentlig sektor i Norge. Han har også en forskningsstilling ved Blekinge Institute of Technology i Sverige og er fast spaltist i e24.\n"
}
]
},
{
"intendedAudience": "Workshopen passer for alle som er opptatt av hvordan man jobber sammen som team, og workshopen passer enten man har prøvd mobb-programmering før eller ikke. Det er heller ikke påkrevd med noen bestemt programmeringsbakgrunn eller erfaring, men det kan være en fordel å ha erfaring med å jobbe i team med utvikling.",
"length": "120",
"format": "workshop",
"language": "no",
"abstract": "Mobb-programmering er når hele teamet jobber om å løse én oppgave, på samme tid, samme sted og med én datamaskin. Det høres kanskje merkelig ut om man ikke har prøvd arbeidsmåten før, men vi mener at mobb-programmering skaper drivkraft og fokus, gir god kvalitet, og gjør teamet mer robust. \n\nOpplevelsen av mobb-programmering blir ulik avhengig av personene i mobben og oppgaven som skal løses. I denne workshopen vil vi derfor utforske og reflektere over mangfold og utfordringer man kan erfare i en mobb.\n\nFørst gir vi en introduksjon til mobb-programmering, og vi definerer noen enkle kjøreregler for mobben. Deretter deler vi oss i flere mobber og løser en liten programmeringsoppgave, samtidig som vi øver på å arbeide etter mobb-reglene. Det finnes ulike strategier og mønstre for samarbeid i mobb. I siste del av workshopen vil vi prøve å identifisere hvilke mønstre og strategier vi erfarte i mobben, og diskutere hvilke mønstre som kunne ha løst eventuelle utfordringer.\n\nEnten mobb-programmering er helt nytt for deg, eller du har prøvd det før og har mye å diskutere, vil du både kunne bidra og lære av denne workshopen!",
"title": "Den mangfoldige mobben",
"workshopPrerequisites": "",
"room": "Workshop D",
"startTime": "2023-09-05T15:45",
"endTime": "2023-09-05T17:45",
"registerLoc": "https://moosehead.javazone.no/#/register/den_mangfoldige_mobben",
"startTimeZulu": "2023-09-05T13:45:00Z",
"endTimeZulu": "2023-09-05T15:45:00Z",
"id": "49a61620-808c-461a-b06d-b8e252e99e01",
"sessionId": "49a61620-808c-461a-b06d-b8e252e99e01",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T15:45",
"startSlotZulu": "2023-09-05T13:45:00Z",
"speakers": [
{
"name": "Heidi Mork",
"twitter": "@heidicmork",
"bio": "Heidi jobber som utvikler i NRK, og har over 10 års erfaring fra ulike team og ulike teknologier. Hun studerte opprinnelige matematikk, men oppdaget på et tidspunkt at programmering var minst like gøy, og har vært en entusiastisk programmerer siden. Heidi er interessert i det meste som har med programmering å gjøre og lar ikke muligheten til å lære noe nytt gå fra seg."
},
{
"name": "Lars Jørgen Tvedt",
"twitter": "",
"bio": "Systemutviklar i 30 år, mykje backend dei siste åra. Har jobba både inhouse og som konsulent, og på mange ulike prosjekt og med masse ulik teknologi."
}
]
},
{
"intendedAudience": "This session is for people working in cloud environments and want to design a robust and flexible system using modern cloud technologies without a big operations team. No particular experience is required.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "As we started to work on the software for Bobsled, we quickly came to the fundamental question: Which core concepts should we apply for our architecture?\nSome of us had very positive experience running Kubernetes operators and liked the concept of the reconciliation pattern, since their robustness and flexibility. But we wanted a leaner approach, which does not include the usual operational burden of running Kubernetes at a scale which would suit our team size a bit better. Especially since our application runs in three clouds simultaneously and for a number of databases, we didn't wanted a lot of pre-provisioned compute infrastructure running the whole time. So we quickly settled on serverless functions and the idea of borrowing a lot of concepts from Kubernetes. \n\nOk, but how did we got all those wishes covered?\nIn this session, we talk about the reconciliation programming pattern and we will have a closer look at how Kubernetes operators are implemented. Then we will check out how we will gain those benefits outside of Kubernetes land and see how some smart choices of technology like firebase, cloud functions and pub/sub helped us implement this.\n\nOn top of that we will discuss how OIDC and OAuth will help us implement a truly cross cloud app that is not only nice and shiny, but also very secure and does not have the need to store a single password.",
"title": "How we build a scalable and truly cross cloud SaaS without Kubernetes",
"room": "Room 1",
"startTime": "2023-09-06T10:20",
"endTime": "2023-09-06T11:20",
"video": "861606511",
"startTimeZulu": "2023-09-06T08:20:00Z",
"endTimeZulu": "2023-09-06T09:20:00Z",
"id": "7576afbf-4260-407a-8a1c-60a837de1a3d",
"sessionId": "7576afbf-4260-407a-8a1c-60a837de1a3d",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T10:20",
"startSlotZulu": "2023-09-06T08:20:00Z",
"speakers": [
{
"name": "Johannes Unterstein",
"twitter": "unterstein",
"bio": ""
}
]
},
{
"intendedAudience": "Architects and engineers working with Kafka. This is useful for anyone wanting to understand how to set up a data pipeline centered around Kafka providing resilience in a production environment with real-time processing. Some basic Kafka knowledge from before is useful but not required.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "It might not be what first comes to mind that shipping goods and processing data have some similar traits and challenges but I am telling you that is absolutely the case. Among these are the stress of delivering on time, all the weird requirements for how to deliver their precious package, and the absolute havoc a single error can have.\n\nIt can be daunting to have many needs to cater to and face the risk of errors that can halt entire operations. However, some actions can be taken when setting up the architecture to minimize this. \n\nI will walk you through how we have divided our pipeline on an architecture level through Apache Kafka and on a software level through threads to handle backpressure and other failure scenarios. We have successfully used this design for years but as with all designs, it has its limitations. I’ll share both the good and the bad of this design. Finally, it’s not enough to talk about dividing a pipeline without talking about what this actually means and how you define your division because it’s not as obvious as it might first seem. You will be introduced to what the terms “tenant” and “tenant isolation” can mean in this context.\n",
"title": "Streaming without stress: flexibility and error-handling in a data distribution pipeline",
"room": "Room 3",
"startTime": "2023-09-07T13:00",
"endTime": "2023-09-07T13:45",
"video": "862030881",
"startTimeZulu": "2023-09-07T11:00:00Z",
"endTimeZulu": "2023-09-07T11:45:00Z",
"id": "fac5c27a-2d7a-4d63-81eb-76879ab3a0b4",
"sessionId": "fac5c27a-2d7a-4d63-81eb-76879ab3a0b4",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T13:00",
"startSlotZulu": "2023-09-07T11:00:00Z",
"speakers": [
{
"name": "Joanna Eriksson",
"twitter": "",
"bio": "Joanna Eriksson works as a data engineer at the Norwegian company Schibsted. She holds a master's degree in Computer Science and has been working as a software engineer for almost a decade. Her career has been focused on architecture and code for JVM-based applications with big data technologies such as Kafka and Spark. Having found a true passion in data engineering she enjoys sharing this with others who want to evolve in the data engineering domain."
}
]
},
{
"intendedAudience": "Alle som drømmer om noe men kanskje ikke helt vet hva det er enda.",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Etter å ha vært leder i 15 år, fikk jeg min første jobb som programmerer. I dag kan jeg endelig si at jeg jobber med det jeg egentlig vil, men slik har ikke alltid vært. Så, hvorfor ble det slik? Dette er min historie om hvordan jeg etter mange år endelig forsto hva jeg ønsket å bruke tiden min på, og hvordan jeg gikk frem for å oppnå det. \n\nKom og hør hva jeg lærte om meg selv og hva som må til for å både forstå og akseptere ens egentlige drømmer, samt hva man kan gjøre for å nå dem. Målet med historien min er både å inspirere andre til å følge drømmene sine, men også dele erfaringer om hva slags støtte man trenger underveis i denne prosessen. Jeg har oppnådd min drøm, men jeg gjorde det ikke alene.",
"title": "Fra powerpoint til programmering - jobber du med det du egentlig vil? ",
"startTime": "2023-09-07T14:20",
"endTime": "2023-09-07T14:40",
"room": "Room 2",
"video": "862046913",
"startTimeZulu": "2023-09-07T12:20:00Z",
"endTimeZulu": "2023-09-07T12:40:00Z",
"id": "52273d93-c92f-4a19-9ac2-75e19a7dc81a",
"sessionId": "52273d93-c92f-4a19-9ac2-75e19a7dc81a",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T14:20",
"startSlotZulu": "2023-09-07T12:20:00Z",
"speakers": [
{
"name": "Jens-Christian Bjerkek",
"twitter": "",
"bio": "Jens-Christian har jobbet med ledelse av utviklere og utviklingsprosjekter i 15 år, før han for 3 år siden ble programmerer på fulltid. "
}
]
},
{
"intendedAudience": "Dette er et tema som relevant for alle - vi skal jo alle bli gamle. Bevissthet rundt digitalt utenforskap er viktig for alle som jobber innenfor IT-bransjen. Dette gjelder utviklere, forretningsutviklere, designere, UX'ere, data scientists, produkteiere, teamledere. Vi skal alle bli gamle, og kommer alle til å kjenne på både de fysiske og kognitive utfordringene alderdom medbringer. \n\nDerfor vil nok ikke digitalt sterke voksne mennesker i dag, nødvendigvis være digitalt sterke eldre i fremtiden. For digital kompetanse - det er dessverre ikke er noe vi klarer å opprettholde hele livet. \n\nDette er også et tema som har fått mye mediedekning den siste tiden, og berører hele IT-bransjen.\n\nVi holdt dette foredraget under Booster 2023 i Bergen, hvor det var en godt og blandet publikum fra ulike deler og roller i IT-bransjen. Dette var en av tilbakemeldinger, fra en test-leder:\n\n\"Rett før påske var jeg så heldig å få bli med på Booster konferansen i Bergen. Det var skikkelig gøy, inspirerende og lærerikt. For min del ble det to dager med gode presentasjoner, lyntaler og experience reports. Flere av de jeg så på inspirerte meg og fikk meg til å tenke, men det spesielt 1 som utpekte seg ekstra. Det var \"Vi skal alle bli gamle - husk å designe for eldre\". Det som fikk meg til å tenke var det de fortalte om at den oppegående digitale voksne i dag, også en dag vil bli \"akterutseilt\" og mindre digital kompetent. Vi har fokus på universell utforming i arbeidet vi gjør, men det er ikke nok. Den universelle utforming vil ikke hjelpe på de kognitive problemene som oppstår når man sakte men sikkert blir eldre og mindre digital kompetent.\"  \n",
"length": "20",
"format": "lightning-talk",
"language": "no",
"abstract": "Det snakkes mye om digitalt utenforskap. Vi har jo som mål å være digitalt inkluderende, men det finnes dessverre mange som faller utenfor. Eldre er en av gruppene hvor flere opplever digitalt utenforskap. Vi skal også bli mange flere eldre i nær fremtid: Statistisk Sentralbyrå estimerer at det i 2030 vil være flere eldre (65+) enn barn og unge (0-19), og at det i 2040 kommer til å være ca 50% flere eldre sammenlignet med i dag. En stor eldrebølge er på vei!\n\nDessverre er det mange som har oppfatningen om at eldres digitaliseringsutfordringer er et generasjonsproblem som bokstavelig talt vil dø ut. Når dagens eldre går bort, vil neste generasjon eldre være digitalt modne og kompetente, ikke sant? Dette er nok dessverre ønsketenkning. For hva skjer når neste generasjon eldre møter neste generasjon med med ny teknologi? Hva skjer når dagens unge i fremtiden møter alderdommens fysiske og kognitive anstrengelser?\n\nFor det er kjipt å bli gammel; Du får forverret syn, hørsel og mobilitet. Likevel medfører alderdom også mange usynlige endringer. Kognitive funksjonsnedsettinger kan medføre store utfordringer og påvirker blant annet korttidshukommelsen. Det blir vanskeligere å lære seg nye ting, problemløse, være effektiv og gjenta eller bevare informasjon. Alt dette påvirker eldres evne til å navigere det digitale samfunnet. \n\nVi ønsker å fremme empati og kaste lys på hva som skjer med digitale brukere når de blir eldre. Målet er å hjelpe skapere av digitale løsninger å navigere og bistå den fortsettende eldrebølgen, og konsekvensene dette kan få for vårt digitale samfunn. Vi vil dele våre erfaringer og komme med praktiske tips til hvordan designe for eldre.\n",
"title": "Vi skal alle bli gamle – husk å designe for eldre",
"startTime": "2023-09-07T09:00",
"endTime": "2023-09-07T09:20",
"room": "Room 2",
"video": "861949053",
"startTimeZulu": "2023-09-07T07:00:00Z",
"endTimeZulu": "2023-09-07T07:20:00Z",
"id": "00f77863-cc58-41b4-b092-a07668b7af53",
"sessionId": "00f77863-cc58-41b4-b092-a07668b7af53",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T09:00",
"startSlotZulu": "2023-09-07T07:00:00Z",
"speakers": [
{
"name": "Elen Haksø",
"twitter": "",
"bio": "Elen jobber i dag hos Kantega i Oslo, og er for tiden utleid til Sparebank 1. Hun har en Master i Interaksjonsjonsdesign fra Norges teknisk-naturvitenskapelige universitet, og Bachelor i Mediedesign fra Høyskolen i Volda og Griffith University. Hun har også studert UX Design hos Noroff. "
}
]
},
{
"intendedAudience": "Hvis du kan litt JavaScript eller ønsker å følge med på det nyeste.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Hvert år kommer ny funksjonalitet til språket JavaScript. La meg gå gjennom hva som er nytt i 2023.",
"title": "Nyheter i JavaScript - ES2023",
"startTime": "2023-09-07T09:20",
"endTime": "2023-09-07T09:30",
"room": "Room 2",
"video": "861948019",
"startTimeZulu": "2023-09-07T07:20:00Z",
"endTimeZulu": "2023-09-07T07:30:00Z",
"id": "96fbeec6-c7df-4e14-aa47-0c99fb789ebf",
"sessionId": "96fbeec6-c7df-4e14-aa47-0c99fb789ebf",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T09:00",
"startSlotZulu": "2023-09-07T07:00:00Z",
"speakers": [
{
"name": "Gaute Meek Olsen",
"twitter": "GauteMeekOlsen",
"bio": "Gaute jobber som utvikler i Capra. Ellers har han som regel et hobby prosjekt på siden og liker å holde seg oppdatert innenfor programmering."
}
]
},
{
"intendedAudience": "The talk is aimed for engineers that work with data.\nIt is not for beginners, because it assumes pre-existing knowledge of database concepts. However, no knowledge of ClickHouse or columnar-oriented databases is required.\n",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "An open source columnar database ClickHouse is in many ways exceptional - it is exceptionally fast, exceptionally efficient, but also, at times exceptionally confusing. \n\nIts approach to handling data goes against many principles and concepts that we use in other databases. To give some examples: its primary index doesn't index each row and doesn't guarantee uniqueness; a secondary index is used to skip data and doesn't point to specific rows; JOINS is a complex topic and transactions are supported partially, not to mention that its SQL dialect holds a couple of surprises up its sleeve. \n\nBut, all that said, if used correctly, ClickHouse is a superb solution for online analytical processing (OLAP).\n\nThe goal of this talk is to help you get the most of ClickHouse and avoid the pitfalls. We'll talk about OLAP and columnar databases. We'll touch topics of indexing, searching and disk storage. We'll look at the reasons behind the most puzzling concepts of ClickHouse, so that by the end of the talk you find them not only logical, but maybe even fascinating.\n\nIf your challenge is analysing terabytes of data - this talk is for you. If you're a data scientist looking for tools to work with big data - this talk is for you. And, of course, if you are just curious about what makes ClickHouse crazy fast - this talk is for you as well.",
"title": "ClickHouse: what is behind the fastest columnar database",
"room": "Room 4",
"startTime": "2023-09-06T09:00",
"endTime": "2023-09-06T09:45",
"video": "861597565",
"startTimeZulu": "2023-09-06T07:00:00Z",
"endTimeZulu": "2023-09-06T07:45:00Z",
"id": "620aa954-1a6c-4c8f-8525-a99735a6f2ca",
"sessionId": "620aa954-1a6c-4c8f-8525-a99735a6f2ca",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T09:00",
"startSlotZulu": "2023-09-06T07:00:00Z",
"speakers": [
{
"name": "Olena Kutsenko",
"twitter": "@OlenaKutsenko",
"bio": "Olena is a Sr. Developer Advocate at Aiven. With a background in software engineering, she's led teams and developed mission-critical applications at Nokia, HERE Technologies, and AWS. Currently, she works at Aiven where she supports developers and customers in using open-source data technologies such as Apache Kafka, ClickHouse, and OpenSearch. She is also an international public speaker and regularly present at conferences around the world. She holds AWS Developer and Solutions Architect certifications, and is also a Confluent Catalyst."
}
]
},
{
"intendedAudience": "The lecture is addressed to people involved mainly in design activities (employees and owners of creative agencies, design teams, etc). However it will be also interesting for programmers, who seek inspiration and insight.",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "If AI within seconds can create a high quality image, write a program code, or generate texts that are indistinguishable from human-made text articles then... what is the point of learning and self-development? Is there any?\n\nThe wide adoption of AI-based systems will boost productivity in many fields, but who will benefit? We will probably pay for this with an irreversible loss of competence, intellectual degradation and perhaps impossible to reproduce, accompanying mankind from the beginning of civilization, the ability to express oneself through art and creativity.\n\nBut maybe there are human qualities that are irreplaceable by AI which are worth developing and nurturing? Are products devoid of the human factor becoming our new reality, or maybe the whole AI revolution is an opportunity to start to notice and appreciate what is truly human?\n\nGoals of my talk:\n- describe changes in the creative ecosystem caused by generative AIs (current and trends),\n- consider the benefits and costs of these changes (from the perspective of the individual and the market),\n- discuss which qualities of human creative work are inimitable by AI and how it might motivates to further learning, development and building competences (from the perspective of the individual and market opportunities),\n- discuss the changes in standards of work and education that universities and business should think about,\n- consider alternative models of creative activity in a world dominated by generative AI models.",
"title": "Human factor. Design, creativity and meaning in the AI era.",
"room": "Room 3",
"startTime": "2023-09-06T14:20",
"endTime": "2023-09-06T15:20",
"video": "861701263",
"startTimeZulu": "2023-09-06T12:20:00Z",
"endTimeZulu": "2023-09-06T13:20:00Z",
"id": "4e652d2b-b51d-4da3-97c4-7729cdfcf0e2",
"sessionId": "4e652d2b-b51d-4da3-97c4-7729cdfcf0e2",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T14:20",
"startSlotZulu": "2023-09-06T12:20:00Z",
"speakers": [
{
"name": "Paweł Nowak",
"twitter": "https://twitter.com/nowy_me",
"bio": "Paweł Nowak has been working as a Service Designer and UX Designer since 2008. He is a founder of NOWY - research and design agency (2016) and a founder of WUD Silesia conference - the biggest non-profit design conference in Poland (2010). His professional mission is to design interventions and technologies that empower people and enhance their potential and competencies. He uses complexity science as a lens for gaining new perspectives in the design field. Paweł is a lecturer at the University of Social Sciences and Humanities in Katowice and Sektor 3.0 Fund startup incubation program manager. After work he supports as a mentor, teacher and designer social and educational initiatives."
}
]
},
{
"intendedAudience": "Denne talken er for alle som er avhengige av andre mennesker for at det endelige resultatet av jobben man gjør skal bli bra. Du får mest glede av talken hvis du er opptatt av, eller ønsker å reflektere over, hvilken betydning kvaliteten på relasjoner har for resultatene man oppnår i en teknologisk hverdag, og hva du selv kan gjøre for å øke kvaliteten. Utviklere, arkitekter, domeneeksperter og ledere vil alle ha nytte av denne erfaringsdelingen.",
"length": "45",
"format": "presentation",
"language": "no",
"abstract": "Gull er ettertraktet og verdifullt. Veien til gullet er ofte kronglete, kaotisk og konfliktfylt. Jeg deler noen av mine mest lærerike øyeblikk som utvikler, forsker og teknologileder - og hvordan jeg fant gull. Gullet, altså innsikten og verktøyene som du skal få, fant jeg som regel i møte med mennesker. Både fine og, noen ganger, ganske friksjonsfylte menneskemøter - ofte litt bortenfor kode, nord for rammeverk og bakenfor metodikk. Om du også innimellom støter (på) folk i jobbhverdagen din, er denne talken for deg.",
"title": " Jakten på gull - verdifull lærdom fra menneskelige sammenstøt",
"room": "Room 5",
"startTime": "2023-09-07T10:20",
"endTime": "2023-09-07T11:05",
"video": "861988293",
"startTimeZulu": "2023-09-07T08:20:00Z",
"endTimeZulu": "2023-09-07T09:05:00Z",
"id": "226dd399-249a-4bf7-b74d-d643321b78f5",
"sessionId": "226dd399-249a-4bf7-b74d-d643321b78f5",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T10:20",
"startSlotZulu": "2023-09-07T08:20:00Z",
"speakers": [
{
"name": "Heidi Brovold",
"twitter": "",
"bio": "Heidi er informatiker, teknologileder og \"design tenker\". Hun er opptatt av samspillet mellom innovasjon, læring og ledelse. Læring er forutsetningen for all utvikling. Derfor leter Heidi etter læring i de fleste situasjoner. Hun har hatt roller som utvikler, forskningsleder, prosjekt- og programleder og CTO. Nå er hun CPO i Capra Consulting."
}
]
},
{
"intendedAudience": "The talk is suitable for everyone interested in JavaScript, and especially for those who want to know how to suggest a new language feature and participate in implementing it.\nThe talk will present a step-by-step outline of what is needed to implement a (small) language feature.",
"length": "20",
"format": "lightning-talk",
"language": "en",
"abstract": "JavaScript is an actively evolving language, and its design is overseen by ECMA Technical Committee TC39. We'll explain briefly how TC39 works and give a concrete outline of the steps needed to implement a new language feature. The talk is based on our own experience implementing `array.group()` for the SpiderMonkey engine.",
"title": "How you can participate in evolving JavaScript",
"startTime": "2023-09-06T11:40",
"endTime": "2023-09-06T12:00",
"room": "Room 2",
"video": "861647808",
"startTimeZulu": "2023-09-06T09:40:00Z",
"endTimeZulu": "2023-09-06T10:00:00Z",
"id": "6ccc0670-330a-40cc-bd97-803ab9a30ff9",
"sessionId": "6ccc0670-330a-40cc-bd97-803ab9a30ff9",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T11:40",
"startSlotZulu": "2023-09-06T09:40:00Z",
"speakers": [
{
"name": "Mikhail Barash",
"twitter": "mikhail_barash",
"bio": "Mikhail is a delegate at ECMA TC39, and works as a researcher at the University of Bergen, where he leads the language tooling team of Bergen Language Design Laboratory. Mikhail enjoys designing and implementing (domain-specific) languages, and likes to inspire people to participate in impactful projects."
},
{
"name": "Rolf Martin Glomsrud",
"twitter": "",
"bio": "Rolf is a master student at the Bergen Language Design Laboratory at UiB. He has been onboard implementing the `array.group()` feature already during his bachelor degree studies. Rolf enjoys solving difficult programming challenges using modern features of programming languages."
}
]
},
{
"intendedAudience": "By attending this talk, the participants will gain a deeper understanding of the benefits and challenges associated with synthetic tabular data generation. They will learn about the different techniques used for synthetic data generation and how to evaluate the quality of generated data. The talk will also explore how federated learning benefits synthetic data generation and highlight the challenges associated with it and ways to address it. \n\nData Scientists, Software Developers with a basic understanding of data science concepts will benefit the most from this talk. Familiarity with data privacy regulations and experience working with cancer data would be an added advantage but not necessary. The talk is designed to be informative and accessible to a wide range of participants, including researchers, data scientists, software developers and healthcare professionals interested in leverage synthetic data for their own use cases. ",
"length": "60",
"format": "presentation",
"language": "en",
"abstract": "Generating Synthetic Cancer data is a critical innovation focus area at the Cancer Registry of Norway due to various benefits offered by synthetic data, such as easier data sharing for promoting cancer research, improved data quality for building better prediction models, better privacy protection for patients, and better developer productivity through easier software testing. \nIn this talk, first, I focus on different synthetic cancer data generation techniques explored at the Cancer Registry for various use cases. Additionally, I share our view on why no synthetic data generation technique is best for all our use cases. Furthermore, I talk about evaluating the quality of synthetic data and the challenges in answering the following questions:\na) How good is the generated synthetic data compared to the real data? \nb) How well does the generated synthetic data preserve patient privacy? \nc) Does the synthetic data satisfy the purpose for which it is generated? \nd) How much bias is introduced by synthetic data? Furthermore, I also talk about how federated learnings add more benefits to generating synthetic data and the new challenges that federated learning introduces regarding security, privacy, accountability and auditability. Finally, I will talk about our efforts towards addressing these challenges. \n",
"title": "Ready to Enhance your Data Practices? Explore Synthetic Data Generation for Improved Data Sharing, Data Quality, Privacy, and Developer Productivity ",
"room": "Room 3",
"startTime": "2023-09-07T11:40",
"endTime": "2023-09-07T12:40",
"video": "861989649",
"startTimeZulu": "2023-09-07T09:40:00Z",
"endTimeZulu": "2023-09-07T10:40:00Z",
"id": "c8846eb8-65ac-4ce0-a0a9-97347cc0d311",
"sessionId": "c8846eb8-65ac-4ce0-a0a9-97347cc0d311",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T11:40",
"startSlotZulu": "2023-09-07T09:40:00Z",
"speakers": [
{
"name": "Narasimha Raghavan Veeeraragavan",
"twitter": "",
"bio": "Narasimha Raghavan Veeraragavan is currently a Special Adviser with the Cancer Registry of Norway. He is a key player in delivering technical architecture and innovative solutions to continuously strengthen the security and privacy of cancer patients’ datasets in Norway. Additionally, as part of his role, he is involved in several national and international research projects and collaborates with many reputed national and international partners. Before, he led several technical initiatives in global companies. He has four patents and a few peer-reviewed research papers in reputed conferences and journals. His initiatives resulted in large-scale software products launched globally with millions of users worldwide. From 2017 to 2022, Narasimha was invited to teach courses at the Department of Informatics at the University of Oslo. The audience to his course where professional software developers working in public and private sectors who wanted to be architects. His course has received best rating from the participants.  "
}
]
},
{
"intendedAudience": "Utviklere som er interessert i kompetansebygging og hvordan et felles fagmiljø styrkes ved at vi både lærer av hverandre og lære bort til hverandre.  Og de som er interessert i å høre om at tenåringer med stor IT-interesse snart kommer til å puste dem i ryggen med et ferskt fagbrev i lommen.  Hvis det er noen rekrutterere og andre \"suits\" i salen vil de ha nytte av å høre om en ny gruppe utviklere som snart dukker opp i søkebunken deres.",
"length": "10",
"format": "lightning-talk",
"language": "no",
"abstract": "Vi trenger flere utviklere i bransjen vår, og et nytt sted å finne dem er den nye utdanningsretningen i videregående skole hvor ungdom tar to år på yrkesfaglig skole, og deretter jobber som lærlinger ute i arbeidslivet i to år. \n Utdanningen avsluttes med fagprøve og kandidatene som lykkes har da et fagbrev som systemutvikler.  De fleste kjenner til yrkesfaglig utdanning for IKT-Drift, men dette er en ny linje som retter seg mot de som ønsker å bli systemutviklere.\n\nDet første kullet har nå vært ute i lærlingjobben i ett år, og to av dem er hos oss i Statnett.  To til starter i august og vi har dermed fire lærlinger tilsammen.  På JavaZone stiller vi med en vaskekte lærling og med faglig leder for lærlingene våre.\n\nVi ønsker å fortelle om våre erfaringer med lærlinger, både hva de lærer hos oss og hvordan de bidrar til vårt fagmiljø for systemutviklere.\nVed å være med på utdanningsløpet for ungdom med stor interesse for systemutvikling og IT generelt kan vi også være med på å påvirke hva de skal lære.  Vårt mål er at disse lærlingene skal fungere som systemutviklere i våre team når de er ferdige med utdannelsen, da vil de også kunne konkurrere med utviklere med lengre utdanning og dermed bidra til å få opp antallet systemutviklere i Norge. \n \nVi tror at med god veiledning og to års erfaring som lærling vil disse være et interessant bidrag til fagmiljøene for systemutvikling.  \n",
"title": "Erfaringer med lærlinger i IT-Utviklerfaget",
"room": "Room 2",
"startTime": "2023-09-07T13:20",
"endTime": "2023-09-07T13:30",
"video": "862028839",
"startTimeZulu": "2023-09-07T11:20:00Z",
"endTimeZulu": "2023-09-07T11:30:00Z",
"id": "897a17c2-a925-44f1-bc09-97a2ae3e2d76",
"sessionId": "897a17c2-a925-44f1-bc09-97a2ae3e2d76",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T13:00",
"startSlotZulu": "2023-09-07T11:00:00Z",
"speakers": [
{
"name": "Tore Skjulstad Bryhni",
"twitter": "@torebryhni",
"bio": "Leder i dag et fagmiljø med ca 70 systemutviklere med både ansatte og innleide i Statnett, har tidligere gjort ting som ligner i Politiets IT-tjeneste.  Før dette var han utvikler i 12 år og jobbet med å lage digitale kart.  Er mest opptatt av å lage fagmiljø \"av utviklere, for utviklere\" og mener at teknisk forståelse for systemutvikling er en viktig egenskap når du leder slike fagfolk."
},
{
"name": "Mads Pettersen Vengnes",
"twitter": "",
"bio": "Mads er lærling i IT-Utviklerfaget.  Etter to år som elev på Kuben VGS innen IT-Utvikling er han nå lærling hos Statnett og vil fortelle om erfaringene derfra."
}
]
},
{
"intendedAudience": "Målgruppen for denne workshopen er utviklere som ikke har noen erfaring med OIDC fra før, eller utviklere som har litt erfaring men ønsker en dypere forståelse av flytene. ",
"length": "120",
"format": "workshop",
"language": "no",
"abstract": "Du har kanskje oppdaget at flere nettsider og apper de siste årene lar deg logge inn med tredjeparts aktører, som Google, Facebook, Vipps, osv. I denne workshopen skal vi gå gjennom hvordan dette gjøres i praksis med å ta en titt under panseret til OpenID Connect (OIDC) standarden, og hvordan den åpner opp for å gi en tredjeparts aktør ansvaret for å gjennomføre autentiseringen av brukeren.\n\nAutentisering med OIDC er noe som vanligvis bør løses med å bruke biblioteker. I denne workshopen kommer vi derimot til å gjøre alt manuelt for å få en grundig forståelse for hvordan flyten fungerer.",
"title": "Hvordan gjøres en autentisering med Openid Connect sånn egentlig",
"workshopPrerequisites": "Vi har laget støttekode i Python, men det er ingen problemer å enten løse oppgaven i et annet språk eller med cURL, Postman, osv. \nDersom du ønsker å bruke støttekoden vil vi få utnyttet tiden bedre dersom du har lastet den ned og verifisert at du får kjørt programmet på maskinen du tar med deg. \nRepoet til workshopen finner du her: \nhttps://github.com/kantega/oidc-workshop-public",
"room": "Workshop B",
"startTime": "2023-09-05T15:45",
"endTime": "2023-09-05T17:45",
"registerLoc": "https://moosehead.javazone.no/#/register/hvordan_gjores_en_autentisering_med_openid_connect_sann_egentlig",
"startTimeZulu": "2023-09-05T13:45:00Z",
"endTimeZulu": "2023-09-05T15:45:00Z",
"id": "28bf955e-43fc-45f3-b5d4-43f39a34c467",
"sessionId": "28bf955e-43fc-45f3-b5d4-43f39a34c467",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T15:45",
"startSlotZulu": "2023-09-05T13:45:00Z",
"speakers": [
{
"name": "Øyvind Kallevik Grutle",
"twitter": "",
"bio": "Øyvind er en utvikler med lidenskap for å finne ut hvem du er på nett, og har jobbet i BankID og Vipps de siste 7 årene med nettopp det."
},
{
"name": "Isak Eriksen Bjørn",
"twitter": "",
"bio": "Isak er en utvikler med variert erfaring med OAuth2.0 og OIDC. Både som utvikler på Oidc-tjenester hos BankId og Vipps og som bruker av OIDC tjenester i andre prosjekter.\nFor tiden er han å finne hos Statens Pensjonskasse der han beregner premier."
}
]
},
{
"intendedAudience": "Audience:\n- Developers/DevOps engineers working or interested with Kubernetes. \n- Basic understanding of Kubernetes.\nBenefits:\n- understanding OWASP Kubernetes Top 10\n- understanding how a series of simple misconfigurations/issues can lead to a successful cyberattack\n- \"feeling\" the danger in practise, which is much more useful than the theoretical slides\n- understanding best practices of hardening k8s clusters/workloads\n\n",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "Do you want to see live Kubernetes hacking? Come to see interactive demos where your newly registered accounts in a k8s application are hijacked.\n\nThis talk guides you through various security risks of Kubernetes, focusing on OWASP Kubernetes Top 10 list. In live demos, you will find out how to exploit a range of vulnerabilities or misconfigurations in your k8s clusters, attacking containers, pods, network, or k8s components, leading to an ultimate compromise of user accounts in an exemplary web application. \n\nYou will learn about common mistakes and vulnerabilities along with the best practices for hardening your Kubernetes systems.\n",
"title": "The Hacker's Guide to Kubernetes",
"room": "Room 7",
"startTime": "2023-09-06T17:00",
"endTime": "2023-09-06T17:45",
"video": "861717794",
"startTimeZulu": "2023-09-06T15:00:00Z",
"endTimeZulu": "2023-09-06T15:45:00Z",
"id": "7cfdea78-c212-47b5-b8e5-229eba756492",
"sessionId": "7cfdea78-c212-47b5-b8e5-229eba756492",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-06T17:00",
"startSlotZulu": "2023-09-06T15:00:00Z",
"speakers": [
{
"name": "Patrycja Wegrzynowicz",
"twitter": "yonlabs",
"bio": "Patrycja is a lead engineer at Form3, working on reliability and performance of UK payments. She is also the founder of Yon Labs, a startup focusing on automated tools for detection and refactoring of security vulnerabilities, performance anti-patterns, or cloud issues and providing consultancy in Java, C++, Go, and cloud technologies.\n \nShe is a regular speaker at software conferences, including CodeOne, JavaOne, Devoxx, JFokus, and others. She was awarded an Oracle Groundbreaker Ambassador title in 2020 and 2021. She was also named as one of Top 10 Women in Tech in Poland in 2016.\n\nHer interests focus on automated software engineering, mainly static and dynamic analysis techniques to support software verification, optimization, and deployment.\n \nThe link to the slides: https://www.slideshare.net/patrycjawegrzynowicz3/the-hackers-guide-to-kubernetes\n"
}
]
},
{
"intendedAudience": "Security Proffesionals, Architects, Tech Leads",
"length": "120",
"format": "workshop",
"language": "en",
"abstract": "“Software supply chain” is a term describing everything that happens to code from the time it leaves the developers fingers until it runs in production. The code needs to be compiled, tested, packaged and deployed, and these steps take place in a variety of systems and use lots of complex third party solutions. Our apps also depend on an increasing number of third party libraries and frameworks that we often know next to nothing about.\n\nSeveral initiatives have been started in an attempt to address the issues surrounding supply chain integrity, the most noticeable one being Supply chain Levels for Software Artifacts - SLSA. SLSA aims to be vendor neutral and is backed by major players like the Cloud Native Computing Foundation and Google in addition to startups such as Chainguard.\n\nCosign - Sigstore is a Linux Foundation project which is developing Cosign, a container signing, verification and storage in an Open Container Initiative (OCI) registry, making signatures invisible infrastructure.\nKyverno - Kyverno is a policy engine designed for Kubernetes. With Kyverno, policies are managed as Kubernetes resources and no new language is required to write policies.\n\nIn this workshop we will make a practical approach to securing your container applications and verify that the container has not been tampered with since it was built.",
"title": "Securing the Container Supply Chain Workshop",
"workshopPrerequisites": "Laptop\nGitHub account\nDocker\nLocal Kubernetes cluster\n* https://kubernetes.io/docs/tutorials/hello-minikube/\n* https://kind.sigs.k8s.io/\n* https://github.com/abiosoft/colima\n\nFull workshop install guide: https://github.com/nais/salsa-workshop/blob/main/labs/lab-0/README.md",
"room": "Workshop B",
"startTime": "2023-09-05T13:30",
"endTime": "2023-09-05T15:30",
"registerLoc": "https://moosehead.javazone.no/#/register/securing_the_container_supply_chain_workshop",
"startTimeZulu": "2023-09-05T11:30:00Z",
"endTimeZulu": "2023-09-05T13:30:00Z",
"id": "bec70ca4-6db2-49ff-9049-52a8fa0f9e7d",
"sessionId": "bec70ca4-6db2-49ff-9049-52a8fa0f9e7d",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-05T13:30",
"startSlotZulu": "2023-09-05T11:30:00Z",
"speakers": [
{
"name": "Hans Kristian Flaatten",
"twitter": "",
"bio": " Platform Engineering at the Norwegian Labour and Welfare Administration (NAV) responsible for the NAIS platform. NAIS is an application platform built to increase development speed by providing our developers at NAV with the best possible tools to develop and run their applications."
},
{
"name": "Jan-Kåre Solbakken",
"twitter": "",
"bio": "Developer with occasional strays to the security side for 20+ years, for the most part on the JVM. "
},
{
"name": "Youssef Bel Mekki",
"bio": "Platform/Devops developer at NAV. I have a pretty short career. I've started my journey late, learning development in university in an age of 30. Been working in NAV ever sense, never regretted my decision towards programming. I love working with people and with the combination of programming to come up with cleaver and user friendly solutions.",
"twitter": ""
}
]
},
{
"intendedAudience": "Java developers, or anyone running code on the JVM, who cares about performance, lowering operational costs, or carbon footprint of their service.\nThe topic is fairly advanced but I'm going to try my best to make sure anyone who is familiar with Java idioms can understand what's going on.",
"length": "45",
"format": "presentation",
"language": "en",
"abstract": "In this talk, we’ll discuss a severe JVM performance issue, the methodology leading to its discovery, how it affects the whole Java ecosystem, and what you could do to avoid it in your code.\n\nThis scalability bottleneck has existed in OpenJDK for decades; the most surprising aspect is how such an impactful problem has been undetected for so long, eluding existing diagnostic tooling, while many common idioms will trigger the issue. Chances are high that your code is affected as well.\n\nAfter explaining the issue, we’ll show some real-world code from popular OSS libraries and how we worked around the problem in several popular OSS Java libraries with simple, small changes, leading to significant efficiency improvements.",
"title": "Cracking the scalability wall",
"room": "Room 5",
"startTime": "2023-09-07T13:00",
"endTime": "2023-09-07T13:45",
"video": "862026246",
"startTimeZulu": "2023-09-07T11:00:00Z",
"endTimeZulu": "2023-09-07T11:45:00Z",
"id": "6bcfb467-baaf-4920-b11d-6603e0efd4dc",
"sessionId": "6bcfb467-baaf-4920-b11d-6603e0efd4dc",
"conferenceId": "5c979d4b-9f92-43e3-a8c2-e3de0298d8de",
"startSlot": "2023-09-07T13:00",
"startSlotZulu": "2023-09-07T11:00:00Z",
"speakers": [
{
"name": "Sanne Grinovero",
"twitter": "SanneGrinovero",
"bio": "Sanne Grinovero has been a member of the Hibernate team for about 15 years and contributed to many other OSS Java projects as well; among others he was a member of the R&D team creating Quarkus, Hibernate Search, Infinispan, Hibernate OGM, Hibernate Reactive.\n\nDeeply interested in performance and concurrency challenges around data access, scalability, and exploring integration with new storage technologies, distributed systems and search engines.\n\nHe lived in Holland, Italy, the Caribbean Islands, Chile, Portugal and currently hacks in the countryside near London.\n"
}
]
}
]
}
    "#.to_string()
}