use crate::bot::api::Api;
use crate::bot::shared::Shared;
use frankenstein::{Message, Update};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task;

pub struct Service {
    shared: Arc<Shared>,
}

impl Service {
    pub fn new(telegram_token: &str) -> Self {
        let api = Api::new(telegram_token);
        let shared = Arc::new(Shared::new(api));

        Self { shared }
    }

    pub async fn run(&self) {
        loop {
            match &self.shared.api.get_updates().await {
                Ok(updates) => self.schedule(updates).await,
                Err(e) => log::error!("Failed to fetch updates: {:?}. Retrying...", e),
            }
        }
    }

    fn group_by_user_id(updates: &Vec<Update>) -> HashMap<u64, Vec<Message>> {
        let mut result = HashMap::<u64, Vec<Message>>::new();
        for update in updates {
            if let Some(message) = update.message() {
                if let Some(user) = message.from() {
                    let s = result.get_mut(&user.id);
                    if s.is_none() {
                        result.insert(user.id, vec![message]);
                    } else {
                        s.unwrap().push(message);
                    }
                }
            }
        }

        return result;
    }

    async fn schedule(&self, updates: &Vec<Update>) {
        let grouped = Service::group_by_user_id(updates);
        for (user_id, messages) in grouped.iter() {
            let m = messages.clone();
            task::spawn(async move {
                println!("{:#?}", m);
            });
        }
        
//        for update in updates {
//            let u = update.clone();
//            let h = self.handler.clone();
//
//            task::spawn(async move {
//                h.handle(u).await;
//            });
//        }
    }
}
