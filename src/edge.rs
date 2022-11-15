use std::rc::Rc;

/// An `Edge` is a pair of nodes with a super - sub relationship.
///
/// In a taxonomy which allows nodes to have more than one superordinate node,
/// a node might be a sub node in more than one edge. In practice, an `Edge`
/// allows us to define a node's distinct location in the taxonomy.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Edge<K> {
    super_id: Option<Rc<K>>,
    node_id: Rc<K>,
}

/// Position defines a node's position in the taxonomy.
impl<K> Edge<K> {
    pub(crate) fn new(super_id: Option<Rc<K>>, node_id: Rc<K>) -> Self {
        Edge { super_id, node_id }
    }

    pub(crate) fn super_id(&self) -> Option<Rc<K>> {
        self.super_id.clone()
    }

    pub(crate) fn node_id(&self) -> Rc<K> {
        self.node_id.clone()
    }
}
