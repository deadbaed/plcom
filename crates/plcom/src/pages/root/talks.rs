use crate::prelude::*;

#[derive(Clone, PartialEq)]
struct Talk {
    title: String,
    date: Date,
    location: String,
    link: Link<'static>,
}

impl Talk {
    pub fn new(
        title: impl Into<String>,
        date: Date,
        location: impl Into<String>,
        link: Link<'static>,
    ) -> Self {
        Self {
            title: title.into(),
            date,
            location: location.into(),
            link,
        }
    }
}

impl IntoAny for Talk {
    fn into_any(self) -> AnyView {
        view! {
        <div class=tw_join!("rounded-2xl", "w-full", "bg-teal-950", "p-6")>

            <div class=tw_join!("text-xl", "font-semibold", "mb-4")>{self.title}</div>

            <div class=tw_join!("flex")>
                <div class=tw_join!(
                    "inline-flex", "items-center"
                )>
                    {Icon::Calendar.into_any()}
                    <span class=tw_join!("ml-2")>{self.date.to_string()}</span>
                </div>
            </div>

            <div class=tw_join!("flex")>
                <div class=tw_join!(
                    "inline-flex", "items-center"
                )>{Icon::Location.into_any()} <span class=tw_join!("ml-2")>{self.location}</span></div>
            </div>

            {outline_button_link(self.link).into_any()}
        </div>
    }.into_any()
    }
}

pub fn talks() -> impl IntoAny {
    let talks = [
        Talk::new(
            "Vim",
            Date {
                year: 2023,
                month: 2,
            },
            "Epitech Rennes",
            Link::slides(uri!("/pub/talks/vim.pdf").into()),
        ),
        Talk::new(
            "CLion",
            Date {
                year: 2021,
                month: 3,
            },
            "Epitech Rennes",
            Link::slides(uri!("/pub/talks/clion.pdf").into()),
        ),
        Talk::new(
            "git & devops 2",
            Date {
                year: 2021,
                month: 2,
            },
            "Epitech Rennes",
            Link::slides(uri!("/pub/talks/git-devops2.pdf").into()),
        ),
        Talk::new(
            "pass4thewin",
            Date {
                year: 2021,
                month: 2,
            },
            "Epitech Rennes",
            Link::slides(uri!("/pub/talks/pass4thewin.pdf").into()),
        ),
        Talk::new(
            "git & devops",
            Date {
                year: 2020,
                month: 5,
            },
            "Epitech Rennes",
            Link::slides(uri!("/pub/talks/git-devops.pdf").into()),
        ),
        Talk::new(
            "git gud",
            Date {
                year: 2019,
                month: 5,
            },
            "Epitech Rennes",
            Link::slides(uri!("/pub/talks/git-tek.pdf").into()),
        ),
    ];

    view! {
        <div>
            <h1 class=tw_join!("text-4xl", "font-bold", "mb-4")>"Talks"</h1>
            <p class=tw_join!(
                "text-lg"
            )>
                "Giving a talk is the opportunity to share what I know, and helps me reduce my fear of public speaking."
            </p>

            <div class=tw_join!(
                "mt-4", "grid", "grid-cols-1", "sm:grid-cols-2", "lg:grid-cols-3", "gap-6",
                "place-content-center"
            )>
                {talks.into_iter().map(|talk| talk.into_any()).collect_view()}
            </div>

        </div>
    }
}
