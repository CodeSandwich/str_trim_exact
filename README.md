[![Crates.io](https://img.shields.io/crates/d/trim_matches_exactly.svg)](https://crates.io/crates/trim_matches_exactly)
[![Docs.rs](https://docs.rs/trim_matches_exactly/badge.svg)](https://docs.rs/crate/trim_matches_exactly)
[![Build Status](https://travis-ci.org/CodeSandwich/trim_matches_exactly.svg?branch=master)](https://travis-ci.org/CodeSandwich/trim_matches_exactly)

Extension trait for controlled trimming of prefixes and suffixes of `&str` and `String`.

Provided methods trim only if the given pattern matches exact number of times, otherwise they return
the unmodified `&str`. This can be used for primitive parsing and text analysis.

```rust
assert_eq!(Ok("trimmed"), "not trimmed".trim_left_matches_exactly("not ", 1));
assert_eq!(Err("not trimmed"), "not trimmed".trim_left_matches_exactly("very ", 1));
assert_eq!(Ok("trimmed"), "tttrimmed".trim_left_matches_exactly('t', 2));
```

