use mccbot::database::{DatabaseConnection, PgConnectOptions};
use mccbot::error::{terminate, ErrorCode};
use mccbot::settings;
use mccbot::settings::Settings;

use std::path::Path;

use frankenstein::PassportElementError::PassportElementErrorTranslationFilesVariant;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    pretty_env_logger::init();

    let settings_path = Path::new("settings.toml");
    let settings = Settings::from_file(&settings_path).unwrap_or_else(|e| {
        log::error!(
            "Failed to read settings from '{}'. Error: {}",
            settings_path.to_str().unwrap(),
            e
        );
        terminate(ErrorCode::InvalidSettings);
    });

    let db_options = settings.to_database_options();
    let db = DatabaseConnection::new(db_options)
        .await
        .unwrap_or_else(|e| {
            log::error!("Failed to connect to the database. Error: {}", e);
            terminate(ErrorCode::FailedConnectToDatabase);
        });

    mccbot::bot::Service::new(&settings.tokens.telegram)
        .run()
        .await;
}
