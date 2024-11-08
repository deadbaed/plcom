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

#[rocket::launch]
fn rocket() -> _ {
    let server = rocket::build()
        .mount("/", rocket::fs::FileServer::from("public"))
        .mount(
            "/",
            rocket::routes![root_route, email_route, wallpapers_route],
        )
        .register("/", rocket::catchers![not_found]);

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
