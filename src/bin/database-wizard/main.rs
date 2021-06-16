use std::path::Path;

use clap::Clap;

use cli::{Commands, Options};
use mccbot::database::PgConnectOptions;
use mccbot::settings::Settings;

use crate::error::{ErrorCode, terminate};

mod cli;
mod error;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let main_options = Options::parse();

    let settings_path = Path::new(&main_options.settings);
    let settings = Settings::from_file(&settings_path).unwrap_or_else(|e| {
        log::error!(
            "Failed to read settings file: '{}'. Error: {}",
            main_options.settings,
            e
        );
        terminate(ErrorCode::InvalidSettings);
    });

    match main_options.command {
        Commands::Create(opts) => {
            create(&settings, &opts.master_username).await;
        }

        Commands::Migrate => {
            migrate(&settings).await;
        }
    }

    println!("Complete!");
    Ok(())
}

async fn create(settings: &Settings, master_username: &str) {
    let master_password = request_secret(&format!(
        "Enter the password for master username: {}",
        master_username
    ));

    let db = &settings.database;

    {
        let options = PgConnectOptions::new()
            .host(&db.host)
            .port(db.port)
            .username(master_username)
            .password(&master_password);
        let connection = service::connect(options).await;

        service::init_database(&connection, &db.username, &db.password, &db.database).await;
    }

    migrate(settings).await;
}

async fn migrate(settings: &Settings) {
    let options = settings.to_database_options();
    let connection = service::connect(options).await;
    service::migrate(&connection).await;

    log::info!("Database migration finished!");
}

fn request_secret(message: &str) -> String {
    println!("{}", message);
    rpassword::read_password().expect("Failed to read a password from stdin!")
}
