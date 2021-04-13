use crate::misc;

use log::error;

pub async fn create_database(
    connection: &sqlx::Pool<sqlx::Postgres>,
    owner_username: &str,
    owner_password: &str,
    database_name: &str,
) {
    create_user_instance(&connection, owner_username, owner_password).await;
    create_database_instance(&connection, owner_username, database_name).await;
}

async fn create_user_instance(
    connection: &sqlx::Pool<sqlx::Postgres>,
    owner_username: &str,
    owner_password: &str,
) {
    sqlx::query(&format!(
        "CREATE USER {} WITH ENCRYPTED PASSWORD '{}'",
        &owner_username, &owner_password
    ))
    .execute(connection)
    .await.unwrap_or_else(|_| {
        error!("Failed to create user: {}", owner_username);
        misc::terminate(ExitCode::UserCreationFailure)
    })
    .expect("Failed to create user");
}

async fn create_database_instance(
    connection: &sqlx::Pool<sqlx::Postgres>,
    owner_username: &str,
    database_name: &str,
) {
    sqlx::query(&format!(
        "CREATE DATABASE \"{}\" WITH OWNER = {} ENCODING = 'UTF-8'",
        &owner_username, &database_name
    ))
    .execute(connection)
    .await
    .expect("Failed to create database");
}
