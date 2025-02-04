use std::fmt::Display;
use std::error::Error;
use std::cell::{Ref, RefMut};

#[derive(Debug)]
pub struct ParseNodeError;

impl Display for ParseNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseNodeError")
    }
}

impl Error for ParseNodeError {}

pub trait TryAsRef<T> {
    type Error;
    fn try_as_ref(&self) -> Result<Ref<'_, T>, Self::Error>;
}

pub trait TryAsMut<T> {
    type Error;
    fn try_as_mut(&self) -> Result<RefMut<'_, T>, Self::Error>;
}