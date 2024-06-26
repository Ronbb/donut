use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    base::{Executable, Next},
    cursor::Cursor,
    error::Error,
};

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub script: String,
    pub incomings: Vec<Executable>,
    pub outgoings: Vec<Executable>,
}

impl Node {
    pub async fn execute(&self, _: Arc<RwLock<Cursor>>) -> Result<Next, Error> {
        // TODO execute script
        Ok(Next::Null)
    }
}
