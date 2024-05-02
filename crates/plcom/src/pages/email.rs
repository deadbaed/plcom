use crate::prelude::*;

#[component]
pub fn EmailPage() -> impl IntoView {
    view! {
        <ContentPage title="Email">
            <p>
                "Send an email if you want to work with me, propose a project idea, or just to say hi!"
            </p>
            <div class=tw_join!("my-4")>
                <ButtonLink
                    link=Link::new(
                        "mailto:wwwATphilippeloctaux~DOT~com",
                        "www at philippeloctaux dot com",
                    )

                    icon=Icon::Email
                />
            </div>

            <p class=tw_join!(
                "mb-2"
            )>
                "If you want to encrypt your message, I have a "
                <UnderlineLink link=Link::new("/pub/pgp-0x69771CD04BA82EC0.txt", "pgp key")/>
                " at your disposal."
            </p>
            <p class=tw_join!(
                "mb-2"
            )>
                "I also have a " <UnderlineLink link=Link::new("/keybase.txt", "Keybase")/>
                " account, but I do not check it often."
            </p>
        </ContentPage>
    }
}

