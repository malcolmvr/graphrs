use std::fmt::{Display, Formatter, Result};

/**
If errors occur when creating or processing a `Graph` this `Error` struct will be returned.
*/
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
An enumeration of different kinds of errors that can occur while creating and
analyzing [Graph](./struct.Graph.html) objects.
*/
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// Contradictory paths were found when computing shortest paths.
    ContradictoryPaths,
    /// A duplicate `Edge` was added to a [Graph](./struct.Graph.html) that doesn't
    /// support multi `Edge`s.
    DuplicateEdge,
    /// An argument to a function was not a valid value.
    InvalidArgument,
    /// A [Node](./struct.Node.html) was requested from a [Graph](./struct.Graph.html) but the
    /// [Node](./struct.Node.html) doesn't exist.
    NodeNotFound,
    /// No partitions were found.
    NoPartitions,
    /// The specified communities did not form a partition of a [Graph](./struct.Graph.html).
    NotAPartition,
    /// An [Edge](./struct.Edge.html) was requested from a [Graph](./struct.Graph.html) but the
    /// [Edge](./struct.Edge.html) doesn't exist.
    EdgeNotFound,
    /// An algorithm requiring an [Edge](./struct.Edge.html) to have a weight was invoked
    /// but an [Edge](./struct.Edge.html) that did not have a weight value (was f64::NAN) was
    /// found in the [Graph](./struct.Graph.html).
    EdgeWeightNotSpecified,
    /// An algorithm failed to converge to the specified tolerance within the specified number
    /// of iterations of a power iteration method.
    PowerIterationFailedConvergence,
    /// An error occurred while reading a graph from a file.
    ReadError,
    /// An [Edge](./struct.Edge.html) where `u` and `v` were the same was added to a
    /// [Graph](./struct.Graph.html) that doesn't allow self-loops.
    SelfLoopsFound,
    /// A method was invoked on a [Graph](./struct.Graph.html) whose
    /// [GraphSpecs](./struct.GraphSpecs.html) are not supported by the method.
    WrongMethod,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ErrorKind::ContradictoryPaths => write!(f, "contradictory paths"),
            ErrorKind::DuplicateEdge => write!(f, "duplicate edge detected"),
            ErrorKind::EdgeNotFound => write!(f, "edge not found"),
            ErrorKind::EdgeWeightNotSpecified => write!(f, "edge weight not found"),
            ErrorKind::InvalidArgument => write!(f, "invalid argument"),
            ErrorKind::NodeNotFound => write!(f, "node not found"),
            ErrorKind::NoPartitions => write!(f, "no partitions were found"),
            ErrorKind::NotAPartition => write!(f, "communities were not a partition"),
            ErrorKind::PowerIterationFailedConvergence => write!(f, "failed to converge to the specified tolerance within the specified number of iterations"),
            ErrorKind::ReadError => write!(f, "error reading graph from file"),
            ErrorKind::SelfLoopsFound => write!(f, "self loops found"),
            ErrorKind::WrongMethod => write!(f, "wrong method was used"),
        }
    }
}
