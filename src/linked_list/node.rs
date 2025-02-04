use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

use crate::linked_list::error::ParseNodeError;
use super::error::{TryAsMut, TryAsRef};

pub type NodePtr<T> = Option<Rc<RefCell<T>>>;

pub struct ListNode<T: Clone> {
    pub data: T,
    pub next: NodePtr<ListNode<T>>
}

impl<T: Clone> ListNode<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }
    pub fn new_ptr(data: T) -> NodePtr<Self> {
        Some(Rc::new(RefCell::new(Self::new(data))))
    }
}

impl<T: Clone> TryAsRef<ListNode<T>> for NodePtr<ListNode<T>> {
    type Error = ParseNodeError;
    fn try_as_ref(&self) -> Result<Ref<'_, ListNode<T>>, Self::Error> {
        match self {
            Some(node_ptr) => Ok(node_ptr.borrow()),
            None => Err(ParseNodeError)
        }
    }
}

impl<T: Clone> TryAsMut<ListNode<T>> for NodePtr<ListNode<T>> {
    type Error = ParseNodeError;
    fn try_as_mut(&self) -> Result<RefMut<'_, ListNode<T>>, Self::Error> {
        match self {
            Some(node_ptr) => Ok(node_ptr.borrow_mut()),
            None => Err(ParseNodeError)
        }
    }
}