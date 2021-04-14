use mccbot::database::DatabaseConnection;
use mccbot::error;

pub async fn create_database(
    connection: &DatabaseConnection,
    owner_username: &str,
    owner_password: &str,
    database_name: &str,
) {
    create_user_instance(&connection, owner_username, owner_password).await;
    create_database_instance(&connection, owner_username, database_name).await;
}

async fn create_user_instance(
    connection: &DatabaseConnection,
    owner_username: &str,
    owner_password: &str,
) {
    sqlx::query(&format!(
        "CREATE USER {} WITH ENCRYPTED PASSWORD '{}'",
        &owner_username, &owner_password
    ))
    .execute(&connection.handle)
    .await
    .unwrap_or_else(|_| {
        log::error!("Failed to create user: {}", owner_username);
        error::terminate(error::ExitCode::UserCreationFailure)
    });
}

async fn create_database_instance(
    connection: &DatabaseConnection,
    owner_username: &str,
    database_name: &str,
) {
    sqlx::query(&format!(
        "CREATE DATABASE \"{}\" WITH OWNER = {} ENCODING = 'UTF-8'",
        &owner_username, &database_name
    ))
    .execute(&connection.handle)
    .await
    .unwrap_or_else(|_| {
        log::error!("Failed to create database: {}", database_name);
        error::terminate(error::ExitCode::DatabaseCreationFailure)
    });
}
