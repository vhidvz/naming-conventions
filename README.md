# Naming Conventions

[![crates.io][crate-image]][crate-link]
![MSRV][msrv-image]
[![Documentation][doc-image]][doc-link]
[![LICENSE][license-image]][license-link]
[![codecov][codecov-image]][codecov-link]
[![Build Status][build-image]][build-link]
[![dependency status][deps-image]][deps-link]
![downloads][downloads-image]

Simple and Fast naming convention library.

## Quick Start Guide

```rust
use naming_conventions::{get_convention, CaseName};

fn main() {
  let snake_case = get_convention(CaseName::SnakeCase);

  let string = "camelCase";

  println!("to snake_case: {}", snake_case.to(string).unwrap());
  println!("is snake_case: {}", snake_case.is(string).unwrap());
}
```

```rust
use naming_conventions::{to_snake_case, is_snake_case};

fn main() {
  let string = "camelCase";

  println!("to snake_case: {}", to_snake_case(string).unwrap());
  println!("is snake_case: {}", is_snake_case(string).unwrap());
}
```

## License

[MIT](https://github.com/vhidvz/naming-conventions/blob/main/LICENSE)

[//]: # "badges"
[crate-image]: https://img.shields.io/crates/v/naming-conventions?label=latest
[crate-link]: https://crates.io/crates/naming-conventions
[doc-image]: https://img.shields.io/docsrs/naming-conventions
[doc-link]: https://docs.rs/naming-conventions
[msrv-image]: https://img.shields.io/badge/rustc-1.68+-blue.svg
[build-image]: https://github.com/vhidvz/naming-conventions/actions/workflows/ci.yml/badge.svg
[build-link]: https://github.com/vhidvz/naming-conventions/actions/workflows/ci.yml
[license-image]: https://img.shields.io/github/license/vhidvz/workflow-js?style=flat
[license-link]: https://github.com/vhidvz/workflow-js/blob/master/LICENSE
[codecov-image]: https://raw.githubusercontent.com/vhidvz/naming-conventions/main/docs/coverage/badges/flat.svg
[codecov-link]: https://htmlpreview.github.io/?https://github.com/vhidvz/naming-conventions/blob/main/docs/coverage/index.html
[deps-image]: https://deps.rs/repo/github/vhidvz/naming-conventions/status.svg
[deps-link]: https://deps.rs/repo/github/vhidvz/naming-conventions
[downloads-image]: https://img.shields.io/crates/d/naming-conventions.svg
