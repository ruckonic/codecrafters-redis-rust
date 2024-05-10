use crate::{
    resp::{errors::Error, types::RespType},
    types::store::Store,
};

use super::resp_command::{RESPCommand, RESPCommandName, RESPMinMaxArgs};

pub struct Get {
    pub args: Vec<String>,
}

impl RESPCommandName for Get {
    fn command_name(&self) -> &'static str {
        "get"
    }
}

impl RESPMinMaxArgs for Get {
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

impl RESPCommand for Get {
    fn execute(&mut self, store: &mut Store) -> RespType {
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

        let key = key.unwrap();
        let store = store.lock();

        if store.is_err() {
            return Error::Custom {
                message: "Error getting data".to_string(),
            }
            .into();
        }

        let store = store.unwrap();
        let value = store.get(key.as_str());

        if value.is_none() {
            return RespType::Null;
        }

        let value = value.unwrap().clone();

        let bulk_string = RespType::BulkString {
            len: value.len(),
            value,
        };

        return bulk_string;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use crate::types::store::Store;

    fn create_store() -> Store {
        Arc::new(Mutex::new(HashMap::new()))
    }

    #[test]
    fn test_get() {
        let mut store = create_store();

        store
            .lock()
            .unwrap()
            .insert("key".to_string(), "value".to_string());

        let mut get = Get {
            args: vec!["key".to_string()],
        };

        let result = get.execute(&mut store);

        assert_eq!(
            result,
            RespType::BulkString {
                len: 5,
                value: "value".to_string(),
            }
        );
    }

    #[test]
    fn test_get_invalid() {
        let mut store = create_store();
        let mut get = Get { args: vec![] };

        let result = get.execute(&mut store);

        assert_eq!(
            result,
            RespType::SimpleError(Error::WrongNumberOfArguments {
                command: "get".to_string(),
            })
        );
    }

    #[test]
    fn test_get_key_not_found() {
        let mut store = create_store();
        let mut get = Get {
            args: vec!["key".to_string()],
        };

        let result = get.execute(&mut store);

        assert_eq!(
            result,
            RespType::Null,
        );
    }
}

