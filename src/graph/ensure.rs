use super::Graph;
use crate::{Error, ErrorKind};
use std::fmt::Display;
use std::hash::Hash;

impl<T, A> Graph<T, A>
where
    T: Eq + Clone + PartialOrd + Ord + Hash + Send + Sync + Display,
    A: Clone,
{
    /// Returns an `Err` if the `graph` is not a directed graph.
    pub fn ensure_directed(&self) -> Result<(), Error>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone + Send + Sync,
    {
        if !self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "This method is not applicable to undirected graphs.".to_string(),
            });
        }
        Ok(())
    }

    /// Returns an `Err` if the `graph` is a directed graph.
    pub fn ensure_undirected(&self) -> Result<(), Error>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone + Send + Sync,
    {
        if self.specs.directed {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "This method is not applicable to directed graphs.".to_string(),
            });
        }
        Ok(())
    }

    /// Returns an `Err` if the `graph` is not a directed graph.
    pub fn ensure_not_multi_edges(&self) -> Result<(), Error>
    where
        T: Hash + Eq + Clone + Ord + Display + Send + Sync,
        A: Clone + Send + Sync,
    {
        if self.specs.multi_edges {
            return Err(Error {
                kind: ErrorKind::WrongMethod,
                message: "This method is not applicable to multi-edge graphs.".to_string(),
            });
        }
        Ok(())
    }

    /// Returns an `Err` if the any of the `graph`'s edges do not have a weight.
    pub fn ensure_weighted(&self) -> Result<(), Error>
    where
        T: Hash + Eq + Clone + Ord,
        A: Clone,
    {
        if !self.edges_have_weight() {
            return Err(Error {
                kind: ErrorKind::EdgeWeightNotSpecified,
                message: "Not all edges in the graph have a weight.".to_string(),
            });
        }
        Ok(())
    }
}
