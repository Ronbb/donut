use std::time::Duration;

use crate::{cursor::Cursor, error::Error, flow::Flow};

pub struct Task {
    pub name: String,
    pub script: String,
    pub incomings: Vec<Port>,
    pub outgoings: Vec<Port>,
}

pub enum Port {
    Task(Task),
    Flow(Flow),
}

pub enum Operation {
    // move to the next task
    Next,
    // move to the specific task
    One(Port),
    // start multi tasks in parallel, create children cursors
    Parallel(Vec<Port>),
    // select the first task that is ready
    Select(Vec<Port>),
    // wait for a duration and move to the specific task
    Wait(Port, Duration),
    // terminate the cursor and remove the cursor from scheduler
    Complete,
    // bubble up to the parent cursor
    // all siblings will be terminated and the parent cursor will continue
    Bubble,
}

impl Task {
    pub fn execute(&self, _: &mut Cursor) -> Result<(), Error> {
        // TODO execute script
        Ok(())
    }
}
