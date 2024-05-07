use super::resp_command::{RESPCommand, RESPCommandName, RESPMinMaxArgs};
use crate::{
    resp::{errors::Error, types::RespType},
    types::store::Store,
};

pub struct Echo {
    pub(crate) args: Vec<String>,
}

impl RESPCommandName for Echo {
    fn command_name(&self) -> &'static str {
        "echo"
    }
}

impl RESPMinMaxArgs for Echo {
    fn args_len(&self) -> usize {
        self.args.len()
    }

    fn min_args(&self) -> usize {
        1
    }

    fn max_args(&self) -> usize {
        1
    }
}

impl RESPCommand for Echo {
    fn execute(&mut self, _: &mut Store) -> RespType {
        if self.is_invalid() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }

        let message = self.args.get(0);

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
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    fn create_store() -> Arc<Mutex<HashMap<String, String>>> {
        Arc::new(Mutex::new(HashMap::new()))
    }

    #[test]
    fn min_args() {
        let args = vec![];
        let mut echo_command = Echo { args };
        let mut store = create_store();

        let wrong_number_of_args_err = echo_command.execute(&mut store);

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
        let mut echo_command = Echo { args };
        let mut store = create_store();

        let wrong_number_of_args_err = echo_command.execute(&mut store);

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
        let mut echo_command = Echo { args };
        let mut store = create_store();

        let resp = echo_command.execute(&mut store);

        assert_eq!(
            resp,
            RespType::BulkString {
                len: 5,
                value: "value".to_string(),
            }
        );
    }
}
