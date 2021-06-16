use crate::bot::command::CommandKind::List;
use std::fmt::{Display, Formatter, Result};

pub enum CommandKind {
    Help,
    Authorize,
    Vacancies,
    List,
    View,
    Create,
    Delete,
    Exit,
}

const HELP_COMMAND: &str = "/help";
const AUTHORIZE_COMMAND: &str = "/authorize";
const VACANCIES_COMMAND: &str = "/vacancies";
const LIST_COMMAND: &str = "/list";
const VIEW_COMMAND: &str = "/view";
const CREATE_COMMAND: &str = "/create";
const DELETE_COMMAND: &str = "/delete";
const EXIT_COMMAND: &str = "/exit";

impl CommandKind {
    pub fn try_parse(s: &str) -> Option<CommandKind> {
        match s {
            HELP_COMMAND => Some(CommandKind::Help),
            AUTHORIZE_COMMAND => Some(CommandKind::Authorize),
            VACANCIES_COMMAND => Some(CommandKind::Vacancies),
            LIST_COMMAND => Some(CommandKind::List),
            VIEW_COMMAND => Some(CommandKind::View),
            CREATE_COMMAND => Some(CommandKind::Create),
            DELETE_COMMAND => Some(CommandKind::Delete),
            EXIT_COMMAND => Some(CommandKind::Exit),
            _ => None,
        }
    }
}

impl Display for CommandKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                CommandKind::Help => HELP_COMMAND,
                CommandKind::Authorize => AUTHORIZE_COMMAND,
                CommandKind::Vacancies => VACANCIES_COMMAND,
                CommandKind::List => LIST_COMMAND,
                CommandKind::View => VIEW_COMMAND,
                CommandKind::Create => CREATE_COMMAND,
                CommandKind::Delete => DELETE_COMMAND,
                CommandKind::Exit => EXIT_COMMAND,
            }
        )
    }
}

pub struct Command {
    pub kind: CommandKind,
    pub args: Vec<String>,
}

impl Command {
    pub fn try_parse(text: &str) -> Option<Command> {
        debug_assert!(!text.is_empty());

        let t = text.trim();

        if !t.starts_with('/') {
            return None;
        }

        let kind: CommandKind;
        let mut split = t.split(' ');

        match split.next() {
            Some(command) => match CommandKind::try_parse(command) {
                Some(parsed) => kind = parsed,
                None => return None,
            },

            _ => return None,
        }

        Some(Self {
            kind,
            args: split
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect(),
        })
    }
}
