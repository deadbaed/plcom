use leptos::*;
use prelude::*;

pub mod app;
pub mod common;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

pub mod prelude {
    pub use super::ContentPage;
    pub use crate::common::icon::*;
    pub use crate::common::link::*;
    pub use crate::common::resume::*;
    pub use crate::common::wallpapers::*;
    pub use crate::common::*;
    pub use leptos::*;
    pub use leptos_meta::*;
    pub use tailwind_fuse::tw_join;
    pub use http::Uri;
}

#[component]
pub fn ContentPage(
    #[prop(into, optional)] title: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <leptos_meta::Title text=title.get()></leptos_meta::Title>
        <div class=tw_join!("container", "mx-auto", "px-4", "py-16")>
            <h1 class=tw_join!("text-3xl", "sm:text-4xl", "font-bold")>{title}</h1>
            <UnderlineLink link=Link::new("/", "← Home")/>
            <div class=tw_join!("mt-8")>{children()}</div>
        </div>
    }
}
