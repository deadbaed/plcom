use crate::prelude::*;

#[derive(Clone, PartialEq)]
struct Talk {
    title: String,
    date: Date,
    location: String,
    link: Link,
}

impl Talk {
    pub fn new(
        title: impl Into<String>,
        date: Date,
        location: impl Into<String>,
        link: Link,
    ) -> Self {
        Self {
            title: title.into(),
            date,
            location: location.into(),
            link,
        }
    }
}

#[component]
fn Talk(#[prop(into)] talk: MaybeSignal<Talk>) -> impl IntoView {
    view! {
        <div class=tw_join!("rounded-2xl", "w-full", "bg-teal-950", "p-6")>

            <h3 class=tw_join!("text-xl", "font-semibold", "mb-4")>{talk.get().title}</h3>

            <div class=tw_join!("flex")>
                <div class=tw_join!(
                    "inline-flex", "items-center"
                )>
                    {Icon::Calendar}
                    <span class=tw_join!("ml-2")>{talk.get().date.to_string()}</span>
                </div>
            </div>

            <div class=tw_join!("flex")>
                <div class=tw_join!(
                    "inline-flex", "items-center"
                )>{Icon::Location} <span class=tw_join!("ml-2")>{talk.get().location}</span></div>
            </div>

            <OutlineButtonLink link=talk.get().link/>
        </div>
    }
}

#[component]
pub fn Talks() -> impl IntoView {
    let talks = [
        Talk::new(
            "Vim",
            Date {
                year: 2023,
                month: 2,
            },
            "Epitech Rennes",
            Link::slides("/pub/talks/vim.pdf"),
        ),
        Talk::new(
            "CLion",
            Date {
                year: 2021,
                month: 3,
            },
            "Epitech Rennes",
            Link::slides("/pub/talks/clion.pdf"),
        ),
        Talk::new(
            "git & devops 2",
            Date {
                year: 2021,
                month: 2,
            },
            "Epitech Rennes",
            Link::slides("/pub/talks/git-devops2.pdf"),
        ),
        Talk::new(
            "pass4thewin",
            Date {
                year: 2021,
                month: 2,
            },
            "Epitech Rennes",
            Link::slides("/pub/talks/pass4thewin.pdf"),
        ),
        Talk::new(
            "git & devops",
            Date {
                year: 2020,
                month: 5,
            },
            "Epitech Rennes",
            Link::slides("/pub/talks/git-devops.pdf"),
        ),
        Talk::new(
            "git gud",
            Date {
                year: 2019,
                month: 5,
            },
            "Epitech Rennes",
            Link::slides("/pub/talks/git-tek.pdf"),
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
                {talks
                    .into_iter()
                    .map(|t| {
                        view! { <Talk talk=t/> }
                    })
                    .collect_view()}
            </div>

        </div>
    }
}

