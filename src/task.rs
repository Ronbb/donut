use std::time::Duration;

use crate::{cursor::Cursor, error::Error};

pub struct Task {
    pub name: String,
    pub script: String,
}

pub enum Operation<'procedure> {
    // move to the next task
    Next,
    // move to the specific task
    One(&'procedure Task),
    // start multi tasks in parallel, create children cursors
    Parallel(Vec<&'procedure Task>),
    // select the first task that is ready
    Select(Vec<&'procedure Task>),
    // wait for a duration and move to the specific task
    Wait(&'procedure Task, Duration),
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
