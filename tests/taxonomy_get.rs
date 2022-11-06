use std::rc::Rc;
// Integration with kodiak's taxonomy library
use kodiak_taxonomy::Identity;

// Shared code across integration tests
use crate::setup::*;
use crate::Concept;

// Tests:
// pub fn get(&self, id: &H) -> Option<&T>

#[test]
fn test_get_some() {
    let mut tax = setup_tax_empty();

    let element = Concept::new("Animal");
    let super_id = element.id();
    let _ = tax.add(None, element);

    let result: Option<&Concept> = tax.get(Rc::new(super_id));

    assert!(result.is_some());
}

#[test]
fn test_get_none() {
    let tax = setup_tax_empty();

    let element = Concept::new("Animal");
    let super_id = element.id();

    let result: Option<&Concept> = tax.get(Rc::new(super_id));

    assert!(result.is_none());
}
