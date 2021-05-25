mod cli;
mod error;
mod service;

use clap::Clap;
use cli::{Commands, Options};
use mccbot::database::PgConnectOptions;
use mccbot::log::init_with_info_level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let main_options = Options::parse();

    match main_options.command {
        Commands::Create(command_options) => {
            handle_create_command(
                &main_options.host,
                main_options.port,
                &command_options.master_username,
                &command_options.owner_username,
                &command_options.database_name,
            )
            .await;
        }

        Commands::Migrate(command_options) => {
            handle_migrate_command(
                &main_options.host,
                main_options.port,
                &command_options.database_name,
                &command_options.owner_username,
            )
            .await;
        }
    }

    println!("Complete!");
    Ok(())
}

async fn handle_create_command(
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

    {
        let options = PgConnectOptions::new()
            .host(host)
            .port(port)
            .username(master_username)
            .password(&master_password);
        let connection = service::connect(options).await;

        service::init_database(&connection, owner_username, &owner_password, database_name).await;
    }
    {
        let options = PgConnectOptions::new()
        .host(host)
        .port(port)
        .database(database_name)
        .username(owner_username)
        .password(&owner_password);

        let connection = service::connect(options).await;
        service::migrate(&connection).await;
    }
}

async fn handle_migrate_command(host: &str, port: u16, database_name: &str, username: &str) {
    let password = request_password(&format!("Enter password for user: {}", username));

    let options = PgConnectOptions::new()
        .host(host)
        .port(port)
        .database(database_name)
        .username(username)
        .password(&password);

    let connection = service::connect(options).await;

    service::migrate(&connection).await;

    log::info!("Database migration finished!");
}

fn request_password(message: &str) -> String {
    println!("{}", message);
    rpassword::read_password().expect("Failed to read a password from stdin!")
}
