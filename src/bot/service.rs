use crate::bot::api::AsyncApiWrapper;
use frankenstein::ChatIdEnum;
use frankenstein::GetUpdatesParams;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, Update, Error};
use tokio::task;

pub struct Service {
    api: AsyncApiWrapper,
}

impl Service {
    pub fn new(telegram_token: &str) -> Self {
        Self {
            api: AsyncApiWrapper::new(telegram_token),
        }
    }

    pub async fn run(&self) {
        loop {
            match self.api.get_updates().await.unwrap() {
                Ok(updates) => self.schedule(updates),
                Err(e) => log::error!("Failed to fetch updates: {:#?}. Retrying...", e),
            }
        }
    }

    fn schedule(&self, updates: Vec<Update>) {
        for update in updates {
            let a = self.api.clone();
            let u = update.clone();

            task::spawn(async move {
                process_update(a, u);
            });
        }
    }
}

fn process_update(api: AsyncApiWrapper, update: Update) {
    println!("Update {}", update.update_id);
}
