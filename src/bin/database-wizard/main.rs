mod cli;
mod service;

use clap::Clap;
use cli::{Commands, Options};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let main_options = Options::parse();

    match main_options.command {
        Commands::Create(command_options) => {
            execute_create_command(
                &main_options.host,
                main_options.port,
                &command_options.master_username,
                &command_options.owner_username,
                &command_options.database_name,
            )
            .await;
        }

        Commands::Migrate(command_options) => {
            execute_migrate_command(
                &main_options.host,
                main_options.port,
                &command_options.owner_username,
            )
            .await;
        }
    }

    Ok(())
}

async fn execute_create_command(
    host: &str,
    port: u16,
    master_username: &str,
    owner_username: &str,
    database_name: &str,
) {
    let master_password = request_password(&format!(
        "Enter the password for master username: {}",
        master_username
    ));

    let owner_password = request_password(&format!(
        "Enter the password for the new user: {}",
        owner_username
    ));

    let connection =
        service::create_db_connection(host, port, master_username, &master_password).await;

    service::create_database(&connection, owner_username, &owner_password, database_name).await;

    run_migration(host, port, owner_username, &owner_password).await;

    log::info!("Database initialization finished!");
}

async fn execute_migrate_command(host: &str, port: u16, username: &str) {
    let password = request_password(&format!("Enter password for user: {}", username));

    run_migration(host, port, username, &password).await;

    log::info!("Database migration finished!");
}

async fn run_migration(host: &str, port: u16, username: &str, password: &str) {
    let connection = service::create_db_connection(&host, port, &username, &password).await;

    service::run_migrations(&connection).await;
}

fn request_password(message: &str) -> String {
    println!("{}", message);
    rpassword::read_password().expect("Failed to read a password from stdin!")
}
