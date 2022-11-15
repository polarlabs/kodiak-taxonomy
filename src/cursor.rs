use std::rc::Rc;

#[derive(Eq, PartialEq, Clone, Debug)]
pub(crate) struct Cursor<K> {
    super_id: Option<Rc<K>>,
    node_index: usize,
}

/// Cursor remembers the last node returned while traversing the taxonomy.
impl<K> Cursor<K> {
    pub(crate) fn new(super_id: Option<Rc<K>>, node_index: usize) -> Self {
        Cursor { super_id, node_index }
    }

    pub(crate) fn super_id(&self) -> Option<Rc<K>> {
        self.super_id.clone()
    }

    pub(crate) fn node_index(&self) -> usize {
        self.node_index
    }
}
