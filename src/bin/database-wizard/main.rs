mod cli;
mod service;

use clap::Clap;
use cli::{Commands, Options};
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

            service::create_database(
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