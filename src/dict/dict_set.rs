use super::node::{DictNode, Word};
use crate::{
    adt::{queue::Queue, stack::Stack},
    dict::{
        error::{TryDowngrade, TryUpgrade},
        node::TryLink,
    },
    linked_list::{
        error::{TryAsMut, TryAsRef},
        list::LinkedList,
        node::NodePtr,
    },
};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct DictSetError;

impl Display for DictSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DictSetError")
    }
}

impl Error for DictSetError {}

pub struct DictSet {
    root: NodePtr<DictNode>,
    count: usize,
}

impl DictSet {
    pub fn new() -> Self {
        Self {
            root: DictNode::root(),
            count: 0,
        }
    }
    pub fn aho_corasick<S: AsRef<str>>(slice: &[S]) -> Result<Self, Box<dyn Error>> {
        let mut dict_set = DictSet::new();
        dict_set.insert_all(slice);
        dict_set.link_suffixes()?;
        Ok(dict_set)
    }
    pub fn word_count(&self) -> usize {
        self.count
    }
    pub fn insert(&mut self, word: &str) {
        let mut ptr = self.root.clone();
        for c in word.chars() {
            let mut next_ptr = None;
            if let Ok(mut ref_ptr) = ptr.try_as_mut() {
                if !ref_ptr.has_child(c) {
                    ref_ptr.add_child(c, &ptr);
                }
                next_ptr = ref_ptr.get_child(c);
            }
            ptr = next_ptr;
        }
        if let Ok(mut ref_ptr) = ptr.try_as_mut() {
            if let Word::No = ref_ptr.output {
                self.count += 1;
                ref_ptr.output = Word::Yes(word.into());
            }
        };
    }
    pub fn insert_all<S: AsRef<str>>(&mut self, slice: &[S]) {
        for string in slice {
            self.insert(string.as_ref());
        }
    }
    pub fn remove(&mut self, word: &str) -> Result<(), Box<dyn Error>> {
        let mut stack: Box<dyn Stack<NodePtr<DictNode>>> =
            Box::new(LinkedList::<NodePtr<DictNode>>::new());
        let mut ptr = self.root.clone();
        for c in word.chars() {
            if ptr.try_as_ref()?.has_child(c) {
                ptr = Some(ptr.clone().try_as_ref()?.get_child(c).unwrap());
                stack.push(ptr.clone());
            } else {
                return Err(Box::new(DictSetError));
            }
        }
        if ptr.try_as_ref()?.is_word() {
            ptr.try_as_mut()?.output = Word::No;
            if ptr.try_as_ref()?.children.is_empty() {
                while let Some(back_ptr) = stack.pop() {
                    let parent = back_ptr.try_parent()?;
                    if parent.try_as_ref()?.is_root() || parent.try_as_ref()?.is_word() {
                        parent
                            .try_as_mut()?
                            .children
                            .remove(&back_ptr.try_as_ref()?.state);
                        break;
                    }
                }
            }
        }
        Ok(())
    }
    pub fn link_suffixes(&mut self) -> Result<(), Box<dyn Error>> {
        let mut queue: Box<dyn Queue<NodePtr<DictNode>>> =
            Box::new(LinkedList::<NodePtr<DictNode>>::new());
        for root_child in self.root.try_as_mut()?.get_children() {
            root_child.try_as_mut()?.suffix = Some(self.root.try_downgrade()?);
            root_child.try_as_mut()?.get_children().for_each(|child| {
                queue.offer(child.clone());
            });
        }
        while let Some(node_ptr) = queue.poll() {
            let mut link = node_ptr.try_parent()?.try_suffix()?;
            let state = node_ptr.try_as_ref()?.state;
            while !link.try_as_ref()?.is_root() && !link.try_as_ref()?.has_child(state) {
                link = link.try_suffix()?;
            }
            if link.try_as_ref()?.has_child(state) {
                node_ptr.try_as_mut()?.suffix =
                    Some(link.try_as_ref()?.get_child(state).try_downgrade()?);
            } else {
                node_ptr.try_as_mut()?.suffix = Some(self.root.try_downgrade()?);
            }
            for child in node_ptr.try_as_ref()?.get_children() {
                queue.offer(child.clone());
            }
        }
        Ok(())
    }
    pub fn search<F: Fn(usize, &str)>(&self, pat: &str, on_match: F) -> Result<(), Box<dyn Error>> {
        let mut ptr = self.root.clone();
        for (i, c) in pat.chars().enumerate() {
            while !ptr.try_as_ref()?.is_root() && !ptr.try_as_ref()?.has_child(c) {
                ptr = ptr.try_suffix()?;
            }
            if ptr.try_as_ref()?.has_child(c) {
                ptr = Some(ptr.clone().try_as_ref()?.get_child(c).unwrap());
            }
            let mut check = ptr.clone();
            while !check.try_as_ref()?.is_root() {
                if let Word::Yes(word) = &check.try_as_ref()?.output {
                    on_match(i - word.len() + 1, &word);
                }
                check = check.try_suffix()?;
            }
        }
        Ok(())
    }
    pub fn contains(&self, seq: &str) -> bool {
        let mut ptr = self.root.clone();
        for c in seq.chars() {
            let mut next_ptr = None;
            if let Ok(ref_ptr) = ptr.try_as_ref() {
                if !ref_ptr.has_child(c) {
                    return false;
                }
                next_ptr = ref_ptr.get_child(c);
            }
            ptr = next_ptr;
        }
        if let Ok(ref_ptr) = ptr.try_as_ref() {
            match ref_ptr.output {
                Word::Yes(_) => return true,
                _ => return false,
            }
        };
        false
    }
    pub fn display(&self) {
        let mut queue: Box<dyn Queue<NodePtr<DictNode>>> =
            Box::new(LinkedList::<NodePtr<DictNode>>::new());
        queue.offer(self.root.clone());
        while let Some(node_ptr) = queue.poll() {
            if let Ok(node) = node_ptr.try_as_ref() {
                println!(
                    "Node: {:?}, Parent: {:?}, Suffix: {:?}",
                    node.state,
                    node.parent.try_upgrade().unwrap_or_default().borrow().state,
                    node.suffix.try_upgrade().unwrap_or_default().borrow().state
                );
                node.get_children().for_each(|child| {
                    queue.offer(child.clone());
                });
            }
        }
    }
}
