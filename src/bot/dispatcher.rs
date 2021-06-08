use crate::bot::api::AsyncApiWrapper;
use frankenstein::Update;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::RwLock;
use tokio::time::Duration;

enum State {
    None,
}

pub struct Dispatcher {
    api: AsyncApiWrapper,
    user_states: RwLock<HashMap<isize, State>>,
}

impl Dispatcher {
    pub fn new(api: AsyncApiWrapper) -> Self {
        Self {
            api,
            user_states: RwLock::new(HashMap::new()),
        }
    }

    pub fn dispatch(&self, update: Update) {
        println!("GOT!");
    }
}
