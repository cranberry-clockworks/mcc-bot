use crate::bot::command::CommandKind;

fn default_command_list() -> String {
    format!(
        "Список команд:\n\
        `{0}` — показать список команд\n\
        
        Доска служений:\n\
        `{1}` — Создать новую вакансию\n\
        `{2} <id>` — Удалить вакансию\n\
        `{3}` — Просмотреть список открытых вакансий\n",
        CommandKind::Help,
        CommandKind::VacanciesCreate,
        CommandKind::VacanciesDelete,
        CommandKind::VacanciesView
    )
}

pub fn help_message() -> String {
    format!(
        "Бот Московской Центральной Церкви\n\n{}",
        default_command_list()
    )
}

pub fn command_not_found_message() -> String {
    format!("Неизвестная команда\\!\n\n{0}", default_command_list())
}

pub fn vacancies_enter_title() -> String {
    String::from("Введите заголовок новой вакансии:")
}

pub fn vacancies_enter_description() -> String {
    String::from("Введите детальное описание вакансии:")
}
