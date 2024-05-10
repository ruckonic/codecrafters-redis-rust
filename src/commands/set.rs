use crate::{
    resp::{errors::Error, types::RespType},
    types::store::Store,
};

use super::resp_command::{RESPCommand, RESPCommandName, RESPMinMaxArgs};

pub struct Set {
    pub args: Vec<String>,
}

impl RESPCommandName for Set {
    fn command_name(&self) -> &'static str {
        "set"
    }
}

impl RESPMinMaxArgs for Set {
    fn min_args(&self) -> usize {
        2
    }

    fn max_args(&self) -> usize {
        3
    }

    fn args_len(&self) -> usize {
        self.args.len()
    }
}

impl RESPCommand for Set {
    fn execute(&mut self, store: &mut Store) -> RespType {
        if self.is_invalid() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }

        let key = self.args.get(0);
        let value = self.args.get(1);

        if key.is_none() || value.is_none() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }

        let key = key.unwrap();
        let value = value.unwrap();
        let store = store.lock();

        if store.is_err() {
            return RespType::SimpleError(Error::Custom {
                message: "Error saving data".to_string(),
            });
        }

        let mut store = store.unwrap();

        let _ = store.insert(key.to_string(), value.to_string());

        return RespType::SimpleString {
            value: "OK".to_string(),
        };
    }
}
