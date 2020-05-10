# `shpat` - Sasha's Rust Patterns

This crate contains a bunch of patterns that I often met while programming in
Rust. I put them in a single crate, so other Rustaceans may also enjoy them.

This crate has the following goals:
  - as lightweight as possible, no dependencies,
  - integrate patterns with each others.

## Current patterns

### `apply`

Some methods in Rust don't take ownership, and just take reference of an object.
This may be nice in some situations, but this does not allow to chain methods.
The `Apply` trait allows to bring back method chaining for methods which take
mutable references.

For example, `HashMap` can be created easily:

```rust
use std::collections::HashMap;

let map = HashMap::new()
    .apply(|m| m.insert("manatee", 42)
    .apply(|m| m.insert("horse", 101);
```

The `Apply` trait is implemented for every `Sized` type. It also provides
`apply_keep`, which returns the value returned by the inner method and the
object itself (which may be usefull when one wants to insert a value in a hash
map, but want to keep returned value), and `apply_unwrap`, which will call
`unwrap` on every `Unwrapable` returned value.
