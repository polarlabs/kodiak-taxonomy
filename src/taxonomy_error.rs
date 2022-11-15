use std::error::Error;
use std::fmt::{Debug, Display};
use std::rc::Rc;

/// Errors that might occur when managing a taxonomy.
// TaxonomyError is not Copy because we want to avoid to place a Copy bound on type K.
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum TaxonomyError<K> {
    /// Edge (a tuple of a super- and its sub-node) already exists.
    DuplicateEdge(Option<Rc<K>>, Rc<K>),

    /// Node is already part of the taxonomy.
    DuplicateNode(Rc<K>),

    /// Node is already a root-node of the taxonomy.
    DuplicateRootNode(Rc<K>),

    /// Node is already a sub-node of this super-node.
    DuplicateSubNode(Rc<K>, Rc<K>),

    /// Edge does not exist in taxonomy.
    EdgeNotFound(Option<Rc<K>>, Rc<K>),

    /// Operation on this node would create a loop.
    LoopDetected(Rc<K>),

    /// Node has at least one sub-node.
    NodeHasSubNode(Rc<K>),

    /// Node not found in taxonomy.
    NodeNotFound(Rc<K>),

    /// Source and destination are equal.
    SourceEqualsDestination,
}

#[doc(hidden)]
impl<K: Debug> Error for TaxonomyError<K> {}

// Excluded from code coverage check because error messages are trivial and not considered to be part of the public API
#[cfg(not(tarpaulin_include))]
#[doc(hidden)]
impl<K: Debug> Display for TaxonomyError<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Rust API Guideline - C-GOOD-ERR
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
