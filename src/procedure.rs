use std::{collections::HashMap, sync::Arc};

use crate::{flow::Flow, provider::Provider, task::Task};

pub struct Procedure {
    pub name: String,
    pub tasks: HashMap<String, Arc<Task>>,
    pub providers: HashMap<String, Arc<Provider>>,
    pub flows: HashMap<String, Arc<Flow>>,
}
