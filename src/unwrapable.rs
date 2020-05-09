use std::fmt::Debug;

pub trait Unwrapable<T>: Sized {
    fn unwrap(s: Self) -> T;
}

impl<T, E> Unwrapable<T> for Result<T, E>
where
    E: Debug,
{
    fn unwrap(s: Self) -> T {
        Result::unwrap(s)
    }
}

impl<T> Unwrapable<T> for Option<T> {
    fn unwrap(s: Self) -> T {
        Option::unwrap(s)
    }
}
