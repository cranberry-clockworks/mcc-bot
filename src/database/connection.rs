use sqlx::postgres::{PgPoolOptions, PgSslMode};

pub use sqlx::postgres::PgConnectOptions;

pub struct DatabaseConnection {
    pub connection: sqlx::Pool<sqlx::Postgres>,
}

impl DatabaseConnection {
    pub async fn new(options: PgConnectOptions) -> Result<DatabaseConnection, sqlx::Error> {
        Ok(DatabaseConnection {
            connection: PgPoolOptions::new().connect_with(options).await?,
        })
    }
}
