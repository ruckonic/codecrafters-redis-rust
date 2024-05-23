use std::{
    collections::HashMap, hash::RandomState
};

use crate::models::StoreValue;

pub type Store = HashMap<String, StoreValue, RandomState>;

pub fn create_store() -> Store {
    HashMap::new()
}

