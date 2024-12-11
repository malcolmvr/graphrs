#[doc = include_str!("../README.md")]
#[macro_use]
extern crate doc_comment;
doc_comment!(include_str!("../README.md"));

mod edge;
pub use edge::Edge;

mod error;
pub use error::{Error, ErrorKind};

mod ext;

mod graph;
pub use graph::Graph;

pub(crate) use graph::adjacent_node::AdjacentNode;

pub mod algorithms;
pub mod generators;
pub mod readwrite;

mod graph_specs;
pub use graph_specs::{
    EdgeDedupeStrategy, GraphSpecs, MissingNodeStrategy, SelfLoopsFalseStrategy,
};

mod node;
pub use node::Node;
