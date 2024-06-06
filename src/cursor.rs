use std::{
    borrow::Borrow,
    collections::HashMap,
    sync::{Arc, Weak},
};

use crate::{
    context::Context,
    error::Error,
    procedure::Procedure,
    scheduler::Scheduler,
    task::{Operation, Task},
};

pub struct Cursor {
    id: String,
    scheduler: Weak<Scheduler>,
    context: Context,
    procedure: Weak<Procedure>,
    current_task: Weak<Task>,
    next_operation: Operation,
    parent: Option<Weak<Cursor>>,
    children: HashMap<String, Arc<Cursor>>,
}

impl Cursor {
    // execute current
    pub fn execute(&mut self) -> Result<(), Error> {
        if let Some(current_task) = self.current_task.upgrade() {
            current_task.execute(self)?;
        } else {
            return Err(Error::Canceled);
        }

        self.handle_next()?;
        Ok(())
    }

    fn handle_next(&mut self) -> Result<(), Error> {
        match self.next_operation.borrow() {
            Operation::Next => {}
            Operation::One(_) => {}
            Operation::Parallel(_) => {}
            Operation::Select(_) => {}
            Operation::Wait(_, _) => {}
            Operation::Complete => {}
            Operation::Bubble => {}
        }
        Ok(())
    }
}
