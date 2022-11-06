# kodiak-taxonomy

This library crate is a building block of the Kodiak project, thus the naming of the crate.
Although, Kodiak has some quite specific requirements for a taxonomy, `kodiak-taxonomy` is kept generic,
provides value on its own and might be of interest for other projects as well.

So, feel free to use it. If you consider using `kodiak-taxonomy` in your project but are missing functionality,
don't hesitate to file an issue on Github.

We are looking forward to your feedback.

# TL;DR

Kodiak's specific requirements regarding its taxonomy:
- An element can have more than one superordinate element
- The top of the taxonomy allows multiple elements, i.e. users are free to create multiple root-nodes
- Elements might be complemented by arbitrary metadata (still todo: not implemented yet)
- Edges (a tuple of a super and its sub element) might be complemented with arbitrary attributes (still todo: not implemented yet)

# Known issues / limitations
- 🏗️ Version 0.1.0 does not yet power other projects, so API has not yet proven it's power in practice.
- 🚧 Code is fully covered by unit tests, however, some integration tests are still missing and documentation has room for improvement.
- Rust API Guidelines have not yet been considered.
- 🐧 Version 0.1.x is developed and tested on Linux only.

# Roadmap and future considerations

## Version 0.2.0 (planned)
- Review and follow Rust API Guidelines
- Add support for arbitrary metadata / attributes at the Node and Edge level.

## Version 0.1.0 (delivered)
- Initial release.

# Additional resources

- Homepage polarlabs: [polarlabs.io](https://www.polarlabs.io)
- Crate: [crates.io/kodiak-taxonomy](https://crates.io/crates/kodiak-taxonomy)
- Lib documentation: [docs.rs/kodiak-taxonomy](https://docs.rs/kodiak-taxonomy/)
