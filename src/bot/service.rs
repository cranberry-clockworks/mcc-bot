use crate::bot::api::Api;
use crate::bot::dispatcher::Dispatcher;
use crate::bot::shared::Shared;
use frankenstein::Update;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time;

pub struct Service {
    shared: Arc<Shared>,
    dispatcher: Arc<Dispatcher>,
}

impl Service {
    pub fn new(telegram_token: &str) -> Self {
        let api = Api::new(telegram_token);
        let shared = Arc::new(Shared::new(api));
        let dispatcher = Arc::new(Dispatcher::new(shared.clone()));

        Self { shared, dispatcher }
    }

    pub async fn run(&self) {
        loop {
            match &self.shared.api.get_updates().await {
                Ok(updates) => self.schedule(updates).await,
                Err(e) => log::error!("Failed to fetch updates: {:?}. Retrying...", e),
            }
        }
    }

    async fn schedule(&self, updates: &Vec<Update>) {
        for update in updates {
            let u = update.clone();
            let d = self.dispatcher.clone();

            task::spawn(async move {
                d.dispatch(u).await;
            });
        }
    }
}
