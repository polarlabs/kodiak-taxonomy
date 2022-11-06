use crate::tests::taxonomy::Concept;
use crate::{Identity, Taxonomy};

use uuid::Uuid;

use std::collections::{HashMap, LinkedList};

pub fn setup_tax_empty() -> Taxonomy<Uuid, Concept> {
    let tax: Taxonomy<Uuid, Concept> = Taxonomy::new();
    tax
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
/// ├── Tierhalter
/// ├── Tierheime
/// │   ├── Hunde (duplicate)
/// │   │   └── ...
/// │   └── Katzen (duplicate)
/// └── Tierschutz
///
pub(crate) fn setup_tax_animals() -> (
    Taxonomy<Uuid, Concept>,
    HashMap<&'static str, Uuid>,
    LinkedList<(Uuid, Concept)>,
) {
    let mut tax = setup_tax_empty();
    let mut ids = HashMap::new();
    let mut list = LinkedList::new();

    // Level 1
    let c_tiere = Concept::new("Tiere");
    let id_tiere = c_tiere.id();
    ids.insert("Tiere", id_tiere);
    let c_tierhalter = Concept::new("Tierhalter");
    let id_tierhalter = c_tierhalter.id();
    ids.insert("Tierhalter", id_tierhalter);
    let c_tierheime = Concept::new("Tierheime");
    let id_tierheime = c_tierheime.id();
    ids.insert("Tierheime", id_tierheime);
    let c_tierschutz = Concept::new("Tierschutz");
    let id_tierschutz = c_tierschutz.id();
    ids.insert("Tierschutz", id_tierschutz);

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

    let _ = tax.add(None, c_tierhalter.clone());

    let _ = tax.add(None, c_tierheime.clone());

    let _ = tax.append(Some(id_tierheime), id_hunde);
    let _ = tax.append(Some(id_tierheime), id_katzen);

    let _ = tax.add(None, c_tierschutz.clone());

    // Traversal
    list.push_back((id_tiere.clone(), c_tiere.clone()));
    list.push_back((id_haustiere.clone(), c_haustiere.clone()));
    list.push_back((id_fische.clone(), c_fische.clone()));
    list.push_back((id_hunde.clone(), c_hunde.clone()));
    list.push_back((id_doggen.clone(), c_doggen.clone()));
    list.push_back((id_schaeferhunde.clone(), c_schaeferhunde.clone()));
    list.push_back((id_katzen.clone(), c_katzen.clone()));
    list.push_back((id_nutztiere.clone(), c_nutztiere.clone()));
    list.push_back((id_gefluegel.clone(), c_gefluegel.clone()));
    list.push_back((id_huehner.clone(), c_huehner.clone()));
    list.push_back((id_puten.clone(), c_puten.clone()));
    list.push_back((id_rind.clone(), c_rind.clone()));
    list.push_back((id_zootiere.clone(), c_zootiere.clone()));
    list.push_back((id_affen.clone(), c_affen.clone()));
    list.push_back((id_schlangen.clone(), c_schlangen.clone()));
    list.push_back((id_saeugetiere.clone(), c_saeugetiere.clone()));
    list.push_back((id_affen.clone(), c_affen.clone()));
    list.push_back((id_hunde.clone(), c_hunde.clone()));
    list.push_back((id_doggen.clone(), c_doggen.clone()));
    list.push_back((id_schaeferhunde.clone(), c_schaeferhunde.clone()));
    list.push_back((id_katzen.clone(), c_katzen.clone()));
    list.push_back((id_waale.clone(), c_waale.clone()));
    list.push_back((id_voegel.clone(), c_voegel.clone()));
    list.push_back((id_huehner.clone(), c_huehner.clone()));
    list.push_back((id_puten.clone(), c_puten.clone()));
    list.push_back((id_tierhalter.clone(), c_tierhalter.clone()));
    list.push_back((id_tierheime.clone(), c_tierheime.clone()));
    list.push_back((id_hunde.clone(), c_hunde.clone()));
    list.push_back((id_doggen.clone(), c_doggen.clone()));
    list.push_back((id_schaeferhunde.clone(), c_schaeferhunde.clone()));
    list.push_back((id_katzen.clone(), c_katzen.clone()));
    list.push_back((id_tierschutz.clone(), c_tierschutz.clone()));

    (tax, ids, list)
}
