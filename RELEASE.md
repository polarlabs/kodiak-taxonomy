Just a short checklist on merging a feature branch and publishing a release.

# Merge a feature branch

* Review CI results of to be merged feature branch
* Merge feature branch

```
git checkout main
git merge --squash <FEATURE_BRANCH>
```

* Change version in Cargo.toml

```
cargo clean
cargo geiger --all-features --output-format GitHubMarkdown --update-readme
```
 
* Review README.md
* Update roadmap in README.md

```
git add .
git commit -m "feat!: ... ."
git push
```

# Publish a new version on crates.io

From the project root:

```
cargo doc
```

* Review docs locally

```
cargo publish --dry-run
cargo package --list
cargo publish
```

# Publish a new release on GitHub

On GitHub:

* Create a tag with pattern vx.y.z
* Release title: kodiak-taxonomy-vx.y.z
* Write some release notes (mainly inspired by roadmap and git log)

# Links

[The Cargo Book - Publishing a new version of an existing crate](https://doc.rust-lang.org/cargo/reference/publishing.html#publishing-a-new-version-of-an-existing-crate)
