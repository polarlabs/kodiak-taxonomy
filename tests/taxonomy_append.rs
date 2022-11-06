use std::rc::Rc;
// Integration with kodiak's taxonomy library
use kodiak_taxonomy::{Identity, TaxonomyError};

// Shared code across integration tests
use crate::setup::*;
use crate::Concept;

// Tests:
// pub fn append(&mut self, super_id: H, elt: T) -> Result<&mut Self, TaxonomyError<H>>

#[test]
fn test_append_ok() {
    let mut tax = setup_tax_empty();

    let element = Concept::new("Animal");
    let super_id = element.id();
    let _ = tax.add(None, element);

    let element = Concept::new("Vertebrate");
    let result = tax.add(Some(super_id), element);

    assert!(result.is_ok());

    let element = Concept::new("Mammal");
    let result = tax.add(Some(super_id), element);

    assert!(result.is_ok());
}

#[test]
fn test_append_node_not_found_err() {
    let mut tax = setup_tax_empty();

    let element = Concept::new("Animal");
    let super_id = element.id();

    let element = Concept::new("Vertebrate");

    let result = tax.add(Some(super_id), element).err();

    let expectation = Some(TaxonomyError::NodeNotFound(Rc::new(super_id)));
    assert_eq!(result, expectation);
}
