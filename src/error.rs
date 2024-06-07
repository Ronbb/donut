#[derive(Debug)]
pub enum Error {
    Canceled,
    NoNextNode { procedure: String, node: String },
}
