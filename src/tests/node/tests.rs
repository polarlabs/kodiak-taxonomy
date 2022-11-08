#[cfg(test)]
mod tests {
    use crate::tests::node::*;
    use crate::Identity;
    use std::rc::Rc;

    #[test]
    fn test_root_only_node() {
        let node = setup_root_only_node();

        assert_eq!(node.subs().len(), 0);
        assert_eq!(node.count_subs(), 0);
        assert_eq!(node.has_sub(), false);

        assert_eq!(node.supers().len(), 0);
        assert_eq!(node.count_super(), 1);
        assert_eq!(node.has_super(), true);

        assert_eq!(node.is_root(), true);
    }

    #[test]
    fn test_root_node_with_one_sub() {
        let node = setup_root_node_with_one_sub();

        assert_eq!(node.subs().len(), 1);
        assert_eq!(node.count_subs(), 1);
        assert_eq!(node.has_sub(), true);

        assert_eq!(node.supers().len(), 0);
        assert_eq!(node.count_super(), 1);
        assert_eq!(node.has_super(), true);

        assert_eq!(node.is_root(), true);
    }

    #[test]
    fn test_two_root_nodes_with_first_being_sub_of_second() {
        let (root_node1, root_node2) = setup_two_root_nodes_with_first_being_sub_of_second();

        assert_eq!(root_node1.subs().len(), 0);
        assert_eq!(root_node1.count_subs(), 0);
        assert_eq!(root_node1.has_sub(), false);

        assert_eq!(root_node1.supers().len(), 1);
        assert_eq!(root_node1.count_super(), 2);
        assert_eq!(root_node1.has_super(), true);

        assert_eq!(root_node1.is_root(), true);

        assert_eq!(root_node2.subs().len(), 2);
        assert_eq!(root_node2.count_subs(), 2);
        assert_eq!(root_node2.has_sub(), true);

        assert_eq!(root_node2.supers().len(), 0);
        assert_eq!(root_node2.count_super(), 1);
        assert_eq!(root_node2.has_super(), true);

        assert_eq!(root_node2.is_root(), true);

        // Check if last sub of root_node2 is root_node1
        assert_eq!(*root_node2.subs().back().unwrap(), root_node1.id)
    }

    #[test]
    fn test_root_node_with_four_subs() {
        let (node, ids) = setup_root_node_with_four_subs();

        assert_eq!(node.subs().len(), 4);
        assert_eq!(node.count_subs(), 4);
        assert_eq!(node.has_sub(), true);

        let mut k: usize = 0;
        for sub in node.subs() {
            assert_eq!(*sub, ids[k]);
            k += 1;
        }
    }

    #[test]
    fn test_root_node_with_four_subs_append_sub_ok() {
        let (mut node, _) = setup_root_node_with_four_subs();

        let sw = Concept::new("Switches");
        let sw_id = Rc::new(sw.id());

        node.append_sub(sw_id.clone());
        assert_eq!(node.subs().len(), 5);
        assert_eq!(node.count_subs(), 5);
        assert_eq!(node.has_sub(), true);
        assert_eq!(*node.subs().back().unwrap(), sw_id);
    }

    #[test]
    fn test_root_node_with_four_subs_append_sub_at_pos0_ok() {
        let (mut node, _) = setup_root_node_with_four_subs();

        let sw = Concept::new("Switches");
        let sw_id = Rc::new(sw.id());

        node.append_sub_at(sw_id.clone(), 0);
        assert_eq!(*node.subs().front().unwrap(), sw_id);
    }

    #[test]
    fn test_root_node_with_four_subs_append_sub_at_index2_ok() {
        let (mut node, ids) = setup_root_node_with_four_subs();
        let index: usize = 2;

        let sw = Concept::new("Switches");
        let sw_id = Rc::new(sw.id());

        node.append_sub_at(sw_id.clone(), index);
        assert_eq!(*node.subs().iter().nth(index).unwrap(), sw_id);

        // Check order of sub-nodes
        let mut k: usize = 0;
        let mut i: usize = 0;
        for sub in node.subs() {
            if k == index {
                assert_eq!(*sub, sw_id);
            } else {
                assert_eq!(*sub, ids[i]);
                i += 1;
            }
            k += 1;
        }
    }

    #[test]
    fn test_root_node_with_four_subs_append_sub_at_index_matching_len_ok() {
        let (mut node, ids) = setup_root_node_with_four_subs();
        let pos: usize = node.subs().len();

        let sw = Concept::new("Switches");
        let sw_id = Rc::new(sw.id());

        node.append_sub_at(sw_id.clone(), pos);
        assert_eq!(*node.subs().iter().nth(node.subs().len() - 1).unwrap(), sw_id);

        // Check order of sub-nodes
        let mut k: usize = 0;
        let mut i: usize = 0;
        for sub in node.subs() {
            if k == pos {
                assert_eq!(*sub, sw_id);
            } else {
                assert_eq!(*sub, ids[i]);
                i += 1;
            }
            k += 1;
        }
    }

    #[test]
    fn test_root_node_with_four_subs_prepend_sub_ok() {
        let (mut node, _) = setup_root_node_with_four_subs();

        let sw = Concept::new("Switches");
        let sw_id = Rc::new(sw.id());

        node.prepend_sub(sw_id.clone());
        assert_eq!(node.subs().len(), 5);
        assert_eq!(node.count_subs(), 5);
        assert_eq!(node.has_sub(), true);
        assert_eq!(*node.subs().front().unwrap(), sw_id);
    }

    #[test]
    fn test_root_node_with_four_subs_is_root_ok() {
        let (root_node, sub_nodes) = setup_root_node_with_four_subs_in_vec();

        assert_eq!(root_node.is_root(), true);
        for sub in sub_nodes {
            assert_eq!(sub.is_root(), false);
        }
    }

    #[test]
    fn test_root_node_with_four_subs_remove_sub_ok() {
        let (mut root_node, sub_nodes) = setup_root_node_with_four_subs_in_vec();
        let seq = vec![2, 3, 0, 1]; // Removes from middle pos, back pos, front pos and last pos
        let mut k: usize = 0;

        while k <= 3 {
            let node_id = sub_nodes.iter().nth(seq[k]).unwrap().id.clone();
            root_node.remove_sub(node_id);

            assert_eq!(root_node.subs().len(), 3 - k);
            assert_eq!(root_node.count_subs(), 3 - k);
            assert_eq!(root_node.has_sub(), k < 3);

            k += 1;
        }
    }

    #[test]
    fn remove_super() {
        let (mut root_node, mut sub_nodes) = setup_root_node_with_four_subs_in_vec();
        let seq = vec![2, 3, 0, 1]; // Removes from middle pos, back pos, front pos and last pos
        let mut k: usize = 0;

        while k <= 3 {
            let sub_node = sub_nodes.iter_mut().nth(seq[k]).unwrap();
            let root_node_id = root_node.id();
            sub_node.remove_super(Some(root_node_id));

            assert_eq!(sub_node.count_super(), 0);

            k += 1;
        }

        root_node.remove_super(None);
        assert_eq!(root_node.has_super(), false);
    }

    #[test]
    fn test_sub_at() {
        let (node, ids) = setup_root_node_with_four_subs();

        let mut i: usize = 0;
        for id in ids {
            assert_eq!(node.sub_at(i), Some(id));
            i += 1;
        }
        assert_eq!(node.sub_at(node.count_subs()), None);
    }
}
