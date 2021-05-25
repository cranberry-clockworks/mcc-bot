mod creation;
mod migration;
mod query;

pub use creation::init_database;
pub use migration::migrate;

use mccbot::database::{DatabaseConnection, PgConnectOptions};
use crate::error::{terminate, ErrorCode};
use sqlx::postgres::PgSslMode;

pub async fn connect(options: PgConnectOptions) -> DatabaseConnection {
    let o = options.ssl_mode(PgSslMode::Prefer);

    DatabaseConnection::new(o).await.unwrap_or_else(|e| {
        log::error!(
            "Failed to connect to the database! Error: {}",
            e.into_database_error().unwrap()
        );
        terminate(ErrorCode::ConnectionFailure)
    })
}
