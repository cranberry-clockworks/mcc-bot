use frankenstein::{ChatId, Error, GetUpdatesParams, SendMessageParams, TelegramApi, Update};
use tokio::sync::Mutex;
use tokio::task;

pub struct Api {
    api: Mutex<Box<frankenstein::Api>>,
    update_parameters: Mutex<GetUpdatesParams>,
}

impl Api {
    pub fn new(token: &str) -> Self {
        let timeout_seconds: u32 = 1;
        let mut params = GetUpdatesParams::new();
        params.set_timeout(Some(timeout_seconds));
        params.set_allowed_updates(Some(vec!["message".to_string()]));

        Self {
            api: Mutex::new(Box::new(frankenstein::Api::new(token))),
            update_parameters: Mutex::new(params),
        }
    }

    pub async fn get_updates(&self) -> Result<Vec<Update>, Error> {
        let updates: Vec<Update>;

        let mut params_locked = self.update_parameters.lock().await;
        {
            let api_locked = self.api.lock().await;
            updates = task::block_in_place(|| api_locked.get_updates(&params_locked))?.result;
        }

        if let Some(latest) = updates.iter().map(|u| u.update_id).max() {
            params_locked.set_offset(Some(latest + 1));
        }

        return Ok(updates);
    }

    pub async fn send_reply(&self, text: String, chat_id: i64) -> Result<(), Error> {
        let mut send_params = SendMessageParams::new(ChatId::Integer(chat_id), text);
//        send_params.set_parse_mode(Some(String::from("MarkdownV2")));

        let api_locked = self.api.lock().await;
        let _ = task::block_in_place(|| api_locked.send_message(&send_params))?;
        Ok(())
    }
}
