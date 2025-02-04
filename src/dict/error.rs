use std::error::Error;
use std::fmt::Display;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
pub struct ParseWeakError;

impl Display for ParseWeakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseWeakError")
    }
}

impl Error for ParseWeakError {}

pub trait TryUpgrade<T> {
    type Error;
    fn try_upgrade(&self) -> Result<Rc<RefCell<T>>, Self::Error>;
}

pub trait TryDowngrade<T> {
    type Error;
    fn try_downgrade(&self) -> Result<Weak<RefCell<T>>, Self::Error>;
}