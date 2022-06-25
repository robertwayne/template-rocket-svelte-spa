#![forbid(unsafe_code)]

mod cors;
mod csp;
mod postgres;

#[macro_use]
use csp::ContentSecurityPolicy;

use cors::Cors;
use dotenv::dotenv;
use postgres::Postgres;
use relative_path::RelativePath;
use rocket::{fs::NamedFile, shield::Shield};

use std::{env, path::PathBuf};

fn get_root_path() -> PathBuf {
    RelativePath::new("dist/").to_logical_path(env!("CARGO_MANIFEST_DIR"))
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
        .attach(Cors::default())
        .attach(ContentSecurityPolicy::default())
        .attach(Shield::default())
        .attach(Postgres::default())
        .mount("/assets", routes![static_files])
        .mount("/robots.txt", routes![static_files])
        .mount("/", routes![index])
}
