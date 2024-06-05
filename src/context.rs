use std::sync::Arc;

use crate::state::State;

pub struct Context {
    pub state: Arc<State>,
}
