use crate::bot::api::Api;
use crate::bot::states::BotState;
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct Shared {
    pub api: Api,
    pub frames: Mutex<HashMap<u64, BotState>>,
}

impl Shared {
    pub fn new(api: Api) -> Self {
        Self {
            api,
            frames: Mutex::new(HashMap::new()),
        }
    }
}
