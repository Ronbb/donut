use std::cell::RefCell;
use std::sync::{Arc, Weak};
use std::time::SystemTime;

use crate::cursor::Cursor;
use crate::error::Error;
use crate::flow::Flow;
use crate::node::Node;

#[derive(Clone)]
pub enum Executable {
    Node(Weak<Node>),
    Flow(Weak<Flow>),
    Selection(Vec<Weak<Flow>>),
}

#[derive(Clone)]
pub enum Next {
    // do nothing and will execute the current node again
    Null,
    // move to the next node
    Continue,
    // move to the specific node
    One(Executable),
    // start multi nodes in parallel, create children cursors
    Parallel(Vec<Executable>),
    // select the first node that is ready
    Select(Vec<Executable>),
    // wait for a deadline and move to the specific node
    Wait(Executable, SystemTime),
    // terminate the cursor and remove the cursor from scheduler
    Complete,
    // bubble up to the parent cursor
    // all siblings will be terminated and the parent cursor will continue
    Bubble,
}

impl Executable {
    pub async fn execute(&self, cursor: Arc<RefCell<Cursor>>) -> Result<Next, Error> {
        match self {
            Executable::Node(node) => {
                if let Some(node) = node.upgrade() {
                    node.execute(cursor.clone()).await?;
                } else {
                    return Err(Error::Canceled);
                }
            }
            Executable::Flow(flow) => {
                if let Some(flow) = flow.upgrade() {
                    flow.execute(cursor.clone()).await?;
                } else {
                    return Err(Error::Canceled);
                }
            }
            Executable::Selection(flows) => todo!(),
        }

        Ok(Next::Null)
    }

    // get outgoings
    pub fn outgoings(&self) -> Vec<Executable> {
        match self {
            Executable::Node(node) => {
                if let Some(node) = node.upgrade() {
                    node.outgoings.clone()
                } else {
                    vec![]
                }
            }
            Executable::Flow(flow) => {
                if let Some(flow) = flow.upgrade() {
                    vec![Executable::Node(flow.target_node.clone())]
                } else {
                    vec![]
                }
            }
            Executable::Selection(flows) => flows
                .iter()
                .map(|flow| Executable::Flow(flow.clone()))
                .collect(),
        }
    }
}
