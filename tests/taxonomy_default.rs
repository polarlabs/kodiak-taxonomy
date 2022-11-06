/*
#[test]
fn test_has_subordinates() {
    let mut tax = Taxonomy::new();

    let node = Node::new(Concept::new("Animal"));
    let super_id = Uuid::parse_str(node.id().as_str()).unwrap();
    tax.add(node);

    let node = Node::new(Concept::new("Vertebrate"));
    let sub1a_id = Uuid::parse_str(node.id().as_str()).unwrap();
    tax.append(super_id, node);

    let arbitrary_id = Uuid::new_v4();

    assert_eq!(
        tax.has_subordinates(arbitrary_id),
        Err(NodeNotFound(arbitrary_id))
    );
    assert_eq!(tax.has_subordinates(super_id), Ok(true));
    assert_eq!(tax.has_subordinates(sub1a_id), Ok(false));
}
*/

/*
#[test]
fn test_remove_node() {
    let mut tax = Taxonomy::new();

    let node = Node::new(Concept::new("Animal"));
    let super_id = Uuid::parse_str(node.id().as_str()).unwrap();
    tax.add(node);

    let node = Node::new(Concept::new("Vertebrate"));
    let sub1a_id = Uuid::parse_str(node.id().as_str()).unwrap();
    tax.append(super_id, node);

    let node = Node::new(Concept::new("Invertebrate"));
    let sub1b_id = Uuid::parse_str(node.id().as_str()).unwrap();
    tax.append(super_id, node);

    let node = Node::new(Concept::new("Mammal"));
    let sub2a_id = Uuid::parse_str(node.id().as_str()).unwrap();
    tax.append(sub1a_id, node);

    tax.remove(sub1a_id);

    tax.remove(sub1b_id);
    tax.remove(sub2a_id);
    tax.remove(sub1a_id);

    tax.remove(super_id);
    assert_eq!(tax.root_nodes.len(), 0);
}
*/

/*
#[test]
fn test_traverse_taxonomy() {
    let mut tax = Taxonomy::new();

    let node = Node::new(Concept::new("Animal"));
    let super_id = Uuid::parse_str(node.id().as_str()).unwrap();
    tax.add(node);

    let node = Node::new(Concept::new("Vertebrate"));
    let sub1a_id = Uuid::parse_str(node.id().as_str()).unwrap();
    tax.append(super_id, node);

    let node = Node::new(Concept::new("Invertebrate"));
    let sub1b_id = Uuid::parse_str(node.id().as_str()).unwrap();
    tax.append(super_id, node);

    let node = Node::new(Concept::new("Mammal"));
    let sub2a_id = Uuid::parse_str(node.id().as_str()).unwrap();
    tax.append(sub1a_id, node);

    tax.traverse();
    assert_eq!(1, 1);
}
*/

// todos:
// cover remove and remove_recursively with tests
// implement traverse
// write a consistency check
// implement append_at
