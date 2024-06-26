use crate::state::State;

#[derive(Clone)]
pub struct Context {
    pub state: State,
}

impl Context {
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }
}
