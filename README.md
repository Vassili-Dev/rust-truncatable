# Truncatable

Truncatable is a small rust crate for Strings that should append a follower when truncated (eg, ellipsis).

## Usage

```
use truncatable::Trucatable;
let to_truncate = Truncatable::from("Hello World!").truncator("~~".into());
assert_eq!(to_trucate.truncate(5), String::from("Hello~~"));
```
