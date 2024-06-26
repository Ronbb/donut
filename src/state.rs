use std::collections::HashMap;

#[derive(Clone)]
pub struct State {
    value: HashMap<String, Variant>,
}

#[derive(Clone)]
pub enum Variant {
    Null,
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Variant>),
    Object(HashMap<String, Variant>),
}

impl State {
    // new
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
        }
    }

    // set
    pub fn set(&mut self, key: String, value: Variant) {
        self.value.insert(key, value);
    }

    // get
    pub fn get(&self, key: &str) -> Option<&Variant> {
        self.value.get(key)
    }

    // remove
    pub fn remove(&mut self, key: &str) -> Option<Variant> {
        self.value.remove(key)
    }

    // has
    pub fn has(&self, key: &str) -> bool {
        self.value.contains_key(key)
    }
}
