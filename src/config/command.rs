use crate::errors::NotesError;

#[derive(PartialEq)]
pub enum Command {
    Add,
}

impl Command {
    pub fn from(s: String) -> Result<Command, NotesError> {
        match s.as_str() {
            "add" => Ok(Command::Add),
            _ => Err(NotesError::InvalidCommand),
        }
    }
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Command::Add => write!(f, "add"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_from() {
        assert_eq!(Command::from("add".to_string()), Ok(Command::Add));
        assert_eq!(
            Command::from("invalid".to_string()),
            Err(NotesError::InvalidCommand)
        );
    }
}
