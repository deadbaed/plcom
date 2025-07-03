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
    }.into_any()
}
