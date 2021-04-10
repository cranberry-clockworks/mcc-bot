mod cli;

use clap::Clap;
use cli::{Commands, CreateOptions, Options};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();

    match options.command {
        Commands::Create(opts) => {
            create_database(&opts);
        }
    }

    Ok(())
}

fn request_password(message: &str) -> String {
    println!("{}", message);
    rpassword::read_password().expect("Failed to read a password from stdin!")
}

fn create_database(options: &CreateOptions) {
    let master_password= request_password(&format!("Enter the master password for user name: {}", options.master_user_name));
    let owner_password = request_password(&format!("Enter the new password for the user: {}", options.owner_user_name));
}
