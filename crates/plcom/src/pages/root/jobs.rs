use crate::prelude::*;
use super::experience::*;

impl IntoView for Work {
    fn into_view(self) -> View {
        view! {
            <div class=tw_join!("w-full", "rounded-2xl", "bg-sky-950")>

                <div class=tw_join!(
                    "p-6", "justify-between", "h-full"
                )>

                    {ExperienceHeader::new(
                        self.start_date,
                        self.end_date,
                        self.name,
                        self.position,
                        Some(&self.logo),
                    )} <div class=tw_join!("space-y-2")>

                        <p>{self.description}</p>

                        <div>
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
                        </div>

                        <div>
                            <div class=tw_join!(
                                "mt-6 flex flex-wrap gap-x-6 gap-y-4"
                            )>
                                {self
                                    .technologies
                                    .iter()
                                    .map(|t| {
                                        view! {
                                            <span class=tw_join!(
                                                "inline-flex", "items-center", "rounded-md", "bg-blue-100",
                                                "px-2", "py-1", "font-medium", "text-blue-700"
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

#[component]
pub fn Jobs() -> impl IntoView {
    view! {
        <div>
            <h1 class=tw_join!("text-4xl", "font-bold", "mb-4")>"Professional Experiences"</h1>

            <div class=tw_join!(
                "mt-4", "grid", "grid-cols-1", "sm:grid-cols-2", "gap-4"
            )>{resume::WORK.collect_view()}</div>
        </div>
    }
}

