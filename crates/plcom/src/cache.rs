use rocket::fairing::{self, Fairing};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};

#[derive(Debug)]
pub struct CacheControl {
    duration_secs: u32,
}

impl Default for CacheControl {
    fn default() -> Self {
        CacheControl {
            duration_secs: 60 * 60, // 60 secs * 60 minutes
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
        // Aggressive caching
        if request.method() == Method::Get && response.status() == Status::Ok {
            response.set_header(Header::new(
                "Cache-Control",
                format!("public, max-age={}", self.duration_secs),
            ));
        }
    }
}
