use std::str::Lines;

use super::errors::Error;

#[derive(Debug)]
pub enum RespType {
  BulkString{len: usize, value:  String},
  SimpleString{value: String},
  Array{len: usize, values: Vec<RespType>},
}

impl RespType {
  pub fn from_str(value: String) -> Result<Self, Error> {
      let mut lines = value.lines();
      return resp_type_parse(&mut lines);
  }

  pub fn to_string(&self) -> String {
      match self {
          RespType::BulkString { len, value } => {
            if *len == 0 {
                return "$-1\r\n".to_string();
            }
            
            return  format!("${}\r\n{}\r\n", len, value)
          },
          RespType::SimpleString { value } => {
              format!("+{}\r\n", value)
          },
          RespType::Array { len, values } => {
              let mut result = format!("*{}\r\n", len);
              for v in values {
                  result.push_str(&v.to_string());
              }
              result
          },
      }
  }
}

fn resp_type_parse<'a>(lines: &mut Lines<'a>) -> Result<RespType, Error> {
  let mut chars = lines.next().unwrap().chars();

  return match chars.next() {
      Some('*') => {
          let len_string = chars.collect::<String>();
          let len = len_string.parse::<usize>().unwrap();
          let mut values: Vec<RespType> = Vec::new();

          for _ in 0..len {
              let cm = resp_type_parse(lines).unwrap();
              values.push(cm);
          }

          Ok(RespType::Array {
              len,
              values,
          })
      },
      Some('$') => {
          let len_string = chars.collect::<String>();
          let len = len_string.parse::<usize>().unwrap();
          let value = lines.next().unwrap();

           Ok(RespType::BulkString {
              len,
              value: value.to_string(),
          })
      },
      _ => return Err(Error::Invalid),
  };
}