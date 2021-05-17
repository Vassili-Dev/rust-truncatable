# Truncatable

[![Crates.io][crates-badge]][crates-url]
[crates-badge]: https://img.shields.io/crates/v/truncatable.svg
[crates-url]: https://crates.io/crates/truncatable

Truncatable is a small rust crate for Strings that should append a follower when truncated (eg, ellipsis).

## Usage

```
use truncatable::Trucatable;
let to_truncate = Truncatable::from("Hello World!").truncator("~~".into());
assert_eq!(to_trucate.truncate(5), String::from("Hello~~"));
```
