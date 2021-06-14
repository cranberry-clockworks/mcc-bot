use super::api::Api;
use crate::bot::shared::Shared;
use frankenstein::EditMessageResponse::Message;
use frankenstein::Update;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tokio::time::Duration;

pub struct Dispatcher {
    shared: Arc<Shared>,
}

impl Dispatcher {
    pub fn new(shared: Arc<Shared>) -> Self {
        Self { shared }
    }

    pub async fn dispatch(&self, update: Update) {
        if let Some(message) = &update.message {
            if let (Some(user), Some(text)) = (&message.from(), &message.text()) {
                self.dispatch_unpacked(user.id, message.chat().id, text.to_string())
                    .await;
            } else {
                log::debug!("Received message without text.");
            }
        } else {
            log::debug!("Received non text message while expected.");
        }
    }

    async fn dispatch_unpacked(&self, user_id: isize, chat_id: isize, text: String) {
        if text.eq("/help") {
            self.handle_help_command(chat_id).await;
        }
    }

    async fn handle_help_command(&self, chat_id: isize) {
        let help_message = "This is mcc bot!\n\
            List of available commands:\n\
            `/help` - See this command"
            .to_string();

        self.shared
            .api
            .send_reply(help_message, chat_id)
            .await
            .unwrap_or_else(|e| {
                log::error!("Failed to send a reply: {:?}!", e);
            })
    }
}