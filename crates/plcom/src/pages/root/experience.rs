use tailwind_fuse::*;
use leptos::*;
use crate::common::Date;
use crate::common::resume::Logo;

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

#[component]
fn ExperienceLogo(
    #[prop(into)] image: MaybeSignal<String>,
    /// Name of the experience, used in the alt of the image
    #[prop(into)]
    name: MaybeSignal<String>,
    #[prop(into, optional)] background: MaybeSignal<ImageBackground>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        let background = background.get();
        let logo = LogoOptions { background };
        logo.with_class(class.get())
    });
    let alt = format!("{} logo", name.get());

    view! { <img {..attributes} loading="lazy" src=image.get() alt=alt class=class/> }
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

impl IntoView for ExperienceHeader {
    fn into_view(self) -> View {
        let logo = match self.logo {
            Some(logo) => view! {
                <ExperienceLogo
                    image=logo.file
                    name=self.name.clone()
                    background=logo.options.map(|o| o.background).unwrap_or_default()
                    class=tw_join!("mr-4")
                />
            }
            .into_view(),
            None => view! {}.into_view(),
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
        .into_view()
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
