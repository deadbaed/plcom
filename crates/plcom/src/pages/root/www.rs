use crate::prelude::*;

#[derive(Clone, PartialEq)]
struct Www {
    link: Link,
    icon: Icon,
}

#[component]
pub fn Www() -> impl IntoView {
    let www = [
        Www {
            link: Link::new("https://twitter.com/philippeloctaux", "Twitter"),
            icon: Icon::Twitter,
        },
        Www {
            link: Link::new("https://t.me/philippeloctaux", "Telegram"),
            icon: Icon::Telegram,
        },
        Www {
            link: Link::new("https://mastodon.social/@philt3r", "Mastodon"),
            icon: Icon::Mastodon,
        },
        Www {
            link: Link::new("https://github.com/deadbaed", "GitHub"),
            icon: Icon::Github,
        },
        Www {
            link: Link::new("https://linkedin.com/in/philippeloctaux", "LinkedIn"),
            icon: Icon::Linkedin,
        },
        Www {
            link: Link::new("/email", "Email"),
            icon: Icon::Email,
        },
    ];
    view! {
        <div class=tw_join!(
            "grid", "grid-cols-3", "lg:grid-cols-6", "gap-4", "place-content-center"
        )>

            {www
                .into_iter()
                .map(|w| {
                    view! {
                        <div class=tw_join!("w-full", "h-auto", "md:w-auto")>
                            <div class=tw_join!("text-center")>
                                <ButtonLink
                                    link=w.link
                                    icon=w.icon
                                    hide_text_small_display=true
                                    attributes=vec![
                                        ("target", Attribute::String(Oco::Borrowed("_blank"))),
                                    ]
                                />

                            </div>
                        </div>
                    }
                })
                .collect_view()}

        </div>
    }
}
