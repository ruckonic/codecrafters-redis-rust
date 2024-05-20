use std::{
    collections::HashMap, hash::RandomState, sync::{Arc, Mutex}
};

use crate::models::StoreValue;

pub type Store = Arc<Mutex<HashMap<String, StoreValue, RandomState>>>;

pub fn create_store() -> Store {
    Arc::new(Mutex::new(HashMap::new()))
}

