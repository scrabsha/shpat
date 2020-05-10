//! Unifies the behaviour of types which represent a success or failure
//! dichotomy.
//!
//! The goal is to be able to call unwrap on these these types, and get the
//! underlying success value, or panic the program.
//!
//! This trait is closely tied to the `Try` trait.

use std::fmt::Debug;

/// Unifies the behaviour of types which represent a success or a failure.
pub trait Unwrappable<T>: Sized {
    /// Returns the underlying value, or panics the program if `self` is a
    /// failure.
    fn unwrap(s: Self) -> T;
}

impl<T, E> Unwrappable<T> for Result<T, E>
where
    E: Debug,
{
    fn unwrap(s: Self) -> T {
        Result::unwrap(s)
    }
}

impl<T> Unwrappable<T> for Option<T> {
    fn unwrap(s: Self) -> T {
        Option::unwrap(s)
    }
}

#[cfg(test)]
mod result {
    use super::*;

    #[test]
    fn unwrap_ok_path() {
        let r: Result<_, ()> = Ok(42);
        assert_eq!(Unwrappable::unwrap(r), 42);

        let r: Result<(), ()> = Ok(());
        assert_eq!(Unwrappable::unwrap(r), ());
    }

    #[test]
    #[should_panic]
    fn unwrap_err_path() {
        let r: Result<(), _> = Err(42);
        Unwrappable::unwrap(r);
    }
}

#[cfg(test)]
mod option {
    use super::*;

    #[test]
    fn unwrap_some_path() {
        let o = Some(42);
        assert_eq!(Unwrappable::unwrap(o), 42);

        let o = Some(());
        assert_eq!(Unwrappable::unwrap(o), ());
    }

    #[test]
    #[should_panic]
    fn unwrap_none_path() {
        let o: Option<()> = None;
        Unwrappable::unwrap(o);
    }
}
