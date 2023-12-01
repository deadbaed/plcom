use rocket::fairing::{self, Fairing};
use rocket::http::{ContentType, Header};
use rocket::{Request, Response};

#[derive(Debug)]
pub struct CacheControl {
    duration_secs: u32,
    types: Vec<ContentType>,
    routes: Vec<&'static str>,
}

impl Default for CacheControl {
    fn default() -> Self {
        CacheControl {
            duration_secs: 60 * 60, // 60 secs * 60 minutes
            types: vec![ContentType::CSS, ContentType::JavaScript],
            routes: vec!["/wallpapers", "/pub", "/images", "/icons"],
        }
    }
}

#[rocket::async_trait]
impl Fairing for CacheControl {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Cache Control",
            kind: fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let mut should_cache = false;

        // Check if content type matches
        if let Some(content_type) = response.content_type() {
            if self.types.contains(&content_type) {
                should_cache = true;
            }
        }

        // Check if route matches
        self.routes
            .iter()
            .filter(|s| request.uri().path().starts_with(*s))
            .for_each(|_| should_cache = true);

        if should_cache {
            response.set_header(Header::new(
                "Cache-Control",
                format!("public, max-age={}", self.duration_secs),
            ));
        }
    }
}
