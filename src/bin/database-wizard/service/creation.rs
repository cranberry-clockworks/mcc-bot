use crate::service::query::QueryTool;
use crate::error::{terminate, ErrorCode};

use mccbot::database::DatabaseConnection;

pub async fn init_database(
    database: &DatabaseConnection,
    owner_username: &str,
    owner_password: &str,
    database_name: &str,
) {
    let tool = QueryTool::new(database);
    tool.create_user(owner_username, owner_password)
        .await
        .unwrap_or_else(|e| {
            log::error!("Failed to create user: {}. Error: {}", owner_username, e);
            terminate(ErrorCode::UserCreationFailure)
        });

    tool.create_database(database_name, owner_username)
        .await
        .unwrap_or_else(|e| {
            log::error!("Failed to create database: {}. Error: {}", database_name, e);
            terminate(ErrorCode::DatabaseCreationFailure)
        });
}
