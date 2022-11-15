#![forbid(unsafe_code)]
#![deny(missing_docs)]
// Lints for rustdoc
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
//#![deny(missing_doc_code_examples)] (unstable)
#![deny(rustdoc::invalid_codeblock_attributes)]
//#![deny(rustdoc::invalid_html_tags)] (unstable)
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::bare_urls)]
#![warn(rustdoc::private_doc_tests)]
/*
Collection of useful rustdoc options awaiting their implementation.
#![doc(html_logo_url = "https://example.com/logo.jpg")]
#![doc(html_favicon_url = "https://example.com/favicon.ico")]
*/

//! Get things organized with this powerful, yet easy to use taxonomy.
//!
//! This library crate is a building block of the Kodiak project, thus the naming of the crate.
//! Although, Kodiak has some quite specific requirements for a taxonomy, `kodiak-taxonomy` is kept generic,
//! provides value on its own and might be of interest for other projects as well.
//!
//! A taxonomy[^Wikipedia] is a classification scheme to organize things
//! according to their types. Often, taxonomies offer a strictly hierarchical organization providing a
//! tree-like structure. However, in practice, such taxonomies limit our ability to model our complex reality.
//!
//! When looking for a powerful taxonomy which overcomes such limitations, `kodiak-taxonomy` might be a good fit
//! for you. So, feel free to use it. If you consider using `kodiak-taxonomy` in your project but are missing functionality
//! or have any other concerns, don't hesitate to file an issue on Github.
//!
//! We are looking forward to your feedback.
//!
//! # Getting started
//!
//! The following example showcases how to use `kodiak-taxonomy`. We build a [`Taxonomy`] supporting two processes:
//! Human Resource Management (HRM) and Configuration Management Database (CMDB). In both processes we
//! have to organize users and we want to keep the concept of a User identical across the organization.
//! So, we add User to both processes. This taxonomy would look like:
//!
//! ```text
//! ├── HRM
//! │   └── User
//! └── CMDB
//!     ├── User (identical to HRM::User)
//!     └── Device
//!         ├── Server
//!         └── Client
//! ```
//!
//! Here is the code.
//!
//! ```rust
//! use kodiak_taxonomy::{Identity, Taxonomy, TaxonomyError};
//!
//! // Simplistic type `Class` to store in taxonomy (kodiak-taxonomy supports arbitrary types).
//! #[derive(Debug)]
//! struct Class {
//!   name: String,
//! }
//!
//! // As a prerequisite for `kodiak-taxonomy` we have to implement the Identity trait for `Class`.
//! // String implements `Hash` and `Eq` thus it's easy to implement `Identity` on top of it.
//! impl Identity<String> for Class {
//!   fn id(&self) -> String {
//!     self.name.clone()
//!   }
//! }
//!
//! fn main() -> Result<(), TaxonomyError<String>> {
//!   let mut tax: Taxonomy<String, Class> = Taxonomy::new();
//!
//!   // Create various items and store their ids
//!   let hrm = Class{name: "HRM".to_string()};
//!   let hrm_id = hrm.id();
//!
//!   let user = Class{name: "User".to_string()};
//!   let user_id = user.id();
//!
//!   let cmdb = Class{name: "CMDB".to_string()};
//!   let cmdb_id = cmdb.id();
//!
//!   // Add HRM and CMDB as root-node.
//!   tax.add(None, hrm)?
//!      .add(None, cmdb);
//!
//!   // Add User as a sub-node of HRM
//!   tax.add(Some(hrm_id), user);
//!
//!   // Append User as a sub-node of CMDB, we use append() because
//!   // User has been added to taxonomy before.
//!   tax.append(Some(cmdb_id.clone()), user_id);
//!
//!   // Create and add another `Class`. This time we get the id from the taxonomy.
//!   let device = Class{name: "Device".to_string()};
//!   tax.add(Some(cmdb_id), device);
//!   let device_id = tax.last_updated_node().unwrap().to_string();
//!
//!   // Add additional sub-nodes to Devices.
//!   let server = Class{name: "Server".to_string()};
//!   let client = Class{name: "Client".to_string()};
//!   tax.add(Some(device_id.clone()), server)?
//!      .add(Some(device_id), client);
//!
//!   // Traverse the taxonomy and print the classes.
//!   while let Some(class) = tax.traverse() {
//!     println!("{:?}", class);
//!   }
//!
//!   Ok(())
//! }
//! ```
//!
//! The library provides many more functions. Have a look at detailed documentation provided.
//!
//! # TL;DR
//!
//! Kodiak's specific requirements regarding its taxonomy and its implementation status:
//! 1. [x] The top of the taxonomy allows multiple elements, i.e. users are free to create multiple root-nodes and are not forced to invent a super-node.
//! 2. [x] An element can have more than one superordinate element
//! 3. [ ] Elements might be complemented by arbitrary meta data (still todo: not implemented yet)
//! 4. [ ] Edges (a tuple of a super and its sub element) might be complemented with arbitrary attributes (still todo: not implemented yet)
//!
//! [^Wikipedia]: [Taxonomy @ Wikipedia](https://en.wikipedia.org/wiki/Taxonomy)

// Keep crate's module structure completely private, see public re-exports below.
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
// (avoids explicitly listing re-exports in crate documentation as there is no alternate path to those items)
pub use taxonomy::Taxonomy;
pub use taxonomy_error::TaxonomyError;
pub use traits::Identity;
