mod prelude {
    pub use crate::common::icon::Icon;
    pub use crate::common::link::*;
    pub use crate::common::Date;
    pub use crate::views::*;
    pub use leptos::prelude::*;
    pub use rocket::uri;
    pub use tailwind_fuse::tw_join;
}

mod cache;
mod common;
mod pages;
mod views;

use pages::*;
use rocket::fairing::AdHoc;

#[derive(rocket::serde::Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Config {
    assets: String,
    blog_feed: String,
    wallpapers: Option<String>,
}

pub fn config() -> rocket::figment::Figment {
    use rocket::figment::providers::*;
    use rocket::figment::Figment;
    use rocket::Config;

    // rocket defaults
    Figment::from(Config::default())
        // from env variables directly
        .merge(Env::prefixed("PLCOM_").ignore(&["PROFILE"]).global())
}

#[rocket::launch]
fn rocket() -> _ {
    let rocket = rocket::custom(config());
    let figment = rocket.figment();

    let config: Config = figment.extract().expect("server configuration");
    println!("Using configuration {config:#?}");

    let server = rocket;

    // Serve wallpapers through rocket only during development
    // In prodution, use nginx
    let server = match config.wallpapers {
        Some(path) => server.mount(
            "/wallpapers/files",
            rocket::fs::FileServer::from(path).rank(30),
        ),
        None => server,
    };

    let server = server
        .mount("/", rocket::fs::FileServer::from(config.assets))
        .mount(
            "/",
            rocket::routes![root_route, email_route, wallpapers_route],
        )
        .register("/", rocket::catchers![not_found])
        .attach(AdHoc::config::<Config>());

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
    }
}
