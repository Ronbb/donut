use std::{collections::HashMap, sync::Arc};

use tokio::{select, sync::RwLock};

use crate::{
    base::{Executable, Next},
    cursor::Cursor,
    error::Error,
    procedure::Procedure,
    provider::Provider,
};

pub struct Scheduler {
    pub procedures: RwLock<Vec<Arc<Procedure>>>,
    pub cursors: RwLock<Vec<Arc<RwLock<Cursor>>>>,
    pub providers: HashMap<String, Arc<RwLock<Provider>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            procedures: RwLock::new(vec![]),
            cursors: RwLock::new(vec![]),
            providers: HashMap::new(),
        }
    }

    pub async fn start_procedure(
        scheduler: Arc<RwLock<Self>>,
        procedures: Vec<Arc<Procedure>>,
    ) -> Result<(), Error> {
        let s = &mut scheduler.write().await;
        for procedure in procedures {
            let cursor =
                Cursor::from_procedure(Arc::downgrade(&scheduler), Arc::downgrade(&procedure))
                    .await;
            s.cursors.write().await.push(cursor.clone());
            s.loop_cursor(cursor);
        }

        Ok(())
    }

    async fn loop_cursor(&mut self, cursor: Arc<RwLock<Cursor>>) -> Result<(), Error> {
        loop {
            let mut cursor_ref = cursor.write().await;
            let (_, rx, cancel) = cursor_ref.signals();

            select! {
                _ = cancel.cancelled() => {
                    cursor.write().await.complete().await;
                    break;
                }
                next = rx.recv() => {
                    if let Some(next) = next {
                        self.handle_next_operation(cursor.clone(), next).await?;
                    }
                    let next = self.execute_current(cursor.clone()).await?;
                    self.handle_next_operation(cursor.clone(), next).await?;
                    if cursor.read().await.is_complete() {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    // execute with cursor
    async fn execute_current(&mut self, cursor: Arc<RwLock<Cursor>>) -> Result<Next, Error> {
        if cursor.read().await.is_complete() {
            return Ok(Next::Complete);
        }

        cursor.read().await.current().execute(cursor.clone()).await
    }

    // handle parallel operation
    async fn handle_parallel(
        &mut self,
        cursor: Arc<RwLock<Cursor>>,
        executables: &Vec<Executable>,
    ) -> Result<(), Error> {
        cursor.write().await.create_children(executables).await;

        Ok(())
    }

    async fn handle_next_operation(
        &mut self,
        cursor: Arc<RwLock<Cursor>>,
        next: Next,
    ) -> Result<(), Error> {
        match next {
            Next::Null => Ok(()),
            Next::Continue => {
                let current_cursor = cursor.read().await;
                let current = current_cursor.current();
                let outgoings = &current.outgoings();
                match outgoings.len() {
                    0 => {
                        cursor.write().await.complete().await;
                    }
                    1 => {
                        cursor
                            .write()
                            .await
                            .set_current(outgoings.first().unwrap().clone());
                    }
                    _ => {
                        self.handle_parallel(cursor.clone(), outgoings).await?;
                    }
                }
                Ok(())
            }
            Next::One(executable) => {
                cursor.write().await.set_current(executable);
                Ok(())
            }
            Next::Parallel(executables) => {
                self.handle_parallel(cursor.clone(), &executables).await?;
                Ok(())
            }
            Next::Select(ref executables) => {
                let mut flows = vec![];
                for executable in executables {
                    match executable {
                        Executable::Node(node) => {
                            if let Some(_) = node.upgrade() {
                                cursor.write().await.set_current(executable.clone());
                                return Ok(());
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
                        Executable::Procedure(procedure) => {
                            if let Some(_) = procedure.upgrade() {
                                cursor.write().await.set_current(executable.clone());
                                return Ok(());
                            } else {
                                return Err(Error::Canceled);
                            }
                        }
                    }
                }

                match flows.len() {
                    0 => {
                        cursor.write().await.complete().await;
                    }
                    1 => {
                        cursor
                            .write()
                            .await
                            .set_current(Executable::Flow(flows.pop().unwrap()));
                    }
                    _ => {
                        cursor
                            .write()
                            .await
                            .set_current(Executable::Selection(flows));
                    }
                }

                Ok(())
            }
            Next::Wait(executable, time) => {
                // delay to time
                tokio::time::sleep_until(time).await;
                cursor.write().await.set_current(executable);
                Ok(())
            }
            Next::Complete => {
                cursor.write().await.complete().await;
                Ok(())
            }
            Next::Bubble => {
                cursor.write().await.complete_and_bubble().await?;
                Ok(())
            }
        }
    }
}
