#[cfg(test)]
mod tests {
    use crate::tests::taxonomy::*;
    use crate::TaxonomyError::*;
    use crate::{Edge, Identity, Node};

    use uuid::Uuid;

    use std::rc::Rc;

    #[test]
    fn default() {
        let tax = setup_taxonomy_default();

        assert_eq!(tax.nodes.len(), 0);
        assert_eq!(tax.node0.len(), 0);
        assert!(tax.last_updated_node.is_none());
        assert_eq!(tax.cursor.len(), 0);
    }

    #[test]
    fn test_new_taxonomy() {
        let tax = setup_tax_empty();

        assert_eq!(tax.nodes.len(), 0);
        assert!(tax.last_updated_node.is_none());
        assert_eq!(tax.node0.len(), 0);
    }

    #[test]
    fn add() {
        let (mut tax, _, list) = setup_tax_animals();
        let counter = tax.nodes.len();

        // Test adding a new root-node
        let c_tierwohl = Concept::new("Tierwohl");
        let id_tierwohl = Rc::new(c_tierwohl.id());
        let super_id = None;

        let counter_node0_pre = tax.node0.len();

        let result = tax.add(super_id, c_tierwohl);
        assert!(result.is_ok());

        let counter_node0_post = tax.node0.len();

        assert_eq!(tax.nodes.len(), counter + 1);
        assert_eq!(counter_node0_post, counter_node0_pre + 1);
        assert_eq!(tax.last_updated_node().unwrap(), id_tierwohl);
        assert_eq!(tax._get_node_opt(id_tierwohl).unwrap().supers().len(), 0);

        // Test adding a new sub-node
        let c_nagetiere = Concept::new("Nagetiere");
        let id_nagetiere = Rc::new(c_nagetiere.id());
        let super_id = list.iter().nth(1).unwrap().0;

        let counter_subs_pre = tax._get_node_opt(Rc::new(super_id.clone())).unwrap().count_subs();

        let result = tax.add(Some(super_id.clone()), c_nagetiere.clone());
        assert!(result.is_ok());

        let counter_subs_post = tax._get_node_opt(Rc::new(super_id.clone())).unwrap().count_subs();

        assert_eq!(tax.nodes.len(), counter + 2);
        assert_eq!(counter_subs_post, counter_subs_pre + 1);
        assert_eq!(tax.last_updated_node().unwrap(), id_nagetiere);
        assert_eq!(
            *tax._get_node_opt(Rc::new(super_id.clone())).unwrap().subs().back().unwrap(),
            id_nagetiere
        );
        assert_eq!(tax._get_node_opt(id_nagetiere.clone()).unwrap().supers().len(), 1);

        // Test adding a duplicate node
        let result = tax.add(Some(super_id.clone()), c_nagetiere).err();
        let expectation = DuplicateNode(id_nagetiere);
        assert_eq!(result, Some(expectation));
    }

    #[test]
    fn append() {
        let (mut tax, ids, _) = setup_tax_animals();
        let last_updated_node = tax.last_updated_node().unwrap();

        // Append a non-existing node to a non-existing super-node
        let node_id = Uuid::new_v4();
        let super_id = Uuid::new_v4();
        let result = tax.append(Some(super_id), node_id.clone()).err();
        let expectation = NodeNotFound(Rc::new(node_id));
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Append an existing node to a non-existing super-node
        let id_affen = ids.get("Affen").unwrap().clone();
        let super_id = Uuid::new_v4();
        let result = tax.append_at(Some(super_id), id_affen.clone(), 0).err();
        let expectation = NodeNotFound(Rc::new(super_id));
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Append an existing node to one of its own super-nodes
        let id_affen = ids.get("Affen").unwrap().clone();
        let id_zootiere = ids.get("Zootiere").unwrap().clone();
        let result = tax.append(Some(id_zootiere), id_affen.clone()).err();
        let expectation = DuplicateSubNode(Rc::new(id_zootiere), Rc::new(id_affen));
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Append an existing node to one of its own sub-nodes (loop detection)
        let id_zootiere = ids.get("Zootiere").unwrap().clone();
        let id_schlangen = ids.get("Schlangen").unwrap().clone();
        let result = tax.append(Some(id_schlangen), id_zootiere.clone()).err();
        let expectation = LoopDetected(Rc::new(id_zootiere));
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Append an existing root-node to root-nodes
        let id_tiere = ids.get("Tiere").unwrap().clone();
        let super_id = None;
        let result = tax.append(super_id, id_tiere.clone()).err();
        let expectation = DuplicateRootNode(Rc::new(id_tiere));
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Append an existing node to root-nodes
        let id_nutztiere = ids.get("Nutztiere").unwrap().clone();
        let super_id = None;
        assert!(tax.append(super_id, id_nutztiere.clone()).is_ok());
        assert_eq!(*tax.node0.iter().last().unwrap(), Rc::new(id_nutztiere));

        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_nutztiere));

        // Append an existing node to another existing nodes
        let id_waale = ids.get("Waale & Delfine").unwrap().clone();
        let id_tierschutz = ids.get("Tierschutz").unwrap().clone();
        assert!(tax.append(Some(id_tierschutz), id_waale.clone()).is_ok());

        let tierschutz = tax._get_node_opt(Rc::new(id_tierschutz.clone())).unwrap();
        assert_eq!(tierschutz.subs().contains(&Rc::new(id_waale)), true);
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_waale));
    }

    #[test]
    fn append_at() {
        let (mut tax, ids, _) = setup_tax_animals();
        let last_updated_node = tax.last_updated_node().unwrap();

        // Append a non-existing node to a non-existing super-node
        let node_id = Uuid::new_v4();
        let super_id = Uuid::new_v4();
        let result = tax.append_at(Some(super_id), node_id.clone(), 0).err();
        let expectation = NodeNotFound(Rc::new(node_id));
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Append an existing node to a non-existing super-node
        let id_affen = ids.get("Affen").unwrap().clone();
        let super_id = Uuid::new_v4();
        let result = tax.append_at(Some(super_id), id_affen.clone(), 0).err();
        let expectation = NodeNotFound(Rc::new(super_id));
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Append an existing node to one of its own super-nodes
        let id_affen = ids.get("Affen").unwrap().clone();
        let id_zootiere = ids.get("Zootiere").unwrap().clone();
        let result = tax.append_at(Some(id_zootiere), id_affen.clone(), 0).err();
        let expectation = DuplicateSubNode(Rc::new(id_zootiere), Rc::new(id_affen));
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Append an existing node to one of its own sub-nodes (loop detection)
        let id_zootiere = ids.get("Zootiere").unwrap().clone();
        let id_schlangen = ids.get("Schlangen").unwrap().clone();
        let result = tax.append_at(Some(id_schlangen), id_zootiere.clone(), 0).err();
        let expectation = LoopDetected(Rc::new(id_zootiere));
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Append an existing root-node to root-nodes
        let id_tiere = ids.get("Tiere").unwrap().clone();
        let super_id = None;
        let result = tax.append_at(super_id, id_tiere.clone(), tax.node0.len() / 2).err();
        let expectation = DuplicateRootNode(Rc::new(id_tiere));
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Append an existing node to root-nodes
        let id_nutztiere = ids.get("Nutztiere").unwrap().clone();
        let super_id = None;
        assert!(tax.append_at(super_id, id_nutztiere.clone(), tax.node0.len() / 2).is_ok());
        assert_eq!(*tax.node0.iter().nth(tax.node0.len() / 2).unwrap(), Rc::new(id_nutztiere));

        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_nutztiere));

        // Append an existing node to another existing nodes
        let id_waale = ids.get("Waale & Delfine").unwrap().clone();
        let id_tierschutz = ids.get("Tierschutz").unwrap().clone();
        assert!(tax.append_at(Some(id_tierschutz), id_waale.clone(), 5).is_ok());

        let tierschutz = tax._get_node_opt(Rc::new(id_tierschutz.clone())).unwrap();
        assert_eq!(tierschutz.subs().contains(&Rc::new(id_waale)), true);
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_waale));
    }

    #[test]
    fn move_to() {
        let (mut tax, ids, _) = setup_tax_animals();
        let last_updated_node = tax.last_updated_node().unwrap();

        // Move a non-existing node to a non-existing node
        let node_id = Rc::new(Uuid::new_v4());
        let to_super_id = Rc::new(Uuid::new_v4());
        let result = tax.move_to(node_id.clone(), None, Some(to_super_id.clone()), 0).err();
        let expectation = NodeNotFound(node_id);
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Move an existing node from a non-existing node to an existing node
        let id_affen = Rc::new(ids.get("Affen").unwrap().clone());
        let from_super_id = Rc::new(Uuid::new_v4());
        let id_tierschutz = Rc::new(ids.get("Tierschutz").unwrap().clone());
        let result = tax
            .move_to(id_affen.clone(), Some(from_super_id.clone()), Some(id_tierschutz.clone()), 0)
            .err();
        let expectation = NodeNotFound(from_super_id);
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Move an existing node to a none-existing node
        let id_affen = Rc::new(ids.get("Affen").unwrap().clone());
        let to_super_id = Rc::new(Uuid::new_v4());
        let result = tax.move_to(id_affen.clone(), None, Some(to_super_id.clone()), 0).err();
        let expectation = NodeNotFound(to_super_id);
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Move with a non-existing source-edge, i. e. source super node and node exists but do not share a super-sub-relationship.
        let id_affen = Rc::new(ids.get("Affen").unwrap().clone());
        let id_voegel = Rc::new(ids.get("Vögel").unwrap().clone());
        let id_tierschutz = Rc::new(ids.get("Tierschutz").unwrap().clone());
        let result = tax
            .move_to(id_affen.clone(), Some(id_voegel.clone()), Some(id_tierschutz.clone()), 0)
            .err();
        let expectation = EdgeNotFound(Some(id_voegel), id_affen);
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Move to an already existing edge.
        let id_affen = Rc::new(ids.get("Affen").unwrap().clone());
        let id_zootiere = Rc::new(ids.get("Zootiere").unwrap().clone());
        let id_saeugetiere = Rc::new(ids.get("Säugetiere").unwrap().clone());
        let result = tax
            .move_to(id_affen.clone(), Some(id_zootiere.clone()), Some(id_saeugetiere.clone()), 0)
            .err();
        let expectation = DuplicateEdge(Some(id_saeugetiere), id_affen);
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Move node to become a new root-node
        let id_schlangen = Rc::new(ids.get("Schlangen").unwrap().clone());
        let id_zootiere = Rc::new(ids.get("Zootiere").unwrap().clone());
        assert!(tax.move_to(id_schlangen.clone(), Some(id_zootiere.clone()), None, 0).is_ok());

        let zootiere = tax._get_node_opt(id_zootiere.clone()).unwrap();
        assert_eq!(zootiere.subs().contains(&id_schlangen), false);
        assert_eq!(tax.node0.contains(&id_schlangen), true);
        assert_eq!(tax.last_updated_node().unwrap(), id_schlangen);

        // Move to a new super-node (non-root)
        let id_affen = Rc::new(ids.get("Affen").unwrap().clone());
        let id_zootiere = Rc::new(ids.get("Zootiere").unwrap().clone());
        let id_tierschutz = Rc::new(ids.get("Tierschutz").unwrap().clone());
        assert!(tax
            .move_to(id_affen.clone(), Some(id_zootiere.clone()), Some(id_tierschutz.clone()), 0)
            .is_ok());

        let zootiere = tax._get_node_opt(id_zootiere.clone()).unwrap();
        assert_eq!(zootiere.subs().contains(&id_affen), false);

        let tierschutz = tax._get_node_opt(id_tierschutz.clone()).unwrap();
        assert_eq!(tierschutz.subs().contains(&id_affen), true);

        assert_eq!(tax.last_updated_node().unwrap(), id_affen);

        // todo: test move to a defined position
    }

    #[test]
    fn remove() {
        let (mut tax, ids, _) = setup_tax_animals();
        let last_updated_node = tax.last_updated_node().unwrap();

        // Remove a non-existing node.
        let node_id = Rc::new(Uuid::new_v4());
        let result = tax.remove(node_id.clone()).err();
        let expectation = NodeNotFound(node_id);
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Remove a node with sub-nodes
        let id_hunde = Rc::new(ids.get("Hunde").unwrap().clone());
        let result = tax.remove(id_hunde.clone()).err();
        let expectation = NodeHasSubNode(id_hunde.clone());
        assert_eq!(result, Some(expectation));
        assert_eq!(tax.last_updated_node().unwrap(), last_updated_node);

        // Remove nodes without sub-nodes and only one super-node
        let id_doggen = Rc::new(ids.get("Doggen").unwrap().clone());
        assert!(tax.remove(id_doggen.clone()).is_ok());
        assert_eq!(tax.nodes.contains_key(&id_doggen), false);
        assert_eq!(tax.last_updated_node().unwrap(), id_doggen);

        let id_schaeferhunde = Rc::new(ids.get("Schäferhunde").unwrap().clone());
        assert!(tax.remove(id_schaeferhunde.clone()).is_ok());
        assert_eq!(tax.nodes.contains_key(&id_schaeferhunde), false);
        assert_eq!(tax.last_updated_node().unwrap(), id_schaeferhunde);

        assert_eq!(tax._get_node_opt(id_hunde.clone()).unwrap().has_sub(), false);

        // Remove nodes without sub-nodes and only one super-node (non-root-node)
        assert!(tax.remove(id_hunde.clone()).is_ok());
        assert_eq!(tax.nodes.contains_key(&id_hunde), false);
        assert_eq!(tax.last_updated_node().unwrap(), id_hunde);

        //
        // Remove node without sub-nodes and multiple super-nodes (non-root-node)
        //
        let id_katzen = Rc::new(ids.get("Katzen").unwrap().clone());

        // Get a list of super-nodes of the to be removed node
        let supers = tax._get_node_opt(id_katzen.clone()).unwrap().supers();

        assert!(tax.remove(id_katzen.clone()).is_ok());
        assert_eq!(tax.nodes.contains_key(&id_katzen), false);
        assert_eq!(tax.last_updated_node().unwrap(), id_katzen);

        for super_node in supers {
            assert_eq!(tax._get_node_opt(super_node).unwrap().subs().contains(&id_katzen), false);
        }

        // Remove node without sub-nodes and one super-node (root-node)
        let id_tierheime = Rc::new(ids.get("Tierheime").unwrap().clone());
        assert!(tax.remove(id_tierheime.clone()).is_ok());
        assert_eq!(tax.nodes.contains_key(&id_tierheime), false);
        assert_eq!(tax.node0.contains(&id_tierheime), false);
    }

    #[test]
    fn remove_from_edge_none_first_root_node() {
        let (mut tax, ids, _) = setup_tax_animals();

        // Remove an existing edge: Edge(None, first root-node)
        let super_id = None;
        let node_id = Rc::new(ids.get("Tiere").unwrap().clone());
        assert!(tax.remove_from(Edge::new(super_id, node_id.clone())).is_ok());
        assert_eq!(tax.node0.contains(&node_id), false);
        assert_eq!(tax.nodes.contains_key(&node_id), false);
        assert_eq!(tax.last_updated_node().unwrap(), node_id);
    }

    #[test]
    fn remove_from() {
        let (mut tax, ids, _) = setup_tax_animals();

        // Remove a non-existing edge of non-existing nodes
        let super_id = Rc::new(Uuid::new_v4());
        let node_id = Rc::new(Uuid::new_v4());
        let result = tax.remove_from(Edge::new(Some(super_id.clone()), node_id.clone())).err();
        let expectation = EdgeNotFound(Some(super_id), node_id);
        assert_eq!(result, Some(expectation));

        // Remove a non-existing edge of an existing super-node and a non-existing sub-node
        let id_hunde = Rc::new(ids.get("Hunde").unwrap().clone());
        let node_id = Rc::new(Uuid::new_v4());
        let result = tax.remove_from(Edge::new(Some(id_hunde.clone()), node_id.clone())).err();
        let expectation = EdgeNotFound(Some(id_hunde), node_id);
        assert_eq!(result, Some(expectation));

        // Remove a non-existing edge of a non-existing super-node and an existing sub-node
        let super_id = Rc::new(Uuid::new_v4());
        let id_hunde = Rc::new(ids.get("Hunde").unwrap().clone());
        let result = tax.remove_from(Edge::new(Some(super_id.clone()), id_hunde.clone())).err();
        let expectation = EdgeNotFound(Some(super_id), id_hunde);
        assert_eq!(result, Some(expectation));

        // Remove a non-existing edge of two existing nodes which don't share a super-sub relationship
        let id_hunde = Rc::new(ids.get("Hunde").unwrap().clone());
        let id_katzen = Rc::new(ids.get("Katzen").unwrap().clone());
        let result = tax.remove_from(Edge::new(Some(id_hunde.clone()), id_katzen.clone())).err();
        let expectation = EdgeNotFound(Some(id_hunde), id_katzen);
        assert_eq!(result, Some(expectation));

        // Remove an existing edge of a root-node with no other super-nodes and no sub-nodes.
        let (mut tax, ids, _) = setup_tax_animals();
        let id_tierhalter = Rc::new(ids.get("Tierhalter").unwrap().clone());
        assert!(tax.remove_from(Edge::new(None, id_tierhalter.clone())).is_ok());
        assert_eq!(tax.node0.contains(&id_tierhalter), false);
        assert_eq!(tax.nodes.contains_key(&id_tierhalter), false);
        assert_eq!(tax.last_updated_node().unwrap(), id_tierhalter);

        // Remove an existing edge of a root-node with no other super-nodes but with sub-nodes which have other super-nodes as well
        let id_tierheime = Rc::new(ids.get("Tierheime").unwrap().clone());
        let id_katzen = Rc::new(ids.get("Katzen").unwrap().clone());
        let id_hunde = Rc::new(ids.get("Hunde").unwrap().clone());
        assert!(tax.remove_from(Edge::new(None, id_tierheime.clone())).is_ok());
        assert_eq!(tax.node0.contains(&id_tierheime), false);
        assert_eq!(tax.nodes.contains_key(&id_tierheime), false);
        assert_eq!(tax.nodes.contains_key(&id_hunde), true);
        assert_eq!(tax.nodes.contains_key(&id_katzen), true);
        assert_eq!(tax.last_updated_node().unwrap(), id_tierheime);

        // Remove an existing edge of a non-root-node with no other super-nodes but with sub-nodes which have and haven't other super-nodes as well
        let id_tiere = Rc::new(ids.get("Tiere").unwrap().clone());
        let id_saeugetiere = Rc::new(ids.get("Säugetiere").unwrap().clone());
        let id_affen = Rc::new(ids.get("Affen").unwrap().clone());
        let id_hunde = Rc::new(ids.get("Hunde").unwrap().clone());
        let id_katzen = Rc::new(ids.get("Katzen").unwrap().clone());
        let id_waale = Rc::new(ids.get("Waale & Delfine").unwrap().clone());
        assert!(tax
            .remove_from(Edge::new(Some(id_tiere.clone()), id_saeugetiere.clone()))
            .is_ok());
        assert_eq!(tax.node0.contains(&id_saeugetiere), false);
        assert_eq!(tax.nodes.contains_key(&id_saeugetiere), false);
        assert_eq!(tax.nodes.contains_key(&id_affen), true);
        assert_eq!(tax.nodes.contains_key(&id_hunde), true);
        assert_eq!(tax.nodes.contains_key(&id_katzen), true);
        assert_eq!(tax.nodes.contains_key(&id_waale), false);
        assert_eq!(tax.last_updated_node().unwrap(), id_saeugetiere);
    }

    #[test]
    fn remove_recursively() {
        let (mut tax, ids, _) = setup_tax_animals();

        // Remove a non-existing node.
        let node_id = Rc::new(Uuid::new_v4());
        let result = tax.remove_recursively(node_id.clone()).err();
        let expectation = NodeNotFound(node_id);
        assert_eq!(result, Some(expectation));

        // Remove a node with multiple super- and sub-nodes
        let id_hunde = Rc::new(ids.get("Hunde").unwrap().clone());
        let id_doggen = Rc::new(ids.get("Doggen").unwrap().clone());
        let id_schaeferhunde = Rc::new(ids.get("Schäferhunde").unwrap().clone());

        // Get a list of super-nodes of the to be removed node
        let supers = tax._get_node_opt(id_hunde.clone()).unwrap().supers();

        assert!(tax.remove_recursively(id_hunde.clone()).is_ok());
        for super_node in supers {
            assert_eq!(tax._get_node_opt(super_node).unwrap().subs().contains(&id_hunde), false);
        }
        assert!(tax._get_node_opt(id_hunde.clone()).is_none());
        assert!(tax._get_node_opt(id_doggen.clone()).is_none());
        assert!(tax._get_node_opt(id_schaeferhunde.clone()).is_none());
        assert_eq!(tax.last_updated_node().unwrap(), id_hunde);
    }

    #[test]
    fn traverse() {
        let (mut tax, _, list) = setup_tax_animals();

        let mut i = 0;
        while let Some(concept) = tax.traverse_mut() {
            let c = list.iter().nth(i).unwrap().1.clone();
            assert_eq!(*concept, c);
            i += 1;
        }
    }

    #[test]
    fn traverse_mut() {
        let (mut tax, _, list) = setup_tax_animals();

        let mut i = 0;
        while let Some(concept) = tax.traverse_mut() {
            let c = list.iter().nth(i).unwrap().1.clone();
            assert_eq!(*concept, c);
            i += 1;
        }
    }

    //
    // Testing Taxonomy's private functions
    //

    #[test]
    fn _add_non_root_node() {
        let (mut tax, _, list) = setup_tax_animals();
        let counter = tax.nodes.len();

        let c_nagetiere = Concept::new("Nagetiere");
        let id_nagetiere = Rc::new(c_nagetiere.id());

        let super_id = Rc::new(list.iter().nth(1).unwrap().0);
        let counter_subs_pre = tax._get_node_opt(super_id.clone()).unwrap().count_subs();

        tax._add_non_root_node(super_id.clone(), Node::new(c_nagetiere));

        let counter_subs_post = tax._get_node_opt(super_id.clone()).unwrap().count_subs();

        assert_eq!(tax.nodes.len(), counter + 1);
        assert_eq!(counter_subs_post, counter_subs_pre + 1);
        assert_eq!(tax.last_updated_node().unwrap(), id_nagetiere);
        assert_eq!(
            *tax._get_node_opt(super_id.clone()).unwrap().subs().back().unwrap(),
            id_nagetiere
        )
    }

    #[test]
    fn _add_root_node_to_empty_tax() {
        let mut tax = setup_tax_empty();
        let counter = 2;

        let c_animal = Concept::new("Animal");
        let id_animal = c_animal.id();

        let c_plant = Concept::new("Plant");
        let id_plant = c_plant.id();

        tax._add_root_node(Node::new(c_animal));
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_animal));

        tax._add_root_node(Node::new(c_plant));
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_plant));

        assert_eq!(tax.nodes.len(), counter);
        assert_eq!(tax.nodes.get(&id_animal).unwrap().id(), Rc::new(id_animal));
        assert_eq!(tax.nodes.get(&id_plant).unwrap().id(), Rc::new(id_plant));

        assert_eq!(tax.node0.len(), counter);
        assert_eq!(tax.node0.iter().nth(0).unwrap().clone(), Rc::new(id_animal));
        assert_eq!(tax.node0.iter().nth(1).unwrap().clone(), Rc::new(id_plant));
    }

    #[test]
    fn _add_root_node_to_non_empty_tax() {
        let (mut tax, _, _) = setup_tax_animals();
        let counter_nodes = tax.nodes.len() + 1;
        let counter_node0 = tax.node0.len() + 1;

        let c_zoos = Concept::new("Zoos");
        let id_zoos = c_zoos.id();

        tax._add_root_node(Node::new(c_zoos));

        assert_eq!(tax.nodes.len(), counter_nodes);
        assert_eq!(tax.nodes.get(&id_zoos).unwrap().id(), Rc::new(id_zoos));

        assert_eq!(tax.node0.len(), counter_node0);
        assert_eq!(tax.node0.back().unwrap().clone(), Rc::new(id_zoos));
    }

    #[test]
    fn _append_at() {
        // Append a node as 1st sub-node to super-node.
        let (mut tax, _, list) = setup_tax_animals();
        let super_id = Rc::new(list.front().unwrap().0);
        let node_id = Rc::new(list.back().unwrap().0);
        let index = 0;
        assert_eq!(
            tax._get_node_opt(node_id.clone()).unwrap().supers().contains(&super_id),
            false
        );
        tax._append_at(super_id.clone(), node_id.clone(), index);
        assert_eq!(*tax._get_node_opt(super_id.clone()).unwrap().subs().front().unwrap(), node_id);
        assert_eq!(tax._get_node_opt(node_id).unwrap().supers().contains(&super_id), true);

        // Append a node in the middle of sub-nodes to super-node.
        let (mut tax, _, list) = setup_tax_animals();
        let super_id = Rc::new(list.front().unwrap().0);
        let node_id = Rc::new(list.back().unwrap().0);
        let index = tax._get_node_opt(super_id.clone()).unwrap().subs().len() / 2;
        assert_eq!(
            tax._get_node_opt(node_id.clone()).unwrap().supers().contains(&super_id),
            false
        );
        tax._append_at(super_id.clone(), node_id.clone(), index);
        assert_eq!(
            *tax._get_node_opt(super_id.clone()).unwrap().subs().iter().nth(index).unwrap(),
            node_id
        );
        assert_eq!(tax._get_node_opt(node_id).unwrap().supers().contains(&super_id), true);

        // Append a node as last sub-node to super-node.
        let (mut tax, _, list) = setup_tax_animals();
        let super_id = Rc::new(list.front().unwrap().0);
        let node_id = Rc::new(list.back().unwrap().0);
        let index = tax._get_node_opt(super_id.clone()).unwrap().subs().len();
        assert_eq!(
            tax._get_node_opt(node_id.clone()).unwrap().supers().contains(&super_id),
            false
        );
        tax._append_at(super_id.clone(), node_id.clone(), index);
        assert_eq!(*tax._get_node_opt(super_id.clone()).unwrap().subs().back().unwrap(), node_id);
        assert_eq!(tax._get_node_opt(node_id).unwrap().supers().contains(&super_id), true);

        // Append a node as last sub-node to super-node (index out of bounds)
        let (mut tax, _, list) = setup_tax_animals();
        let super_id = Rc::new(list.front().unwrap().0);
        let node_id = Rc::new(list.back().unwrap().0);
        let index = 1000;
        assert_eq!(
            tax._get_node_opt(node_id.clone()).unwrap().supers().contains(&super_id),
            false
        );
        tax._append_at(super_id.clone(), node_id.clone(), index);
        assert_eq!(*tax._get_node_opt(super_id.clone()).unwrap().subs().back().unwrap(), node_id);
        assert_eq!(tax._get_node_opt(node_id).unwrap().supers().contains(&super_id), true);
    }

    #[test]
    fn _append_root_at() {
        // Append a non-root node as 1st root-node to taxonomy.
        let (mut tax, ids, _) = setup_tax_animals();
        let node_id = Rc::new(ids.get("Haustiere").unwrap().clone());
        let index = 0;
        assert_eq!(tax._get_node_opt(node_id.clone()).unwrap().is_root(), false);
        tax._append_root_at(node_id.clone(), index);
        assert_eq!(*tax.node0.front().unwrap(), node_id);
        assert_eq!(tax._get_node_opt(node_id).unwrap().is_root(), true);

        // Append a non-root node in the middle of existing root-nodes.
        let (mut tax, ids, _) = setup_tax_animals();
        let node_id = Rc::new(ids.get("Haustiere").unwrap().clone());
        let index = tax.node0.len() / 2;
        assert_eq!(tax._get_node_opt(node_id.clone()).unwrap().is_root(), false);
        tax._append_root_at(node_id.clone(), index);
        assert_eq!(*tax.node0.iter().nth(index).unwrap(), node_id);
        assert_eq!(tax._get_node_opt(node_id).unwrap().is_root(), true);

        // Append a non-root node as last root-node to taxonomy.
        let (mut tax, ids, _) = setup_tax_animals();
        let node_id = Rc::new(ids.get("Haustiere").unwrap().clone());
        let index = tax.node0.len();
        assert_eq!(tax._get_node_opt(node_id.clone()).unwrap().is_root(), false);
        tax._append_root_at(node_id.clone(), index);
        assert_eq!(*tax.node0.back().unwrap(), node_id);
        assert_eq!(tax._get_node_opt(node_id).unwrap().is_root(), true);

        // Append a non-root node as last root-node to taxonomy (index out of bounds).
        let (mut tax, ids, _) = setup_tax_animals();
        let node_id = Rc::new(ids.get("Haustiere").unwrap().clone());
        let index = 1000;
        assert_eq!(tax._get_node_opt(node_id.clone()).unwrap().is_root(), false);
        tax._append_root_at(node_id.clone(), index);
        assert_eq!(*tax.node0.back().unwrap(), node_id);
        assert_eq!(tax._get_node_opt(node_id).unwrap().is_root(), true);
    }

    #[test]
    fn _enumerate_subs() {
        let (tax, ids, _) = setup_tax_animals();

        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Tiere").unwrap().clone())).len(), 17);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Tierheime").unwrap().clone())).len(), 4);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Haustiere").unwrap().clone())).len(), 5);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Nutztiere").unwrap().clone())).len(), 4);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Zootiere").unwrap().clone())).len(), 2);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Säugetiere").unwrap().clone())).len(), 6);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Vögel").unwrap().clone())).len(), 2);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Geflügel").unwrap().clone())).len(), 2);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Rind").unwrap().clone())).len(), 0);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Fische").unwrap().clone())).len(), 0);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Hunde").unwrap().clone())).len(), 2);
        assert_eq!(tax._enumerate_subs(Rc::new(ids.get("Katzen").unwrap().clone())).len(), 0);

        // Unknown Id
        assert_eq!(tax._enumerate_subs(Rc::new(Uuid::new_v4())).len(), 0);
    }

    #[test]
    fn _err_duplicate_node() {
        let (tax, _, list) = setup_tax_animals();

        // Error with existing root-node
        let node_id = tax.node0.front().unwrap().clone();
        let result = tax._err_duplicate_node(node_id.clone()).err();
        let expectation = DuplicateNode(node_id);
        assert_eq!(result, Some(expectation));

        // Error with existing root-node
        let node_id = Rc::new(list.iter().nth(1).unwrap().0);
        let result = tax._err_duplicate_node(node_id.clone()).err();
        let expectation = DuplicateNode(node_id);
        assert_eq!(result, Some(expectation));

        // Ok with arbitrary node
        let node_id = Rc::new(Uuid::new_v4());
        assert!(tax._err_duplicate_node(node_id).is_ok());
    }

    #[test]
    fn _err_duplicate_root_node() {
        let (tax, _, list) = setup_tax_animals();

        // Error with existing root-node
        let node_id = tax.node0.front().unwrap().clone();
        let result = tax._err_duplicate_root_node(node_id.clone()).err();
        let expectation = DuplicateRootNode(node_id);
        assert_eq!(result, Some(expectation));

        // Ok with existing non-root-node
        let node_id = Rc::new(list.iter().nth(1).unwrap().0);
        assert!(tax._err_duplicate_root_node(node_id).is_ok());
    }

    #[test]
    fn _err_duplicate_sub_node() {
        let (tax, _, list) = setup_tax_animals();

        // Error with existing super- / sub-node
        let super_id = Rc::new(list.iter().nth(0).unwrap().0);
        let node_id = Rc::new(list.iter().nth(1).unwrap().0);
        let result = tax._err_duplicate_sub_node(super_id.clone(), node_id.clone()).err();
        let expectation = DuplicateSubNode(super_id, node_id);
        assert_eq!(result, Some(expectation));

        // Ok with existing super-node and an indirect sub-node
        let super_id = Rc::new(list.iter().nth(0).unwrap().0);
        let node_id = Rc::new(list.iter().nth(4).unwrap().0);
        assert!(tax._err_duplicate_sub_node(super_id.clone(), node_id.clone()).is_ok());
    }

    #[test]
    fn _err_duplicate_edge() {
        let (tax, _, list) = setup_tax_animals();

        // Edge with existing root-node
        let super_id = None;
        let node_id = Rc::new(list.front().unwrap().0);
        let edge = Edge::new(super_id.clone(), node_id.clone());
        let result = tax._err_duplicate_edge(&edge).err();
        let expectation = DuplicateEdge(super_id, node_id);
        assert_eq!(result, Some(expectation));

        // Edge with existing super-node and sub-node
        let super_id = Rc::new(list.iter().nth(0).unwrap().0);
        let node_id = Rc::new(list.iter().nth(1).unwrap().0);
        let edge = Edge::new(Some(super_id.clone()), node_id.clone());
        let result = tax._err_duplicate_edge(&edge).err();
        let expectation = DuplicateEdge(Some(super_id), node_id);
        assert_eq!(result, Some(expectation));
    }

    #[test]
    fn _err_edge_not_found() {
        let (tax, _, list) = setup_tax_animals();

        // Edge with a non-existing root-node
        let super_id = None;
        let node_id = Rc::new(Uuid::new_v4());
        let edge = Edge::new(super_id, node_id.clone());
        let result = tax._err_edge_not_found(&edge).err();
        let expectation = EdgeNotFound(None, node_id);
        assert_eq!(result, Some(expectation));

        // Edge with existing root-node
        let edge = Edge::new(None, Rc::new(list.front().unwrap().0));
        assert!(tax._err_edge_not_found(&edge).is_ok());

        // Edge with existing super-node and sub-node
        let super_id = Rc::new(list.iter().nth(0).unwrap().0);
        let node_id = Rc::new(list.iter().nth(1).unwrap().0);
        let edge = Edge::new(Some(super_id), node_id);
        assert!(tax._err_edge_not_found(&edge).is_ok());

        // Edge with existing super-node and an indirect sub-node
        let super_id = Rc::new(list.iter().nth(0).unwrap().0);
        let node_id = Rc::new(list.iter().nth(4).unwrap().0);
        let edge = Edge::new(Some(super_id.clone()), node_id.clone());
        let result = tax._err_edge_not_found(&edge).err();
        let expectation = EdgeNotFound(Some(super_id), node_id);
        assert_eq!(result, Some(expectation));

        // Edge with existing super-node and a coordinate node
        let super_id = Rc::new(list.iter().nth(0).unwrap().0);
        let node_id = Rc::new(list.back().unwrap().0);
        let edge = Edge::new(Some(super_id.clone()), node_id.clone());
        let result = tax._err_edge_not_found(&edge).err();
        let expectation = EdgeNotFound(Some(super_id), node_id);
        assert_eq!(result, Some(expectation));

        // Edge with a non-existing super-node and an arbitrary sub-node
        let super_id = Rc::new(Uuid::new_v4());
        let node_id = Rc::new(list.iter().nth(1).unwrap().0);
        let edge = Edge::new(Some(super_id.clone()), node_id.clone());
        let result = tax._err_edge_not_found(&edge).err();
        let expectation = EdgeNotFound(Some(super_id), node_id);
        assert_eq!(result, Some(expectation));
    }

    #[test]
    fn _err_loop_detected() {
        let (tax, ids, _) = setup_tax_animals();

        // Append the same element to itself
        let id = Rc::new(ids.get("Schäferhunde").unwrap().clone());
        let expectation = LoopDetected(id.clone());
        assert_eq!(
            tax._err_loop_detected(id.clone(), id.clone(), None).err().unwrap(),
            expectation
        );

        // Append a node's direct super-node to itself as sub-node
        let id_hunde = Rc::new(ids.get("Hunde").unwrap().clone());
        let id_saeugetiere = Rc::new(ids.get("Säugetiere").unwrap().clone());
        let expectation = LoopDetected(id_saeugetiere.clone());
        assert_eq!(
            tax._err_loop_detected(id_hunde.clone(), id_saeugetiere.clone(), None)
                .err()
                .unwrap(),
            expectation
        );

        // Append a node's indirect super-node to itself as sub-node
        let id_hunde = Rc::new(ids.get("Hunde").unwrap().clone());
        let id_tiere = Rc::new(ids.get("Tiere").unwrap().clone());
        let expectation = LoopDetected(id_tiere.clone());
        assert_eq!(tax._err_loop_detected(id_hunde, id_tiere, None).err().unwrap(), expectation);

        // Append a node's coordinate node to itself as sub-node
        let id_haustiere = Rc::new(ids.get("Haustiere").unwrap().clone());
        let id_saeugetiere = Rc::new(ids.get("Säugetiere").unwrap().clone());
        assert!(tax._err_loop_detected(id_haustiere, id_saeugetiere, None).is_ok());

        // Append a node to another super-node
        let id_zootiere = Rc::new(ids.get("Zootiere").unwrap().clone());
        let id_hunde = Rc::new(ids.get("Hunde").unwrap().clone());
        assert!(tax._err_loop_detected(id_zootiere, id_hunde, None).is_ok());
    }

    #[test]
    fn _err_node_has_sub() {
        let (tax, _, list) = setup_tax_animals();

        let super_id = tax.last_updated_node().unwrap();
        let super_node = tax.nodes.get(&super_id).unwrap();
        assert!(tax._err_node_has_sub(super_node).is_ok());

        let super_id = Rc::new(list.front().unwrap().0);
        let super_node = tax.nodes.get(&super_id).unwrap();
        let result = tax._err_node_has_sub(super_node).err();
        let expectation = Some(NodeHasSubNode(super_id));
        assert_eq!(result, expectation);
    }

    #[test]
    fn _err_node_not_found() {
        let (tax, _, _) = setup_tax_animals();

        let node_id = tax.last_updated_node().unwrap();
        assert!(tax._err_node_not_found(node_id).is_ok());

        let node_id = Rc::new(Uuid::new_v4());
        let result = tax._err_node_not_found(node_id.clone()).err();
        let expectation = Some(NodeNotFound(node_id));
        assert_eq!(result, expectation);
    }

    #[test]
    fn _get_node_id_from_cursor() {
        let (mut tax, _, list) = setup_tax_animals();

        let (super_id, node_id) = tax._get_node_id_from_cursor();
        assert_eq!(super_id, None);
        assert_eq!(node_id, None);

        tax._next();
        let (super_id, node_id) = tax._get_node_id_from_cursor();
        assert_eq!(super_id, None);
        assert_eq!(node_id, Some(Rc::new(list.iter().nth(0).unwrap().0)));

        tax._next();
        let (super_id, node_id) = tax._get_node_id_from_cursor();
        assert_eq!(super_id, Some(Rc::new(list.iter().nth(0).unwrap().0)));
        assert_eq!(node_id, Some(Rc::new(list.iter().nth(1).unwrap().0)));
    }

    #[test]
    fn _get_node_opt() {
        let (tax, _, _) = setup_tax_animals();

        let node_id = tax.last_updated_node().unwrap();
        assert!(tax._get_node_opt(node_id).is_some());

        let node_id = Rc::new(Uuid::new_v4());
        assert!(tax._get_node_opt(node_id).is_none());
    }

    #[test]
    fn _get_node_res() {
        let (tax, _, _) = setup_tax_animals();

        let node_id = tax.last_updated_node().unwrap();
        assert!(tax._get_node_res(node_id).is_ok());

        let node_id = Rc::new(Uuid::new_v4());
        let result = tax._get_node_res(node_id.clone()).err();
        let expectation = Some(NodeNotFound(node_id));
        assert_eq!(result, expectation);
    }

    #[test]
    fn _get_node_mut_opt() {
        let (mut tax, _, _) = setup_tax_animals();

        let node_id = tax.last_updated_node().unwrap();
        assert!(tax._get_node_mut_opt(node_id).is_some());

        let node_id = Rc::new(Uuid::new_v4());
        assert!(tax._get_node_mut_opt(node_id).is_none());
    }

    #[test]
    fn _get_node_mut_res() {
        let (mut tax, _, _) = setup_tax_animals();

        let node_id = tax.last_updated_node().unwrap();
        assert!(tax._get_node_mut_res(node_id).is_ok());

        let node_id = Rc::new(Uuid::new_v4());
        let result = tax._get_node_mut_res(node_id.clone()).err();
        let expectation = Some(NodeNotFound(node_id));
        assert_eq!(result, expectation);
    }

    #[test]
    fn _get_root_node_id_at() {
        let (mut tax, _, list) = setup_tax_animals();

        // In example taxonomy first and last nodes are root-nodes
        assert_eq!(*tax._get_root_node_id_at(0).unwrap(), list.front().unwrap().1.id());
        assert_eq!(
            *tax._get_root_node_id_at(tax.node0.len() - 1).unwrap(),
            list.back().unwrap().1.id()
        );

        let c_pflanzen = Concept::new("Pflanzen");
        let id_pflanzen = c_pflanzen.id();
        let _ = tax.add(None, c_pflanzen);
        assert_eq!(*tax._get_root_node_id_at(tax.node0.len() - 1).unwrap(), id_pflanzen);

        assert_eq!(tax._get_root_node_id_at(tax.node0.len()), None);
    }

    #[test]
    fn _next() {
        let (mut tax, _, list) = setup_tax_animals();

        let mut n = 0;
        while let Some(node_id) = tax._next() {
            if let Some(element) = tax.get(node_id) {
                assert_eq!(*element, list.iter().nth(n).unwrap().1);
                n += 1;
            }
        }

        assert_eq!(list.len(), n);
    }

    #[test]
    fn _post_update() {
        let (mut tax, _, _) = setup_tax_animals();

        let c_pflanzen = Concept::new("Pflanzen");
        let id_pflanzen = c_pflanzen.id();

        let c_blumen = Concept::new("Blumen");
        let id_blumen = c_blumen.id();

        let c_baeume = Concept::new("Bäume");
        let id_baeume = c_baeume.id();

        let c_graeser = Concept::new("Graeser");
        let id_graeser = c_graeser.id();

        let _ = tax.add(None, c_pflanzen);
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_pflanzen));
        let _ = tax.add(None, c_blumen);
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_blumen));

        let _ = tax.add(None, c_baeume);
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_baeume));
        let _ = tax.add(None, c_graeser);
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_graeser));

        let _ = tax.append(Some(id_pflanzen.clone()), id_baeume);
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_baeume));
        let _ = tax.append_at(Some(id_pflanzen.clone()), id_blumen, 0);
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_blumen));

        let _ = tax.move_to(Rc::new(id_graeser), None, Some(Rc::new(id_pflanzen)), 2).unwrap();
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_graeser));

        let _ = tax.remove(Rc::new(id_graeser));
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_graeser));

        let _ = tax.remove_from(Edge::new(None, Rc::new(id_blumen)));
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_blumen));

        let _ = tax.remove_recursively(Rc::new(id_pflanzen));
        assert_eq!(tax.last_updated_node().unwrap(), Rc::new(id_pflanzen));
    }

    #[test]
    fn _pre_update() {
        // _pre_update does not do anything, so testing is skipped
        /*
        let (_tax, _, _) = setup_tax_animals();

        let c_pflanzen = Concept::new("Pflanzen");
        let id_pflanzen = c_pflanzen.id();

        let c_blumen = Concept::new("Blumen");
        let id_blumen = c_blumen.id();

        let c_baeume = Concept::new("Bäume");
        let id_baeume = c_baeume.id();

        let c_graeser = Concept::new("Graeser");
        let id_graeser = c_graeser.id();

        let _ = tax.add(None, c_pflanzen);
        let _ = tax.add(None, c_blumen);
        let _ = tax.add(None, c_baeume);
        let _ = tax.add(None, c_graeser);

        let _ = tax.append(Some(id_pflanzen.clone()), id_baeume);

        let _ = tax.append_at(Some(id_pflanzen.clone()), id_blumen, 0);

        let _ = tax.move_to(Rc::new(id_graeser), None, Some(Rc::new(id_pflanzen)), 2);

        let _ = tax.remove(Rc::new(id_graeser));

        let _ = tax.remove_from(Edge::new(None, Rc::new(id_blumen)));

        let _ = tax.remove_recursively(Rc::new(id_pflanzen));
         */
    }

    #[test]
    fn _remove_non_root_node() {
        let (mut tax, _, _) = setup_tax_animals();

        let num_root_nodes = tax.nodes.len();

        // Remove node with arbitrary id
        let node_id = Rc::new(Uuid::new_v4());
        tax._remove_non_root_node(node_id.clone());
        assert_eq!(tax.nodes.len(), num_root_nodes);
        assert_ne!(tax.last_updated_node().unwrap(), node_id);

        // Remove node
        let node_id = tax.node0.iter().last().unwrap().clone();
        tax._remove_non_root_node(node_id.clone());

        assert_eq!(tax.nodes.len(), num_root_nodes - 1);
        assert_eq!(tax.nodes.contains_key(&node_id), false);
        assert_eq!(tax.last_updated_node().unwrap(), node_id);
    }

    #[test]
    fn _remove_root_node() {
        let (mut tax, _, _) = setup_tax_animals();
        let min_len: usize = 4;

        let num_root_nodes = tax.node0.len();
        assert_eq!(num_root_nodes, min_len);

        let middle = (num_root_nodes as i32 / 2) as usize;

        // Remove root-node with arbitrary id
        let node_id = Rc::new(Uuid::new_v4());
        tax._remove_root_node(node_id.clone());
        assert_eq!(tax.node0.len(), num_root_nodes);
        assert_ne!(tax.last_updated_node().unwrap(), node_id);

        // Remove root-node from the middle
        let node_id = tax.node0.iter().nth(middle).unwrap().clone();
        let node_id_successor = tax.node0.iter().nth(middle + 1).unwrap().clone();
        tax._remove_root_node(node_id.clone());

        assert_eq!(tax.node0.len(), num_root_nodes - 1);
        assert_eq!(tax.node0.contains(&node_id), false);
        assert_eq!(tax.nodes.contains_key(&node_id), false);
        assert_eq!(tax.node0.iter().nth(middle).unwrap().clone(), node_id_successor);
        assert_eq!(tax.last_updated_node().unwrap(), node_id);

        // Remove root-node from the back
        let node_id = tax.node0.iter().last().unwrap().clone();
        let node_id_predecessor = tax.node0.iter().nth(tax.node0.len() - 2).unwrap().clone();
        tax._remove_root_node(node_id.clone());

        assert_eq!(tax.node0.len(), num_root_nodes - 2);
        assert_eq!(tax.node0.contains(&node_id), false);
        assert_eq!(tax.nodes.contains_key(&node_id), false);
        assert_eq!(tax.node0.iter().last().unwrap().clone(), node_id_predecessor);
        assert_eq!(tax.last_updated_node().unwrap(), node_id);

        // Remove root-node from the front
        let node_id = tax.node0.iter().nth(0).unwrap().clone();
        let node_id_successor = tax.node0.iter().nth(1).unwrap().clone();
        tax._remove_root_node(node_id.clone());

        assert_eq!(tax.node0.len(), num_root_nodes - 3);
        assert_eq!(tax.node0.contains(&node_id), false);
        assert_eq!(tax.nodes.contains_key(&node_id), false);
        assert_eq!(tax.node0.iter().nth(0).unwrap().clone(), node_id_successor);
        assert_eq!(tax.last_updated_node().unwrap(), node_id);
    }
}
