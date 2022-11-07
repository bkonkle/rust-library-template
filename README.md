# Rust Library Template

A template for [cargo generate](https://github.com/cargo-generate/cargo-generate) that aims to be a good starting point for Rust libraries that will be hosted on GitHub.

Forked from [rust-github](https://rust-github.github.io) with inspiration from [opinionated-rust-template](https://github.com/tomkarw/opinionated-rust-template).

## Usage

First install `cargo-generate`:

```sh
cargo install cargo-generate
```

Then generate a new project with this template:

```sh
cargo generate bkonkle/rust-library-template
```

## Includes

- [pre-commit](https://pre-commit.com/) - A framework for managing and maintaining multi-language pre-commit hooks.
- [cargo-nextest](https://github.com/nextest-rs/nextest#readme) - A next-generation test runner for Rust.
- [cargo-spellcheck](https://github.com/drahnr/cargo-spellcheck#readme) - Check your spelling with `hunspell` and/or `nlprule`.
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) - About
Cargo subcommand to easily use LLVM source-based code coverage (-C instrument-coverage).
- [cargo-audit](https://github.com/rustsec/rustsec/tree/HEAD/cargo-audit#readme) - Audit Cargo.lock files for crates with security vulnerabilities reported to the [RustSec Advisory Database](https://github.com/RustSec/advisory-db/).
