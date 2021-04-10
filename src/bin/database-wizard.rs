mod cli;

use clap::Clap;
use cli::{Commands, Options};
use sqlx::postgres;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();

    match options.command {
        Commands::Create(opts) => {
            let master_password = request_password(&format!(
                "Enter the master password for user name: {}",
                opts.master_user_name
            ));

            let owner_password = request_password(&format!(
                "Enter the new password for the user: {}",
                opts.owner_user_name
            ));

            create_database(
                &options.host,
                options.port,
                &opts.master_user_name,
                &master_password,
                &opts.owner_user_name,
                &owner_password,
                &opts.database_name,
            )
            .await;
        }
    }

    Ok(())
}

fn request_password(message: &str) -> String {
    println!("{}", message);
    rpassword::read_password().expect("Failed to read a password from stdin!")
}

async fn create_database(
    host: &str,
    port: u16,
    master_user_name: &str,
    master_password: &str,
    owner_user_name: &str,
    owner_password: &str,
    database_name: &str,
) {
    let connection_options = postgres::PgConnectOptions::new()
        .host(host)
        .username(master_user_name)
        .password(&master_password)
        .port(port)
        .ssl_mode(postgres::PgSslMode::Prefer);

    let connection = postgres::PgPoolOptions::new()
        .connect_with(connection_options)
        .await
        .expect("Failed to connecto to the Postgrese server!");

    create_user_instance(&connection, owner_user_name, database_name).await;
    create_database_instance(connection, owner_user_name, &owner_password).await;
}

async fn create_user_instance(
    connection: &sqlx::Pool<sqlx::Postgres>,
    owner_user_name: &str,
    owner_password: &str,
) {
    sqlx::query(&format!(
        "CREATE USER {} WITH ENCRYPTED PASSWORD '{}'",
        &owner_user_name, &owner_password
    ))
    .execute(connection)
    .await
    .expect("Failed to create user");
}

async fn create_database_instance(
    connection: sqlx::Pool<sqlx::Postgres>,
    owner_user_name: &str,
    database_name: &str,
) {
    sqlx::query(&format!(
        "CREATE DATABASE \"{}\" WITH OWNER = {} ENCODING = 'UTF-8'",
        &owner_user_name, &database_name
    ))
    .execute(&connection)
    .await
    .expect("Failed to create database");
}
