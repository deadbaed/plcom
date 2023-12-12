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
    let server = rocket::build()
        .mount("/", FileServer::from("public"))
        .mount("/", routes![root, email, wallpapers_route])
        .register("/", catchers![not_found]);

    if cfg!(debug_assertions) {
        server
    } else {
        server
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
}

#[catch(404)]
fn not_found() -> templates::NotFound<'static> {
    templates::NotFound {
        title: "404 Not found",
        year: chrono::Utc::now().year(),
    }
}

#[get("/?<wallpaper>")]
fn root(wallpaper: Option<&str>) -> templates::Root<'static> {
    templates::Root {
        title: "Philippe Loctaux",
        year: chrono::Utc::now().year(),
        wallpaper: wallpaper
            .and_then(Wallpaper::find)
            .or_else(Wallpaper::random),
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

#[get("/wallpapers")]
fn wallpapers_route() -> templates::Wallpapers<'static> {
    templates::Wallpapers {
        title: "Wallpapers",
        year: chrono::Utc::now().year(),
        wallpapers: WALLPAPERS,
    }
}
