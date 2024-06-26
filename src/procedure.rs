use std::{collections::HashMap, sync::Arc};

use crate::{flow::Flow, node::Node};

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
}
