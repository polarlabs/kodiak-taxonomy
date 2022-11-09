Quality is at the core of the Kodiak brand. We leverage Github Actions, a powerful CI/CD engine, to make sure we meet the expectations. 

Implemented quality checks:

* cargo test
* cargo fmt --all -- --check
* cargo clippy --all-features -- -D warnings
* cargo tarpaulin --ignore-tests

Code coverage report is uploaded to Codecov.

Implemented security checks:

* cargo audit (runs if dependencies have changed and scheduled once a day)

# Links

[https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3|Github Actions by Luca Palmieri]

[https://github.com/actions-rs|Rust support for Github Actions]

[https://github.com/codecov/codecov-action|Action for Codecov Integration]
