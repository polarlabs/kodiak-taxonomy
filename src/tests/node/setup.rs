use crate::node::Node;
use crate::tests::node::Concept;
use crate::Identity;

use uuid::Uuid;

use std::rc::Rc;

///
/// /
/// └── Organisation
pub(crate) fn setup_root_only_node() -> Node<Uuid, Concept> {
    let org = Concept::new("Organisation");
    Node::new(org)
}

///
/// /
/// ├── Organisation
///     └── Department
pub(crate) fn setup_root_node_with_one_sub() -> Node<Uuid, Concept> {
    let org = Concept::new("Organisation");
    let dep = Concept::new("Department");

    let mut node = Node::new(org);

    node.append_sub(Rc::new(dep.id()));
    node
}

///
/// /
/// ├── Employee
/// └── Organisation
///     ├── Department
///     └── Employee (duplicate)
pub(crate) fn setup_two_root_nodes_with_first_being_sub_of_second(
) -> (Node<Uuid, Concept>, Node<Uuid, Concept>) {
    let empl = Concept::new("Employee");
    let empl_id = empl.id();
    let org = Concept::new("Organisation");
    let org_id = org.id();
    let dep = Concept::new("Department");

    let mut root_node1 = Node::new(empl);

    let mut root_node2 = Node::new(org);
    root_node2.append_sub(Rc::new(dep.id()));

    root_node1.add_super(Some(Rc::new(org_id.clone())));
    root_node2.append_sub(Rc::new(empl_id));

    (root_node1, root_node2)
}

///
/// /
/// ├── Devices
///     ├── Clients
///     ├── Firewalls
///     ├── Mobiles
///     └── Servers
pub(crate) fn setup_root_node_with_four_subs() -> (Node<Uuid, Concept>, Vec<Rc<Uuid>>) {
    let dev = Concept::new("Devices");

    let cli = Concept::new("Clients");
    let cli_id = Rc::new(cli.id());

    let fire = Concept::new("Firewalls");
    let fire_id = Rc::new(fire.id());

    let mob = Concept::new("Mobiles");
    let mob_id = Rc::new(mob.id());

    let srv = Concept::new("Servers");
    let srv_id = Rc::new(srv.id());

    let mut node = Node::new(dev);
    node.append_sub(cli_id.clone());
    node.append_sub(fire_id.clone());
    node.append_sub(mob_id.clone());
    node.append_sub(srv_id.clone());

    let v = vec![cli_id, fire_id, mob_id, srv_id];
    (node, v)
}

///
/// /
/// ├── Devices
///     ├── Clients
///     ├── Firewalls
///     ├── Mobiles
///     └── Servers
pub(crate) fn setup_root_node_with_four_subs_in_vec(
) -> (Node<Uuid, Concept>, Vec<Node<Uuid, Concept>>) {
    let dev = Concept::new("Devices");
    let dev_id = Rc::new(dev.id());

    let cli = Concept::new("Clients");
    let cli_id = Rc::new(cli.id());

    let fire = Concept::new("Firewalls");
    let fire_id = Rc::new(fire.id());

    let mob = Concept::new("Mobiles");
    let mob_id = Rc::new(mob.id());

    let srv = Concept::new("Servers");
    let srv_id = Rc::new(srv.id());

    let mut node = Node::new(dev);
    node.append_sub(cli_id.clone());
    node.append_sub(fire_id.clone());
    node.append_sub(mob_id.clone());
    node.append_sub(srv_id.clone());

    let mut cli_node = Node::new(cli);
    cli_node.remove_super(None);
    cli_node.add_super(Some(dev_id.clone()));

    let mut fire_node = Node::new(fire);
    fire_node.remove_super(None);
    fire_node.add_super(Some(dev_id.clone()));

    let mut mob_node = Node::new(mob);
    mob_node.remove_super(None);
    mob_node.add_super(Some(dev_id.clone()));

    let mut srv_node = Node::new(srv);
    srv_node.remove_super(None);
    srv_node.add_super(Some(dev_id));

    let v = vec![cli_node, fire_node, mob_node, srv_node];
    (node, v)
}
