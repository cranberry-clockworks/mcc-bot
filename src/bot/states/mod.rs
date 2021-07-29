use crate::bot::api::Api;
use crate::database::Requester;

mod default;
mod vacancies;

pub struct Context<'a, 'db> {
    pub user_id: i64,
    pub chat_id: i64,
    pub api: &'a Api,
    pub db: Requester<'db>,
}

impl<'a, 'db> Context<'a, 'db> {
    pub async fn send_reply(&self, message: String) {
        self.api
            .send_reply(message, self.chat_id)
            .await
            .unwrap_or_else(|error| log::error!("Failed to send reply. Error:{:#?}", error));
    }
}

#[derive(Clone)]
pub enum BotState {
    Default,
    VacancyCreate(VacancyCreateState),
}

#[derive(Clone)]
pub enum VacancyCreateState {
    InputTitle,
    InputDescription { title: String },
}

impl BotState {
    pub async fn next(&self, message: &str, context: &Context<'_, '_>) -> BotState {
        match &self {
            BotState::Default => default::next(&self, message, context).await,
            BotState::VacancyCreate(s) => vacancies::next_create_state(s, message, context).await,
        }
    }
}
