use std::sync::{Arc, Weak};

use tokio::sync::RwLock;

use crate::{base::Next, cursor::Cursor, error::Error, node::Node};

#[derive(Clone, Debug)]
pub struct Flow {
    pub name: String,
    pub source_node: Weak<Node>,
    pub target_node: Weak<Node>,
    pub condition_script: String,
}

impl Flow {
    pub async fn check_condition(&self, _: Arc<RwLock<Cursor>>) -> Result<bool, Error> {
        let result = false;
        Ok(result)
    }

    pub async fn execute(&self, _: Arc<RwLock<Cursor>>) -> Result<Next, Error> {
        Ok(Next::Null)
    }
}
