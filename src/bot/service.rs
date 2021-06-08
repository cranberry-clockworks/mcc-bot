use crate::bot::api::AsyncApiWrapper;
use crate::bot::dispatcher::Dispatcher;
use frankenstein::Update;
use std::ops::Deref;
use std::sync::Arc;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time;
use std::time::Duration;

pub struct Service {
    api: AsyncApiWrapper,
    dispatcher: Arc<Dispatcher>,
}

impl Service {
    pub fn new(telegram_token: &str) -> Self {
        let api = AsyncApiWrapper::new(telegram_token);
        Self {
            api: api.clone(),
            dispatcher: Arc::new(Dispatcher::new(api)),
        }
    }

    pub async fn run(&self) {
        loop {
            match self.api.get_updates().await.unwrap() {
                Ok(updates) => self.schedule(updates).await,
                Err(e) => log::error!("Failed to fetch updates: {:#?}. Retrying...", e),
            }
        }
    }

    async fn schedule(&self, updates: Vec<Update>) {
        for update in updates {
            let d = self.dispatcher.clone();
            let u = update.clone();

            let _ = task::spawn(async move {
                d.dispatch(&u).await;
            });
        }
    }
}
