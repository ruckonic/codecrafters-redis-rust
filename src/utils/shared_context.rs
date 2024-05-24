use std::sync::{Arc, Mutex};
use super::context::Context;

pub type SharedContext = Arc<Mutex<Context>>;

pub fn create_shared_context(context: Context) -> SharedContext {
    Arc::new(Mutex::new(context))
}
