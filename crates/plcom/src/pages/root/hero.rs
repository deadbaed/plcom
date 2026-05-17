use crate::common::wallpapers::Wallpaper;
use crate::prelude::*;

fn wallpaper_info(wallpaper: &'static Wallpaper) -> impl IntoAny {
    view! {
        <div class=tw_join!(
            "absolute", "bottom-3", "sm:bottom-5", "left-2", "sm:left-5", "inline-block",
            "backdrop-blur-lg", "backdrop-brightness-75", "rounded-xl", "shadow-2xl", "p-2",
            "space-y-0.5", "sm:space-y-2"
        )>

            // See more
            <div class=tw_join!("flex")>
                <div class=tw_join!(
                    "inline-flex", "items-center"
                )>
                    {Icon::Map.into_any()}
                    <a class=tw_join!("ml-1", "text-sm", "underline") href="/wallpapers">
                        "See more!"
                    </a>
                </div>
            </div>

            // Location
            <div class=tw_join!("flex")>
                <div class=tw_join!(
                    "inline-flex", "items-center"
                )>
                    {Icon::Location.into_any()}
                    <span class=tw_join!(
                        "ml-1", "text-sm"
                    )>
                        {wallpaper.location.precise}
                        <span class=tw_join!(
                            "hidden", "md:inline"
                        )>", "{wallpaper.location.broad}</span>
                    </span>
                </div>
            </div>

            // Date
            <div class=tw_join!("flex")>
                <div class=tw_join!(
                    "inline-flex", "items-center"
                )>
                    {Icon::Calendar.into_any()}
                    <span class=tw_join!("ml-1", "text-sm")>{wallpaper.date}</span>
                </div>
            </div>

        </div>
    }.into_any()
}

pub fn hero(wallpaper: Option<&'static Wallpaper>) -> impl IntoAny {
    let (wallpaper_info, background_image) = match wallpaper {
        Some(wallpaper) => (
            wallpaper_info(wallpaper).into_any(),
            format!("background-image: url(/wallpapers/files/{});", wallpaper.filename),
        ),
        None => (().into_any(), "".to_string()),
    };

    view! {
        <div class=tw_join!("bg-gradient-to-r", "from-red-900", "via-teal-900", "to-fuchsia-900")>
            <div
                id="wallpaper"
                class=tw_join!(
                    "relative", "text-white", "w-full", "h-(--almostscreen)", "bg-center", "bg-cover"
                )

                style=background_image
            >

                <div class=tw_join!(
                    "container", "mx-auto", "px-8", "py-16", "w-full", "h-full", "justify-center",
                    "items-center", "flex", "flex-col"
                )>
                    <div class=tw_join!(
                        "inline-block", "backdrop-blur-lg", "backdrop-brightness-75", "rounded-3xl",
                        "shadow-2xl", "px-4", "py-6", "sm:px-8", "sm:py-12", "space-y-4"
                    )>
                        <h1 class=tw_join!(
                            "text-3xl", "sm:text-4xl", "font-bold"
                        )>"Philippe Loctaux"</h1>
                        <h2 class=tw_join!(
                            "sm:text-xl", "font-semibold"
                        )>"Developer of all sorts. Epitech alumni, class of 2023."</h2>
                    </div>
                </div>

                {wallpaper_info}
            </div>
        </div>
    }
}
