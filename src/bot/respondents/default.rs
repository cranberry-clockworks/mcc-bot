use crate::bot::command::{Command, CommandKind};
use crate::bot::Frame;
use crate::localization;

pub fn response(message: &str, frame: &mut Frame) -> Option<String> {
    let command = Command::try_parse(message);
    if command.is_none() {
        return Some(localization::command_not_found_message());
    }

    match &command.unwrap().kind {
        CommandKind::Help => Some(generate_help_message()),
        _ => Some(localization::command_not_found_message()),
    }
}

fn generate_help_message() -> String {
    localization::help_message()
}
