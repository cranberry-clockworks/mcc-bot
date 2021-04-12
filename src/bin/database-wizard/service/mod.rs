mod creation;
mod migration;

pub use creation::create_database;
pub use migration::run_migrations;

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
        .expect("Failed to connect to the Postgrese server!")
}
