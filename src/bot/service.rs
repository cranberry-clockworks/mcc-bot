use crate::bot::states::{BotState, Context};
use crate::bot::{api::Api, shared::Shared};
use crate::database::{DatabaseConnection, Requester};
use frankenstein::{Message, Update};
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Arc;
use tokio::task;

pub struct Service {
    shared: Arc<Shared>,
}

impl Service {
    pub fn new(telegram_token: &str, db: DatabaseConnection) -> Self {
        let api = Api::new(telegram_token);
        let shared = Arc::new(Shared::new(api, db));

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

    fn group_by_user_id(updates: &Vec<Update>) -> HashMap<i64, Vec<Message>> {
        let mut result = HashMap::<i64, Vec<Message>>::new();
        for update in updates {
            if let Some(message) = update.message() {
                if let Some(user) = message.from() {
                    // According Telegram API documentation user id has no more than
                    // 52 significant bytes. Therefore is perfectly stores in i64.
                    // On the other hand PostgresSQL could store u64 only as NUMERIC
                    // type. Summing up pros and cons we use i64 to store the id value.
                    let user_id = (user.id as i64).try_into().unwrap();
                    result.entry(user_id).or_insert(Vec::new()).push(message);
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

    async fn get_initial_state(user_id: i64, shared: &Arc<Shared>) -> BotState {
        let states = shared.frames.lock().await;
        states.get(&user_id).unwrap_or(&BotState::Default).clone()
    }

    async fn store_final_state(state: BotState, user_id: i64, shared: &Arc<Shared>) {
        let mut states = shared.frames.lock().await;
        states.insert(user_id, state);
    }

    async fn process_batch(user_id: i64, messages: Vec<Message>, shared: Arc<Shared>) {
        let mut state = Service::get_initial_state(user_id, &shared).await;

        for message in &messages {
            Service::process_message(message, user_id, &mut state, &shared).await;
        }

        Service::store_final_state(state, user_id, &shared).await;
    }

    async fn process_message(
        message: &Message,
        user_id: i64,
        state: &mut BotState,
        shared: &Arc<Shared>,
    ) {
        if let Some(text) = message.text() {
            let context = Context {
                user_id,
                chat_id: message.chat().id(),
                api: &shared.api,
                db: Requester::new(&shared.db),
            };

            *state = state.next(&text, &context).await;
        }
    }
}
