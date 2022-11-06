use std::rc::Rc;
// Integration with kodiak's taxonomy library
use kodiak_taxonomy::{Identity, TaxonomyError};

// Shared code across integration tests
use crate::setup::*;
use crate::Concept;

// Tests:
// pub fn add(&mut self, elt: T) -> Result<&mut Self, TaxonomyError<H>>

#[test]
fn test_add_ok() {
    let mut tax = setup_tax_empty();

    let element = Concept::new("Animal");
    let result = tax.add(None, element);

    assert!(result.is_ok());
}

#[test]
fn test_add_duplicate_node_found_err() {
    let mut tax = setup_tax_empty();

    let element = Concept::new("Animal");
    let id = element.id();
    let _ = tax.add(None, element.clone());

    // Adding the element a second time fails
    let result = tax.add(None, element).err();
    let expectation = Some(TaxonomyError::DuplicateNode(Rc::new(id)));
    assert_eq!(result, expectation);
}
