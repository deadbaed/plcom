use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::*,
    Link, UnderlineLink,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use tailwind_fuse::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let formatter = |text| format!("{text} — Philippe Loctaux");

    view! {
        <Html lang="en"/>

        <Stylesheet id="leptos" href="/pkg/plcom.css"/>

        // sets the document title
        <Title formatter/>

        <Meta name="viewport" content="width=device-width"/>

        // favicon
        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png"/>
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png"/>
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png"/>
        <Link rel="manifest" href="/site.webmanifest"/>
        <Link
            rel="mask-icon"
            href="/safari-pinned-tab.svg"
            attrs=vec![("color", Attribute::String(Oco::Borrowed("#0c4a6e")))]
        />
        <Meta name="msapplication-TileColor" content="#0c4a6e"/>
        <Meta name="theme-color" content="#0c4a6e"/>

        // stats
        <Script
            defer="true"
            src="https://plausible.y.z.x4m3.rocks/js/script.js"
            attrs=vec![("data-domain", Attribute::String(Oco::Borrowed("philippeloctaux.com")))]
        />

        // actual routes
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Body class=tw_join!("flex", "flex-col", "min-h-screen", "bg-gray-900", "text-white")/>
            <main class=tw_join!("flex-grow")>
                <Routes>
                    <Route path="" view=RootPage ssr=SsrMode::Async/>
                    <Route path="email" view=EmailPage/>
                    <Route path="wallpapers" view=WallpapersPage/>
                </Routes>
            </main>
            <footer class=tw_join!("bg-black")>
                <div class=tw_join!("container", "mx-auto", "px-4", "py-8")>
                    <p>
                        "© 2015 - "{crate::get_year()}" Philippe Loctaux, made with "
                        <UnderlineLink link=Link::new("https://leptos.dev", "Leptos")/>
                    </p>
                </div>
            </footer>
        </Router>
    }
}
