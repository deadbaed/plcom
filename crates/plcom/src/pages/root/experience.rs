use crate::common::resume::Logo;
use crate::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass, Clone, Copy, PartialEq)]
#[tw(class = r#"h-16 w-16 rounded-xl"#)]
struct LogoOptions {
    background: ImageBackground,
}

#[derive(TwVariant, PartialEq)]
enum ImageBackground {
    #[tw(class = "p-2 bg-white")]
    Transparent,
    #[tw(default, class = "")]
    Plain,
}

fn experience_logo(
    image: String,
    // Name of the experience, used in the alt of the image
    name: String,
    background: ImageBackground,
    class: String,
) -> impl IntoView {
    let class = LogoOptions { background }.with_class(class);
    let alt = format!("{} logo", name);

    view! { <img loading="lazy" src=image alt=alt class=class/> }
}

struct ExperienceLogo {
    file: String,
    options: Option<LogoOptions>,
}

pub struct ExperienceHeader {
    name: String,
    description: String,
    date_start: Date,
    date_end: Option<Date>,
    logo: Option<ExperienceLogo>,
}

impl IntoAny for ExperienceHeader {
    fn into_any(self) -> AnyView {
        let logo = match self.logo {
            Some(logo) => experience_logo(
                logo.file,
                self.name.clone(),
                logo.options.map(|o| o.background).unwrap_or_default(),
                tw_join!("mr-4"),
            )
            .into_any(),
            None => ().into_any(),
        };

        let date = match self.date_end {
            Some(end) => format!("{} - {}", self.date_start, end),
            None => format!("Since {}", self.date_start),
        };

        view! {
            <div class=tw_join!("flex", "flex-col")>
                <div class=tw_join!(
                    "flex", "flex-row"
                )>
                    {logo} <div class=tw_join!("flex", "flex-col", "justify-evenly")>
                        <div class=tw_join!(
                            "text-xl", "md:text-2xl", "font-semibold"
                        )>{self.name}</div>
                        <div class=tw_join!("text-xs", "md:text-sm")>{date}</div>
                    </div>
                </div>
                <p class=tw_join!(
                    "text-xl", "md:text-2xl", "font-semibold", "my-4"
                )>{self.description}</p>
            </div>
        }
        .into_any()
    }
}

impl ExperienceHeader {
    pub fn new(
        start_date: &str,
        end_date: Option<&str>,
        name: &str,
        description: &str,
        logo: Option<&Logo>,
    ) -> Self {
        let date_start: Vec<_> = start_date.split('-').collect();

        let date_end = end_date.map(|end| {
            let end: Vec<_> = end.split('-').collect();
            Date {
                year: end[0].parse().expect("not a number"),
                month: end[1].parse().expect("not a number"),
            }
        });

        let logo = logo.map(|logo| ExperienceLogo {
            file: logo.file.into(),
            options: if logo.transparent_background {
                Some(LogoOptions {
                    background: ImageBackground::Transparent,
                })
            } else {
                None
            },
        });

        Self {
            name: name.into(),
            description: description.into(),
            date_start: Date {
                year: date_start[0].parse().expect("not a number"),
                month: date_start[1].parse().expect("not a number"),
            },
            date_end,
            logo,
        }
    }
}
