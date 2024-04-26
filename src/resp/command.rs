use std::collections::HashMap;
use std::sync::{Arc,Mutex};

use super::errors::Error;
use super::types::RespType;


#[derive(Debug)]
pub enum Command {
    Ping { value: Option<String> },
    Echo { value: String },
    Set { key: String, value: String },
    Get { key: String },
}

impl Command {
    pub fn from_resp(resp_value: RespType) -> Result<Self, Error> {
        match resp_value {
            RespType::Array { len: commands_len, values } => { 
                let mut resp_values_iter = values.iter();
                let command = resp_values_iter.next().unwrap();

                match command {
                    RespType::BulkString { len: _, value } => {        
                        match value.to_uppercase().as_str() {
                            "PING" => {
                                let is_ping_command_valid = commands_len == 0 || commands_len > 2;

                                if  is_ping_command_valid {
                                    return Err(Error::Invalid);
                                }

                                let next_value = resp_values_iter.next();

                                Ok(Command::Ping {
                                    value: match next_value {
                                        Some(RespType::BulkString { len: _, value }) => Some(value.to_string()),
                                        _ => None,
                                    },
                                })
                                
                            }
                            "ECHO" => {
                                let is_echo_command_valid = commands_len > 1 && commands_len < 3;
                                
                                if !is_echo_command_valid {
                                    return Err(Error::Invalid);
                                }

                                let next_value = resp_values_iter.next();

                                if next_value.is_none() {
                                    return Err(Error::Invalid);
                                }

                                Ok(Command::Echo {
                                    value: match next_value.unwrap() {
                                        RespType::BulkString { len: _, value } => value.to_string(),
                                        _ => return Err(Error::Invalid),
                                    }
                                })   
                            },

                            "SET" => {
                                let is_set_command_valid = commands_len > 2 && commands_len < 4;
                                
                                if !is_set_command_valid {
                                    return Err(Error::Invalid);
                                }

                                let key = match resp_values_iter.next() {
                                    Some(RespType::BulkString { len: _, value }) => value.to_string(),
                                    _ => return Err(Error::Invalid),
                                };

                                let value = match resp_values_iter.next() {
                                    Some(RespType::BulkString { len: _, value }) => value.to_string(),
                                    _ => return Err(Error::Invalid),
                                };

                                Ok(Command::Set {
                                    key,
                                    value,
                                })
                            },
                            "GET" => {
                                let is_get_command_valid = commands_len > 1 && commands_len < 3;
                                
                                if !is_get_command_valid {
                                    return Err(Error::Invalid);
                                }

                                let key = match resp_values_iter.next() {
                                    Some(RespType::BulkString { len: _, value }) => value.to_string(),
                                    _ => return Err(Error::Invalid),
                                };

                                Ok(Command::Get {
                                    key,
                                })
                            },
                            _ => return Err(Error::Invalid),
                        }
                    }
                    _ => return Err(Error::Invalid),
                }
            },
            _ => return Err(Error::Invalid),   
            
        }
    }

    pub fn execute(&self, store: Arc<Mutex<HashMap<String, String>>>) -> String {
        match self {
            Command::Ping { value } => {
                let response = match value {
                    Some(v) => v.to_string(),
                    None => String::from("PONG"),
                };

                return  RespType::SimpleString { value: response }.to_string();
            },
            Command::Echo { value } => {
                return  RespType::SimpleString { value: value.to_string() }.to_string();
            },
            Command::Set { key, value } => {
                store.lock().unwrap().insert(key.to_string(), value.to_string());
                return  RespType::SimpleString { value: "OK".to_string() }.to_string();
            },
            Command::Get { key } => {
                let val = store.lock().unwrap();
                let val = val.get(key);

                if let Some(v) = val {
                    return RespType::BulkString { len: v.len(), value: v.to_string() }.to_string();
                }

                
                return RespType::BulkString { len: 0, value: "".to_string() }.to_string();
            },
        }
    }
}

