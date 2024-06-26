use std::sync::{Arc, Weak};

use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    RwLock,
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    base::{Executable, Next},
    context::Context,
    error::Error,
    procedure::Procedure,
    scheduler::Scheduler,
};

pub struct Cursor {
    _weak: Weak<RwLock<Cursor>>,

    id: String,
    scheduler: Weak<RwLock<Scheduler>>,
    context: Context,
    procedure: Weak<Procedure>,
    current: Executable,
    parent: Option<Weak<RwLock<Cursor>>>,
    children: RwLock<Vec<Arc<RwLock<Cursor>>>>,
    is_complete: bool,
    cancel: CancellationToken,
    rx: Receiver<Next>,
    tx: Sender<Next>,
}

impl Cursor {
    pub async fn from_procedure(
        scheduler: Weak<RwLock<Scheduler>>,
        procedure: Weak<Procedure>,
    ) -> Arc<RwLock<Cursor>> {
        let (tx, rx) = channel(100);
        let cancel = CancellationToken::new();

        let cursor = Self {
            _weak: Weak::new(),
            id: Uuid::now_v7().to_string(),
            scheduler,
            context: Context::new(),
            procedure: procedure.clone(),
            parent: None,
            current: Executable::Procedure(procedure),
            children: RwLock::new(vec![]),
            is_complete: false,
            cancel,
            rx,
            tx,
        };

        Cursor::insert_ptr(Arc::new(RwLock::new(cursor))).await
    }

    pub async fn create_children(&self, executables: &Vec<Executable>) {
        let mut children = vec![];
        for executable in executables {
            let (tx, rx) = channel(100);
            let child = Cursor {
                _weak: Weak::new(),
                id: Uuid::now_v7().to_string(),
                scheduler: self.scheduler.clone(),
                context: self.context.clone(),
                procedure: self.procedure.clone(),
                current: executable.clone(),
                parent: Some(self._weak.clone()),
                children: RwLock::new(vec![]),
                is_complete: false,
                cancel: self.cancel.child_token(),
                rx,
                tx,
            };
            children.push(Cursor::insert_ptr(Arc::new(RwLock::new(child))).await);
        }

        *self.children.write().await = children;
    }

    async fn insert_ptr(cursor: Arc<RwLock<Cursor>>) -> Arc<RwLock<Cursor>> {
        cursor.write().await._weak = Arc::downgrade(&cursor);
        cursor
    }

    // get id
    pub fn id(&self) -> &str {
        &self.id
    }

    // get scheduler
    pub fn scheduler(&self) -> Result<Arc<RwLock<Scheduler>>, Error> {
        match self.scheduler.upgrade() {
            Some(scheduler) => Ok(scheduler),
            None => Err(Error::Canceled),
        }
    }

    // get context
    pub fn context(&self) -> &Context {
        &self.context
    }

    // get procedure
    pub fn procedure(&self) -> &Weak<Procedure> {
        &self.procedure
    }

    // get current
    pub fn current(&self) -> &Executable {
        &self.current
    }

    // set current
    pub fn set_current(&mut self, current: Executable) {
        self.current = current;
    }

    // get parent
    pub fn parent(&self) -> Result<Option<Arc<RwLock<Cursor>>>, Error> {
        match &self.parent {
            Some(parent) => match parent.upgrade() {
                Some(parent) => Ok(Some(parent)),
                None => Err(Error::Canceled),
            },
            None => Ok(None),
        }
    }

    // is complete
    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    // complete
    pub async fn complete(&mut self) {
        self.is_complete = true;
        self.cancel.cancel();
    }

    // complete and bubble
    pub async fn complete_and_bubble(&mut self) -> Result<(), Error> {
        self.complete().await;
        if let Some(parent) = self.parent()? {
            parent.write().await.complete().await;
        }

        Ok(())
    }

    // signals
    pub fn signals(&mut self) -> (&Sender<Next>, &mut Receiver<Next>, &CancellationToken) {
        (&self.tx, &mut self.rx, &self.cancel)
    }
}
