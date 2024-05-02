use http::Uri;
use leptos::*;
use tailwind_fuse::tw_join;

use crate::Icon;

#[derive(Clone, PartialEq)]
pub struct Link {
    pub label: String,
    pub uri: Uri,
}

impl Link {
    pub fn new(uri: &'static str, label: impl Into<String>) -> Self {
        Self {
            uri: Uri::from_static(uri),
            label: label.into(),
        }
    }
    pub fn slides(uri: &'static str) -> Self {
        Self::new(uri, "Slides")
    }
}

#[component]
pub fn UnderlineLink(
    #[prop(into)] link: MaybeSignal<Link>,
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = tailwind_fuse::tw_merge!("underline", class.get());
    view! {
        <a href=link.get().uri.to_string() {..attributes} class=class target="_blank">
            {link.get().label}
        </a>
    }
}

type HideTextSmallDisplay = bool;

#[component]
pub fn ButtonLink(
    #[prop(into)] link: MaybeSignal<Link>,
    #[prop(into, optional)] icon: Option<MaybeSignal<Icon>>,
    #[prop(into, optional)] hide_text_small_display: Option<MaybeSignal<HideTextSmallDisplay>>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let text_css = hide_text_small_display
        .and_then(|hide| {
            if hide.get() {
                Some(tw_join!("hidden", "sm:inline"))
            } else {
                None
            }
        })
        .unwrap_or(tw_join!("ml-2", "sm:ml-0", "text-center"));

    view! {
        <a
            href=link.get().uri.to_string()
            {..attributes}
            aria-label=link.get().label
            class=tw_join!(
                "inline-flex", "bg-sky-900", "hover:bg-sky-700", "transition-all", "duration-200",
                "text-white", "font-bold", "py-2", "px-4", "rounded-xl", "items-center"
            )
        >

            {icon}
            <div class=tw_join!("inline-flex", "items-center")>
                <span class=text_css>{link.get().label}</span>
            </div>
        </a>
    }
}

#[component]
pub fn OutlineButtonLink(
    #[prop(into)] link: MaybeSignal<Link>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <a
            href=link.get().uri.to_string()
            {..attributes}
            class=tw_join!(
                "mt-4", "inline-flex", "bg-transparent", "hover:bg-sky-700", "text-white",
                "font-semibold", "py-1.5", "px-4", "rounded-xl", "items-center", "border",
                "border-white", "hover:border-transparent", "transition-all", "duration-200"
            )
        >

            {Icon::Link}
            <div class=tw_join!("inline-flex", "items-center")>
                <span class=tw_join!("ml-2", "sm:ml-0", "text-center")>{link.get().label}</span>
            </div>
        </a>
    }
}
