use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

use crate::{base::Next, cursor::Cursor, error::Error, node::Node};

#[derive(Clone)]
pub struct Flow {
    pub name: String,
    pub source_node: Weak<Node>,
    pub target_node: Weak<Node>,
    pub condition_script: String,
}

impl Flow {
    pub async fn check_condition(&self, _: Arc<RefCell<Cursor>>) -> Result<bool, Error> {
        let result = false;
        Ok(result)
    }

    pub async fn execute(&self, _: Arc<RefCell<Cursor>>) -> Result<Next, Error> {
        Ok(Next::Null)
    }
}
