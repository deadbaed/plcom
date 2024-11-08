pub mod icon;
pub mod link;

#[derive(Clone, PartialEq)]
pub struct Date {
    pub year: u32,
    pub month: u8,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let month = match self.month {
            1 => "January",
            2 => "Februrary",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => panic!("wtf not a month"),
        };
        write!(f, "{month} {}", self.year)
    }
}

pub mod resume {
    #[derive(Clone, Copy)]
    pub struct Work {
        pub name: &'static str,
        pub position: &'static str,
        pub start_date: &'static str,
        pub end_date: Option<&'static str>,
        pub logo: Logo,
        pub description: &'static str,
        pub highlights: &'static [&'static str],
        pub technologies: &'static [&'static str],
        pub link: Option<ResumeLink>,
    }

    #[derive(Clone, Copy)]
    pub struct Logo {
        pub file: &'static str,
        pub transparent_background: bool,
    }

    #[derive(Clone, Copy)]
    pub struct Project {
        pub name: &'static str,
        pub description: &'static str,
        pub start_date: &'static str,
        pub end_date: Option<&'static str>,
        pub presentation: &'static [&'static str],
        pub highlights: &'static [&'static str],
        pub keywords: &'static [&'static str],
        pub link: Option<ResumeLink>,
        pub logo: Option<Logo>,
        pub image: Option<Image>,
    }

    #[derive(Clone, Copy)]
    pub struct Image {
        pub file: &'static str,
        pub position: &'static str,
    }

    #[derive(Clone, Copy)]
    pub struct ResumeLink {
        pub uri: &'static str,
        pub label: &'static str,
        pub not_available: bool,
    }

    use crate::common::link::{Link, outline_button_link};
    use leptos::prelude::*;
    use tailwind_fuse::tw_join;
    impl IntoAny for ResumeLink {
        fn into_any(self) -> AnyView {
            if !self.not_available {
                let link = Link::parse(self.uri, self.label);
                outline_button_link(link).into_any()
            } else {
                view! {
                    <span class=tw_join!(
                        "mt-4", "cursor-not-allowed", "inline-flex", "max-w-fit", "bg-gray-400",
                        "text-gray-600", "font-semibold", "py-1.5", "px-4", "rounded-xl",
                        "items-center"
                    )>{self.label}</span>
                }
                .into_any()
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Education {
        pub institution: &'static str,
        pub study_type: &'static str,
        pub area: &'static str,
        pub start_date: &'static str,
        pub end_date: Option<&'static str>,
        pub logo: Option<Logo>,
        pub courses: &'static [&'static str],
    }

    include!(concat!(env!("OUT_DIR"), "/resume.rs"));
}

pub mod wallpapers {

    #[derive(Clone, Copy)]
    pub struct Wallpaper {
        pub filename: &'static str,
        pub date: &'static str,
        pub gps: Gps,
        pub location: Location,
    }

    #[derive(Clone, Copy)]
    pub struct Gps {
        pub latitude: f32,
        pub longitude: f32,
    }

    impl std::fmt::Display for Gps {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}, {}]", self.latitude, self.longitude)
        }
    }

    #[derive(Clone, Copy)]
    pub struct Location {
        pub precise: &'static str,
        pub broad: &'static str,
    }

    impl Wallpaper {
        pub fn random() -> Option<&'static Wallpaper> {
            use nanorand::{ChaCha20, Rng};
            use std::ops::Range;

            let range = Range {
                start: 0,
                end: WALLPAPERS.len(),
            };

            WALLPAPERS.get(ChaCha20::new().generate_range(range))
        }

        pub fn find(filename: &str) -> Option<&'static Wallpaper> {
            WALLPAPERS.iter().find(|w| w.filename.contains(filename))
        }
    }

    include!(concat!(env!("OUT_DIR"), "/wallpapers.rs"));
}
