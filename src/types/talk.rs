use crate::types::Link;
use rocket::uri;

pub struct Talk {
    pub title: &'static str,
    pub date: &'static str,
    pub location: &'static str,
    pub link: Link,
}

impl Talk {
    pub fn new() -> Vec<Self> {
        vec![
            Talk {
                title: "Vim",
                date: "February 2023",
                location: "Epitech Rennes",
                link: Link {
                    uri: uri!("https://philippeloctaux.com/pub/talks/vim.pdf"),
                    label: "Slides",
                },
            },
            Talk {
                title: "CLion",
                date: "March 2021",
                location: "Epitech Rennes",
                link: Link {
                    uri: uri!("https://philippeloctaux.com/pub/talks/clion.pdf"),
                    label: "Slides",
                },
            },
            Talk {
                title: "git & devops 2",
                date: "February 2021",
                location: "Epitech Rennes",
                link: Link {
                    uri: uri!("https://philippeloctaux.com/pub/talks/git-devops2.pdf"),
                    label: "Slides",
                },
            },
            Talk {
                title: "pass4thewin",
                date: "February 2021",
                location: "Epitech Rennes",
                link: Link {
                    uri: uri!("https://philippeloctaux.com/pub/talks/pass4thewin.pdf"),
                    label: "Slides",
                },
            },
            Talk {
                title: "git & devops",
                date: "May 2020",
                location: "Epitech Rennes",
                link: Link {
                    uri: uri!("https://philippeloctaux.com/pub/talks/git-devops.pdf"),
                    label: "Slides",
                },
            },
            Talk {
                title: "git gud",
                date: "May 2019",
                location: "Epitech Rennes",
                link: Link {
                    uri: uri!("https://philippeloctaux.com/pub/talks/git-tek.pdf"),
                    label: "Slides",
                },
            },
        ]
    }
}
