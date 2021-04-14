use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

pub struct DatabaseConnection {
    pub handle: sqlx::Pool<sqlx::Postgres>,
}

impl DatabaseConnection {
    pub async fn connect(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> Result<DatabaseConnection, sqlx::Error> {
        let options = PgConnectOptions::new()
            .host(host)
            .username(&username)
            .password(&password)
            .port(port)
            .ssl_mode(PgSslMode::Prefer);

        Ok(DatabaseConnection {
            handle: PgPoolOptions::new().connect_with(options).await?,
        })
    }
}
