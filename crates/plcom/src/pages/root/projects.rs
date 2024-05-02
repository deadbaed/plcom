use crate::prelude::*;
use super::experience::*;

impl IntoView for Project {
    fn into_view(self) -> View {
        let (css_image_position, css_image_position_corner) = match self.image {
            Some(image) => {
                let position = match image.position {
                    "left" => tw_join!("2xl:flex items-stretch justify-between"),
                    "right" => {
                        tw_join!("2xl:flex items-stretch justify-between 2xl:flex-row-reverse")
                    }
                    _ => todo!("match on an enum instead of raw strings"),
                };

                let corner = match image.position {
                    "left" => tw_join!("2xl:rounded-tr-none", "2xl:rounded-l-2xl"),
                    "right" => tw_join!("2xl:rounded-tl-none", "2xl:rounded-r-2xl"),
                    _ => todo!("match on an enum instead of raw strings"),
                };
                let corner = tw_join!(
                    "flex",
                    "w-full",
                    "2xl:w-1/2",
                    "grow",
                    "rounded-t-2xl",
                    "object-cover",
                    corner
                );

                (position, corner)
            }
            None => ("".into(), "".into()),
        };

        view! {
            <div class=tw_join!(
                "w-full", "rounded-2xl", "bg-pink-950", css_image_position
            )>

                {if let Some(image) = self.image {
                    view! {
                        <img
                            loading="lazy"
                            src=image.file
                            alt=format!("{} image", self.name)
                            class=css_image_position_corner
                        />
                    }
                        .into_view()
                } else {
                    view! {}.into_view()
                }}
                <div class=tw_join!(
                    "p-6", "justify-between", "h-full"
                )>

                    {ExperienceHeader::new(
                        self.start_date,
                        self.end_date,
                        self.name,
                        self.description,
                        self.logo.as_ref(),
                    )}
                    <div class=tw_join!(
                        "space-y-2"
                    )>

                        {self
                            .presentation
                            .iter()
                            .map(|p| {
                                view! { <p>{*p}</p> }
                            })
                            .collect_view()} <div>
                            <ul class=tw_join!(
                                "list-disc", "mt-6"
                            )>
                                {self
                                    .highlights
                                    .iter()
                                    .map(|h| {
                                        view! { <li class=tw_join!("ml-5")>{*h}</li> }
                                    })
                                    .collect_view()}
                            </ul>
                        </div> <div>

                            <div class=tw_join!(
                                "mt-6", "grid", "grid-cols-2", "sm:grid-cols-3", "gap-x-6",
                                "gap-y-4"
                            )>
                                {self
                                    .keywords
                                    .iter()
                                    .map(|t| {
                                        view! {
                                            <span class=tw_join!(
                                                "items-center", "rounded-md", "bg-blue-100", "px-2", "py-1",
                                                "font-medium", "text-blue-700",
                                            )>{*t}</span>
                                        }
                                    })
                                    .collect_view()}
                            </div>

                        </div>
                    </div> {self.link}

                </div>
            </div>
        }.into_view()
    }
}

    type ImageProject = Project;
    type TextProject = Project;

    enum DisplayProject {
        Text(Box<(TextProject, Option<TextProject>)>),
        Image(ImageProject)
    }

impl IntoView for DisplayProject {
    fn into_view(self) -> View {
        match self {
            Self::Image(image) => image.into_view(),
            Self::Text(boxy) => {
                let (text1, text2) = *boxy;
            view! {
                <div class=tw_join!(
                    "my-4", "grid", "grid-cols-1", "sm:grid-cols-2", "gap-4"
                )>{text1} {text2}</div>
            }.into_view()

            }
        }
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    let mut projects = vec![];
    let mut iter = resume::PROJECTS.iter();

    while let Some(cur_proj) = iter.next() {
        if cur_proj.image.is_some() {
            projects.push(DisplayProject::Image(*cur_proj));
        } else {
            let next_text = iter.next();
            projects.push(DisplayProject::Text(Box::new((*cur_proj, next_text.copied()))));
        }
    }

    view! {
        <div>

            <h1 class=tw_join!("text-4xl", "font-bold", "mb-4")>"Projects"</h1>

            {projects}
        </div>
    }
}
