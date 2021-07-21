use crate::bot::{api::Api, Frame};
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct Shared {
    pub api: Api,
    pub frames: Mutex<HashMap<u64, Frame>>,
}

impl Shared {
    pub fn new(api: Api) -> Self {
        Self {
            api,
            frames: Mutex::new(HashMap::new()),
        }
    }
}
