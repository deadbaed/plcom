use crate::prelude::*;
use rocket::http::uri::Uri;
use tailwind_fuse::tw_merge;

#[derive(Clone, PartialEq)]
pub struct Link<'a> {
    pub label: String,
    pub uri: Uri<'a>,
}

impl<'a> Link<'a> {
    pub fn new(uri: Uri<'a>, label: impl Into<String>) -> Self {
        Self {
            uri,
            label: label.into(),
        }
    }
    pub fn parse(uri: &'static str, label: impl Into<String>) -> Self {
        Self {
            uri: Uri::parse_any(uri).expect("not a real uri"),
            label: label.into(),
        }
    }
    pub fn slides(uri: Uri<'a>) -> Self {
        Self::new(uri, "Slides")
    }
}

pub fn underline_link(
    link: Link,
    class: Option<String>,
) -> impl IntoView {
    let class = class.unwrap_or_default();
    let class = tw_merge!("underline", class);
    view! {
        <a href=link.uri.to_string() class=class>
            {link.label}
        </a>
    }
    .into_view()
}

type HideTextSmallDisplay = bool;

pub fn button_link(
    link: Link,
    icon: Option<Icon>,
    hide_text_small_display: Option<HideTextSmallDisplay>,
) -> impl IntoAny {
    let text_css = hide_text_small_display
        .and_then(|hide| {
            if hide {
                Some(tw_join!("hidden", "sm:inline"))
            } else {
                None
            }
        })
        .unwrap_or(tw_join!("ml-2", "sm:ml-0", "text-center"));

    let icon = icon
        .map(|icon| icon.into_any())
        .unwrap_or_else(|| ().into_any());

    view! {
        <a
            href=link.uri.to_string()
            aria-label=link.label.to_string()
            class=tw_join!(
                "inline-flex", "bg-sky-900", "hover:bg-sky-700", "transition-all", "duration-200",
                "text-white", "font-bold", "py-2", "px-4", "rounded-xl", "items-center"
            )
        >

            {icon}
            <div class=tw_join!("inline-flex", "items-center")>
                <span class=text_css>{link.label.to_string()}</span>
            </div>
        </a>
    }
    .into_any()
}

pub fn outline_button_link(
    link: Link,
) -> impl IntoAny {
    view! {
        <a
            href=link.uri.to_string()
            class=tw_join!(
                "mt-4", "inline-flex", "bg-transparent", "hover:bg-sky-700", "text-white",
                "font-semibold", "py-1.5", "px-4", "rounded-xl", "items-center", "border",
                "border-white", "hover:border-transparent", "transition-all", "duration-200"
            )
        >

            {Icon::Link.into_any()}
            <div class=tw_join!("inline-flex", "items-center")>
                <span class=tw_join!("ml-2", "sm:ml-0", "text-center")>{link.label}</span>
            </div>
        </a>
    }
    .into_any()
}
