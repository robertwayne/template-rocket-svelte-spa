use std::env;

use rocket::{
    fairing::{self, Fairing},
    Build, Rocket,
};
use sqlx::PgPool;

/// Creates and attaches a Postgres connection pool to the global &State<T> in
/// Rocket.
///
/// FIXME: Replace with built-in Rocket DB pools.
#[derive(Debug, Default)]
pub struct Postgres;

#[rocket::async_trait]
impl Fairing for Postgres {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Postgres",
            kind: fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(url.as_str())
            .await
            .expect("Failed to connect to PostgreSQL database.");

        Ok(rocket.manage(pool))
    }
}
