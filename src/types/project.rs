use crate::types::{Link, Logo};
use rocket::uri;

pub enum Position {
    Left,
    Right,
}

pub struct Image {
    pub file: &'static str,
    pub position: Position,
}
pub enum ProjectLink {
    Available(Link),
    NotAvailable,
}

pub struct ProjectWithImage {
    pub name: &'static str,
    pub tagline: &'static str,
    pub dates: &'static str,

    pub description: Vec<&'static str>,
    pub accomplishments: Vec<&'static str>,
    pub technologies: Vec<&'static str>,

    pub link: Option<ProjectLink>,
    pub logo: Option<Logo>,
    pub image: Image,
}

#[derive(Default)]
pub struct ProjectWithoutImage {
    pub name: &'static str,
    pub tagline: &'static str,
    pub dates: &'static str,

    pub description: Vec<&'static str>,
    pub accomplishments: Vec<&'static str>,
    pub technologies: Vec<&'static str>,

    pub link: Option<ProjectLink>,
    pub logo: Option<Logo>,
}

pub enum ProjectKind {
    WithImage(ProjectWithImage),
    WithoutImage((ProjectWithoutImage, ProjectWithoutImage)),
}

impl ProjectKind {
    pub fn new() -> Vec<Self> {
        vec![
            ProjectKind::WithoutImage((
                ProjectWithoutImage {
                    name: "ezidam",
                    tagline: "Identity and Access Management system",
                    dates: "Since January 2023",

                    description: vec![
                        "A simple identity and access management system for SMEs or personal use.",
                        "Low maintenance required, easy to deploy and to backup.",
                    ],
                    accomplishments: vec![
                        "Users management",
                        "Roles management",
                        "Assign users to roles and the other way around",
                        "OAuth2 / OIDC applications (code flow)",
                        "Multi-Factor Authentication (TOTP)",
                        "Password reset (via email or backup token)",
                        "Simple administration panel",
                        "Good security measures for users and administrators",
                    ],
                    technologies: vec![
                        "Rust",
                        "SQLite",
                        "OAuth2 / OIDC",
                        "TOTP",
                        "SMTP",
                        "Docker",
                    ],

                    logo: Some(Logo {
                        file: "/icons/ezidam.png",
                        transparent_background: true,
                    }),
                    ..Default::default()
                },
                ProjectWithoutImage {
                    name: "pass4thewin",
                    tagline: "Password manager",
                    dates: "November 2020 - January 2021",

                    description: vec![
                        "Port of passwordstore, the standard unix password manager on the Windows platform.",
                        "Warning! Unfinished command line application, may cause data corruption when using existing passwords.",
                    ],
                    accomplishments: vec![
                        "Creation of a store",
                        "List secrets",
                        "Decrypt secret",
                        "Insert or generate secrets",
                        "Edit existing secrets",
                        "Synchronisation with git",
                        "TOTP support",
                    ],
                    technologies: vec![
                        "Windows",
                        "Rust",
                        "OpenPGP",
                        "libgit2",
                    ],

                    link: Some(ProjectLink::Available(Link {
                        uri: uri!("https://github.com/x4m3/pass4thewin"),
                        label: "Source code",
                    })),
                    ..Default::default()
                },
            )),
            ProjectKind::WithImage(

                ProjectWithImage{
                    name: "NaviaRent",
                    tagline: "Epitech Innovative Project",
                    dates: "September 2020 - January 2023",

                    description: vec!["A B2B platform helping rentals of standup paddle boards."],
                    accomplishments: vec![
                        "DevOps of all software in the NaviaRent stack",
                        "Creation of the iOS application",
                        "Contributions to the Android application",
                        "Contributions to the backend server",
                        "Creation and contributions to the web client",
                        "Server administration, backups",
                    ],
                    technologies: vec![
                        "NodeJS",
                        "Angular",
                        "Kotlin",
                        "SwiftUI",
                        "Docker",
                        "GitLab CI/CD",
                        "Raspberry Pi",
                        "ESP32",
                    ],

                    logo: Some(Logo {
                        file: "/icons/naviarent.png",
                        transparent_background: false,
                    }),
                    image: Image {
                        file: "/images/naviarent.jpg",
                        position: Position::Right,
                    },
                    link: Some(ProjectLink::NotAvailable),
                },
            ),
            ProjectKind::WithoutImage((

                ProjectWithoutImage{
                    name: "epitok",
                    tagline: "Presence system at Epitech",
                    dates: "June 2020 - September 2020",

                    description: vec![
                        "A library and web client to simplify students presence at Epitech.",
                        "Students are handed a piece of paper with a 6 digits number (called a \"token\") to verify their presence at school events.",
                        "Teachers use epitok to scan student cards with QR codes on them instead of printing and handing tokens to students.",
                    ],
                    accomplishments: vec![
                        "Reverse engineering of a partially documented web API",
                        "Design, conception",
                        "User experience",
                        "Improvements based of usage of the application",
                    ],
                    technologies: vec![
                        "Rust",
                        "HTML",
                        "Bootstrap",
                        "jQuery",
                        "Docker",
                    ],

                    link: Some(ProjectLink::Available(Link {
                        uri: uri!("https://github.com/x4m3/epitok"),
                        label: "Source code",
                    })),
                    ..Default::default()
                },
                ProjectWithoutImage{
                    name: "epi.today",
                    tagline: "Calendar for Epitech",
                    dates: "December 2019 - February 2020",

                    description: vec![
                        "A viewer of the Epitech intranet calendar.",
                        "Students and teachers glance at their planning without the need to go on the school's intranet.",
                    ],
                    accomplishments: vec![],
                    technologies: vec![
                        "TypeScript",
                        "HTML",
                        "Bootstrap",
                        "Docker",
                    ],

                    link: Some(ProjectLink::Available(Link {
                        uri: uri!("https://github.com/x4m3/epi.today"),
                        label: "Source code",
                    })),
                    ..Default::default()
                },
            )),

            ProjectKind::WithImage(

                ProjectWithImage{
                    name: "canvas.place",
                    tagline: "Timelapse",
                    dates: "April 2017 - January 2020",

                    description: vec![
                        "canvas.place is a shared place to express creativity.",
                        "People from all over the world share one single canvas to paint on.",
                        "I created and maintained a timelapse of the virtual canvas.".into()
                    ],
                    accomplishments: vec![],
                    technologies: vec!["FFmpeg", "Shell scripting", "nginx".into()],

                    logo: Some(Logo {
                        file: "/icons/canvas.png",
                        transparent_background: false,
                    }),
                    image: Image {
                        file: "/images/canvas.png",
                        position: Position::Left,
                    },
                    link: Some(ProjectLink::Available(Link {
                        uri: uri!("https://timelapse.canvas.place"),
                        label: "Website",
                    })),
                },
            )
        ]
    }
}
