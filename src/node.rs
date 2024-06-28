use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    base::{Executable, Next},
    cursor::Cursor,
    error::Error,
    script::Script,
};

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub script: String,
    pub incomings: Vec<Executable>,
    pub outgoings: Vec<Executable>,
}

impl Node {
    pub async fn execute(&self, cursor: Arc<RwLock<Cursor>>) -> Result<Next, Error> {
        // TODO execute script
        let script = Script::new(cursor);
        let next = script.execute_for_next(&self.script).await?;

        Ok(next)
    }
}
