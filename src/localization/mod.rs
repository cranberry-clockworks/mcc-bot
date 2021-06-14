use crate::bot::commands;

pub fn help_message() -> String {
    format!(
        "Бот Московской Центральной Церкви\n\n\
    Список комманд:\n
    {0} — показать список команд.",
        commands::help()
    )
}
