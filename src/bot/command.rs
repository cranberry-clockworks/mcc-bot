use std::fmt::{Display, Formatter, Result};

pub enum CommandKind {
    Help,
    Authorize,
    VacanciesCreate,
    VacanciesDelete,
    VacanciesView,
    Exit,
}

const HELP_COMMAND: &str = "/help";

const AUTHORIZE_COMMAND: &str = "/authorize";

const VACANCIES_CREATE_COMMAND: &str = "/vacancies_create";
const VACANCIES_DELETE_COMMAND: &str = "/vacancies_delete";
const VACANCIES_VIEW_COMMAND: &str = "/vacancies_view";

const EXIT_COMMAND: &str = "/exit";

impl CommandKind {
    pub fn try_parse(s: &str) -> Option<CommandKind> {
        match s {
            HELP_COMMAND => Some(CommandKind::Help),
            AUTHORIZE_COMMAND => Some(CommandKind::Authorize),
            VACANCIES_CREATE_COMMAND => Some(CommandKind::VacanciesCreate),
            VACANCIES_DELETE_COMMAND => Some(CommandKind::VacanciesDelete),
            VACANCIES_VIEW_COMMAND => Some(CommandKind::VacanciesView),
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
                CommandKind::VacanciesCreate => VACANCIES_CREATE_COMMAND,
                CommandKind::VacanciesDelete => VACANCIES_DELETE_COMMAND,
                CommandKind::VacanciesView => VACANCIES_VIEW_COMMAND,
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
