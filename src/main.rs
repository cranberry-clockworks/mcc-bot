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
}
