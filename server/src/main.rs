#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

mod cors;
mod csp;
mod postgres;

use dotenv::dotenv;
use relative_path::RelativePath;
use rocket::{fs::NamedFile, shield::Shield};

use std::{env, path::PathBuf};

use crate::{cors::CrossOriginResourceSharing, csp::ContentSecurityPolicy, postgres::Postgres};

/// Gets the root path of the directory containing client files after building
/// with `npm run build`. By default, this is `/dist` from the directory with
/// the `Cargo.toml` file.
fn get_root_path() -> PathBuf {
    RelativePath::new("dist/").to_logical_path(env!("CARGO_MANIFEST_DIR"))
}

/// Matches against the robots.txt within the /dist root directory.
#[get("/<_..>", rank = 0)]
async fn robots() -> Option<NamedFile> {
    NamedFile::open(get_root_path().join("robots.txt"))
        .await
        .ok()
}

/// Matches against any file within the /dist/assets directory.
#[get("/<file..>", rank = 1)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(get_root_path().join("assets/").join(file))
        .await
        .ok()
}

/// Matches against the index.html file within the /dist directory. This is the
/// entry point to your SPA, dynamically populated by Svelte and Vite at build
/// time.
#[get("/<_..>", rank = 2)]
async fn index() -> Option<NamedFile> {
    NamedFile::open(get_root_path().join("index.html"))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(ContentSecurityPolicy::default())
        .attach(CrossOriginResourceSharing::default())
        .attach(Postgres::default())
        .attach(Shield::default())
        .mount("/robots.txt", routes![robots])
        .mount("/assets", routes![static_files])
        .mount("/", routes![index])
}
