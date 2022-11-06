use std::rc::Rc;
// Integration with kodiak's taxonomy library
use kodiak_taxonomy::Identity;

// Shared code across integration tests
use crate::setup::*;
use crate::Concept;

// Tests:
// pub fn get_mut(&mut self, id: &H) -> Option<&mut T>

#[test]
fn test_get_mut_some() {
    let mut tax = setup_tax_empty();

    let element = Concept::new("Animal");
    let super_id = element.id();
    let _ = tax.add(None, element);

    let result: Option<&mut Concept> = tax.get_mut(Rc::new(super_id));

    assert!(result.is_some());
}

#[test]
fn test_get_none() {
    let mut tax = setup_tax_empty();

    let element = Concept::new("Animal");
    let super_id = element.id();

    let result: Option<&mut Concept> = tax.get_mut(Rc::new(super_id));

    assert!(result.is_none());
}
