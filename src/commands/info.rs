use crate::resp::{errors::Error, types::RespType};
use crate::utils::store::Store;

use super::resp_command::{RESPCommand, RESPCommandName, RESPMinMaxArgs};

pub struct Info {
    pub args: Vec<String>,
}

impl RESPCommandName for Info {
    fn command_name(&self) -> &'static str {
        "info"
    }
}

impl RESPMinMaxArgs for Info {
    fn min_args(&self) -> usize {
        1
    }

    fn max_args(&self) -> usize {
        1
    }

    fn args_len(&self) -> usize {
        self.args.len()
    }
}

impl RESPCommand for Info {
    fn execute(&mut self, _: &mut Store) -> RespType {
        if self.is_invalid() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }

        let key = self.args.get(0);

        if key.is_none() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }

        let response = "#Replication\r\nrole:master\r\n".to_string();

        RespType::BulkString{ len: response.len(), value: response }
    }
}

