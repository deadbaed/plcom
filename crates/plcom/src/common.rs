pub mod icon;
pub mod link;

pub fn get_year() -> i32 {
    use chrono::Datelike;
    chrono::Utc::now().year()
}

#[derive(Clone, PartialEq)]
pub struct Date {
    pub year: u32,
    pub month: u8,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}", self.year, self.month)
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

    use crate::{Link, OutlineButtonLink};
    use http::Uri;
    use leptos::*;
    use tailwind_fuse::tw_join;
    impl IntoView for ResumeLink {
        fn into_view(self) -> View {
            if !self.not_available {
                let link = Link {
                    label: self.label.into(),
                    uri: Uri::from_static(self.uri),
                };
                view! { <OutlineButtonLink link=link/> }.into_view()
            } else {
                view! {
                    <span class=tw_join!(
                        "mt-4", "cursor-not-allowed", "inline-flex", "max-w-fit", "bg-gray-400",
                        "text-gray-600", "font-semibold", "py-1.5", "px-4", "rounded-xl",
                        "items-center"
                    )>{self.label}</span>
                }
            .into_view()
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

    #[derive(Clone, Copy)]
    pub struct Location {
        pub precise: &'static str,
        pub broad: &'static str,
    }

    impl Wallpaper {
        pub fn random() -> Option<&'static Wallpaper> {
            let random_value = rand::Rng::gen_range(&mut rand::thread_rng(), 0..WALLPAPERS.len());
            WALLPAPERS.get(random_value)
        }
    }

    include!(concat!(env!("OUT_DIR"), "/wallpapers.rs"));
}
