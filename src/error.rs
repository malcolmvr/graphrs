use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    DuplicateEdge,
    NodeMissing,
    NoEdge,
    SelfLoopsFound,
    WrongMethod,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ErrorKind::DuplicateEdge => write!(f, "duplicate edge"),
            ErrorKind::NodeMissing => write!(f, "node missing"),
            ErrorKind::NoEdge => write!(f, "no edge"),
            ErrorKind::SelfLoopsFound => write!(f, "self loops found"),
            ErrorKind::WrongMethod => write!(f, "wrong method"),
        }
    }
}
