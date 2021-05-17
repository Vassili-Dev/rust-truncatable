# Truncatable

Truncatable is a small rust crate for Strings that should append a follower when truncated (eg, ellipsis).

[![Crates.io][crates-badge]][crates-url]
![Build](https://github.com/Vassili-Dev/rust-truncatable/actions/workflows/build.yml/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/truncatable.svg
[crates-url]: https://crates.io/crates/truncatable

## Usage

```
use truncatable::Truncatable;
let to_truncate = Truncatable::from("Hello World!").truncator("~~".into());
assert_eq!(to_truncate.truncate(5), String::from("Hello~~"));
```
