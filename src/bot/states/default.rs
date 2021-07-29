use crate::bot::command::Command;
use crate::bot::states::vacancies;
use crate::bot::states::{BotState, Context};
use crate::localization;

pub async fn next(state: &BotState, message: &str, context: &Context<'_, '_>) -> BotState {
    let cmd = Command::try_parse(message);
    if cmd.is_none() {
        context
            .send_reply(localization::command_not_found_message())
            .await;
        return BotState::Default;
    }

    match &cmd.unwrap().kind {
        crate::bot::command::CommandKind::Help => {
            context.send_reply(localization::help_message()).await;
            BotState::Default
        }
        crate::bot::command::CommandKind::Authorize => todo!(),
        crate::bot::command::CommandKind::VacanciesCreate => {
            vacancies::enter_create_state(context).await
        }
        crate::bot::command::CommandKind::VacanciesDelete => todo!(),
        crate::bot::command::CommandKind::VacanciesView => vacancies::list_vacancies(context).await,
        crate::bot::command::CommandKind::Exit => todo!(),
    }
}
