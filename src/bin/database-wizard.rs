mod cli;

use clap::Clap;
use cli::{Commands, CreateOptions, Options};
use sqlx::postgres;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();

    match options.command {
        Commands::Create(opts) => {
            create_database(
                &options.host,
                options.port,
                &opts.master_user_name,
                &opts.owner_user_name,
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
    owner_user_name: &str,
    database_name: &str,
) {
    let master_password = request_password(&format!(
        "Enter the master password for user name: {}",
        master_user_name
    ));
    let owner_password = request_password(&format!(
        "Enter the new password for the user: {}",
        owner_user_name
    ));

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
}
