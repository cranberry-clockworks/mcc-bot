mod creation;
mod migration;

pub use creation::create_database;
pub use migration::run_migrations;

use mccbot::database::DatabaseConnection;
use mccbot::error;

pub async fn create_db_connection(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
) -> DatabaseConnection {
    DatabaseConnection::connect(host, port, username, password)
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
