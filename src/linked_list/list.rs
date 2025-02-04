use std::{fmt::Display, mem};
use crate::adt::{queue::Queue, stack::Stack};

use super::{error::{TryAsMut, TryAsRef}, node::{ListNode, NodePtr}};

pub struct LinkedList<T: Clone> {
    head: NodePtr<ListNode<T>>,
    tail: NodePtr<ListNode<T>>,
    size: usize
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None, size: 0 }
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn push(&mut self, item: T) {
        let node_ptr = ListNode::new_ptr(item);
        if self.is_empty() {
            self.tail = node_ptr.clone();
        }
        node_ptr.try_as_mut().unwrap().next = mem::take(&mut self.head);
        self.size += 1;
        self.head = node_ptr;
    }
    pub fn pop(&mut self) -> Option<T> {
        mem::take(&mut self.head).map(|node_ptr| {
            self.head = node_ptr.borrow().next.clone();
            self.size -= 1;
            if self.is_empty() {
                self.tail = None;
            }
            node_ptr.borrow().data.clone()
        })
    }
    pub fn peek(&self) -> Option<T> {
        match self.head.try_as_ref() {
            Ok(node) => Some(node.data.clone()),
            Err(_) => None
        }
    }
    pub fn append(&mut self, item: T) {
        if self.is_empty() {
            self.push(item);
        } else {
            let node = ListNode::new_ptr(item);
            self.tail.try_as_mut().unwrap().next = node.clone();
            self.tail = node;
            self.size += 1;
        }
    }
    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.size = 0;
    }
}

impl<T: Display + Clone> LinkedList<T> {
    pub fn display(&self) {
        print!("[");
        let mut ptr = self.head.clone();
        while let Some(next) = ptr {
            print!("{}", next.borrow().data);
            if next.borrow().next.is_some() {
                print!(", ");
            }
            ptr = next.borrow().next.clone();
        }
        print!("]\n");
    }
}

impl<T: Clone> Stack<T> for LinkedList<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn is_full(&self) -> bool {
        false
    }
    fn size(&self) -> usize {
        self.len()
    }
    fn peek(&self) -> Option<T> {
        self.peek()
    }
    fn push(&mut self, item: T) {
        self.push(item);
    }
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<T: Clone> Queue<T> for LinkedList<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn is_full(&self) -> bool {
        false
    }
    fn size(&self) -> usize {
        self.len()
    }
    fn peek(&self) -> Option<T> {
        self.peek()
    }
    fn offer(&mut self, item: T) {
        self.append(item);
    }
    fn poll(&mut self) -> Option<T> {
        self.pop()
    }
}