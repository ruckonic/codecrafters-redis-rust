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
        2
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::collections::HashMap;

    fn crate_store() -> Store {
        Store::new(Mutex::new(HashMap::<String, String>::new()))
    }

    #[test]
    fn set_command() {
        let mut store =  crate_store();

        let mut set = Set {
            args: vec!["key".to_string(), "value".to_string()],
        };

        let response = set.execute(&mut store);

        assert_eq!(response, RespType::SimpleString {
            value: "OK".to_string(),
        });

        let store = store.lock().unwrap();
        let value = store.get("key");


        assert_eq!(value, Some(&"value".to_string()));
    }

    #[test]
    fn min_args() {
        let mut store =  crate_store();

        let mut set = Set {
            args: vec!["key".to_string()],
        };

        let response = set.execute(&mut store);

        assert_eq!(response, RespType::SimpleError(Error::WrongNumberOfArguments {
            command: "set".to_string(),
        }));
    }

    #[test]
    fn max_args() {
        let mut store =  crate_store();

        let mut set = Set {
            args: vec!["key".to_string(), "value".to_string(), "extra".to_string()],
        };

        let response = set.execute(&mut store);

        assert_eq!(response, RespType::SimpleError(Error::WrongNumberOfArguments {
            command: "set".to_string(),
        }));
    }

}

