use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Error {
    pub msg: String,
    pub code: u32,
    // 0 - OK
    // 1 - Not OK
}

impl Error {
    pub fn from_msg(msg: String) -> Self {
        Self { msg, code: 1 }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Code {}, Message: {}", self.code, self.msg)
    }
}

impl std::error::Error for Error {}
