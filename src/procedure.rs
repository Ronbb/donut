use std::collections::HashMap;

use crate::task::Task;

pub struct Procedure {
    pub name: String,
    pub tasks: HashMap<String, Task>,
}


