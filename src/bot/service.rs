use crate::bot::{api::Api, shared::Shared};
use crate::bot::states::{BotState, Context};
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
            let i = *user_id;
            let m = messages.clone();
            let s = self.shared.clone();
            task::spawn(async move {
                Service::process_batch(i, m, s).await;
            });
        }
    }

    async fn get_initial_state(user_id: u64, shared: &Arc<Shared>) -> BotState {
        let states = shared.frames.lock().await;
        states.get(&user_id).unwrap_or(&BotState::Default).clone()
    }

    async fn store_final_state(state: BotState, user_id: u64, shared: &Arc<Shared>) {
        let mut states = shared.frames.lock().await;
        states.insert(user_id, state);
    }

    async fn process_batch(user_id: u64, messages: Vec<Message>, shared: Arc<Shared>) {
        let mut state = Service::get_initial_state(user_id, &shared).await;

        for message in &messages {
            Service::process_message(message, &mut state, &shared).await;
        }

        Service::store_final_state(state, user_id, &shared).await;
    }

    async fn process_message(message: &Message, state: &mut BotState, shared: &Arc<Shared>) {
        if let Some(text) = message.text() {
            let context = Context {
                chat_id: message.chat().id(),
                api: &shared.api,
            };

            *state = state.next(&text, &context).await;
        }
    }
}
