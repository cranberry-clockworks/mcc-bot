use crate::bot::api::AsyncApiWrapper;
use frankenstein::Api;
use frankenstein::ChatIdEnum;
use frankenstein::GetUpdatesParams;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;

pub struct IncomingListener {
    api: AsyncApiWrapper,
}

impl IncomingListener {
    pub fn new(telegram_token: &str) -> Self {
        Self {
            api: AsyncApiWrapper::new(telegram_token),
        }
    }

    pub async fn run(&self) {
        loop {
            let result = self.api.get_updates().await.unwrap();

            println!("result: {:?}", result);
        }
    }
}
