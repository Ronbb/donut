use std::{
    borrow::Borrow,
    cell::RefCell,
    sync::{Arc, Weak},
};

use tokio::sync::mpsc::{Receiver, Sender};
use tokio_util::sync::CancellationToken;

use crate::{
    base::{Executable, Next},
    context::Context,
    procedure::Procedure,
    scheduler::Scheduler,
};

pub struct Cursor {
    pub(super) id: String,
    pub(super) scheduler: Weak<Scheduler>,
    pub(super) context: Context,
    pub(super) procedure: Weak<Procedure>,
    pub(super) current: Executable,
    pub(super) parent: Option<Weak<RefCell<Cursor>>>,
    pub(super) children: Vec<Arc<RefCell<Cursor>>>,
    pub(super) is_complete: bool,
    pub(super) cancel: CancellationToken,
    pub(super) rx: Receiver<Next>,
    pub(super) tx: Sender<Next>,
}

impl Cursor {
    // get id
    pub fn id(&self) -> &str {
        &self.id
    }

    // get scheduler
    pub fn scheduler(&self) -> &Weak<Scheduler> {
        self.scheduler.borrow()
    }

    // get context
    pub fn context(&self) -> &Context {
        &self.context
    }

    // get procedure
    pub fn procedure(&self) -> &Weak<Procedure> {
        self.procedure.borrow()
    }

    // get parent
    pub fn parent(&self) -> Option<Weak<RefCell<Cursor>>> {
        self.parent.clone()
    }

    // is complete
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    // signals
    pub fn signals(&mut self) -> (&Sender<Next>, &mut Receiver<Next>, &CancellationToken) {
        (&self.tx, &mut self.rx, &self.cancel)
    }
}
