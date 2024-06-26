use super::resp_command::{RESPCommand, RESPCommandName, RESPMinMaxArgs};
use crate::{
    resp::{errors::Error, types::RespType},
    utils::context::Context
};

pub struct Echo(pub Vec<String>);

impl RESPCommandName for Echo {
    fn command_name(&self) -> &'static str {
        "echo"
    }
}

impl RESPMinMaxArgs for Echo {
    fn args_len(&self) -> usize {
        self.0.len()
    }

    fn min_args(&self) -> usize {
        1
    }

    fn max_args(&self) -> usize {
        1
    }
}

impl RESPCommand for Echo {
    fn execute(&mut self, _: &mut Context) -> RespType {
        if self.is_invalid() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }

        let message = self.0.get(0);

        if message.is_none() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }

        let val = message.unwrap();

        RespType::BulkString {
            len: val.len(),
            value: val.to_string(),
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::context::Context;

    #[test]
    fn min_args() {
        let args = vec![];
        let mut echo_command = Echo(args);
        let mut ctx = Context::default();

        let wrong_number_of_args_err = echo_command.execute(&mut ctx);

        assert_eq!(
            wrong_number_of_args_err,
            RespType::SimpleError(Error::WrongNumberOfArguments {
                command: "echo".to_string(),
            })
        );
    }

    #[test]
    fn max_args() {
        let args = vec![String::from("value"), String::from("value2")];
        let mut echo_command = Echo(args);
        let mut ctx = Context::default();

        let wrong_number_of_args_err = echo_command.execute(&mut ctx);

        assert_eq!(
            wrong_number_of_args_err,
            RespType::SimpleError(Error::WrongNumberOfArguments {
                command: "echo".to_string(),
            })
        );
    }

    #[test]
    fn return_bulk_string() {
        let args = vec![String::from("value")];
        let mut echo_command = Echo(args);
        let mut ctx = Context::default(); 

            
        let resp = echo_command.execute(&mut ctx);

        assert_eq!(
            resp,
            RespType::BulkString {
                len: 5,
                value: "value".to_string(),
            }
        );
    }
}

