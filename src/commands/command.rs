use super::resp_command::RESPCommand;
use super::Echo;
use super::Get;
use super::Ping;
use super::Set;

use crate::resp::errors::Error;
use crate::resp::types::RespType;

pub struct Command(String, Vec<String>);

impl Command {
    /// Returns the execute of this [`Command`].
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    /// - The command is unknown.
    /// - The command is not valid.
    pub fn create_command(&self) -> Result<Box<dyn RESPCommand>, Error> {
        let opt = &self.0;
        let args = self.1.clone();

        match opt.to_lowercase().as_str() {
            "echo" => {
                let echo = Echo { args };

                Ok(Box::new(echo))
            }
            "ping" => {
                let ping = Ping { args };

                Ok(Box::new(ping))
            }
            "set" => {
                let set = Set { args };

                Ok(Box::new(set))
            }
            "get" => {
                let get = Get { args };

                Ok(Box::new(get))
            }
            _ => Err(Error::UnknownCommand {
                command: self.0.clone(),
            }),
        }
    }
}

impl TryFrom<RespType> for Command {
    type Error = Error;

    // TODO: Refactor this function
    fn try_from(value: RespType) -> Result<Self, Self::Error> {
        match value {
            RespType::Array {
                len: _,
                values: args,
            } => {
                let comm = args.first();

                if comm.is_none() {
                    return Err(Error::WrongType);
                }

                let comm = comm.unwrap();

                if let RespType::BulkString {
                    len: _,
                    value: command,
                } = comm
                {
                    let args_res: Result<Vec<String>, ()> =
                        args.iter().skip(1).try_fold(vec![], |mut acc, resp_type| {
                            if let RespType::BulkString { len: _, value } = resp_type {
                                acc.push(value.clone());
                                Ok(acc)
                            } else {
                                Err(())
                            }
                        });

                    if let Ok(v) = args_res {
                        return Ok(Command(command.to_string(), v));
                    }

                    Err(Self::Error::UnknownCommand {
                        command: command.to_string(),
                    })
                } else {
                    Err(Error::WrongType)
                }
            }
            _ => {
                return Err(Error::WrongType);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::types::RespType;

    use super::Command;

    #[test]
    fn command() {
        let bulk_string = RespType::BulkString {
            len: 4,
            value: "ping".to_string(),
        };

        let resp_array = RespType::Array {
            len: 1,
            values: vec![bulk_string],
        };

        let command = Command::try_from(resp_array);

        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(command.0, "ping".to_string());
    }

    #[test]
    fn command_with_args() {
        let bulk_string = RespType::BulkString {
            len: 4,
            value: "echo".to_string(),
        };

        let bulk_string_args = RespType::BulkString {
            len: 4,
            value: "Hello".to_string(),
        };

        let resp_array = RespType::Array {
            len: 2,
            values: vec![bulk_string, bulk_string_args],
        };

        let command = Command::try_from(resp_array);

        assert!(command.is_ok());
        let command = command.unwrap();

        assert_eq!(command.0, "echo".to_string());
        assert_eq!(command.1, vec!["Hello".to_string()]);
    }

    #[test]
    fn empty_resp_arr() {
        let resp_array = RespType::Array {
            len: 0,
            values: vec![],
        };

        let command = Command::try_from(resp_array);

        assert!(command.is_err());
    }

    #[test]
    fn wrong_resp_type() {
        let bulk_string = RespType::BulkString {
            len: 4,
            value: "ping".to_string(),
        };

        let command = Command::try_from(bulk_string);

        assert!(command.is_err())
    }

    #[test]
    fn get_ping_command() {
        let bulk_string = RespType::BulkString {
            len: 4,
            value: "ping".to_string(),
        };

        let resp_array = RespType::Array {
            len: 1,
            values: vec![bulk_string],
        };

        let command = Command::try_from(resp_array).unwrap();
        let command = command.create_command();

        assert!(command.is_ok());

        let command = command.unwrap();

        assert_eq!(command.command_name(), "ping");
    }
}
