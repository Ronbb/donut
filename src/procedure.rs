use std::{collections::HashMap, sync::Arc};

use crate::{flow::Flow, node::Node};

pub struct Procedure {
    pub name: String,
    pub nodes: HashMap<String, Arc<Node>>,
    pub flows: HashMap<String, Arc<Flow>>,
}
