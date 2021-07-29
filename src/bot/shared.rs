use crate::bot::api::Api;
use crate::bot::states::BotState;
use crate::database::DatabaseConnection;
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct Shared {
    pub api: Api,
    pub db: DatabaseConnection,
    pub frames: Mutex<HashMap<i64, BotState>>,
}

impl Shared {
    pub fn new(api: Api, db: DatabaseConnection) -> Self {
        Self {
            api,
            db,
            frames: Mutex::new(HashMap::new()),
        }
    }
}
