use super::types::RespType;

#[derive(Debug, PartialEq)]
pub enum Error {
    Custom { message: String },
    WrongType,
    UnknownCommand { command: String },
    WrongNumberOfArguments { command: String },
    Unknown,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::Custom { message } => message.to_string(),
            Self::WrongType => {
                "WRONGTYPE Operation against a key holding the wrong kind of value".to_string()
            }
            Self::UnknownCommand { command } => format!("ERR unknown command '{command}'"),
            Self::WrongNumberOfArguments { command } => {
                format!("wrong number of arguments for '{command}' command")
            }
            Self::Unknown => "Unknown error".to_string(),
        }
    }
}

impl Into<RespType> for Error {
    fn into(self) -> RespType {
        RespType::SimpleError(self)
    }
}
