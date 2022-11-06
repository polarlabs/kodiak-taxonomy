use thiserror::Error;

use std::hash::Hash;
use std::rc::Rc;

#[derive(PartialEq, Eq, Error, Debug)]
pub enum TaxonomyError<K>
where
    K: Hash + Eq,
{
    #[error("edge {0} - {1} is a dublicate")]
    DuplicateEdge(Option<Rc<K>>, Rc<K>),

    #[error("node `{0}` is a duplicate")]
    DuplicateNode(Rc<K>),

    #[error("node `{0}` is a duplicate")]
    DuplicateRootNode(Rc<K>),

    #[error("super-node `{0}` has already sub-node `{1}`")]
    DuplicateSubNode(Rc<K>, Rc<K>),

    #[error("edge between super-node `{0}` and sub-node `{1}` not found")]
    EdgeNotFound(Option<Rc<K>>, Rc<K>),

    #[error("Loop detected when node `{0}` is appended")]
    LoopDetected(Rc<K>),

    #[error("node `{0}` has subordinates")]
    NodeHasSubNode(Rc<K>),

    #[error("node `{0}` not found")]
    NodeNotFound(Rc<K>),

    #[error("source equals destination")]
    SourceEqualsDestination,
}
