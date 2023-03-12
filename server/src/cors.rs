use std::env;

use rocket::{
    fairing::{self, Fairing},
    http::{Header, Method, Status},
    Request, Response,
};

/// Attaches a CORS policy header to defined responses.
#[derive(Debug, Default)]
pub struct CrossOriginResourceSharing;

#[rocket::async_trait]
impl Fairing for CrossOriginResourceSharing {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Cross-Origin Resource Sharing",
            kind: fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string()),
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Accept, Content-Type",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "DELETE, GET, HEAD, OPTIONS, PATCH, POST, PUT",
        ));
        response.set_header(Header::new("Vary", "Origin"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Access-Control-Max-Age", "86400"));

        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
        }
    }
}
