use super::api::Api;
use super::command;
use crate::bot::command::{Command, CommandKind};
use crate::bot::shared::Shared;
use crate::localization;
use frankenstein::EditMessageResponse::Message;
use frankenstein::Update;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tokio::time::Duration;

pub struct MessageHandler {
    shared: Arc<Shared>,
}

pub struct Identifier {
    user_id: isize,
    chat_id: isize,
}

impl MessageHandler {
    pub fn new(shared: Arc<Shared>) -> Self {
        Self { shared }
    }

    pub async fn handle(&self, update: Update) {
        if let Some(message) = &update.message {
            if let (Some(user), Some(text)) = (&message.from(), &message.text()) {
                let ident = Identifier {
                    user_id: user.id,
                    chat_id: message.chat().id,
                };
                self.dispatch_unpacked(text.to_string(), ident).await;
            } else {
                log::debug!("Received message without text.");
            }
        } else {
            log::debug!("Received non text message while expected.");
        }
    }

    async fn dispatch_unpacked(&self, text: String, ident: Identifier) {
        let command = Command::try_parse(&text);
        if command.is_some() {
            self.handle_command(command.unwrap(), ident).await;
            return;
        }
    }

    async fn handle_command(&self, command: Command, ident: Identifier) {
        match &command.kind {
            CommandKind::Help => {
                self.handle_help_command(ident.chat_id).await;
            }
        }
    }

    async fn handle_help_command(&self, chat_id: isize) {
        let reply = localization::help_message();
        self.shared
            .api
            .send_reply(reply, chat_id)
            .await
            .unwrap_or_else(|e| {
                log::error!("Failed to send a reply: {:?}!", e);
            })
    }
}
