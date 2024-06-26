use crate::{
    resp::{errors::Error, types::RespType},
    utils::context::Context
};

use super::resp_command::{RESPCommand, RESPCommandName, RESPMinMaxArgs};

pub struct Ping(pub Vec<String>);

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
        self.0.len()
    }
}

impl RESPCommand for Ping {
    fn execute(&mut self, _: &mut Context) -> RespType {
        if self.is_invalid() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }

        let message = self.0.get(0);

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
        let mut ctx = Context::default();
        let mut ping = Ping(vec![]);
        let response = ping.execute(&mut ctx);

        match response {
            RespType::SimpleString { value } => {
                assert_eq!(value, "PONG");
            }
            _ => panic!("Expected SimpleString"),
        }

        let mut ping = Ping(vec!["hello".to_string()]);

        let response = ping.execute(&mut ctx);

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
        let mut ctx = Context::default();
        let mut ping = Ping(vec![]);
        let response = ping.execute(&mut ctx);

        match response {
            RespType::SimpleString { value } => {
                assert_eq!(value, "PONG");
            }
            _ => panic!("Expected SimpleString"),
        }
    }

    #[test]
    fn max_args() {
        let mut ctx = Context::default();
        let mut ping = Ping (
             vec!["hello".to_string()]
        );

        let response = ping.execute(&mut ctx);

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
        let mut ctx = Context::default();
        let mut ping = Ping (
           vec!["hello".to_string(), "world".to_string()]
        );

        let response = ping.execute(&mut ctx);

        match response {
            RespType::SimpleError(Error::WrongNumberOfArguments { command }) => {
                assert_eq!(command, "ping");
            }
            _ => panic!("Expected WrongNumberOfArguments"),
        }
    }

    #[test]
    fn max_args_fail() {
        let mut ctx = Context::default();
        let mut ping = Ping (
            vec!["hello".to_string(), "world".to_string()],
        );

        let response = ping.execute(&mut ctx);

        match response {
            RespType::SimpleError(Error::WrongNumberOfArguments { command }) => {
                assert_eq!(command, "ping");
            }
            _ => panic!("Expected WrongNumberOfArguments"),
        }
    }
}
