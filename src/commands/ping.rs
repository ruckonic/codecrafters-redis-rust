use crate::{
    resp::{errors::Error, types::RespType},
    utils::store::Store,
};

use super::resp_command::{RESPCommand, RESPCommandName, RESPMinMaxArgs};

pub struct Ping {
    pub args: Vec<String>,
}

impl RESPCommandName for Ping {
    fn command_name(&self) -> &'static str {
        "ping"
    }
}

impl RESPMinMaxArgs for Ping {
    fn min_args(&self) -> usize {
        0
    }

    fn max_args(&self) -> usize {
        1
    }

    fn args_len(&self) -> usize {
        self.args.len()
    }
}

impl RESPCommand for Ping {
    fn execute(&mut self, _: &mut Store) -> RespType {
        if self.is_invalid() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }

        let message = self.args.get(0);

        let response = match message {
            Some(v) => RespType::BulkString {
                len: v.len(),
                value: v.to_string(),
            },
            None => RespType::SimpleString {
                value: "PONG".to_string(),
            },
        };

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_command() {
        let mut ping = Ping { args: vec![] };
        let response = ping.execute(&mut Store::default());

        match response {
            RespType::SimpleString { value } => {
                assert_eq!(value, "PONG");
            }
            _ => panic!("Expected SimpleString"),
        }

        let mut ping = Ping {
            args: vec!["hello".to_string()],
        };

        let response = ping.execute(&mut Store::default());

        match response {
            RespType::BulkString { len, value } => {
                assert_eq!(len, 5);
                assert_eq!(value, "hello");
            }
            _ => panic!("Expected BulkString"),
        }
    }

    #[test]
    fn min_args() {
        let mut ping = Ping { args: vec![] };
        let response = ping.execute(&mut Store::default());

        match response {
            RespType::SimpleString { value } => {
                assert_eq!(value, "PONG");
            }
            _ => panic!("Expected SimpleString"),
        }
    }

    #[test]
    fn max_args() {
        let mut ping = Ping {
            args: vec!["hello".to_string()],
        };

        let response = ping.execute(&mut Store::default());

        match response {
            RespType::BulkString { len, value } => {
                assert_eq!(len, 5);
                assert_eq!(value, "hello");
            }
            _ => panic!("Expected BulkString"),
        }
    }

    #[test]
    fn min_args_fail() {
        let mut ping = Ping {
            args: vec!["hello".to_string(), "world".to_string()],
        };

        let response = ping.execute(&mut Store::default());

        match response {
            RespType::SimpleError(Error::WrongNumberOfArguments { command }) => {
                assert_eq!(command, "ping");
            }
            _ => panic!("Expected WrongNumberOfArguments"),
        }
    }

    #[test]
    fn max_args_fail() {
        let mut ping = Ping {
            args: vec!["hello".to_string(), "world".to_string()],
        };

        let response = ping.execute(&mut Store::default());

        match response {
            RespType::SimpleError(Error::WrongNumberOfArguments { command }) => {
                assert_eq!(command, "ping");
            }
            _ => panic!("Expected WrongNumberOfArguments"),
        }
    }
}
