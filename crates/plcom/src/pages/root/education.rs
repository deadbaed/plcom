use crate::prelude::*;
use super::experience::*;

impl IntoView for Education {
    fn into_view(self) -> View {
        let subtitle = format!("{} in {}", self.study_type, self.area);
        view! {
            <div class=tw_join!(
                "rounded-2xl", "w-full", "bg-amber-950", "p-6"
            )>

                {ExperienceHeader::new(
                    self.start_date,
                    self.end_date,
                    self.institution,
                    &subtitle,
                    self.logo.as_ref(),
                )}
                <div class=tw_join!("space-y-2")>
                    <ul class=tw_join!(
                        "list-disc", "mt-6"
                    )>
                        {self
                            .courses
                            .iter()
                            .map(|h| {
                                view! { <li class=tw_join!("ml-5")>{*h}</li> }
                            })
                            .collect_view()}
                    </ul>
                </div>

            </div>
        }.into_view()
    }
}

#[component]
pub fn EducationList() -> impl IntoView {
    view! {
        <div>

            <h1 class=tw_join!("text-4xl", "font-bold", "mb-4")>"Education"</h1>

            <div class=tw_join!(
                "mt-4", "grid", "grid-cols-1", "md:grid-cols-2", "gap-6", "place-content-center"
            )>{resume::EDUCATION.collect_view()}</div>
        </div>
    }
}

