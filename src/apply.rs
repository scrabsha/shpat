use crate::unwrapable::Unwrapable;

pub trait Apply: Sized {
    fn apply_keep<A, F: FnOnce(&mut Self) -> A>(mut self, f: F) -> (Self, A) {
        let tmp = f(&mut self);
        (self, tmp)
    }

    fn apply<A, F: FnOnce(&mut Self) -> A>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }

    fn apply_unwrap<T, U: Unwrapable<T>, F: FnOnce(&mut Self) -> U>(mut self, f: F) -> Self {
        Unwrapable::unwrap(f(&mut self));
        self
    }
}

impl<T: Sized> Apply for T {}
