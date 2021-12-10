use std::fmt::{Display, Formatter, Result};

/**
If errors occur when creating or processing a `Graph` this `Error` struct will be returned.
**/
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

/**
An enumeration of different kinds of errors that can occur.

`DuplicateEdge`: a redundant `Edge` was added to a `Graph` that doesn't support multi `Edge`s.

`NodeNotFound`: a `Node` was requested from the `Graph` but the `Node` doesn't exist.

`EdgeNotFound`: an `Edge` was requested from the `Graph` but the `Edge` doesn't exist.

`SelfLoopsFound`: `Edge`s were added to an `Graph`, resulting in self-loops, and the `Graph`
does not allow self-loops.

`WrongMethod`: a method was invoked on a `Graph` whose `specs` don't support that method.
**/
#[derive(Clone, Debug)]
pub enum ErrorKind {
    DuplicateEdge,
    NodeNotFound,
    EdgeNotFound,
    SelfLoopsFound,
    WrongMethod,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ErrorKind::DuplicateEdge => write!(f, "duplicate edge"),
            ErrorKind::NodeNotFound => write!(f, "node not found"),
            ErrorKind::EdgeNotFound => write!(f, "edge not found"),
            ErrorKind::SelfLoopsFound => write!(f, "self loops found"),
            ErrorKind::WrongMethod => write!(f, "wrong method"),
        }
    }
}
