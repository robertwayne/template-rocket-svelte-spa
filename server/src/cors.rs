use std::env;

use rocket::{
    fairing::{self, Fairing},
    http::Header,
    Request, Response,
};

#[derive(Debug, Default)]
pub struct Cors;

/// Attaches CORS headers to all responses.
#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Cors",
            kind: fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string()),
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Accept, Content-Type",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "DELETE, GET, HEAD, OPTIONS, PATCH, POST, PUT",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
