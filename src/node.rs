#[cfg(test)]
#[path = "tests/node/tests.rs"]
mod tests;

use crate::Identity;

use std::collections::LinkedList;
use std::hash::Hash;
use std::rc::Rc;

/// Stores a `Node`, identified by an id of type `K` and
/// wrapping the actual data as element of type `V` which
/// is constrained by the trait `Identity`.
///
/// In addition, a `Node` also stores the ids of its super-ordinated
/// and sub-ordinated nodes.
///
/// Users of the library do not directly interact with `Node`.
pub struct Node<K, V>
where
    K: Hash + Eq,
    V: Identity<K>,
{
    id: Rc<K>,
    element: V,
    supers: LinkedList<SuperNode<Rc<K>>>, // Sequence doesn't matter, but if it's a root node SuperNode::None is always the first element in LinkedList
    subs: LinkedList<Rc<K>>,              // Sequence matters
}

/// Represents a super-ordinated node. If `SuperNode` is `None` the `Node`
/// is a root node.
#[derive(PartialEq)]
enum SuperNode<K> {
    None,
    Id(K),
}

impl<K, V> Node<K, V>
where
    K: Hash + Eq,
    V: Identity<K>,
{
    /// Creates a new `Node` by consuming the element of type `V`.
    // Test coverage: { unit = missing, integration = n/a, doc = n/a } -> not ok
    pub(crate) fn new(element: V) -> Self {
        let mut supers = LinkedList::new();
        supers.push_back(SuperNode::new(None));

        Node {
            id: Rc::new(element.id()),
            element,
            supers,
            subs: LinkedList::new(),
        }
    }

    /// Returns node's id wrapped in a `Rc`.
    // Test coverage: { unit = none, integration = n/a, doc = n/a } -> ok
    pub(crate) fn id(&self) -> Rc<K> {
        self.id.clone()
    }

    /// Returns an immutable reference to element.
    // Test coverage: { unit = none, integration = n/a, doc = n/a } -> ok
    pub(crate) fn get(&self) -> &V {
        &self.element
    }

    /// Returns a mutable reference to element.
    // Test coverage: { unit = none, integration = n/a, doc = n/a } -> ok
    pub(crate) fn get_mut(&mut self) -> &mut V {
        &mut self.element
    }

    /// Returns an immutable reference to subordinated nodes' ids.
    // Test coverage: { unit = none, integration = n/a, doc = n/a } -> ok
    pub(crate) fn subs(&self) -> &LinkedList<Rc<K>> {
        &self.subs
    }

    /// Appends the id of a subordinate node to node's sub-nodes.
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    pub(crate) fn append_sub(&mut self, node_id: Rc<K>) -> &mut Self {
        self.subs.push_back(node_id);
        self
    }

    /// Appends the sub-node's id to node's sub-nodes at a specified index.
    /// Note:
    ///   if `index` is greater or equal to the number of sub-nodes, id is appended to the list of sub-nodes
    ///   if `index` is lesser or equal to 0, then id is prepended to the list of sub-nodes
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    pub(crate) fn append_sub_at(&mut self, node_id: Rc<K>, index: usize) -> &mut Self {
        if index >= self.subs.len() {
            self.append_sub(node_id);
        } else if index == 0 {
            self.prepend_sub(node_id);
        } else {
            let mut remain = self.subs.split_off(index);
            self.subs.push_back(node_id);
            self.subs.append(&mut remain);
        }

        self
    }

    /// Prepends the id of a subordinate node to node's sub-nodes.
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    pub(crate) fn prepend_sub(&mut self, id: Rc<K>) -> &mut Self {
        self.subs.push_front(id);
        self
    }

    /// Returns number of subordinate nodes.
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    pub(crate) fn count_subs(&self) -> usize {
        self.subs.len()
    }

    /// Returns true if node has at least one sub.
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    pub(crate) fn has_sub(&self) -> bool {
        !self.subs.is_empty()
    }

    /// Removes a sub node identified by id, silently ignores if there is no sub with this id
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    pub(crate) fn remove_sub(&mut self, node_id: Rc<K>) {
        match self.subs.iter_mut().position(|cursor| *cursor == node_id) {
            None => {}
            Some(index) => {
                let mut remain = self.subs.split_off(index);
                remain.pop_front();
                self.subs.append(&mut remain);
            }
        }
    }

    /// Returns id of a sub-node at self.subs\[index\]
    /// OR
    /// None if index is greater or equal to the number of sub-nodes.
    ///
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    pub(crate) fn sub_at(&self, index: usize) -> Option<Rc<K>> {
        self.subs().iter().nth(index).cloned()
    }

    /// Returns a list of valid super nodes (excluding None)
    // Test coverage: { unit = missing, integration = n/a, doc = n/a } -> not ok
    pub(crate) fn supers(&self) -> LinkedList<Rc<K>> {
        self.supers
            .iter()
            .filter(|cursor| cursor.id() != None)
            .map(|cursor| cursor.id().unwrap())
            .collect()
    }

    /// Adds additional id to node's super-nodes.
    /// - silently ignores if id is already a super-node
    /// - if id is None this node becomes a root-node
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    pub(crate) fn add_super(&mut self, id: Option<Rc<K>>) -> &mut Self {
        match id {
            None => self.supers.push_front(SuperNode::new(None)),
            Some(id) => {
                let super_node = SuperNode::new(Some(id));
                if !self.supers.contains(&super_node) {
                    self.supers.push_back(super_node);
                }
            }
        }

        self
    }

    /// Returns number of valid super nodes (excluding None)
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    #[allow(dead_code)]
    pub(crate) fn count_super(&self) -> usize {
        self.supers.len()
    }

    /// Returns true if node as valid super-nodes (excluding None)
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    #[allow(dead_code)]
    pub(crate) fn has_super(&self) -> bool {
        !self.supers.is_empty()
    }

    /// Removes an id from node's superordinate nodes.
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    pub(crate) fn remove_super(&mut self, id: Option<Rc<K>>) {
        match self.supers.iter().position(|cursor| cursor.id() == id) {
            None => {
                self.supers.pop_front();
            }
            Some(index) => {
                let mut remain = self.supers.split_off(index);
                remain.pop_front();
                self.supers.append(&mut remain);
            }
        }
    }

    /// Returns true when node is a root node, e.g. supers starts with None (or is empty (unreachable))
    // Test coverage: { unit = done, integration = n/a, doc = n/a } -> ok
    pub(crate) fn is_root(&self) -> bool {
        match self.supers.front() {
            None => true,
            Some(super_node) => super_node.id() == None,
        }
    }
}

impl<K> SuperNode<Rc<K>> {
    /// Creates a new `SuperNode`, a wrapper around None or an id.
    // Test coverage: { unit = none, integration = n/a, doc = n/a } -> ok
    fn new(id: Option<Rc<K>>) -> Self {
        match id {
            None => SuperNode::None,
            Some(id) => SuperNode::Id(id),
        }
    }

    /// Returns the `SuperNode`'s id or None:
    // Test coverage: { unit = none, integration = n/a, doc = n/a } -> ok
    fn id(&self) -> Option<Rc<K>> {
        match &self {
            SuperNode::None => None,
            SuperNode::Id(id) => Some(id.clone()),
        }
    }
}
