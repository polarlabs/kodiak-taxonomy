//! This library crate is a building block of the Kodiak project, thus the naming of the crate.
//! Although, Kodiak has some quite specific requirements for a taxonomy, `kodiak-taxonomy` is kept generic,
//! provides value on its own and might be of interest for other projects as well.
//!
//! So, feel free to use it. If you consider using `kodiak-taxonomy` in your project but are missing functionality
//! or have any other concerns, don't hesitate to file an issue on Github.
//!
//! We are looking forward to your feedback.
//!
//! # TL;DR
//!
//! Kodiak's specific requirements regarding its taxonomy:
//! 1. An element can have more than one superordinate element
//! 2. The top of the taxonomy allows multiple elements, i.e. users are free to create multiple root-nodes
//! 3. Elements might be complemented by arbitrary meta data (still todo: not implemented yet)
//! 4. Edges (a tuple of a super and its sub element) might be complemented with arbitrary attributes (still todo: not implemented yet)

// Keep crate's module structure completely private, see public reexports below.
// (also hides modules from crate documentation)
mod cursor;
mod edge;
mod node;
mod taxonomy;
mod taxonomy_error;
mod tests;
mod traits;

// Re-exports for convenient use within crate.
pub(crate) use crate::cursor::Cursor;
pub(crate) use crate::edge::Edge;
pub(crate) use crate::node::Node;

// Publicly re-exporting all items valuable to users.
// (avoids explicitly listing reexports in crate documentation as there is no alternate path to those items)
pub use taxonomy::Taxonomy;
pub use taxonomy_error::TaxonomyError;
pub use traits::Identity;
