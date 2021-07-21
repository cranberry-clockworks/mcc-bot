use crate::bot::command::CommandKind;

pub fn default_command_list() -> String {
    format!(
        "Список команд:\n\
        {0} - показать список команд",
        CommandKind::Help
    )
}

pub fn help_message() -> String {
    format!(
        "Бот Московской Центральной Церкви\n\n{}",
        default_command_list()
    )
}

pub fn command_not_found_message() -> String {
    format!("Неизвестная команда!\n\n{0}", default_command_list())
}
