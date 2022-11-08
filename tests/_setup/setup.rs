use kodiak_taxonomy::{Identity, Taxonomy};
use std::collections::{HashMap, LinkedList};

use crate::Concept;

use uuid::Uuid;

pub fn setup_tax_empty() -> Taxonomy<Uuid, Concept> {
    let tax: Taxonomy<Uuid, Concept> = Taxonomy::new();
    tax
}

// Setups two root nodes, no sub nodes
// |-Animal
// |-Plant
pub fn two_root(tax: &mut Taxonomy<Uuid, Concept>) -> LinkedList<Uuid> {
    let mut list = LinkedList::new();

    let element = Concept::new("Animal");
    let id = element.id();
    let _ = tax.add(None, element);
    list.push_back(id);

    let element = Concept::new("Plant");
    let id = element.id();
    let _ = tax.add(None, element);
    list.push_back(id);

    list
}

// Setups two root nodes and one sub node with the two root nodes as super nodes
// |-Network Device
// |   |-Firewall
// |-Security Device
//     |-Firewall
pub fn sub_with_two_super(tax: &mut Taxonomy<Uuid, Concept>) -> LinkedList<Uuid> {
    let mut list = LinkedList::new();

    let element = Concept::new("Network Device");
    let super1a_id = element.id();
    let _ = tax.add(None, element);
    list.push_back(super1a_id.clone());

    let sub_node = Concept::new("Firewall");
    let sub_id = sub_node.id();
    let _ = tax.add(Some(super1a_id), sub_node);
    list.push_back(sub_id.clone());

    let element = Concept::new("Security Device");
    let super1b_id = element.id();
    let _ = tax.add(None, element);
    list.push_back(super1b_id.clone());

    // Add Firewall a second time
    let _ = tax.append(Some(super1b_id), sub_id);
    list.push_back(sub_id.clone());

    list
}

// Setups two root nodes with two sub nodes each, one level
// |-Device
// |   |-Network Device
// |   |-Security Device
// |-Organisation
//    |-Department
//    |-User
pub fn two_root_with_two_sub_each(tax: &mut Taxonomy<Uuid, Concept>) -> LinkedList<Uuid> {
    let mut list = LinkedList::new();

    let element = Concept::new("Device");
    let root1_id = element.id();
    let _ = tax.add(None, element);
    list.push_back(root1_id.clone());

    let element = Concept::new("Network Device");
    let super1a_id = element.id();
    let _ = tax.add(Some(root1_id), element);
    list.push_back(super1a_id.clone());

    let element = Concept::new("Security Device");
    let super1b_id = element.id();
    let _ = tax.add(Some(root1_id), element);
    list.push_back(super1b_id.clone());

    let element = Concept::new("Organisation");
    let root2_id = element.id();
    let _ = tax.add(None, element);
    list.push_back(root2_id.clone());

    let element = Concept::new("Department");
    let super2a_id = element.id();
    let _ = tax.add(Some(root2_id), element);
    list.push_back(super2a_id.clone());

    let element = Concept::new("User");
    let super2b_id = element.id();
    let _ = tax.add(Some(root2_id), element);
    list.push_back(super2b_id.clone());

    list
}

// Setups two root nodes with several sub nodes which have again sub nodes
// |-Device
// |   |-Network Device
// |   |-Security Device
// |      |-Firewall
// |-Organisation
//    |-User
pub fn two_root_with_super_and_sub(tax: &mut Taxonomy<Uuid, Concept>) -> LinkedList<Uuid> {
    let mut list = LinkedList::new();

    let element = Concept::new("Device");
    let root1_id = element.id();
    let _ = tax.add(None, element);
    list.push_back(root1_id.clone());

    let element = Concept::new("Network Device");
    let super1a_id = element.id();
    let _ = tax.add(Some(root1_id), element);
    list.push_back(super1a_id.clone());

    let element = Concept::new("Security Device");
    let super1b_id = element.id();
    let _ = tax.add(Some(root1_id), element);
    list.push_back(super1b_id.clone());

    let element = Concept::new("Firewall");
    let sub_id = element.id();
    let _ = tax.add(Some(super1b_id), element);
    list.push_back(sub_id.clone());

    let element = Concept::new("Organisation");
    let root2_id = element.id();
    let _ = tax.add(None, element);
    list.push_back(root2_id.clone());

    let element = Concept::new("User");
    let super2a_id = element.id();
    let _ = tax.add(Some(root2_id), element);
    list.push_back(super2a_id.clone());

    list
}

// Setups two root nodes with several sub nodes which have again sub nodes
// |-CRM
// |   |-Customer
// |   |-Contact
// |-CMDB
// |   |-Device
// |   |   |-Network Device
// |   |   |-Security Device
// |   |      |-Firewall
// |   |-Organisation
// |      |-User
// |-SRM
// |-HRM
//    |-User
pub fn four_root_with_super_and_sub(tax: &mut Taxonomy<Uuid, Concept>) -> LinkedList<Uuid> {
    let mut list = LinkedList::new();

    let element = Concept::new("CRM");
    let crm_id = element.id();
    let _ = tax.add(None, element);
    list.push_back(crm_id.clone());

    let element = Concept::new("Customer");
    let customer_id = element.id();
    let _ = tax.add(Some(crm_id), element);
    list.push_back(customer_id.clone());

    let element = Concept::new("Contact");
    let contact_id = element.id();
    let _ = tax.add(Some(crm_id), element);
    list.push_back(contact_id.clone());

    let element = Concept::new("CMDB");
    let cmdb_id = element.id();
    let _ = tax.add(None, element);
    list.push_back(cmdb_id.clone());

    let element = Concept::new("Device");
    let device_id = element.id();
    let _ = tax.add(Some(cmdb_id), element);
    list.push_back(device_id.clone());

    let element = Concept::new("Network Device");
    let net_device_id = element.id();
    let _ = tax.add(Some(device_id), element);
    list.push_back(net_device_id.clone());

    let element = Concept::new("Security Device");
    let sec_device_id = element.id();
    let _ = tax.add(Some(device_id), element);
    list.push_back(sec_device_id.clone());

    let element = Concept::new("Firewall");
    let firewall_id = element.id();
    let _ = tax.add(Some(sec_device_id), element);
    list.push_back(firewall_id.clone());

    /*
    let element = Concept::new("Organisation");
    let org_id = element.id();
    tax.append(element, cmdb_id);
    list.push_back(org_id.clone());
    */

    let user_element = Concept::new("User");
    let user_id = user_element.id();
    let _ = tax.add(Some(firewall_id), user_element);
    list.push_back(user_id.clone());

    let element = Concept::new("SRM");
    let srm_id = element.id();
    let _ = tax.add(None, element);
    list.push_back(srm_id.clone());

    let element = Concept::new("HRM");
    let hrm_id = element.id();
    let _ = tax.add(None, element);
    list.push_back(hrm_id.clone());

    let _ = tax.append(Some(hrm_id), user_id);
    list.push_back(user_id.clone());

    list
}

///
/// /
/// ├── Tiere
/// │   ├── Haustiere
/// │   │   ├── Fische
/// │   │   ├── Hunde
/// │   │   │   ├── Doggen
/// │   │   |   └── Schäferhunde
/// │   │   └── Katzen
/// │   ├── Nutztiere
/// │   │   ├── Geflügel
/// │   │   │   ├── Hühner
/// │   │   |   └── Puten
/// │   │   └── Rind
/// │   ├── Zootiere
/// │   │   ├── Affen
/// │   │   └── Schlangen
/// │   ├── Säugetiere
/// │   │   ├── Affen (duplicate)
/// │   │   ├── Hunde (duplicate)
/// │   │   |   └── ...
/// │   │   ├── Katzen (duplicate)
/// │   │   └── Waale & Delfine
/// │   └── Vögel
/// │       ├── Hühner (duplicate)
/// │       └── Puten (duplicate)
/// ├── Tierheime
///     ├── Hunde (duplicate)
///     │   └── ...
///     └── Katzen (duplicate)
///
pub(crate) fn setup_tax_animals() -> (Taxonomy<Uuid, Concept>, HashMap<&'static str, Uuid>, LinkedList<Concept>) {
    let mut tax = setup_tax_empty();
    let mut ids = HashMap::new();
    let mut list = LinkedList::new();

    // Level 1
    let c_tiere = Concept::new("Tiere");
    let id_tiere = c_tiere.id();
    ids.insert("Tiere", id_tiere);
    let c_tierheime = Concept::new("Tierheime");
    let id_tierheime = c_tierheime.id();
    ids.insert("Tierheime", id_tierheime);

    // Level 2
    let c_haustiere = Concept::new("Haustiere");
    let id_haustiere = c_haustiere.id();
    ids.insert("Haustiere", id_haustiere);
    let c_nutztiere = Concept::new("Nutztiere");
    let id_nutztiere = c_nutztiere.id();
    ids.insert("Nutztiere", id_nutztiere);
    let c_zootiere = Concept::new("Zootiere");
    let id_zootiere = c_zootiere.id();
    ids.insert("Zootiere", id_zootiere);

    let c_saeugetiere = Concept::new("Säugetiere");
    let id_saeugetiere = c_saeugetiere.id();
    ids.insert("Säugetiere", id_saeugetiere);
    let c_voegel = Concept::new("Vögel");
    let id_voegel = c_voegel.id();
    ids.insert("Vögel", id_voegel);

    // Level 3
    let c_gefluegel = Concept::new("Geflügel");
    let id_gefluegel = c_gefluegel.id();
    ids.insert("Geflügel", id_gefluegel);
    let c_rind = Concept::new("Rind");
    let id_rind = c_rind.id();
    ids.insert("Rind", id_rind);

    let c_fische = Concept::new("Fische");
    let id_fische = c_fische.id();
    ids.insert("Fische", id_fische);
    let c_hunde = Concept::new("Hunde");
    let id_hunde = c_hunde.id();
    ids.insert("Hunde", id_hunde);
    let c_katzen = Concept::new("Katzen");
    let id_katzen = c_katzen.id();
    ids.insert("Katzen", id_katzen);

    let c_affen = Concept::new("Affen");
    let id_affen = c_affen.id();
    ids.insert("Affen", id_affen);
    let c_schlangen = Concept::new("Schlangen");
    let id_schlangen = c_schlangen.id();
    ids.insert("Schlangen", id_schlangen);

    let c_waale = Concept::new("Waale & Delfine");
    let id_waale = c_waale.id();
    ids.insert("Waale & Delfine", id_waale);

    // Level 4
    let c_huehner = Concept::new("Hühner");
    let id_huehner = c_huehner.id();
    ids.insert("Hühne", id_huehner);
    let c_puten = Concept::new("Puten");
    let id_puten = c_puten.id();
    ids.insert("Puten", id_puten);

    let c_doggen = Concept::new("Doggen");
    let id_doggen = c_doggen.id();
    ids.insert("Doggen", id_doggen);
    let c_schaeferhunde = Concept::new("Schäferhunde");
    let id_schaeferhunde = c_schaeferhunde.id();
    ids.insert("Schäferhunde", id_schaeferhunde);

    let _ = tax.add(None, c_tiere.clone());

    let _ = tax.add(Some(id_tiere), c_haustiere.clone());

    let _ = tax.add(Some(id_haustiere), c_fische.clone());

    let _ = tax.add(Some(id_haustiere), c_hunde.clone());
    let _ = tax.add(Some(id_hunde), c_doggen.clone());
    let _ = tax.add(Some(id_hunde), c_schaeferhunde.clone());

    let _ = tax.add(Some(id_haustiere), c_katzen.clone());

    let _ = tax.add(Some(id_tiere), c_nutztiere.clone());

    let _ = tax.add(Some(id_nutztiere), c_gefluegel.clone());
    let _ = tax.add(Some(id_gefluegel), c_huehner.clone());
    let _ = tax.add(Some(id_gefluegel), c_puten.clone());

    let _ = tax.add(Some(id_nutztiere), c_rind.clone());

    let _ = tax.add(Some(id_tiere), c_zootiere.clone());
    let _ = tax.add(Some(id_zootiere), c_affen.clone());
    let _ = tax.add(Some(id_zootiere), c_schlangen.clone());

    let _ = tax.add(Some(id_tiere), c_saeugetiere.clone());
    let _ = tax.append(Some(id_saeugetiere), id_affen);
    let _ = tax.append(Some(id_saeugetiere), id_hunde);
    let _ = tax.append(Some(id_saeugetiere), id_katzen);
    let _ = tax.add(Some(id_saeugetiere), c_waale.clone());

    let _ = tax.add(Some(id_tiere), c_voegel.clone());
    let _ = tax.append(Some(id_voegel), id_huehner);
    let _ = tax.append(Some(id_voegel), id_puten);

    let _ = tax.add(None, c_tierheime.clone());
    let _ = tax.append(Some(id_tierheime), id_hunde);

    let _ = tax.append(Some(id_tierheime), id_katzen);

    // Traversal
    list.push_back(c_tiere.clone());
    list.push_back(c_haustiere.clone());
    list.push_back(c_fische.clone());
    list.push_back(c_hunde.clone());
    list.push_back(c_doggen.clone());
    list.push_back(c_schaeferhunde.clone());
    list.push_back(c_katzen.clone());
    list.push_back(c_nutztiere.clone());
    list.push_back(c_gefluegel.clone());
    list.push_back(c_huehner.clone());
    list.push_back(c_puten.clone());
    list.push_back(c_rind.clone());
    list.push_back(c_zootiere.clone());
    list.push_back(c_affen.clone());
    list.push_back(c_schlangen.clone());
    list.push_back(c_saeugetiere.clone());
    list.push_back(c_affen.clone());
    list.push_back(c_hunde.clone());
    list.push_back(c_doggen.clone());
    list.push_back(c_schaeferhunde.clone());
    list.push_back(c_katzen.clone());
    list.push_back(c_waale.clone());
    list.push_back(c_voegel.clone());
    list.push_back(c_huehner.clone());
    list.push_back(c_puten.clone());
    list.push_back(c_tierheime.clone());
    list.push_back(c_hunde.clone());
    list.push_back(c_doggen.clone());
    list.push_back(c_schaeferhunde.clone());
    list.push_back(c_katzen.clone());

    (tax, ids, list)
}
