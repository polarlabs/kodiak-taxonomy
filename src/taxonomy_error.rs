use std::fmt::{Debug, Display};
use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
pub enum TaxonomyError<K> {
    DuplicateEdge(Option<Rc<K>>, Rc<K>),
    DuplicateNode(Rc<K>),
    DuplicateRootNode(Rc<K>),
    DuplicateSubNode(Rc<K>, Rc<K>),
    EdgeNotFound(Option<Rc<K>>, Rc<K>),
    LoopDetected(Rc<K>),
    NodeHasSubNode(Rc<K>),
    NodeNotFound(Rc<K>),
    SourceEqualsDestination,
}

impl<K: Debug> std::error::Error for TaxonomyError<K> {}

// Excluded from code coverage check because error messages are trivial and not considered to be part of the public API
#[cfg(not(tarpaulin_include))]
impl<K: Debug> Display for TaxonomyError<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Rust API Guidline - C-GOOD-ERR
            // The error message given by the Display representation of an error type should be lowercase without trailing punctuation, and typically concise.
            TaxonomyError::DuplicateEdge(super_node, node) => write!(f, "edge {:?} - {:?} is a duplicate", super_node, node),
            TaxonomyError::DuplicateNode(node) => write!(f, "node {:?} is a duplicate", node),
            TaxonomyError::DuplicateRootNode(node) => write!(f, "node {:?} is a duplicate", node),
            TaxonomyError::DuplicateSubNode(super_node, node) => {
                write!(f, "super-node {:?} has already sub-node {:?}", super_node, node)
            }
            TaxonomyError::EdgeNotFound(super_node, node) => write!(
                f,
                "edge between super-node {:?} and sub-node {:?} not found",
                super_node, node
            ),
            TaxonomyError::LoopDetected(node) => write!(f, "loop detected when node {:?} is appended", node),
            TaxonomyError::NodeHasSubNode(node) => write!(f, "node {:?} has sub-nodes", node),
            TaxonomyError::NodeNotFound(node) => write!(f, "node {:?} not found", node),
            TaxonomyError::SourceEqualsDestination => write!(f, "source equals destination"),
        }
    }
}
