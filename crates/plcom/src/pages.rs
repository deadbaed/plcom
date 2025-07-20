mod email;
mod root;
mod wallpapers;

use crate::common::wallpapers::Wallpaper;
use crate::prelude::*;
use rocket::get;

#[derive(rocket::Responder)]
#[response(content_type = "text/html")]
pub struct LeptosResponder(String);

impl From<AnyView> for LeptosResponder {
    fn from(value: AnyView) -> Self {
        Self(value.to_html())
    }
}

#[rocket::catch(404)]
pub fn not_found() -> LeptosResponder {
    content_page(
        "404 Not Found",
        view! {
            <div>"This page could not be found."</div>
        },
    )
    .into()
}

#[get("/?<wallpaper>")]
pub async fn root_route(wallpaper: Option<&str>) -> LeptosResponder {
    let wallpaper = wallpaper
        .and_then(Wallpaper::find)
        .or_else(Wallpaper::random);

    let mut blog = root::Blog::new("https://philippeloctaux.com/blog/atom.xml");
    if let Err(e) = blog.fetch_feed().await {
        println!("Failed to get Atom feed: {e}");
    }

    shell("Philippe Loctaux", root::root_page(wallpaper, blog)).into()
}

#[get("/email")]
pub fn email_route() -> LeptosResponder {
    content_page("Email", email::email_page()).into()
}

#[get("/wallpapers")]
pub fn wallpapers_route() -> LeptosResponder {
    content_page("Wallpapers", wallpapers::wallpapers_page()).into()
}
