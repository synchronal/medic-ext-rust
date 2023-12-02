# Medic Rust

An extension pack for using [medic](https://github.com/synchronal/medic-rs)
with Rust projects.

## Installation

```shell
brew tap synchronal/tap
brew install medic-ext-rust
```

Example `Brewfile`:

```shell
tap 'synchronal/tap'

brew  'synchronal/tap/medic'
brew  'synchronal/tap/medic-ext-rust'
```

## Usage

```toml
[doctor]
checks = [
  { check = "asdf", command = "plugin-installed", args = { plugin = "rust" } },
  { check = "asdf", command = "package-installed", args = { plugin = "rust" } },
  { check = "homebrew" },
  { name = "compile", shell = "cargo build --workspace" },
  { check = "rust", command = "crate-installed", args = { name = "cargo-audit" } },
  { check = "rust", command = "crate-installed", args = { name = "cargo-outdated" } },
  { check = "rust", command = "target-installed", args = { target = "aarch64-apple-darwin" } },
  { check = "rust", command = "target-installed", args = { target = "x86_64-apple-darwin" } },
]

[test]
checks = [
  { name = "Check for warnings", shell = "cargo build --workspace --features strict" },
  { step = "rust", command = "test", verbose = true },
]

[audit]
checks = [
  ## allow failure: chrono and time have known potential segfaults
  { name = "Audit crates", shell = "cargo audit", allow_failure = true, verbose = true },
  { check = "rust", command = "format-check" },
  { step = "rust", command = "clippy" },
]

[outdated]
checks = [
  { check = "rust" },
  { check = "rust", cd: "crates/my-workspace-crate" },
]

[update]
steps = [
  { step = "git", command = "pull" },
  { doctor = {} },
]

[shipit]
steps = [
  { audit = {} },
  { update = {} },
  { test = {} },
  { name = "Release", shell = ".medic/bin/release", verbose = true },
  { step = "git", command = "push" },
  { step = "github", command = "link-to-actions", verbose = true },
]
```

Check out the [release script](https://github.com/synchronal/medic-ext-rust/blob/main/.medic/bin/release)
for an example of packaging a Rust project for GitHub releases.

## medic-check-rust

Checks for whether a Rust project is configured and/or ready to ship.

### crate installed?

Is a given crate installed into the current Rust toolchain?

```shell
medic-check-rust crate-installed --name <name>
medic-check-rust crate-installed --name cargo-audit
```

### formatted?

Is the project properly formatted?

```shell
medic-check-rust format-check
```

### target installed?

Is the given compilation target installed in the current Rust toolchain?

```shell
medic-check-rust target-installed --target <target>
medic-check-rust target-installed --target aarch64-apple-darwin
```


## medic-outdated-rust

Check for outdated crates.


## medic-step-rust

Steps for verifying the project is ready to ship.

## clippy

Run the Rust linter.

```shell
medic-step-rust clippy
```

## test

Run all tests.

```shell
medic-step-rust test
```

