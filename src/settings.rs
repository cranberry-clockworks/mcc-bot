use crate::database::PgConnectOptions;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseConfig,
    pub tokens: Tokens,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    pub database: String,
    pub username: String,
    pub password: String,
}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_port() -> u16 {
    5432
}

#[derive(Deserialize)]
pub struct Tokens {
    pub telegram: String,
}

impl Settings {
    pub fn from_file(filename: &Path) -> Result<Settings, Box<dyn std::error::Error>> {
        let content = read_to_string(filename)?;
        let config: Settings = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn to_database_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.database.host)
            .port(self.database.port)
            .username(&self.database.username)
            .password(&self.database.password)
            .database(&self.database.database)
    }
}

fn read_to_string(filename: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
