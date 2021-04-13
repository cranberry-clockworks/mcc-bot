mod creation;
mod migration;

pub use creation::create_database;
pub use migration::run_migrations;

use mccbot::error;

pub async fn create_db_connection(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
) -> sqlx::Pool<sqlx::Postgres> {
    use sqlx::postgres;

    let options = postgres::PgConnectOptions::new()
        .host(host)
        .username(&username)
        .password(&password)
        .port(port)
        .ssl_mode(postgres::PgSslMode::Prefer);

    postgres::PgPoolOptions::new()
        .connect_with(options)
        .await
        .unwrap_or_else(|_| {
            log::error!(
                "Failed to connect to the database! Host: {}, port: {}, username: {}.",
                host,
                port,
                username
            );
            error::terminate(error::ExitCode::ConnectionFailure)
        })
}
