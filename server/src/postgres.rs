use rocket::{
    fairing::{self, Fairing},
    Build, Rocket,
};
use sqlx::{postgres::PgConnectOptions, PgPool};
use std::env;

/// Creates and attaches a Postgres connection pool to the global &State<T> in
/// Rocket.
///
/// FIXME: Replace with built-in Rocket DB pools.
#[derive(Debug, Default)]
pub struct Postgres;

#[rocket::async_trait]
impl Fairing for Postgres {
    fn info(&self) -> fairing::Info {
        fairing::Info { name: "Postgres", kind: fairing::Kind::Ignite }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let pool = PgPool::connect_with({
            PgConnectOptions::new()
                .host(env::var("PGHOST").unwrap_or_else(|_| "localhost".into()).as_str())
                .port(
                    env::var("PGPORT")
                        .unwrap_or_else(|_| "5432".into())
                        .parse::<u16>()
                        .unwrap_or_default(),
                )
                .username(env::var("PGUSER").unwrap_or_else(|_| "postgres".into()).as_str())
                .password(env::var("PGPASS").unwrap_or_else(|_| "postgres".into()).as_str())
                .database(env::var("PGDBNAME").unwrap_or_else(|_| "postgres".into()).as_str())
        })
        .await
        .expect("Failed to connect to PostgreSQL database.");

        Ok(rocket.manage(pool))
    }
}
