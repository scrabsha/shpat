//! A trait for dropping heavy objects in a new thread.

use std::thread;

/// A trait for dropping heavy objects in a new thread.
///
/// This trait is inspired by [a blog
/// post](https://abramov.io/rust-dropping-things-in-another-thread) written by
/// [Aaron Abramov](https://github.com/aaronabramov/), in which they show that
/// running `drop` may slow down a program.
///
/// # Example
///
/// ```rust
/// use shpat::prelude::*;
///
/// // An object whose drop is likely to take a long time
/// struct Heavy;
///
/// let heavy = Heavy;
///
/// heavy.quick_drop();
/// ```
pub trait QuickDrop: Sized + Send + 'static {
    /// Drops an object in a newly spawned thread.
    fn quick_drop(self) {
        thread::spawn(move || drop(self));
    }
}

impl<T: Sized + Send + 'static> QuickDrop for T {}

#[cfg(test)]
mod simple_object {
    use super::*;

    #[allow(dead_code)]
    struct S {
        a: u8,
        b: u32,
        c: char,
    }

    #[test]
    fn quick_drop() {
        let s = S {
            a: 42,
            b: 101,
            c: '_',
        };

        s.quick_drop();
    }
}

#[cfg(test)]
mod with_generic_type {
    use super::*;

    #[allow(dead_code)]
    struct S<T> {
        val: T,
    }

    #[test]
    fn quick_drop() {
        let s = S {
            val: String::from("foo"),
        };

        s.quick_drop();
    }
}

#[cfg(test)]
mod with_static_lifetime {
    use super::*;

    #[allow(dead_code)]
    struct S<'a> {
        val: &'a usize,
    }

    #[test]
    fn quick_drop() {
        let s: S<'static> = S {
            val: &42,
        };

        s.quick_drop();
    }
}
