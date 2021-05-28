use mccbot::settings;
use mccbot::database::{DatabaseConnection, PgConnectOptions, Token};
use mccbot::error::{terminate, ErrorCode};
use std::path::Path;
use frankenstein::PassportElementError::PassportElementErrorTranslationFilesVariant;
use mccbot::settings::Settings;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    pretty_env_logger::init();

    let settings_path = Path::new("settings.toml");
    let settings = Settings::from_file(&settings_path).unwrap_or_else(
        |e| {
            log::error!("Failed to read settings from '{}'. Error: {}", settings_path.to_str().unwrap(), e);
            terminate(ErrorCode::InvalidSettings);
        }
    );

    // let db = DatabaseConnection::new(options).await.unwrap_or_else(|e| {
    //     log::error!("Failed to connect to the database! Error: {}", e);
    //     terminate(ErrorCode::FailedEstablishConnectionWithDatabase);
    // });
    //
    // let token = Token::get_telegram_token(&db).await.unwrap_or_else(|e| {
    //     log::error!("Failed to fetch telegram token from database! Error: {}", e);
    //     terminate(ErrorCode::BadEnvironment);
    // });
    //
    // println!("Token is: {}", token);
}
