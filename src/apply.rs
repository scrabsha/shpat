use crate::unwrappable::Unwrappable;

pub trait Apply: Sized {
    fn apply_keep<A, F: FnOnce(&mut Self) -> A>(mut self, f: F) -> (Self, A) {
        let tmp = f(&mut self);
        (self, tmp)
    }

    fn apply<A, F: FnOnce(&mut Self) -> A>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }

    fn apply_unwrap<T, U: Unwrappable<T>, F: FnOnce(&mut Self) -> U>(mut self, f: F) -> Self {
        Unwrappable::unwrap(f(&mut self));
        self
    }
}

impl<T: Sized> Apply for T {}
