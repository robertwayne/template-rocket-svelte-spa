#[forbid(unsafe_code)]
#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use relative_path::RelativePath;
use rocket::{fs::NamedFile, http::Method};
use rocket_cors::AllowedOrigins;

use rocket_pg_sqlx::postgres_fairing;
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

    let allowed_origins = AllowedOrigins::All;

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Delete,
            Method::Put,
            Method::Patch,
            Method::Post,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to construct CORS option struct.");

    rocket::build()
        .attach(postgres_fairing())
        .attach(cors)
        .mount("/assets", routes![static_files])
        .mount("/", routes![index])
}
