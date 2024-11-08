use crate::prelude::*;

pub fn email_page() -> impl IntoAny {
    view! {
        <p>
            "Send an email if you want to work with me, propose a project idea, or just to say hi!"
        </p>
        <div class=tw_join!("my-4")>
            {button_link(Link::new(
                    uri!("mailto:wwwATphilippeloctaux~DOT~com").into(),
                    "www at philippeloctaux dot com",
                ), Some(Icon::Email), None).into_any()}
        </div>

        <p class=tw_join!(
            "mb-2"
        )>
            "If you want to encrypt your message, I have a "
            {underline_link(Link::new(uri!("/pub/pgp-0x69771CD04BA82EC0.txt").into(), "pgp key"), None).into_any()}
            " at your disposal."
        </p>
        <p class=tw_join!(
            "mb-2"
        )>
            "I also have a "{underline_link(Link::new(uri!("/keybase.txt").into(), "Keybase"), None).into_any()}
            " account, but I do not check it often."
        </p>
    }.into_any()
}
