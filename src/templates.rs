use crate::filters;
use crate::types::*;
use askama::Template;

#[derive(Template)]
#[template(path = "pages/root.html")]
pub struct Root<'a> {
    pub title: &'a str,
    pub year: i32,
    pub wallpaper: Option<&'static &'static Wallpaper>,
    pub networks: Vec<Network>,
    pub jobs: Vec<Job>,
    pub talks: Vec<Talk>,
    pub friends: Vec<Friend>,
    pub projects: Vec<ProjectKind>,
}

#[derive(rocket::Responder)]
struct RootResponder<'a> {
    template: Root<'a>,
}

#[derive(Template)]
#[template(path = "pages/404.html")]
pub struct NotFound<'a> {
    pub title: &'a str,
    pub year: i32,
}

#[derive(rocket::Responder)]
struct NotFoundResponder<'a> {
    template: NotFound<'a>,
}

#[derive(Template)]
#[template(path = "pages/email.html")]
pub struct Email<'a> {
    pub title: &'a str,
    pub year: i32,
}

#[derive(rocket::Responder)]
struct EmailResponder<'a> {
    template: Email<'a>,
}
