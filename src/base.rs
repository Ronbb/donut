use std::sync::{Arc, Weak};

use tokio::sync::RwLock;
use tokio::time::Instant;

use crate::cursor::Cursor;
use crate::error::Error;
use crate::flow::Flow;
use crate::node::Node;
use crate::procedure::Procedure;

#[derive(Debug, Clone)]
pub enum Executable {
    Node(Weak<Node>),
    Flow(Weak<Flow>),
    Procedure(Weak<Procedure>),
    Selection(Vec<Weak<Flow>>),
}

impl PartialEq for Executable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Executable::Node(node1), Executable::Node(node2)) => {
                Arc::ptr_eq(&node1.upgrade().unwrap(), &node2.upgrade().unwrap())
            }
            (Executable::Flow(flow1), Executable::Flow(flow2)) => {
                Arc::ptr_eq(&flow1.upgrade().unwrap(), &flow2.upgrade().unwrap())
            }
            (Executable::Procedure(procedure1), Executable::Procedure(procedure2)) => Arc::ptr_eq(
                &procedure1.upgrade().unwrap(),
                &procedure2.upgrade().unwrap(),
            ),
            (Executable::Selection(flows1), Executable::Selection(flows2)) => {
                flows1.len() == flows2.len()
                    && flows1.iter().zip(flows2).all(|(flow1, flow2)| {
                        Arc::ptr_eq(&flow1.upgrade().unwrap(), &flow2.upgrade().unwrap())
                    })
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    Wait(Executable, Instant),
    // terminate the cursor and remove the cursor from scheduler
    Complete,
    // bubble up to the parent cursor
    // all siblings will be terminated and the parent cursor will continue
    Bubble,
}

impl Executable {
    pub async fn execute(&self, cursor: Arc<RwLock<Cursor>>) -> Result<Next, Error> {
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
            Executable::Procedure(procedure) => todo!(),
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
            Executable::Procedure(procedure) => todo!(),
        }
    }
}
