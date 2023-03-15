#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

mod cache;
mod cors;
mod csp;
mod postgres;

use std::{
    env,
    path::{Path, PathBuf},
};

use dotenvy::dotenv;
use rocket::{
    fs::{relative, NamedFile},
    shield::Shield,
};
use tracing_subscriber::EnvFilter;

use crate::{cache::CacheControl, cors::CrossOriginResourceSharing, csp::ContentSecurityPolicy};

const DIST: &str = relative!("dist");

/// Matches against the robots.txt within the /dist root directory.
#[get("/<_..>", rank = 0)]
async fn robots() -> Option<NamedFile> {
    NamedFile::open(Path::new(DIST).join("robots.txt"))
        .await
        .ok()
}

/// Matches against any file within the /dist/assets directory.
#[get("/<file..>", rank = 1)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(DIST).join("assets/").join(file))
        .await
        .ok()
}

/// Matches against the index.html file within the /dist directory. This is the
/// entry point to your SPA, dynamically populated by Svelte and Vite at build
/// time.
#[get("/<_..>", rank = 2)]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new(DIST).join("index.html"))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    rocket::build()
        .attach(CacheControl::default())
        .attach(ContentSecurityPolicy::default())
        .attach(CrossOriginResourceSharing::default())
        .attach(Shield::default())
        .mount("/robots.txt", routes![robots])
        .mount("/assets", routes![static_files])
        .mount("/", routes![index])
}
