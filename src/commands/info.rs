use crate::resp::{errors::Error, types::RespType};
use crate::utils::context::Context;

use super::resp_command::{RESPCommand, RESPCommandName, RESPMinMaxArgs};

pub struct Info(pub Vec<String>);

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
        self.0.len()
    }
}

impl RESPCommand for Info {
    fn execute(&mut self, ctx: &mut Context) -> RespType {
        if self.is_invalid() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }

        let key = self.0.get(0);

        if key.is_none() {
            return Error::WrongNumberOfArguments {
                command: self.command_name().to_string(),
            }
            .into();
        }



        // role
        let role = &ctx.config.role;
        let master_replid = &ctx.config.master_replid;
        let master_repl_offset = &ctx.config.master_repl_offset;
        
        let response = String::from("#Replication\r\n");
        let role = format!("role:{}\r\n", role.to_string());
        let master_replid = format!("master_replid:{}\r\n", master_replid.to_string());
        let master_repl_offset = format!("master_repl_offset:{}\r\n", master_repl_offset.to_string());
        
        
        let info = response + &role + &master_replid + &master_repl_offset;


        RespType::BulkString{ len: info.len(), value: info }
    }
}

