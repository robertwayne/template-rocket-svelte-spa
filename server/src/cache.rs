use rocket::{
    fairing::{self, Fairing},
    http::{ContentType, Header},
    Request, Response,
};

/// Attaches a Cache Control header to all responses. The default implementation
/// caches CSS, JS, WOFF2, and WEBP files only, with a max age of 1 year.
#[derive(Debug)]
pub struct CacheControl {
    max_age: u32,
    cache_types: Vec<ContentType>,
}

impl Default for CacheControl {
    fn default() -> Self {
        CacheControl {
            max_age: 60 * 60 * 24 * 365,
            cache_types: vec![
                ContentType::CSS,
                ContentType::JavaScript,
                ContentType::WOFF2,
                ContentType::WEBP,
            ],
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

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        if let Some(content_type) = response.content_type() {
            if self.cache_types.contains(&content_type) {
                response.set_header(Header::new(
                    "Cache-Control",
                    format!("public, max-age={}", self.max_age),
                ));
            }
        }
    }
}
