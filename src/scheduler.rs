use std::{cell::RefCell, collections::HashMap, sync::Arc};

use tokio::{select, sync::mpsc::channel};
use tokio_util::sync::CancellationToken;

use crate::{
    base::{Executable, Next},
    cursor::Cursor,
    error::Error,
    procedure::Procedure,
    provider::Provider,
};

pub struct Scheduler {
    pub procedures: Vec<Arc<Procedure>>,
    pub cursors: Vec<Arc<RefCell<Cursor>>>,
    pub providers: HashMap<String, Arc<RefCell<Provider>>>,
}

impl Scheduler {
    async fn loop_cursor(&mut self, cursor: Arc<RefCell<Cursor>>) -> Result<(), Error> {
        let mut cursor_ref = cursor.borrow_mut();
        let (_, rx, cancel) = cursor_ref.signals();
        loop {
            select! {
                _ = cancel.cancelled() => {
                    cursor.borrow_mut().is_complete = true;
                    break;
                }
                next = rx.recv() => {
                    if let Some(next) = next {
                        self.handle_next_operation(cursor.clone(), next)?;
                    }
                    let next = self.execute_current(cursor.clone()).await?;
                    self.handle_next_operation(cursor.clone(), next)?;
                    if cursor.borrow().is_complete {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    // execute with cursor
    async fn execute_current(&mut self, cursor: Arc<RefCell<Cursor>>) -> Result<Next, Error> {
        if cursor.borrow().is_complete {
            return Ok(Next::Complete);
        }

        cursor.borrow().current.execute(cursor.clone()).await
    }

    // handle parallel operation
    fn handle_parallel(
        &mut self,
        cursor: Arc<RefCell<Cursor>>,
        executables: &Vec<Executable>,
    ) -> Result<(), Error> {
        let mut children = vec![];
        for executable in executables {
            let (tx, rx) = channel(100);
            let child = Cursor {
                id: "".to_string(),
                scheduler: cursor.borrow().scheduler.clone(),
                context: cursor.borrow().context.clone(),
                procedure: cursor.borrow().procedure.clone(),
                current: executable.clone(),
                parent: Some(Arc::downgrade(&cursor)),
                children: vec![],
                is_complete: false,
                cancel: CancellationToken::new(),
                rx,
                tx,
            };
            children.push(Arc::new(RefCell::new(child)));
        }
        cursor.borrow_mut().children = children;
        Ok(())
    }

    fn handle_next_operation(
        &mut self,
        cursor: Arc<RefCell<Cursor>>,
        next: Next,
    ) -> Result<(), Error> {
        match &next {
            Next::Null => Ok(()),
            Next::Continue => {
                let current = cursor.borrow().current.clone();
                let outgoings = &current.outgoings();
                match outgoings.len() {
                    0 => {
                        cursor.borrow_mut().is_complete = true;
                    }
                    1 => {
                        cursor.borrow_mut().current = outgoings.first().unwrap().clone();
                    }
                    _ => {
                        self.handle_parallel(cursor.clone(), outgoings)?;
                    }
                }
                Ok(())
            }
            Next::One(executable) => {
                cursor.borrow_mut().current = executable.clone();
                Ok(())
            }
            Next::Parallel(executables) => {
                self.handle_parallel(cursor.clone(), &executables)?;
                Ok(())
            }
            Next::Select(executables) => {
                let mut flows = vec![];
                for executable in executables {
                    match executable {
                        Executable::Node(ref node) => {
                            if let Some(_) = node.upgrade() {
                                cursor.borrow_mut().current = executable.clone();
                            } else {
                                return Err(Error::Canceled);
                            }
                        }
                        Executable::Flow(flow) => {
                            flows.push(flow.clone());
                        }
                        Executable::Selection(selection) => {
                            for flow in selection {
                                flows.push(flow.clone());
                            }
                        }
                    }
                }

                match flows.len() {
                    0 => {
                        cursor.borrow_mut().is_complete = true;
                    }
                    1 => {
                        cursor.borrow_mut().current = Executable::Flow(flows.pop().unwrap());
                    }
                    _ => {
                        cursor.borrow_mut().current = Executable::Selection(flows);
                    }
                }

                Ok(())
            }
            Next::Wait(executable, duration) => {
                cursor.borrow_mut().current = executable.clone();
                Ok(())
            }
            Next::Complete => {
                cursor.borrow_mut().is_complete = true;
                Ok(())
            }
            Next::Bubble => {
                cursor.borrow_mut().is_complete = true;
                Ok(())
            }
        }
    }
}
