use std::collections::HashMap;

use crate::{
    context::Context,
    error::Error,
    procedure::Procedure,
    scheduler::Scheduler,
    task::{Operation, Task},
};

pub struct Cursor<'procedure, 'parent> {
    id: String,
    scheduler: &'procedure Scheduler,
    context: Context,
    procedure: &'procedure Procedure,
    current_task: &'procedure Task,
    next: Operation<'parent>,
    parent: Option<&'procedure Cursor<'procedure, 'parent>>,
    children: HashMap<String, Cursor<'procedure, 'parent>>,
}

impl Cursor<'_, '_> {
    // execute current
    pub fn execute(&mut self) -> Result<(), Error> {
        self.current_task.execute(self)?;
        Ok(())
    }

    // get id
    pub fn id(&self) -> &str {
        &self.id
    }

    // get scheduler
    pub fn scheduler(&self) -> &Scheduler {
        self.scheduler
    }

    // get context
    pub fn context(&self) -> &Context {
        &self.context
    }

    // get context mut
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    // get procedure
    pub fn procedure(&self) -> &Procedure {
        self.procedure
    }

    // get current task
    pub fn current_task(&self) -> &Task {
        self.current_task
    }

    // get next operation
    pub fn next(&self) -> &Operation {
        &self.next
    }

    // get parent
    pub fn parent(&self) -> Option<&Cursor> {
        self.parent
    }

    // get children
    pub fn children(&self) -> &HashMap<String, Cursor> {
        &self.children
    }
}
