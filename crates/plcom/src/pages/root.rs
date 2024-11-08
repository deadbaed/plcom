mod education;
mod experience;
mod friends;
mod hero;
mod jobs;
mod projects;
mod talks;
mod www;

use crate::common::wallpapers::Wallpaper;
use crate::prelude::*;

pub fn root_page(wallpaper: Option<&'static Wallpaper>) -> impl IntoAny {
    view! {
        {hero::hero(wallpaper).into_any()}

        <div class=tw_join!("container", "mx-auto", "px-4", "md:px-8", "lg:px-16", "py-16")>
            {whoami}
            <div class=tw_join!("my-16", "space-y-16", "md:space-y-32")>
                {www::list().into_any()}
                {jobs::jobs().into_any()}
                {projects::projects().into_any()}
                {education::education_list().into_any()}
                {talks::talks().into_any()}
                {friends::friends().into_any()}
            </div>
        </div>
    }
    .into_any()
}

fn whoami() -> impl IntoView {
    view! {
        <div class=tw_join!("md:flex", "md:flex-row-reverse", "items-center")>
            <div class=tw_join!("md:w-1/2", "mb-4", "md:mb-0")>
                <img
                    src="/phil.png"
                    alt="Phil"
                    class=tw_join!(
                        "rounded-3xl", "bg-sky-900", "h-36", "w-36", "md:mx-auto", "md:h-56",
                        "md:w-56", "lg:h-64", "lg:w-64", "mb-2", "md:mb-0"
                    )
                />

            </div>

            <div class=tw_join!("md:w-1/2")>
                <h1 class=tw_join!("text-4xl", "font-bold", "mb-4")>"About Phil"</h1>
                <h2 class=tw_join!(
                    "text-2xl", "font-semibold", "mb-4"
                )>"Developer of all sorts"</h2>

                <div class=tw_join!("text-lg", "space-y-6")>
                    <p>
                        "I got into computer science by learning about the Linux kernel and administrating servers."
                    </p>
                    <p>
                        "After high school, I became a student at Epitech and learned to tackle technical concepts and apply them quickly by working on small projects."
                    </p>
                    <p>
                        "During my studies at Epitech, I had the opportunity to be a teacher. My role was to assist students with technical problems in their projects."
                    </p>
                    <p>
                        "Now I have experience in software engineering, full-stack web and mobile development, system administration and CI/CD, as well as embedded development."
                    </p>
                    <p>
                        "My goal is to use my knowledge and experience to make software helping its users accomplish their needs."
                    </p>
                </div>
            </div>
        </div>
    }
}
