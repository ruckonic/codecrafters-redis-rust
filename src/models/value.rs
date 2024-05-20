use std::time::Duration;

#[derive(Debug)]
pub struct StoreValue {
    pub(crate) data: String,
    pub(crate) expire_time: Option<Duration>,
    pub(crate) created_at: std::time::SystemTime,
}

impl PartialEq for StoreValue {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.expire_time == other.expire_time
    }
}

impl StoreValue {
    pub fn new(data: String, expire_time: Option<Duration>) -> Self {
        Self {
            data,
            expire_time,
            created_at: std::time::SystemTime::now(),
        }
    }
}

impl StoreValue {
    pub fn is_expired(&self) -> bool {
        if let Some(expire_time) = self.expire_time {
            self.created_at.elapsed().unwrap() > expire_time
        } else {
            false
        }
    }
}

impl From<String> for StoreValue {
    fn from(data: String) -> Self {
        Self {
            data,
            expire_time: None,
            created_at: std::time::SystemTime::now(),
        }
    }
}

impl From<&str> for StoreValue {
    fn from(data: &str) -> Self {
        Self {
            data: data.to_string(),
            expire_time: None,
            created_at: std::time::SystemTime::now(),
        }
    }
}

impl From<StoreValue> for String {
    fn from(store_value: StoreValue) -> String {
        store_value.data
    }
}

impl ToString for StoreValue {
    fn to_string(&self) -> String {
        self.data.clone()
    }
}

impl Default for StoreValue {
    fn default() -> Self {
        Self {
            data: "".to_string(),
            expire_time: None,
            created_at: std::time::SystemTime::now(),
        }
    }
}

