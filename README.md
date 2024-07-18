<div align="center"><h1>sui-literals</h1></div>
<div align="center"><img src="./assets/c-template-logo.jpeg" height="200"><p><p></div>

<div align="center"><h3>🎉 Welcome to sui-literals —  🚀</h3></div>

<div align="center">
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/issues">
    <img src="https://img.shields.io/github/issues/jymchng/sui-literals" alt="GitHub issues" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/network">
    <img src="https://img.shields.io/github/forks/jymchng/sui-literals" alt="GitHub forks" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/stargazers">
    <img src="https://img.shields.io/github/stars/jymchng/sui-literals" alt="GitHub stars" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals">
    <img src="https://img.shields.io/github/license/jymchng/sui-literals" alt="GitHub license" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/commits/main">
    <img src="https://img.shields.io/github/last-commit/jymchng/sui-literals" alt="GitHub last commit" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/jymchng/sui-literals" alt="GitHub contributors" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/pulls">
    <img src="https://img.shields.io/github/issues-pr/jymchng/sui-literals" alt="GitHub pull requests" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/releases">
    <img src="https://img.shields.io/github/release/jymchng/sui-literals" alt="GitHub release" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals">
    <img src="https://img.shields.io/github/repo-size/jymchng/sui-literals" alt="GitHub repo size" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/commits">
    <img src="https://img.shields.io/github/commit-activity/m/jymchng/sui-literals" alt="GitHub commit activity" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals">
    <img src="https://img.shields.io/github/languages/code-size/jymchng/sui-literals" alt="GitHub code size in bytes" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals">
    <img src="https://img.shields.io/github/languages/count/jymchng/sui-literals" alt="GitHub language count" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals">
    <img src="https://img.shields.io/github/languages/top/jymchng/sui-literals" alt="GitHub top language" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/releases">
    <img src="https://img.shields.io/github/downloads/jymchng/sui-literals/total" alt="GitHub download count" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/watchers">
    <img src="https://img.shields.io/github/watchers/jymchng/sui-literals" alt="GitHub watchers" height="20">
  </a>
  <a href="https://github.com/jymchng">
    <img src="https://img.shields.io/github/followers/jymchng?label=Follow" alt="GitHub followers" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/discussions">
    <img src="https://img.shields.io/github/discussions/jymchng/sui-literals" alt="GitHub discussions" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/issues?q=is%3Aissue+is%3Aclosed">
    <img src="https://img.shields.io/github/issues-closed/jymchng/sui-literals" alt="GitHub issues closed" height="20">
  </a>
  <a href="https://github.com/jymchng/sui-literals/milestones">
    <img src="https://img.shields.io/github/milestones/all/jymchng/sui-literals" alt="GitHub milestones" height="20">
  </a>
  <a href="https://github.com/sponsors/jymchng">
    <img src="https://img.shields.io/badge/funding-donate-brightgreen" alt="GitHub funding" height="20">
  </a>
  <img alt="Rust Check" src="https://github.com/jymchng/sui-literals/actions/workflows/check.yml/badge.svg" height="20">
  <img alt="Rust NoSTD" src="https://github.com/jymchng/sui-literals/actions/workflows/nostd.yml/badge.svg" height="20">
  <img alt="Rust Safety" src="https://github.com/jymchng/sui-literals/actions/workflows/safety.yml/badge.svg" height="20">
  <img alt="Rust Scheduled" src="https://github.com/jymchng/sui-literals/actions/workflows/scheduled.yml/badge.svg" height="20">
  <img alt="Rust Test" src="https://github.com/jymchng/sui-literals/actions/workflows/test.yml/badge.svg" height="20">
</div>

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Examples](#examples)
- [Macros](#macros)
- [Debugging](#debugging)
- [Contributing](#contributing)
- [Building The Project](#building-the-project)
- [Running Tests](#running-tests)
- [Linting](#linting)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Overview

`sui-literals` is a macro library designed to transform hexadecimal literals into `ObjectID` or `SuiAddress` types at compile-time. This ensures type safety and compile-time checks for transformation suffixes, streamlining development processes in Rust projects that work with Sui blockchain addresses and object IDs.

It draws inspiration from

## Features

- **Compile-time transformations**: Convert hexadecimal literals into `ObjectID` or `SuiAddress` types at compile time.
- **Error handling**: Custom error types for different stages of transformation.
- **Type safety**: Ensures that literals are properly formatted and suffixed.
- **Debugging**: Debug prints to aid in development and troubleshooting.

## Installation

To use the `sui-literals` library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
sui-literals = "0.1.0"
```

## Usage
To integrate the sui-literals macros into your project, use the `sui_literal!` macro to convert hexadecimal literals into `ObjectID` or `SuiAddress` types.

## Examples
### Valid Usage
```rust
use sui_literals::sui_literal;

let object_id: ObjectID = sui_literal!(0x01b0d52321ce82d032430f859c6df0c52eb9ce1a337a81d56d89445db2d624f0_object);
let sui_address: SuiAddress = sui_literal!(0x01b0d52321ce82d032430f859c6df081d56d89445db2d624f0_address);

println!("{:?}", object_id);
println!("{:?}", sui_address);
```

### Compile-Time Failures
The following example demonstrates a compile-time failure when using the sui_literal macro with an invalid suffix.

```
use sui_literals::sui_literal;
use sui_types::base_types::{ObjectID, SuiAddress};

let object_id = sui_literal!(0x01b0d52321ce82d032430f859c6df0c52eb9ce1a337a81d56d89445db2d624f0_invalid_suffix);
```

## Macros
### `sui_literal!`
The sui_literal macro transforms a hexadecimal literal into either an ObjectID or SuiAddress based on the suffix provided.

### Supported Suffixes
`_object`: Transforms the literal into an ObjectID.

`_address`: Transforms the literal into a SuiAddress.

## Debugging
Debug prints are enabled to aid in development and troubleshooting. These prints can be seen in the console output when running your project.

## Contributing
Contributions are welcome! Please feel free to submit a pull request or open an issue on GitHub.

## Building the Project
To build the project, run:

```
cargo build
```

## Running Tests
To run the tests, execute:

```
cargo test
```

## Linting
To lint the project using Clippy, run:

```
cargo clippy --all-targets --all-features -- -D warnings
```

## License
This project is licensed under the MIT License. See the LICENSE file for more details.

## Acknowledgements
