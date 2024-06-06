#[derive(Debug)]
pub enum Error {
    Canceled,
    NoNextTask { procedure: String, task: String },
}
