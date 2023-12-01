use crate::types::Logo;

pub struct Job {
    pub company: &'static str,
    pub title: &'static str,
    pub dates: &'static str,
    pub logo: Logo,

    pub description: Vec<&'static str>,
    pub accomplishments: Vec<&'static str>,
    pub technologies: Vec<&'static str>,
}

impl Job {
    pub fn new() -> Vec<Self> {
        vec![
            Job {
                company: "Acklio",
                title: "Rust developer",
                dates: "March 2023 - May 2023",
                logo: Logo {
                    file: "/icons/acklio.png",
                    transparent_background: true,
                },
                description: vec!["The first usage of the SCHC framework (RFC 8724) on Rust!"],
                accomplishments: vec![
                    "Creation of Rust bindings of a C library implementing the SCHC framework",
                    "Demonstration of SCHC with applications in Rust on x86 platform",
                    "Proof of concept usage of embedded STM32 controllers exclusively in Rust",
                    "Transmission of knowledge to the technical team",
                ],
                technologies: vec!["Rust", "SCHC", "STM32 controllers", "LoRa", "LoRaWAN"],
            },
            Job {
                company: "Vélorail du Kreiz Breizh",
                title: "Freelance developer",
                dates: "August 2021 - April 2022",
                logo: Logo {
                    file: "/icons/velorail.png",
                    transparent_background: true,
                },
                description: vec![
                    "Creation of an online booking platform focused on the tourist activity of rail biking (vélorail).",
                    "During the first 5 months with the platform, 43% of the bookings were made online.",
                ],
                accomplishments: vec![
                    "Design, UX, booking and payment flow for customers",
                    "Dashboard for managers with calendar view, manual bookings, slots management",
                    "Ability to generate invoices, booking recaps for managers",
                    "Sending emails to customers and managers about bookings",
                    "Online deployment, maintenance of the service",
                ],
                technologies: vec!["Angular", "NestJS", "GraphQL", "Rust", "Stripe"],
            },
            Job {
                company: "Yaakadev",
                title: "Full-Stack developer",
                dates: "April 2021 - July 2021",
                logo: Logo {
                    file: "/icons/yaakadev.png",
                    transparent_background: false,
                },
                description: vec![
                    "Maintenance of existing projects for clients",
                    "Design, development and deployment of multiple projects from scratch:",
                ],
                accomplishments: vec![
                    "Admin dashboard of a local merchants solution",
                    "Calendar planning application with filtering and custom views",
                    "Intranet to upload and download documents",
                ],
                technologies: vec!["NodeJS", "ExpressJS", "Angular", "MongoDB", "CI/CD"],
            },
            Job {
                company: "Epitech",
                title: "Teaching assistant (AER)",
                dates: "February 2020 - April 2021, September 2022 - February 2023",
                logo: Logo {
                    file: "/icons/epitech.png",
                    transparent_background: true,
                },
                description: vec![
                    "Pedagogical supervision of three classes of students.",
                    "Conducting educational activities throughout the school year.",
                ],
                accomplishments: vec![
                    "Start of projects",
                    "Technical help and guidance",
                    "Proctoring exams",
                    "Grading students on their work",
                ],
                technologies: vec!["C", "C++", "Haskell", "Rust", "Web and mobile development"],
            },
            Job {
                company: "Ubiscale",
                title: "Embedded developer",
                dates: "August 2019 - December 2019",
                logo: Logo {
                    file: "/icons/ubiscale.png",
                    transparent_background: true,
                },
                description: vec!["Creation of a home Wifi gateway for an IoT object."],
                accomplishments: vec![
                    "Research, reverse engineering of existing products",
                    "Design and implementation.",
                ],
                technologies: vec!["C on a ESP8266 controller", "Wi-Fi", "Bluetooth"],
            },
        ]
    }
}
