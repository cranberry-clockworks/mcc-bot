use crate::bot::states::{BotState, Context, VacancyCreateState};
use crate::localization;

pub async fn enter_create_state(context: &Context<'_, '_>) -> BotState {
    context
        .send_reply(localization::vacancies_enter_title())
        .await;
    BotState::VacancyCreate(VacancyCreateState::InputTitle)
}

pub async fn next_create_state(
    state: &VacancyCreateState,
    message: &str,
    context: &Context<'_, '_>,
) -> BotState {
    match &state {
        VacancyCreateState::InputTitle => {
            context
                .send_reply(localization::vacancies_enter_description())
                .await;
            BotState::VacancyCreate(VacancyCreateState::InputDescription {
                title: String::from(message),
            })
        }
        VacancyCreateState::InputDescription { title } => {
            context
                .db
                .insert_vacancy(context.user_id, title, message)
                .await;
            context
                .send_reply(format!("Title:\n{}\n\nDescription:\n{}", title, message))
                .await;
            BotState::Default
        }
    }
}
