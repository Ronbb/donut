use crate::{cursor::Cursor, procedure::Procedure};

pub struct Scheduler {
    pub procedures: Vec<Procedure>,
    pub cursors: Vec<Cursor>,
}
