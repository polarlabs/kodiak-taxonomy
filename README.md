# kodiak-taxonomy

[![GitHub Top Language]][lang]
[![Static unsafe]][unsafe]
[![crates.io License]][license-mit]
[![GitHub License]][license-apache]


[![GitHub Latest Release]][github-releases]
[![GitHub Commits]][github-commits]


[![Code Coverage]][codecov]
[![GitHub Build Status]][github-actions-cargo-test]
[![Static docs coverage]][docs]
[![docs.rs]][docs]
[![Libraries.io Dep Status]][libraries]


[![GitHub Security Schedule]][github-actions-cargo-audit-on-schedule]
[![GitHub Security Push]][github-actions-cargo-audit-on-push]


[![GitHub Open Issues]][github-issues]
[![GitHub Closed Issues]][github-issues]


[![crates.io Latest]][crates]
[![crates.io Recent]][crates]

[Code Coverage]: https://img.shields.io/codecov/c/github/polarlabs/kodiak-taxonomy?label=code%20coverage&logo=codecov&logoColor=ffffff&style=flat-square 
[codecov]: https://codecov.io/github/polarlabs/kodiak-taxonomy

[crates.io Recent]: https://img.shields.io/crates/dr/kodiak-taxonomy?logo=docs.rs&color=67001f&style=flat-square
[crates.io Latest]: https://img.shields.io/crates/v/kodiak-taxonomy?label=latest&logo=docs.rs&style=flat-square
[crates]: https://crates.io/crates/kodiak-taxonomy

[crates.io License]: https://img.shields.io/crates/l/kodiak-taxonomy?color=007ec6&style=flat-square
[GitHub License]: https://img.shields.io/github/license/polarlabs/kodiak-taxonomy?color=007ec6&style=flat-square
[license-mit]: https://choosealicense.com/licenses/mit/
[license-apache]: https://choosealicense.com/licenses/apache-2.0/

[Static docs coverage]: https://img.shields.io/badge/docs%20coverage-100%25-success.svg?logo=rust&logoColor=ffffff&style=flat-square
[docs.rs]: https://img.shields.io/docsrs/kodiak-taxonomy?logo=docs.rs&style=flat-square
[docs]: https://docs.rs/kodiak-taxonomy

[GitHub Build Status]: https://img.shields.io/github/workflow/status/polarlabs/kodiak-taxonomy/workflow-cargo-test?logo=github&label=tests&style=flat-square
[github-actions-cargo-test]: https://github.com/polarlabs/kodiak-taxonomy/actions/workflows/cargo-test.yml

[GitHub Security Schedule]: https://img.shields.io/github/workflow/status/polarlabs/kodiak-taxonomy/workflow-cargo-audit-on-schedule?logo=clockify&logoColor=ffffff&label=security%20audit&style=flat-square
[github-actions-cargo-audit-on-schedule]: https://github.com/polarlabs/kodiak-taxonomy/actions/workflows/cargo-audit-on-schedule.yml

[GitHub Security Push]: https://img.shields.io/github/workflow/status/polarlabs/kodiak-taxonomy/workflow-cargo-audit-on-push?logo=github&label=security%20audit&style=flat-square
[github-actions-cargo-audit-on-push]: https://github.com/polarlabs/kodiak-taxonomy/actions/workflows/cargo-audit-on-push.yml

[GitHub Top Language]: https://img.shields.io/github/languages/top/polarlabs/kodiak-taxonomy?color=dea584&logo=rust&style=flat-square
[lang]: https://www.rust-lang.org/

[GitHub Latest Release]: https://img.shields.io/github/v/release/polarlabs/kodiak-taxonomy?include_prereleases&sort=semver&logo=github&label=latest&style=flat-square
[github-releases]: https://github.com/polarlabs/kodiak-taxonomy/releases

[GitHub Commits]: https://img.shields.io/github/commits-since/polarlabs/kodiak-taxonomy/latest?include_prereleases&sort=semver&logo=github&style=flat-square
[github-commits]: https://github.com/polarlabs/kodiak-taxonomy/commits

[GitHub Open Issues]: https://img.shields.io/github/issues-raw/polarlabs/kodiak-taxonomy?logo=github&style=flat-square
[GitHub Closed Issues]: https://img.shields.io/github/issues-closed-raw/polarlabs/kodiak-taxonomy?logo=github&style=flat-square
[github-issues]: https://github.com/polarlabs/kodiak-taxonomy/issues

[Libraries.io Dep Status]: https://img.shields.io/librariesio/github/polarlabs/kodiak-taxonomy?logo=libraries.io&logoColor=ffffff&style=flat-square
[libraries]: https://libraries.io/cargo/kodiak-taxonomy

[Static unsafe]: https://img.shields.io/badge/unsafe-forbidden-success.svg?logo=rust&logoColor=ffffff&style=flat-square
[unsafe]: (https://github.com/rust-secure-code/safety-dance/)

Get things organized with this powerful, yet easy to use taxonomy.

This library crate is a building block of the Kodiak project, thus the naming of the crate.
Although, Kodiak has some quite specific requirements for a taxonomy, `kodiak-taxonomy` is kept generic,
provides value on its own and might be of interest for other projects as well.

A taxonomy[^Wikipedia] is a classification scheme to organize things
according to their types. Often, taxonomies offer a strictly hierarchical organization providing a
tree-like structure. However, in practice, such taxonomies limit our ability to model our complex reality.

When looking for a powerful taxonomy which overcomes such limitations, `kodiak-taxonomy` might be a good fit
for you. So, feel free to use it. If you consider using `kodiak-taxonomy` in your project but are missing functionality
or have any other concerns, don't hesitate to file an issue on GitHub.

We are looking forward to your feedback.

---

You may be looking for:

- [API documentation](https://docs.rs/kodiak-taxonomy/)
- [Release notes](https://github.com/polarlabs/kodiak-taxonomy/releases)

---

# Impressions

todo: show two examples of taxonomies supported by kodiak-taxonomy

Provide additional examples in EXAMPLES.md and link to it.

# TL;DR

Kodiak's specific requirements regarding its taxonomy:
- An element can have more than one superordinate element
- The top of the taxonomy allows multiple elements, i.e. users are free to create multiple root-nodes
- Elements might be complemented by arbitrary metadata (still todo: not implemented yet)
- Edges (a tuple of a super and its sub element) might be complemented with arbitrary attributes (still todo: not implemented yet)

# Known issues / limitations
- 🏗️ Version 0.2.0 does not yet power other projects, so API has not yet proven it's power in practice.
- 🚧 Code is fully covered by unit tests, however, some integration tests are still missing.
- Documentation has still some room for improvement.
- 🐧 Version 0.2.x is developed and tested on Linux only.

# Roadmap and future considerations

## Version 0.7.0 (planned)
- Add performance consideration of public functions to the documentation.

## Version 0.6.0 (planned)
- Implement Iterator trait.
- Implement Display trait for Node.

## Version 0.5.0 (planned)
- Implement Serde's Serialize and Deserialize.

## Version 0.4.0 (planned)
- Add support for arbitrary metadata / attributes at the Node and Edge level.

## Version 0.3.0 (in progress)
- Re-thing the API: implement `Taxonomy` as a thin wrapper around std::collections::HashMap and get rid of TaxonomyError (see [CHECKLIST_RUST_API_CONFORMANCE](CHECKLIST_RUST_API_CONFORMANCE.md))
- Improve documentation (see [CHECKLIST_RUST_API_CONFORMANCE](CHECKLIST_RUST_API_CONFORMANCE.md))
- Publish Software Bill of Material (SBOM) together with any future release.
- Use LLVM engine of Tarpaulin to check code coverage.

## Version 0.2.0 (delivered)
- Review and follow Rust API Guidelines.
- Improve documentation: e.g. comply to C-LINK from Rust API Guidelines.
- Full code coverage with tests, measured by tarpaulin and added as a badge to GitHub project 
- Switch to `#![forbid(unsafe_code)]`

## Version 0.1.0 (delivered)
- Initial release.

# Additional resources

- Homepage polarlabs: [polarlabs.io](https://www.polarlabs.io)
- Crate: [crates.io/kodiak-taxonomy](https://crates.io/crates/kodiak-taxonomy)
- API documentation: [docs.rs/kodiak-taxonomy](https://docs.rs/kodiak-taxonomy/)

# Contributing

See [CONTRIBUTING](CONTRIBUTING.md) for more details.

# Appendix

## Cargo Geiger Safety Report
```

Metric output format: x/y
    x = unsafe code used by the build
    y = total unsafe code found in the crate

Symbols: 
    🔒  = No `unsafe` usage found, declares #![forbid(unsafe_code)]
    ❓  = No `unsafe` usage found, missing #![forbid(unsafe_code)]
    ☢️  = `unsafe` usage found

Functions  Expressions  Impls  Traits  Methods  Dependency

0/0        0/0          0/0    0/0     0/0      🔒  kodiak-taxonomy 0.2.0

0/0        0/0          0/0    0/0     0/0    

```
## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or https://opensource.org/licenses/Apache-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
