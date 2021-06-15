use std::fmt::{Display, Formatter, Result};
use std::str::Split;

pub enum CommandKind {
    Help,
}

const HELP_COMMAND: &str = "/help";

impl CommandKind {
    pub fn try_parse(s: &str) -> Option<CommandKind> {
        match s {
            HELP_COMMAND => Some(CommandKind::Help),

            _ => None,
        }
    }
}

impl Display for CommandKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            CommandKind::Help => {
                write!(f, "{}", HELP_COMMAND)
            }
        }
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
