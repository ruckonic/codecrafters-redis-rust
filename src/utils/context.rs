use crate::config;
use crate::store;

#[derive(Debug)]
pub struct Context {
    pub store: store::Store,
    pub config: config::Config,
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

