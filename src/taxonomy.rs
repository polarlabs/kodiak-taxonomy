#[cfg(test)]
#[path = "tests/taxonomy/tests.rs"]
mod tests;

use crate::Identity;
use crate::Node;
use crate::TaxonomyError;
use crate::TaxonomyError::*;

use super::Cursor;
use super::Edge;

use std::collections::{HashMap, HashSet, LinkedList};
use std::hash::Hash;
use std::rc::Rc;

/// A taxonomy of equally typed nodes which allows a node to have more than one superior.
///
/// Taxonomy wraps the data, also known as element, in `Node`s. The id of a `Node` is defined by
/// the user's implementation of the `Identity` trait.
///
/// Nodes are stored in a hash map using their ids as key, so retrieving elements from taxonomy is very fast
/// Nodes can be added to, removed from and repositioned within the taxonomy.
/// Taxonomy can be traversed.
/// Taxonomy doesn't require to a have a single root node, instead multiple root nodes are allowed.
///
/// Taxonomy knows four types of nodes. Many nodes have multiple types at the same time or change their type
/// during taxonomy lifecycle.
///
/// 1. Root node (*root-node*): node at the top level of the taxonomy. Taxonomy supports multiple root-nodes, so user are not forced to create a fake root node.
/// 2. Superordinate node (*super-node*): node at a *higher* level of the taxonomy (relative to its sub-nodes).
/// 3. Subordinate node (*sub-node*): node at a *lower* level of the taxonomy (relative to its super-node(s)).
/// 4. Coordinate node (*co-node*): node at the *same* level of the taxonomy *and* sharing the same super-node.
///
/// Burden:
/// Implement the `Identity` trait for the nodes' type.
pub struct Taxonomy<K: Hash + Eq, V: Identity<K>> {
    nodes: HashMap<Rc<K>, Node<K, V>>,
    node0: LinkedList<Rc<K>>,
    last_updated_node: Option<Rc<K>>,
    cursor: Vec<Cursor<K>>,
}

impl<K, V> Default for Taxonomy<K, V>
where
    K: Hash + Eq,
    V: Identity<K>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Taxonomy<K, V>
where
    K: Hash + Eq,
    V: Identity<K>,
{
    /// Constructs a new, empty `Taxonomy<T>` holding equally typed elements.
    ///
    /// Taxonomy owns the elements added or appended to it and allows to
    /// get immutable or mutable references.
    /// `T` has to implement the trait `Identity` to uniquely identify the element.
    /// Taxonomy uses the identifier internally to add the element to a `HashMap` thus
    /// the identifier's type `H` has to implement `Hash` and `Eq`.
    ///
    /// # Examples
    ///
    /// This simplistic example shows how to create a taxonomy able to store elements of type `Class`.
    /// The `Identity` trait has to be implemented for `Class`to provide the id `Taxonomy` requires for each
    /// element. Elements with the same Id are treated as equal. So be careful to meet your requirements
    /// when implementing `Identity`.
    ///
    /// The trait bounds `Identity` raises are kept at a minimum: it's only `Hash` and `Eq` as required
    /// by any `HashMap` for its keys.
    ///
    /// ```rust
    /// use kodiak_taxonomy::{Identity, Taxonomy};
    ///
    /// struct Class {
    ///     name: String,
    /// }
    ///
    /// // String implements `Hash` and `Eq` thus it's sufficient to implement `Identity`
    /// impl Identity<String> for Class {
    ///     fn id(&self) -> String {
    ///         self.name.clone()
    ///     }
    /// }
    ///
    /// let mut tax: Taxonomy<String, Class> = Taxonomy::new();
    /// ```
    // Test coverage: { unit = none, integration = missing, doc = done } -> not ok
    pub fn new() -> Self {
        Taxonomy {
            nodes: HashMap::new(),
            node0: LinkedList::new(),
            last_updated_node: None,
            cursor: Vec::new(),
        }
    }

    /// Returns an immutable reference to the element identified by `id` or `None` if id is not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::rc::Rc;
    /// use kodiak_taxonomy::{Identity, Taxonomy};
    ///
    /// struct Class {
    ///     name: String,
    /// }
    ///
    /// // String implements `Hash` and `Eq` thus it's sufficient to implement `Identity`
    /// impl Identity<String> for Class {
    ///     fn id(&self) -> String {
    ///         self.name.clone()
    ///     }
    /// }
    ///
    /// let mut tax: Taxonomy<String, Class> = Taxonomy::new();
    /// let element = Class{name: "Animal".to_string()};
    /// let id = Rc::new(element.id());
    /// tax.add(None, element);
    ///
    /// let animal = tax.get(id).unwrap();
    /// ```
    // Test coverage: { unit = none, integration = missing, doc = done } -> not ok
    pub fn get(&self, node_id: Rc<K>) -> Option<&V> {
        match self._get_node_opt(node_id) {
            None => None,
            Some(node) => Some(node.get()),
        }
    }

    /// Returns a mutable reference to the element identified by `id` or `None` if id is not found.
    ///
    /// # Examples
    /// ```rust
    /// use std::rc::Rc;
    /// use kodiak_taxonomy::{Identity, Taxonomy};
    /// use uuid::Uuid;
    ///
    /// struct Class {
    ///     id: Uuid,
    ///     name: String,
    /// }
    ///
    /// impl Class {
    ///     pub fn new(name: &str) -> Self {
    ///         Class {
    ///             id: Uuid::new_v4(),
    ///             name: name.to_string(),
    ///         }
    ///     }
    /// }
    /// // String implements `Hash` and `Eq` thus it's sufficient to implement `Identity`
    /// impl Identity<String> for Class {
    ///     fn id(&self) -> String {
    ///         self.name.clone()
    ///     }
    /// }
    ///
    /// let mut tax: Taxonomy<String, Class> = Taxonomy::new();
    /// let element = Class::new("Animal");
    /// let id = Rc::new(element.id());
    /// tax.add(None, element);
    ///
    /// let mut element = tax.get_mut(id.clone()).unwrap();
    /// element.name = "Vertebrate".to_string();
    ///
    /// assert_eq!(tax.get(id).unwrap().name, "Vertebrate".to_string());
    /// ```
    // Test coverage: { unit = npne, integration = missing, doc = done } -> not ok
    pub fn get_mut(&mut self, node_id: Rc<K>) -> Option<&mut V> {
        match self._get_node_mut_opt(node_id) {
            None => None,
            Some(node) => Some(node.get_mut()),
        }
    }

    /// Returns the id of the last updated node or `None` if no node has been updated yet.
    ///
    /// # Examples
    /// todo
    // Test coverage: { unit = none, integration = none, doc = missing } -> not ok
    pub fn last_updated_node(&self) -> Option<Rc<K>> {
        self.last_updated_node.clone()
    }

    /// Adds an element of type `V` to taxonomy, either as root-node or as sub-node to a specified super-node.
    ///
    /// Fails if taxonomy already contains the element. Use `append` or `append_at` to append an already added
    /// element (aka existing node) to another node.
    ///
    /// # Errors
    /// `DuplicateElement`: taxonomy already contains the element, identified by `Identity`.
    /// `NodeNotFound`: no node with super_id not found in taxonomy.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kodiak_taxonomy::{Identity, Taxonomy, TaxonomyError};
    ///
    /// struct Class {
    ///   name: String,
    /// }
    ///
    /// // String implements `Hash` and `Eq` thus it's sufficient to implement `Identity`
    /// impl Identity<String> for Class {
    ///     fn id(&self) -> String {
    ///         self.name.clone()
    ///     }
    /// }
    ///
    /// let mut tax: Taxonomy<String, Class> = Taxonomy::new();
    /// match tax.add(None, Class{name: "Animal".to_string()}) {
    ///     Ok(_) => { println!("Element added.") }
    ///     Err(e) => { println!("Error {:#?} when adding element.", e)}
    /// }
    /// ```
    // Test coverage: { unit = done, integration = missing, doc = done } -> not ok
    pub fn add(&mut self, super_id: Option<K>, element: V) -> Result<&mut Self, TaxonomyError<K>> {
        let node_id = Rc::new(element.id());

        // Input validation: element
        self._err_duplicate_node(node_id)?;

        match super_id {
            // Element becomes a root-node
            None => { // tarpaulin: exclude false positive from code coverage
                let node = Node::new(element);
                self._add_root_node(node);
            }
            // Element becomes a sub-node of existing node
            Some(super_id) => {
                let super_id = Rc::new(super_id);

                // Input validation: super_id
                self._err_node_not_found(super_id.clone())?;

                // Adding a new non-root-node / element to taxonomy without loop detection
                let mut node = Node::new(element);
                node.remove_super(None);
                self._add_non_root_node(super_id, node);
            }
        }

        Ok(self)
    }

    /// Appends an element as sub-node to an existing node.
    ///
    /// # Errors
    /// `DuplicateNode`: node already exists at that level in taxonomy
    /// `NodeNotFound(id)`: node with id not found in taxonomy.
    /// `LoopDetected`: appending the node would result in a loop.
    ///
    /// # Examples
    /// todo
    // Test coverage: { unit = done, integration = missing, doc = done } -> not ok
    pub fn append(&mut self, super_id: Option<K>, node_id: K) -> Result<&mut Self, TaxonomyError<K>> {
        let node_id = Rc::new(node_id);

        // Input validation: element
        self._err_node_not_found(node_id.clone())?;

        match super_id {
            // Node is appended to root nodes
            None => { // tarpaulin: exclude false positive from code coverage
                let pos = self.node0.len();
                match self.node0.contains(&node_id) {
                    true => return Err(DuplicateRootNode(node_id)),
                    false => self._append_root_at(node_id, pos),
                };
            }
            // Node is appended to existing super-node
            Some(super_id) => {
                let super_id = Rc::new(super_id);

                // Input validation: super_id
                self._err_node_not_found(super_id.clone())?;

                // Input validation: super_id - node_id
                self._err_duplicate_sub_node(super_id.clone(), node_id.clone())?;

                // Prevent loop and append node to super-node
                self._err_loop_detected(super_id.clone(), node_id.clone(), None)?;
                let pos = self._get_node_opt(super_id.clone()).unwrap().count_subs();
                self._append_at(super_id, node_id, pos);
            }
        }

        Ok(self)
    }

    /// Appends element as sub-node at a specified index of an existing node identified by super_id.
    ///
    /// # Errors
    /// `DuplicateNode`: node already exists at that level in taxonomy
    /// `NodeNotFound(id)`: node with id not found in taxonomy.
    /// `LoopDetected`: appending the node would result in a loop.
    ///
    /// # Examples
    /// todo
    /// ```text
    // Test coverage: { unit = done, integration = missing, doc = missing } -> not ok
    pub fn append_at(&mut self, super_id: Option<K>, node_id: K, index: usize) -> Result<&mut Self, TaxonomyError<K>> {
        let node_id = Rc::new(node_id);

        // Input validation: node_id
        self._err_node_not_found(node_id.clone())?;

        match super_id {
            // Node is appended to root nodes
            None => { // tarpaulin: exclude false positive from code coverage
                match self.node0.contains(&node_id) {
                    true => return Err(DuplicateRootNode(node_id)),
                    false => self._append_root_at(node_id, index),
                };
            }
            // Node is appended to existing super-node
            Some(super_id) => {
                let super_id = Rc::new(super_id);

                // Input validation: super_id
                self._err_node_not_found(super_id.clone())?;

                // Input validation: super_id - node_id
                // todo: a duplicate sub-node is like a duplicate edge, isn't?
                self._err_duplicate_sub_node(super_id.clone(), node_id.clone())?;

                // Prevent loop and append node to super-node
                self._err_loop_detected(super_id.clone(), node_id.clone(), None)?;
                self._append_at(super_id, node_id, index);
            }
        }

        Ok(self)
    }

    /// Moves element either from one super-node to another.
    ///
    /// Supports root and non-root nodes as source and destination.
    /// Does not support moving to another position at the same super-node.
    ///
    /// # Errors
    /// `NodeNotFound(id)`: node specified by id not found in taxonomy.
    /// `EdgeNotFound(from_super_id, node_id)`: Edge not found (Source)
    /// `DuplicateEdge(to_super_id, node_id)`: Edge already exists (Destination)
    /// `LoopDetected(id)`: Loop detected
    ///
    /// # Examples
    /// todo
    ///
    // Test coverage: { unit = done, integration = missing, doc = missing } -> not ok
    pub fn move_to(
        &mut self,
        node_id: Rc<K>,
        from_super_id: Option<Rc<K>>,
        to_super_id: Option<Rc<K>>,
        index: usize,
    ) -> Result<&mut Self, TaxonomyError<K>> {
        // Input validation: node_id
        self._err_node_not_found(node_id.clone())?;

        // Input validation: from_super_id
        if let Some(id) = from_super_id.clone() {
            self._err_node_not_found(id)?;
        }

        // Input validation: to_super_id
        if let Some(id) = to_super_id.clone() {
            self._err_node_not_found(id)?;
        }

        // Input validation: Edge(from_super_id, node_id)
        let from_edge = Edge::new(from_super_id, node_id.clone());
        self._err_edge_not_found(&from_edge)?;

        // Input validation: Edge(to_super_id, node_id)
        // todo: duplicate edge prevents to use move_to for moving an element to another position at the same super_node.
        let to_edge = Edge::new(to_super_id.clone(), node_id.clone());
        self._err_duplicate_edge(&to_edge)?;

        match to_super_id {
            // Node is appended to root nodes
            None => { // tarpaulin: exclude false positive from code coverage
                self._append_root_at(node_id, index);
            }
            // Node is appended to existing super-node
            Some(super_id) => {
                // Prevent loop and append node to super-node
                self._err_loop_detected(super_id.clone(), node_id.clone(), None)?;
                self._append_at(super_id, node_id, index);
            }
        }

        self.remove_from(from_edge)?;

        Ok(self)
    }

    /// Removes a node from taxonomy, even it is represented in multiple places within the taxonomy.
    ///
    /// Refuses removal if node has sub-nodes, use remove_recursively.
    ///
    /// # Errors
    /// `NodeNotFound`: node not found
    /// `NodeHasSubNode`: use remove_recursively to remove a node with all its subs
    ///
    /// # Examples
    /// todo
    // Test coverage: { unit = done, integration = missing, doc = missing } -> not ok
    pub fn remove(&mut self, node_id: Rc<K>) -> Result<&mut Self, TaxonomyError<K>> {
        // Input validation: node_id
        let node = self._get_node_res(node_id.clone())?;

        // Do not remove if node has sub-nodes
        self._err_node_has_sub(node)?;

        // Collect ids of all direct super-nodes
        let super_ids = node.supers().iter().cloned().collect::<Vec<Rc<K>>>();

        // Delete node from all super-nodes
        for super_id in super_ids {
            self._pre_update(super_id.clone());

            let super_node = self._get_node_mut_opt(super_id.clone()).unwrap();
            super_node.remove_sub(node_id.clone());

            self._post_update(super_id);
        }

        let node = self._get_node_res(node_id.clone())?;
        if node.is_root() {
            self._remove_root_node(node_id);
        } else {
            self._remove_non_root_node(node_id);
        }

        Ok(self)
    }

    /// Removes a sub-node from a specified edge (a super- / sub-node relationship).
    ///
    /// Nodes with multiple super-nodes remain in the taxonomy. Only the specified edge
    /// is removed.
    /// Nodes with just one super-node are removed from the taxonomy. In case of any
    /// sub-nodes removal is recursive.
    /// Nodes without a super node (root nodes) are removed from the taxonomy, also
    /// recursively in case there are sub nodes.
    ///
    /// # Errors
    /// `EdgeNotFound`: either super or sub node are not found at all in the taxonomy,
    ///                 or there is no edge between the two.
    ///
    /// # Examples
    /// todo
    // Test coverage: { unit = done, integration = missing, doc = missing } -> not ok
    pub fn remove_from(&mut self, edge: Edge<K>) -> Result<&mut Self, TaxonomyError<K>> {
        self._err_edge_not_found(&edge)?;

        match (edge.super_id(), edge.node_id()) {
            // Node is a root-node because edge's super_id is None
            (None, node_id) => {
                // If node has other super-nodes, only this edge is removed.
                // If node has no other super-node, the edges to its sub-nodes are removed
                //   which might result in a recusive removal of a whole node tree from the taxonomy.
                self._pre_update(node_id.clone());
                let node = self._get_node_mut_opt(node_id.clone()).unwrap();
                node.remove_super(None);

                // Determine index of node (which is an ex-root-node) in node0 and remove it
                if let Some(index) = self.node0.iter().position(|root_node_id| root_node_id.clone() == node_id) {
                    // Optimize for first and last element in self.node0
                    if index == 0 {
                        self.node0.pop_front();
                    } else if index == self.node0.len() - 1 {
                        self.node0.pop_back();
                    } else {
                        let mut remain = self.node0.split_off(index);
                        remain.pop_front();
                        self.node0.append(&mut remain);
                    }
                }

                // If node does not have another super-node it has to be removed
                // from the taxonomy all together.
                let node = self._get_node_mut_opt(node_id.clone()).unwrap();
                if !node.has_super() {
                    // Remove edges to sub-nodes recursively
                    if node.has_sub() {
                        let sub_ids = node.subs().iter().cloned().collect::<Vec<Rc<K>>>();

                        for sub_id in sub_ids {
                            self.remove_from(Edge::new(Some(node_id.clone()), sub_id.clone()))?;
                        }
                    }

                    self.remove(node_id.clone())?;
                }
                self._post_update(node_id);
            }
            // Node is a sub-node
            // If node has multiple super-nodes, only remove it from super-node specified in Edge.
            // If node has no other super-node, the edges to its sub-nodes are removed
            //   which might result in a recusive removal of a whole node tree from the taxonomy.
            // If node has one super-node and no sub-nodes, simply remove it.
            (Some(super_id), node_id) => {
                // Remove node from super-node
                self._pre_update(super_id.clone());
                let super_node = self._get_node_mut_opt(super_id.clone()).unwrap();
                super_node.remove_sub(node_id.clone());
                self._post_update(super_id.clone());

                // Remove super-node from node
                self._pre_update(node_id.clone());
                let node = self._get_node_mut_opt(node_id.clone()).unwrap();
                node.remove_super(Some(super_id));

                // If node does not have another super-node it has to be removed
                // from the taxonomy all together.
                if !node.has_super() {
                    // Remove edges to sub-nodes recursively
                    if node.has_sub() {
                        let sub_ids = node.subs().iter().cloned().collect::<Vec<Rc<K>>>();

                        for sub_id in sub_ids {
                            self.remove_from(Edge::new(Some(node_id.clone()), sub_id.clone()))?;
                        }
                    }

                    self.remove(node_id.clone())?;
                }

                self._post_update(node_id);
            }
        }

        Ok(self)
    }

    /// Remove node from taxonomy: recursively and completely.
    /// - recursively: including its sub-nodes, their sub-nodes and so on.
    /// - completely: from all its super-nodes
    ///
    /// Use remove_from if you want to remove the node only from one of its super-nodes.
    ///
    /// # Errors
    /// `NodeNotFound`: either super or sub node are not found at all in the taxonomy,
    ///                 or there is no edge between the two.
    ///
    /// # Examples
    /// todo
    /// ```text
    // Test coverage: { unit = missing, integration = missing, doc = missing } -> not ok
    pub fn remove_recursively(&mut self, node_id: Rc<K>) -> Result<&mut Self, TaxonomyError<K>> {
        let node = self._get_node_res(node_id.clone())?;
        let subs = node.subs().iter().cloned().collect::<Vec<Rc<K>>>();

        for sub in subs {
            self.remove_recursively(sub.clone())?;
        }

        self.remove(node_id)
    }

    /// Traverses the taxonomy from first to last node, returning immutable references to its elements.
    ///
    /// # Examples
    /// todo
    /// ```text
    // Test coverage: { unit = done, integration = missing, doc = missing } -> not ok
    pub fn traverse(&mut self) -> Option<&V> {
        if let Some(node_id) = self._next() {
            return self.get(node_id);
        }
        None
    }

    /// Traverses the taxonomy from first to last node, returning mutable references to its elements.
    ///
    /// # Examples
    /// todo
    /// ```text
    // Test coverage: { unit = done, integration = missing, doc = missing } -> not ok
    pub fn traverse_mut(&mut self) -> Option<&mut V> {
        if let Some(node_id) = self._next() {
            return self.get_mut(node_id);
        }
        None
    }
}

//
// Private functions
//
impl<K, V> Taxonomy<K, V>
where
    K: Hash + Eq,
    V: Identity<K>,
{
    /// Adds a non-root-node to the taxonomy.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _add_non_root_node(&mut self, super_id: Rc<K>, node: Node<K, V>) -> &mut Self {
        let node_id = node.id();

        self._pre_update(node_id.clone());

        // Add node to taxonomy
        self.nodes.insert(node_id.clone(), node);

        // Append node to super-node as the last node of all sub-nodes
        let index = self._get_node_opt(super_id.clone()).unwrap().count_subs();
        self._append_at(super_id, node_id.clone(), index);

        self._post_update(node_id);

        self
    }

    /// Adds a root-node to the taxonomy.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _add_root_node(&mut self, node: Node<K, V>) -> &mut Self {
        let node_id = node.id();

        self._pre_update(node_id.clone());
        self.nodes.insert(node_id.clone(), node);
        self.node0.push_back(node_id.clone());
        self._post_update(node_id);

        self
    }

    /// Appends a node to a super-node.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _append_at(&mut self, super_id: Rc<K>, node_id: Rc<K>, index: usize) -> &mut Self {
        // Append node as sub-node to super-node.
        self._pre_update(super_id.clone());
        self._get_node_mut_opt(super_id.clone())
            .unwrap()
            .append_sub_at(node_id.clone(), index);
        self._post_update(super_id.clone());

        // Add super-node to node as super-node.
        self._pre_update(node_id.clone());
        self._get_node_mut_opt(node_id.clone()).unwrap().add_super(Some(super_id));
        self._post_update(node_id);

        self
    }

    /// Appends a node to the root of the taxonomy at a specified position.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _append_root_at(&mut self, node_id: Rc<K>, index: usize) -> &mut Self {
        let mut remain = if index <= self.node0.len() {
            self.node0.split_off(index)
        } else {
            self.node0.split_off(self.node0.len())
        };
        remain.push_front(node_id.clone());
        self.node0.append(&mut remain);

        self._pre_update(node_id.clone());

        // Add super node to node
        self._get_node_mut_opt(node_id.clone()).unwrap().add_super(None);

        self._post_update(node_id);

        self
    }

    /// Collects keys of all sub-nodes recursively without duplicates
    /// Returns an empty HashSet when node is not found or there are no sub-nodes
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _enumerate_subs(&self, start_node: Rc<K>) -> HashSet<Rc<K>> {
        let mut subs = HashSet::new();

        // Start with
        match self._get_node_opt(start_node) {
            None => subs,
            Some(node) => {
                // Collect all direct sub-nodes of start_node
                subs.extend(node.subs().iter().cloned());

                // Collect indirect sub-nodes by calling this fn recursively
                for sub_node in node.subs().clone() {
                    subs.extend(self._enumerate_subs(sub_node));
                }

                subs
            }
        }
    }

    /// Err(DuplicateNode)
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _err_duplicate_node(&self, node_id: Rc<K>) -> Result<&Self, TaxonomyError<K>> {
        if self.nodes.contains_key(&node_id) {
            Err(DuplicateNode(node_id))
        } else {
            Ok(self)
        }
    }

    /// Err(DuplicateRootNode)
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _err_duplicate_root_node(&self, node_id: Rc<K>) -> Result<&Self, TaxonomyError<K>> {
        if self.node0.contains(&node_id) {
            Err(DuplicateRootNode(node_id))
        } else {
            Ok(self)
        }
    }

    /// Err(DuplicateSubNode)
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _err_duplicate_sub_node(&self, super_id: Rc<K>, node_id: Rc<K>) -> Result<&Self, TaxonomyError<K>> {
        let super_node = self._get_node_res(super_id.clone())?;

        if super_node.subs().contains(&node_id) {
            Err(DuplicateSubNode(super_id, node_id))
        } else {
            Ok(self)
        }
    }

    /// Err(DuplicateEdge)
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _err_duplicate_edge(&self, edge: &Edge<K>) -> Result<&Self, TaxonomyError<K>> {
        match (edge.super_id(), edge.node_id()) {
            // Root node identified by id
            (None, node_id) => {
                if self.node0.contains(&node_id) {
                    return Err(DuplicateEdge(None, node_id));
                }
            }
            // Sub node identified by id
            (Some(super_id), node_id) => {
                if let Some(super_node) = self._get_node_opt(super_id.clone()) {
                    if super_node.subs().contains(&node_id) {
                        return Err(DuplicateEdge(Some(super_id), node_id));
                    }
                }
            }
        };

        Ok(self)
    }

    /// Err(EdgeNotFound)
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _err_edge_not_found(&self, edge: &Edge<K>) -> Result<&Self, TaxonomyError<K>> {
        match (edge.super_id(), edge.node_id()) {
            // Root node identified by id
            (None, node_id) => {
                if !self.node0.contains(&node_id) {
                    return Err(EdgeNotFound(None, node_id));
                }
            }
            // Sub node identified by id
            (Some(super_id), node_id) => match self._get_node_opt(super_id.clone()) {
                None => return Err(EdgeNotFound(Some(super_id), node_id)),
                Some(super_node) => {
                    if !super_node.subs().contains(&node_id) {
                        return Err(EdgeNotFound(Some(super_id), node_id));
                    }
                }
            },
        };

        Ok(self)
    }

    /// Err(LoopDetected): detects direct and indirect loops.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _err_loop_detected(
        &self,
        super_id: Rc<K>,
        node_id: Rc<K>,
        subs: Option<Rc<HashSet<Rc<K>>>>,
    ) -> Result<&Self, TaxonomyError<K>> {
        // Node and its anticipated super node are identical => loop
        if node_id == super_id {
            return Err(LoopDetected(node_id)); // loop detected
        }
        // Compare node and all its sub-nodes (collected recursively first) with all anticipated super-nodes, any match => loop
        else { // tarpaulin: exclude false positive from code coverage
            // collect sub-nodes recursively if this fn was called with subs: None
            let subs = match subs {
                None => Rc::new(self._enumerate_subs(node_id.clone())),
                Some(subs) => subs,
            };

            // Check if a sub is identical with future super-node => loop
            if subs.contains(&super_id) {
                return Err(LoopDetected(node_id)); // loop detected
            }

            // Check super-nodes recursively, return as soon as a loop is detected
            for next_super in self._get_node_opt(super_id.clone()).unwrap().supers() {
                self._err_loop_detected(next_super.clone(), super_id.clone(), Some(subs.clone()))?;
            }
        }

        Ok(self) // no loop detected
    }

    /// Err(NodeHasSubNodes)
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _err_node_has_sub(&self, node: &Node<K, V>) -> Result<&Self, TaxonomyError<K>> {
        if node.has_sub() {
            Err(NodeHasSubNode(node.id()))
        } else {
            Ok(self)
        }
    }

    /// Err(NodeNotFound)
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _err_node_not_found(&self, node_id: Rc<K>) -> Result<&Self, TaxonomyError<K>> {
        if self.nodes.contains_key(&node_id) {
            Ok(self)
        } else {
            Err(NodeNotFound(node_id))
        }
    }

    /// Returns node id cursor points to.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _get_node_id_from_cursor(&self) -> (Option<Rc<K>>, Option<Rc<K>>) {
        match &self.cursor.last() {
            None => (None, None),
            Some(cursor) => match (cursor.super_id(), cursor.node_index()) {
                (None, node_index) => {
                    let node_id = self._get_root_node_id_at(node_index);
                    (None, node_id)
                }
                (Some(super_id), node_index) => {
                    let super_node = self._get_node_opt(super_id.clone()).unwrap();
                    let node_id = super_node.sub_at(node_index);
                    (Some(super_id), node_id)
                }
            },
        }
    }

    /// Returns an immutable reference to a node
    /// OR
    /// None
    ///
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _get_node_opt(&self, id: Rc<K>) -> Option<&Node<K, V>> {
        self.nodes.get(&id)
    }

    /// Returns an immutable reference to a node
    /// OR
    /// Err(NodeNotFound)
    ///
    // Test coverage: { unit = done, integration = missing, doc = done } -> ok
    fn _get_node_res(&self, node_id: Rc<K>) -> Result<&Node<K, V>, TaxonomyError<K>> {
        match self._get_node_opt(node_id.clone()) {
            None => Err(NodeNotFound(node_id)),
            Some(node) => Ok(node),
        }
    }

    /// Returns a mutable reference to a node
    /// OR
    /// None
    ///
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _get_node_mut_opt(&mut self, id: Rc<K>) -> Option<&mut Node<K, V>> {
        self.nodes.get_mut(&id)
    }

    /// Returns a mutable reference to a node
    /// OR
    /// Err(NodeNotFound)
    ///
    // Test coverage: { unit = done, integration = missing, doc = done } -> ok
    fn _get_node_mut_res(&mut self, node_id: Rc<K>) -> Result<&mut Node<K, V>, TaxonomyError<K>> {
        match self._get_node_mut_opt(node_id.clone()) {
            None => Err(NodeNotFound(node_id)),
            Some(node) => Ok(node),
        }
    }

    /// Returns id of a root-node at self.node0\[index\]
    /// OR
    /// None if index is greater or equal to the number of root-nodes.
    ///
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _get_root_node_id_at(&self, index: usize) -> Option<Rc<K>> {
        self.node0.iter().nth(index).cloned()
    }

    /// Returns next node's id or None if there is no more node in taxonomy.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _next(&mut self) -> Option<Rc<K>> {
        // Start with last node in cursor
        match self.cursor.last() {
            // Cursor is None => init cursor with first node from node0 if available
            None => { // tarpaulin: exclude false positive from code coverage
                if !self.node0.is_empty() {
                    self.cursor.push(Cursor::new(None, 0));
                }
            }
            // Cursor points to a node => determine the next node
            Some(cursor) => {
                match (cursor.super_id(), cursor.node_index()) {
                    // Cursor points to a root-node.
                    // - if root-node has a sub-node => push first sub-node to cursor
                    // - else pop root-node from cursor and
                    //   - if taxonomy has another root-node => push next root-node to cursor
                    //   - else => cursor is None => traversing taxonomy finished
                    (None, node_index) => {
                        let node_id = self._get_root_node_id_at(node_index).unwrap();
                        let node = self._get_node_opt(node_id.clone()).unwrap();

                        if node.has_sub() {
                            self.cursor.push(Cursor::new(Some(node_id), 0));
                        } else {
                            self.cursor.pop();
                            if node_index + 1 < self.node0.len() {
                                self.cursor.push(Cursor::new(None, node_index + 1));
                            }
                        }
                    }

                    // Cursor points to a non-root node.
                    // - if node has a sub-node => push sub-node to cursor
                    // - else pop node from cursor and
                    //   - if super-node has another sub-node (a co-node of the node) => push next sub-node to cursor
                    //   - else => pop until cursor points to another node with co-nodes or a root node
                    (Some(super_id), node_index) => {
                        let super_node = self._get_node_opt(super_id.clone()).unwrap();
                        let node_id = super_node.sub_at(node_index).unwrap();
                        let node = self._get_node_opt(node_id.clone()).unwrap();

                        if node.has_sub() {
                            self.cursor.push(Cursor::new(Some(node_id), 0));
                        } else {
                            self.cursor.pop();
                            let super_node = self._get_node_opt(super_id.clone()).unwrap();

                            if node_index + 1 < super_node.subs().len() {
                                self.cursor.push(Cursor::new(Some(super_id), node_index + 1));
                            } else {
                                while let Some(cursor) = self.cursor.pop() {
                                    match (cursor.super_id(), cursor.node_index()) {
                                        // Root node found, set cursor to co-node if available
                                        (None, node_index) => {
                                            if node_index + 1 < self.node0.len() {
                                                self.cursor.push(Cursor::new(None, node_index + 1));
                                                break; // found next Node, break while loop
                                            }
                                        }
                                        // Non-root node found, set cursor to co-node if available
                                        (Some(super_id), node_index) => {
                                            let super_node = self._get_node_opt(super_id.clone()).unwrap();
                                            if node_index + 1 < super_node.count_subs() {
                                                self.cursor.push(Cursor::new(Some(super_id), node_index + 1));
                                                break; // found next Node, break while loop
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        match self.cursor.last() {
            None => None,
            Some(_) => self._get_node_id_from_cursor().1,
        }
    }

    /// Post function to any node update, e.g. add, append, move, remove.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _post_update(&mut self, node_id: Rc<K>) -> &mut Self {
        if self._get_node_opt(node_id.clone()).is_some() {
            self.last_updated_node = Some(node_id);
        }
        self
    }

    /// Pre function to any node update, e.g. add, append, move, remove.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _pre_update(&mut self, node_id: Rc<K>) -> &mut Self {
        if self._get_node_opt(node_id).is_some() {}
        self // return &mut Taxonomy
    }

    /// Removes a node.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _remove_non_root_node(&mut self, node_id: Rc<K>) -> &mut Self {
        self._pre_update(node_id.clone());

        // Post update required before node_id is removed from taxonomy.
        self._post_update(node_id.clone());

        // Delete node from taxonomy
        self.nodes.remove(&node_id);

        self // return &mut Taxonomy
    }

    /// Removes a root-node identified by id, silently ignores if id is missing.
    // Test coverage: { unit = done, integration = none, doc = none } -> ok
    fn _remove_root_node(&mut self, node_id: Rc<K>) -> &mut Self {
        self._pre_update(node_id.clone());

        // Determine index of root-node in node0 and remove it
        if let Some(index) = self.node0.iter().position(|root_node_id| root_node_id.clone() == node_id) {
            // Optimize for first and last element in self.node0
            if index == 0 {
                self.node0.pop_front();
            } else if index == self.node0.len() - 1 {
                self.node0.pop_back();
            } else {
                let mut remain = self.node0.split_off(index);
                remain.pop_front();
                self.node0.append(&mut remain);
            }
        }

        // Post update required before node_id is removed from taxonomy.
        self._post_update(node_id.clone());

        // Delete node from taxonomy
        self.nodes.remove(&node_id);

        self // return &mut Taxonomy
    }
}
