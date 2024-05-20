use super::errors::Error;
use std::str::Lines;

/// RESP types from redis protocol
#[derive(Debug, PartialEq)]
pub enum RespType {
    BulkString { len: usize, value: String },
    SimpleString { value: String },
    Array { len: usize, values: Vec<RespType> },
    SimpleError(Error),
    Null,
}

impl TryFrom<String> for RespType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        return Self::resp_type_parse(&mut lines);
    }
}

impl TryFrom<&str> for RespType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        return Self::resp_type_parse(&mut lines);
    }
}

impl From<RespType> for String {
    fn from(value: RespType) -> String {
        value.to_string()
    }
}

impl ToString for RespType {
    fn to_string(&self) -> String {
        match self {
            RespType::BulkString { len, value } => {
                if *len == 0 {
                    return "$-1\r\n".to_string();
                }

                return format!("${}\r\n{}\r\n", len, value);
            }
            RespType::SimpleString { value } => {
                format!("+{}\r\n", value)
            }
            RespType::Array { len, values } => {
                let mut result = format!("*{}\r\n", len);

                for v in values {
                    result.push_str(&v.to_string());
                }

                result
            }
            RespType::SimpleError(err) => {
                format!("-{}\r\n", err.to_string())
            }
            // RESP v2 null are null bulk strings this is temp fix for resp v2
            RespType::Null => "$-1\r\n".to_string(),
        }
    }
}

impl RespType {
    // TODO: Refactor to pass characters instead of Lines this can fail when text contain command echo "\r\n"
    fn resp_type_parse<'a>(lines: &mut Lines<'a>) -> Result<RespType, Error> {
        let mut chars = lines.next().unwrap().chars();

        return match chars.next() {
            Some('*') => {
                let len_string = chars.collect::<String>();
                let len = len_string.parse::<usize>().unwrap();
                let mut values: Vec<RespType> = Vec::new();

                for _ in 0..len {
                    let cm = Self::resp_type_parse(lines).unwrap();
                    values.push(cm);
                }

                Ok(RespType::Array { len, values })
            }
            Some('$') => {
                let len_string = chars.collect::<String>();
                let len = len_string.parse::<usize>().unwrap();
                let value = lines.next().unwrap();

                Ok(RespType::BulkString {
                    len,
                    value: value.to_string(),
                })
            }
            _ => return Err(Error::Unknown),
        };
    }
}
