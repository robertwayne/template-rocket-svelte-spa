use rocket::{
    fairing::{self, Fairing},
    http::Header,
    Request, Response,
};

/// Attaches a Content Security Policy header to all responses.
#[derive(Debug, Default)]
pub struct ContentSecurityPolicy;

#[rocket::async_trait]
impl Fairing for ContentSecurityPolicy {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Content Security Policy",
            kind: fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(
            Header::new("Content-Security-Policy",
    "default-src 'self'; script-src 'self'; script-src-elem 'self'; script-src-attr 'self'; style-src 'self'; style-src-elem 'self' 'unsafe-inline'; style-src-attr 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self' data:; connect-src 'self'; media-src 'self'; object-src 'none'; prefetch-src 'self'; child-src 'none'; frame-src 'none'; worker-src 'self'; frame-ancestors 'none'; form-action 'self'; upgrade-insecure-requests; base-uri 'self'; manifest-src 'self' data:"
        ));
    }
}
