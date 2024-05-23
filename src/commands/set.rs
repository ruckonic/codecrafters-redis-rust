use std::time::Duration;

use super::resp_command::{RESPCommand, RESPCommandName, RESPMinMaxArgs};
use crate::models::StoreValue;
use crate::resp::{errors::Error, types::RespType};
use crate::utils::context::Context;

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
        4
    }

    fn args_len(&self) -> usize {
        self.args.len()
    }
}

impl Set {
    fn get_duration(kind: String, value: Option<&String>) -> Option<Duration> {
        if value.is_none() {
            return None;
        }

        let value = value.unwrap();
        let value = value.parse::<u64>();

        if value.is_err() {
            return None;
        }

        let value = value.unwrap();

        return match kind.as_str() {
            "px" => Some(Duration::from_millis(value)),
            "ex" => Some(Duration::from_secs(value)),
            _ => None,
        };
    }
}

impl RESPCommand for Set {
    fn execute(&mut self, ctx: &mut Context) -> RespType {
        let store = &mut ctx.store;

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

        let opt = self.args.get(2);
        let mut duration: Option<Duration> = None;

        if let Some(opt) = opt {
            let next_opt = self.args.get(3);
            let opt = opt.to_lowercase();
            let kind = opt;

            duration = Set::get_duration(kind, next_opt);
        }

        let key = key.unwrap();
        let value = value.unwrap();

        let value = StoreValue::new(value.to_string(), duration);
        let _ = store.insert(key.to_string(), value);

        return RespType::SimpleString {
            value: "OK".to_string(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::context::create_context;

    #[test]
    fn set_value() {
        let context = create_context();
        let mut context = context.lock().unwrap();
        let key = String::from("set_value_key");
        let value = String::from("set_value_value");
        let args = vec![key.clone(), value.clone()];

        let mut set = Set { args };

        let response = set.execute(&mut context);
        let store = &context.store;

        assert_eq!(
            response,
            RespType::SimpleString {
                value: "OK".to_string(),
            }
        );

        let store_value = store.get(key.as_str());

        assert!(store_value.is_some());

        assert_eq!(store_value, Some(&StoreValue::from(value)));
    }

    #[test]
    fn set_value_with_ttl() {
        let context = create_context();
        let mut context = context.lock().unwrap();
        let key = String::from("set_value_key");
        let value = String::from("set_value_value");
        let px = String::from("px");
        let ttl = String::from("1000");
        let args = vec![key.clone(), value.clone(), px.clone(), ttl.clone()];
        let mut set = Set { args };

        let response = set.execute(&mut context);
        let store = &context.store;

        assert_eq!(
            response,
            RespType::SimpleString {
                value: "OK".to_string(),
            }
        );

        let store_value = store.get(key.as_str());

        assert!(store_value.is_some());
        let duration = Set::get_duration(px, Some(&ttl));

        assert_eq!(store_value, Some(&StoreValue::new(value, duration)));
    }

    #[test]
    fn validate_min_args() {
        let context = create_context();
        let mut context = context.lock().unwrap();

        let mut set = Set {
            args: vec!["key".to_string()],
        };

        let response = set.execute(&mut context);

        assert_eq!(
            response,
            RespType::SimpleError(Error::WrongNumberOfArguments {
                command: "set".to_string(),
            })
        );
    }

    #[test]
    fn validate_max_args() {
        let context = create_context();
        let mut context = context.lock().unwrap();

        let mut set = Set {
            args: vec!["key".to_string(), "value".to_string(), "extra".to_string(), "extra".to_string(), "extra".to_string()],
        };

       assert!(!set.is_valid());

        let response = set.execute(&mut context);

        assert_eq!(
            response,
            RespType::SimpleError(Error::WrongNumberOfArguments {
                command: "set".to_string(),
            })
        );
    }
}

