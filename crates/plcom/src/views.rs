use crate::prelude::*;

pub fn shell(title: &str, children: impl IntoAny) -> AnyView {
    const SUFFIX: &str = "Philippe Loctaux";
    let title = if title != SUFFIX {
        format!("{title} - {SUFFIX}")
    } else {
        title.into()
    };

    let year = jiff::Zoned::now().year();
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width"/>
                <link rel="stylesheet" href="/style.css" />
                <title>{title}</title>

                // favicon
                <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"/>
                <link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png"/>
                <link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png"/>
                <link rel="manifest" href="/site.webmanifest"/>
                <link
                    rel="mask-icon"
                    href="/safari-pinned-tab.svg"
                    color="#0c4a6e"
                />
                <meta name="msapplication-TileColor" content="#0c4a6e"/>
                <meta name="theme-color" content="#0c4a6e"/>
            </head>

            <body class=tw_join!("flex", "flex-col", "min-h-screen", "bg-gray-900", "text-white")>
                <main class=tw_join!("flex-grow")>
                    {children.into_any()}
                </main>
                <footer class=tw_join!("bg-black")>
                    <div class=tw_join!("container", "mx-auto", "px-4", "py-8")>
                        <p>"© 2015 - "{year}" Philippe Loctaux, made with "{underline_link(Link::new(uri!("https://leptos.dev").into(), "Leptos"), None).into_any()}"."</p>
                    </div>
                </footer>
            </body>

            <script inner_html=r#"
                window.goatcounter = {
                    path: function(p) { return location.host + p }
                }
            "#></script>
            <script data-goatcounter="https://goatcounter.philt3r.eu/count" async src="https://goatcounter.philt3r.eu/count.js"></script>
        </html>
    }.into_any()
}

pub fn content_page(title: &str, children: impl IntoAny) -> AnyView {
    shell(
        title,
        view! {
            <div class=tw_join!("container", "mx-auto", "px-4", "py-16")>
                <h1 class=tw_join!("text-3xl", "sm:text-4xl", "font-bold")>{title.to_string()}</h1>
                {underline_link(Link::new(uri!("/").into(), "← Home"), None).into_any()}
                <div class=tw_join!("mt-8")>{children.into_any()}</div>
            </div>
        }
        .into_any(),
    )
    .into_any()
}
