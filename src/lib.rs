//! A bunch of patterns that I often met while programming in Rust. I put them
//! in a single crate, so other Rustaceans may also enjoy them.
//!
//! This crate has the following goals:
//!   - as lightweight as possible, no dependencies,
//!   - integrate patterns with each others.
//!
//! # Current patterns
//!
//! ### `apply`
//!
//! Some methods in Rust don't take ownership, and just take reference of an
//! object. This may be nice in some situations, but this does not allow to
//! chain methods. The `Apply` trait allows to bring back method chaining for
//! methods which take mutable references.
//!
//! For example, `HashMap` can be created easily:
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! use shpat::prelude::*;
//!
//! let map = HashMap::new()
//!     .apply(|m| m.insert("manatee", 42))
//!     .apply(|m| m.insert("horse", 101));
//! ```
//!
//! The `Apply` trait is implemented for every `Sized` type. It also provides
//! `apply_keep`, which returns the value returned by the inner method and the
//! object itself (which may be usefull when one wants to insert a value in a
//! hash map, but want to keep returned value), and `apply_unwrap`, which will
//! call `unwrap` on every `Unwrapable` returned value.
//!
//! ## `quick_drop`
//!
//! As shown by [Aaron Abramov](https://github.com/aaronabramov/) in [their
//! blog post](https://abramov.io/rust-dropping-things-in-another-thread),
//! `drop`ping large objects can slow down a Rust program. A solution they
//! suggested is to run the `drop` function in a new thread.
//!
//! The `QuickDrop` trait is provided, which allows, for most objects, to
//! be dropped in a new thread.
//!
//! ```rust
//! use shpat::prelude::*;
//!
//! // An object whose drop is likely to take a long time
//! struct Heavy;
//!
//! let heavy = Heavy;
//!
//! heavy.quick_drop();
//! ```
//!
//! ### Traits required by `QuickDrop`
//!
//! The object on which `quick_drop` is called is moved to a new thread. As
//! such, it has to be `Send`. Additionaly, as `quick_drop` takes ownership of
//! it, the object has to be `Sized`.
//!
//! ## `Unwrappable`
//!
//! The `Unwrappable` trait is an attempt to unify the behavior of types which
//! represent a success or failure dichotomy, such as `Result` and `Option`.
//! These type implement a method which returns the success value, and panics
//! if it was a failure. These behaviours are unified with the `unwrap`
//! function.
//!
//! This trait is implemented for both `Result` and `Option`. It is closely
//! related to the `Try` trait from the standard library.

#![forbid(missing_docs)]
#![forbid(clippy::missing_docs_in_private_items)]
#![forbid(clippy::missing_errors_doc)]

mod apply;
mod quick_drop;
mod unwrappable;

pub mod prelude;
