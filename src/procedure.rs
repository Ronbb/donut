use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    base::{Executable, Next},
    cursor::Cursor,
    error::Error,
    flow::Flow,
    node::Node,
};

#[derive(Debug)]
pub struct Procedure {
    pub name: String,
    pub nodes: HashMap<String, Arc<Node>>,
    pub flows: HashMap<String, Arc<Flow>>,
}

impl Procedure {
    // new
    pub fn new(name: String) -> Self {
        Self {
            name,
            nodes: HashMap::new(),
            flows: HashMap::new(),
        }
    }

    // find executable
    pub fn find(&self, name: &str) -> Result<Executable, Error> {
        let node = self.nodes.get(name);
        if let Some(node) = node {
            return Ok(Executable::Node(Arc::downgrade(node)));
        }

        let flow = self.flows.get(name);
        if let Some(flow) = flow {
            return Ok(Executable::Flow(Arc::downgrade(flow)));
        }

        Err(Error::NotFound {
            procedure: self.name.clone(),
            name: name.to_string(),
        })
    }

    pub async fn execute(&self, _: Arc<RwLock<Cursor>>) -> Result<Next, Error> {
        // TODO start procedure
        Ok(Next::Null)
    }
}
