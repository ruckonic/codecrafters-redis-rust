use crate::resp::{errors::Error, types::RespType};
use crate::utils::context::Context;

use super::resp_command::{RESPCommand, RESPCommandName, RESPMinMaxArgs};

pub struct Get (pub Vec<String>);

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
        self.0.len()
    }
}

impl RESPCommand for Get {
    fn execute(&mut self, ctx: &mut Context) -> RespType {
        let store = &mut ctx.store;

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

        let key = key.unwrap();


        let store_value = store.get(key.as_str());

        if store_value.is_none() {
            return RespType::Null;
        }

        let store_value = store_value.unwrap();
        let value = store_value.data.clone();

        if store_value.is_expired() {
            store.remove(key);
            return RespType::Null;
        }

        let bulk_string = RespType::BulkString {
            len: value.len(),
            value: value.to_string(),
        };

        return bulk_string;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::StoreValue;
    use crate::utils::context::Context;

    #[test]
    fn get_value() {
        let mut ctx = Context::default();
        let store = &mut ctx.store;

        let value = String::from("value");
        let key = "key".to_string();

        store
            .insert(key.clone(), StoreValue::from(value.clone()));

        let mut get = Get(vec![key.clone()]);

        let result = get.execute(&mut ctx);

        assert_eq!(result, RespType::BulkString { len: 5, value });
    }

    #[test]
    fn validate_min_arguments() {
        let mut ctx = Context::default();
        let args = vec![];
        let mut get = Get(args);

        let wrong_number_of_args_error = Error::WrongNumberOfArguments {
            command: get.command_name().to_string(),
        };

        let result = get.execute(&mut ctx);

        assert_eq!(result, RespType::SimpleError(wrong_number_of_args_error));
    }

    #[test]
    fn validate_max_arguments() {
        let mut ctx = Context::default();
        let args = vec!["arg1".to_string(), "arg2".to_string()];
        let mut get = Get(args);

        let wrong_number_of_args_error = Error::WrongNumberOfArguments {
            command: get.command_name().to_string(),
        };

        let result = get.execute(&mut ctx);

        assert_eq!(result, RespType::SimpleError(wrong_number_of_args_error));
    }

    #[test]
    fn resturns_null_when_key_not_found() {
        let mut ctx = Context::default();
        let mut get = Get(vec!["key".to_string()]);

        let result = get.execute(&mut ctx);

        assert_eq!(result, RespType::Null,);
    }

    #[test]
    fn returns_null_when_key_expired() {
        let mut ctx = Context::default();
        let store = &mut ctx.store;
        let created_at = std::time::SystemTime::now() - std::time::Duration::from_secs(100);
        let expire_time = Some(std::time::Duration::from_secs(100));

        let store_value = StoreValue {
            data: "value".to_string(),
            created_at,
            expire_time,
        };

        let key = String::from("key_expired");
        let args = vec![key.clone()];

        store
            .insert(key, store_value);

        let mut get = Get(args);

        let result = get.execute(&mut ctx);

        assert_eq!(result, RespType::Null);
    }
}
