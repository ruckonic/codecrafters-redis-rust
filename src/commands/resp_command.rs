use crate::context::Context;
use crate::resp::types::RespType; 

pub trait RESPCommandName {
    /// Get the name of the command
    fn command_name(&self) -> &'static str;
}

pub trait RESPMinMaxArgs {
    fn args_len(&self) -> usize;
    fn min_args(&self) -> usize;
    fn max_args(&self) -> usize;
    fn is_valid(&self) -> bool {
        let min = self.min_args();
        let max = self.max_args();

        self.args_len() >= min && self.args_len() <= max
    }
    fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

pub trait RESPCommand: RESPCommandName + RESPMinMaxArgs + Send {
    fn execute(&mut self, ctx: &mut Context) -> RespType;
}

