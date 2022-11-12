Just a short checklist on publishing a release.

# Publish a new version on crates.io

From the project root:

```
cargo clean
cargo geiger --all-features --output-format GitHubMarkdown --update-readme
```

* Review README.md
* Build docs and check locally
* Change version in Cargo.toml

```
cargo publish --dry-run
cargo package --list
cargo publish
```

# Publish a new release on GitHub

tbd

# Links

[The Cargo Book - Publishing a new version of an existing crate](https://doc.rust-lang.org/cargo/reference/publishing.html#publishing-a-new-version-of-an-existing-crate)
