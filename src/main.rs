use self::types::*;
use chrono::Datelike;
use rocket::fs::FileServer;
use rocket::{catch, catchers, get, launch, routes};

mod cache;
mod filters;
mod minify;
mod templates;
mod types;
mod wallpapers;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("public"))
        .mount("/", routes![root, email])
        .register("/", catchers![not_found])
        .attach(
            rocket_async_compression::CachedCompression::path_suffix_fairing(vec![
                // Code
                ".js".into(),
                ".css".into(),
                // Documents
                ".pdf".into(),
                ".txt".into(),
            ]),
        )
        .attach(cache::CacheControl::default())
        .attach(minify::Minify)
}

#[catch(404)]
fn not_found() -> templates::NotFound<'static> {
    templates::NotFound {
        title: "404 Not found",
        year: chrono::Utc::now().year(),
    }
}

#[get("/")]
fn root() -> templates::Root<'static> {
    templates::Root {
        title: "Philippe Loctaux",
        year: chrono::Utc::now().year(),
        wallpaper: Wallpaper::random(),
        networks: Network::new(),
        jobs: Job::new(),
        talks: Talk::new(),
        friends: Friend::new(),
        projects: ProjectKind::new(),
    }
}

#[get("/email")]
fn email() -> templates::Email<'static> {
    templates::Email {
        title: "Email",
        year: chrono::Utc::now().year(),
    }
}
