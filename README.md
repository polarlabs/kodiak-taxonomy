# kodiak-taxonomy

[![GitHub Top Language]][lang]
[![crates.io License]][license-mit]
[![Github License]][license-apache]


[![GitHub Latest Release]][github-releases]
[![GitHub Commits]][github-commits]


[![Github Build Status]][github-actions-cargo-test]
[![Code Coverage]][codecov]
[![docs.rs]][docs]
[![Github Security Audit]][github-actions-cargo-audit]
[![Libraries.io Dep Status]][libraries]


[![GitHub Open Issues]][github-issues]
[![GitHub Closed Issues]][github-issues]


[![crates.io Latest]][crates]
[![crates.io Recent]][crates]

[Code Coverage]: https://img.shields.io/codecov/c/github/polarlabs/kodiak-taxonomy?logo=codecov&logoColor=ffffff&style=flat-square 
[codecov]: https://codecov.io/github/polarlabs/kodiak-taxonomy

[crates.io Recent]: https://img.shields.io/crates/dr/kodiak-taxonomy?logo=docs.rs&color=67001f&style=flat-square
[crates.io Latest]: https://img.shields.io/crates/v/kodiak-taxonomy?label=latest&logo=docs.rs&style=flat-square
[crates]: https://crates.io/crates/kodiak-taxonomy

[crates.io License]: https://img.shields.io/crates/l/kodiak-taxonomy?color=007ec6&style=flat-square
[GitHub License]: https://img.shields.io/github/license/polarlabs/kodiak-taxonomy?color=007ec6&style=flat-square
[license-mit]: https://choosealicense.com/licenses/mit/
[license-apache]: https://choosealicense.com/licenses/apache-2.0/

[docs.rs]: https://img.shields.io/docsrs/kodiak-taxonomy?logo=docs.rs&style=flat-square
[docs]: https://docs.rs/kodiak-taxonomy

[Github Build Status]: https://img.shields.io/github/workflow/status/polarlabs/kodiak-taxonomy/workflow-cargo-test?logo=github&label=tests&style=flat-square
[github-actions-cargo-test]: https://github.com/polarlabs/kodiak-taxonomy/actions/workflows/cargo-test.yml

[Github Security Audit]: https://img.shields.io/github/workflow/status/polarlabs/kodiak-taxonomy/workflow-cargo-audit?logo=github&label=security%20audit&style=flat-square
[github-actions-cargo-audit]: https://github.com/polarlabs/kodiak-taxonomy/actions/workflows/cargo-audit.yml

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

This library crate is a building block of the Kodiak project, thus the naming of the crate.
Although, Kodiak has some quite specific requirements for a taxonomy, `kodiak-taxonomy` is kept generic,
provides value on its own and might be of interest for other projects as well.

So, feel free to use it. If you consider using `kodiak-taxonomy` in your project but are missing functionality,
don't hesitate to file an issue on Github.

We are looking forward to your feedback.

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
- 🏗️ Version 0.1.0 does not yet power other projects, so API has not yet proven it's power in practice.
- 🚧 Code is fully covered by unit tests, however, some integration tests are still missing and documentation has room for improvement.
- Rust API Guidelines have not yet been considered.
- 🐧 Version 0.1.x is developed and tested on Linux only.

# Roadmap and future considerations

## Version 0.3.0 (planned)
- Implement Iterator trait.
- Implement Serde's Serialize and Deserialize.

## Version 0.2.0 (in progress)
- Review and follow Rust API Guidelines.
- Improve documentation: e.g. comply to C-LINK from Rust API Guidelines.
- Full code coverage with tests, measured by tarpaulin and added as a badge to Github project 
- Add support for arbitrary metadata / attributes at the Node and Edge level.

## Version 0.1.0 (delivered)
- Initial release.

# Additional resources

- Homepage polarlabs: [polarlabs.io](https://www.polarlabs.io)
- Crate: [crates.io/kodiak-taxonomy](https://crates.io/crates/kodiak-taxonomy)
- Lib documentation: [docs.rs/kodiak-taxonomy](https://docs.rs/kodiak-taxonomy/)

# Contributing

See CONTRIBUTING.md for more details.
