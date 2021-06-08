use super::api::AsyncApiWrapper;
use frankenstein::EditMessageResponse::Message;
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

    pub async fn dispatch(&self, update: &Update) {
        if let Some(message) = &update.message {
            if let (Some(user), Some(text)) = (&message.from(), &message.text()) {
                self.dispatch_unpacked(user.id, message.chat().id, text.to_string())
                    .await;
            } else {
                log::warn!("Failed to unpack message.");
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
        let _ = self
            .api
            .send_reply("hello".to_string(), chat_id)
            .await
            .unwrap()
            .unwrap_or_else(|e| {
                log::error!("Failed to send reply: {:?}", e);
            });
    }
}
