use std::sync::Arc;
use std::sync::Mutex;

use crate::config;
use crate::store;

#[derive(Debug)]
pub struct Context {
    pub store: store::Store,
    pub config: config::Config,
}

pub fn create_context() -> Arc<Mutex<Context>> {
    Arc::new(Mutex::new(Context::default()))
}

impl Context {
    pub fn new(store: store::Store, config: config::Config) -> Self {
        Self {
            store,
            config,
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            store: store::create_store(),
            config: config::load().unwrap(),
        }
    }
}

