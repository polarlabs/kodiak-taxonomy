// Integration with kodiak's taxonomy library
use kodiak_taxonomy::Identity;

// Shared code across integration tests
use crate::setup::*;

#[test]
fn test_traverse() {
    let (mut tax, _, mut list) = setup_tax_animals();

    while let Some(element) = tax.traverse() {
        assert_eq!(list.pop_front().unwrap().id(), element.id());
    }
    assert_eq!(list.len(), 0);
}
