use crate::bot::command;
use crate::bot::command::CommandKind;

pub fn help_message() -> String {
    format!(
        "Бот Московской Центральной Церкви\n\n\
    Список комманд:\n
    {0} — показать список команд.",
        CommandKind::Help
    )
}
