use std::collections::HashMap;

use crate::{provider::Provider, task::Task};

pub struct Procedure {
    pub name: String,
    pub tasks: HashMap<String, Task>,
    pub providers: HashMap<String, Provider>,
}
