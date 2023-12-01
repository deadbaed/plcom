use rocket::fairing::{self, Fairing, Kind};
use rocket::http::ContentType;
use rocket::{Request, Response};

pub struct Minify;

#[rocket::async_trait]
impl Fairing for Minify {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Minify HTML",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        if response.content_type() == Some(ContentType::HTML) {
            let body = response.body_mut();

            if let Ok(original) = body.to_bytes().await {
                let cfg = minify_html::Cfg {
                    // Be HTML spec compliant
                    do_not_minify_doctype: true,
                    ensure_spec_compliant_unquoted_attribute_values: true,
                    keep_spaces_between_attributes: true,

                    // The rest
                    keep_closing_tags: true,
                    keep_html_and_head_opening_tags: true,
                    minify_css: false,
                    minify_js: true,
                    ..Default::default()
                };
                let minified = minify_html::minify(&original, &cfg);
                response.set_sized_body(minified.len(), std::io::Cursor::new(minified));
            }
        }
    }
}
