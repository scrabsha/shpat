//! Method chaining even if the method does not take ownership.
//!
//! Some methods in the standard library modify their object inner state. While
//! this is suitable in most cases, it breaks the method-chaining pattern. For
//! example, a `HashMap` must be declared and filled with this code:
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! let mut map = HashMap::new();
//! map.insert("hello", "world");
//! map.insert("animal", "farm");
//! ```
//!
//! The `Apply` trait can make this initialization clearer:
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! use shpat::prelude::*;
//!
//! let map = HashMap::new()
//!     .apply(|m| m.insert("hello", "world"))
//!     .apply(|m| m.insert("animal", "farm"));
//! ```

use crate::unwrappable::Unwrappable;

/// Allows to perform method chaining on functions which take reference.
pub trait Apply: Sized {
    /// Applies `f` to `self`, returning the modified `self`, and the value
    /// returned by the function.
    ///
    /// This can be useful if you care about the values returned by `f`:
    ///
    /// ```
    /// use shpat::prelude::*;
    ///
    /// let v = vec![1, 2, 3, 4];
    /// let (new_v, popped) = v.apply_keep(|v| v.pop());
    ///
    /// assert_eq!(new_v, [1, 2, 3]);
    /// assert_eq!(popped, Some(4));
    /// ```
    fn apply_keep<A, F: FnOnce(&mut Self) -> A>(mut self, f: F) -> (Self, A) {
        let tmp = f(&mut self);
        (self, tmp)
    }

    /// Applies `f` to `self`, discarding the value returned by `f`.
    ///
    /// The value returned by `f` is discarded. If it is needed, then
    /// `apply_keep` should be considered.
    ///
    /// ```
    /// use shpat::prelude::*;
    ///
    /// let v = vec![1, 2, 3];
    /// let v_pushed = v.apply(|v| v.push(4));
    /// 
    /// assert_eq!(v_pushed, [1, 2, 3, 4]);
    /// ```
    fn apply<A, F: FnOnce(&mut Self) -> A>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }

    /// Applies `f` to `self`, unwrapping the value returned by `Unwrappable`.
    ///
    /// This can be usefull if you need to ensure that `f` does not fail at
    /// runtime.
    ///
    /// # Panics
    ///
    /// This method is guaranteed to panic if `f` returns a panickable value.
    ///
    /// # Examples
    ///
    /// In the following example, each call to `apply_unwrap` returns a `Some`
    /// variant, so the following program does not panic:
    ///
    /// ```
    /// use shpat::prelude::*;
    ///
    /// let v = vec![1, 2, 3];
    /// v.apply_unwrap(Vec::pop)
    ///     .apply_unwrap(Vec::pop)
    ///     .apply_unwrap(Vec::pop);
    /// ```
    ///
    /// Here, an attempt to `pop` an empty `Vec` is performed. As such, a `None`
    /// variant is returned, which panics the program:
    ///
    /// ```should_panic
    /// use shpat::prelude::*;
    ///
    /// let v = vec![2];
    /// v.apply_unwrap(Vec::pop)
    ///     .apply_unwrap(Vec::pop);
    /// ```
    fn apply_unwrap<T, U: Unwrappable<T>, F: FnOnce(&mut Self) -> U>(mut self, f: F) -> Self {
        Unwrappable::unwrap(f(&mut self));
        self
    }
}

// Automatic implementation of the `Apply` trait for any sized type.
impl<T: Sized> Apply for T {}

#[cfg(test)]
mod apply {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn build_hashmap() {
        let left = HashMap::with_capacity(3)
            .apply(|m| m.insert("foo", 101))
            .apply(|m| m.insert("bar", 42))
            .apply(|m| m.insert("baz", 1969));

        let right = {
            let mut tmp = HashMap::with_capacity(3);
            tmp.insert("foo", 101);
            tmp.insert("bar", 42);
            tmp.insert("baz", 1969);
            tmp
        };
        assert_eq!(left, right);
    }

    #[test]
    fn replace_value_in_hashmap() {
        let tmp = HashMap::new().apply(|m| m.insert(42, '!'));

        let (left_map, inner_value) = tmp.apply_keep(|m| m.insert(42, '_'));
        let right_map = HashMap::new().apply(|m| m.insert(42, '_'));

        assert_eq!(left_map, right_map);
        assert_eq!(inner_value, Some('!'));
    }

    #[test]
    fn unwrap_non_panic_path() {
        let tmp = HashMap::new().apply(|m| m.insert(42, '!'));
        let left = tmp.apply_unwrap(|m| m.insert(42, '~'));
        let right = HashMap::new().apply(|m| m.insert(42, '~'));

        assert_eq!(left, right);
    }

    #[test]
    #[should_panic]
    fn unwrap_panic_path() {
        let _ = Vec::<()>::new().apply_unwrap(|v| v.pop());
    }
}
