use std::fmt::Debug;

pub trait Unwrappable<T>: Sized {
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
