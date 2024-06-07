use std::{cell::RefCell, sync::Arc, time::Duration};

use crate::{
    base::{Executable, Next},
    cursor::Cursor,
    error::Error,
};

#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub script: String,
    pub incomings: Vec<Executable>,
    pub outgoings: Vec<Executable>,
}

impl Node {
    pub async fn execute(&self, _: Arc<RefCell<Cursor>>) -> Result<Next, Error> {
        // TODO execute script
        Ok(Next::Null)
    }
}
