#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

mod cors;
mod csp;
mod postgres;

use cors::CrossOriginResourceSharing;
use csp::ContentSecurityPolicy;
use dotenv::dotenv;
use postgres::Postgres;
use relative_path::RelativePath;
use rocket::{fs::NamedFile, shield::Shield};

use std::{env, path::PathBuf};

/// Gets the root path of the directory containing client files after building
/// with `npm run build`. By default, this is `/dist` from the directory with
/// the `Cargo.toml` file.
fn get_root_path() -> PathBuf {
    RelativePath::new("dist/").to_logical_path(env!("CARGO_MANIFEST_DIR"))
}

#[get("/<_..>", rank = 0)]
async fn robots() -> Option<NamedFile> {
    NamedFile::open(get_root_path().join("robots.txt"))
        .await
        .ok()
}

#[get("/<_..>", rank = 2)]
async fn index() -> Option<NamedFile> {
    NamedFile::open(get_root_path().join("index.html"))
        .await
        .ok()
}

#[get("/<file..>", rank = 1)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(get_root_path().join("assets/").join(file))
        .await
        .ok()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(CrossOriginResourceSharing::default())
        .attach(ContentSecurityPolicy::default())
        .attach(Shield::default())
        .attach(Postgres::default())
        .mount("/robots.txt", routes![robots])
        .mount("/assets", routes![static_files])
        .mount("/", routes![index])
}
