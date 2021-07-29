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
            BotState::Default
        }
    }
}

pub async fn list_vacancies(context: &Context<'_, '_>) -> BotState {
    let reply = context
        .db
        .select_vacancies()
        .await
        .iter()
        .map(|v| format!("#:{}\t {}", v.id, v.title))
        .collect::<Vec<String>>()
        .join("\n");

    context.send_reply(reply).await;

    BotState::Default
}
