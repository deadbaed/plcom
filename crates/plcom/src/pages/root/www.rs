use crate::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Www {
    link: Link<'static>,
    icon: Icon,
}

impl IntoAny for Www {
    fn into_any(self) -> AnyView {
        view! {
            <div class=tw_join!("w-full", "h-auto", "md:w-auto")>
                <div class=tw_join!("text-center")>
                    {button_link(self.link, Some(self.icon), Some(true)).into_any()}
                </div>
            </div>
        }
        .into_any()
    }
}

pub fn list() -> impl IntoAny {
    let www = [
        Www {
            link: Link::new(
                uri!("https://twitter.com/philippeloctaux").into(),
                "Twitter",
            ),
            icon: Icon::Twitter,
        },
        Www {
            link: Link::new(uri!("https://t.me/philippeloctaux").into(), "Telegram"),
            icon: Icon::Telegram,
        },
        Www {
            link: Link::new(uri!("https://mastodon.social/@philt3r").into(), "Mastodon"),
            icon: Icon::Mastodon,
        },
        Www {
            link: Link::new(uri!("https://github.com/deadbaed").into(), "GitHub"),
            icon: Icon::Github,
        },
        Www {
            link: Link::new(
                uri!("https://linkedin.com/in/philippeloctaux").into(),
                "LinkedIn",
            ),
            icon: Icon::Linkedin,
        },
        Www {
            link: Link::new(uri!("/email").into(), "Email"),
            icon: Icon::Email,
        },
    ];
    view! {
        <div class=tw_join!(
            "grid", "grid-cols-3", "lg:grid-cols-6", "gap-4", "place-content-center"
        )>

        {www.into_iter().map(|w| {w.into_any()}).collect_view()}

        </div>
    }
    .into_any()
}
