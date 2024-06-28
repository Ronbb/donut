#[derive(Debug)]
pub enum Error {
    Canceled,
    NotFound { procedure: String, name: String },
    NoNextNode { procedure: String, node: String },
    ScriptFailed { reason: String },
}

impl From<mlua::Error> for Error {
    fn from(error: mlua::Error) -> Self {
        Error::ScriptFailed {
            reason: error.to_string(),
        }
    }
}
