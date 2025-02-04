use std::{
    cell::{Ref, RefCell, RefMut}, collections::HashMap, error::Error, rc::{Rc, Weak}
};

use crate::linked_list::{
    error::{ParseNodeError, TryAsMut, TryAsRef},
    node::NodePtr
};

use super::error::{ParseWeakError, TryUpgrade, TryDowngrade};

pub type WeakPtr<T> = Option<Weak<RefCell<T>>>;

#[derive(Default)]
pub enum Word {
    #[default]
    No,
    Root,
    Yes(String)
}

#[derive(Default)]
pub struct DictNode {
    pub state: char,
    pub output: Word,
    pub parent: WeakPtr<DictNode>,
    pub suffix: WeakPtr<DictNode>,
    pub children: HashMap<char, NodePtr<DictNode>>
}

impl DictNode {
    pub fn new(state: char) -> Self {
        Self {
            state,
            output: Word::No,
            parent: None,
            suffix: None,
            children: HashMap::new()
        }
    }
    pub fn new_ptr(state: char) -> NodePtr<Self> {
        Some(Rc::new(RefCell::new(Self::new(state))))
    }
    pub fn root() -> NodePtr<Self> {
        let mut node = DictNode::new('\0');
        node.output = Word::Root;
        Some(Rc::new(RefCell::new(node)))
    }
    pub fn is_root(&self) -> bool {
        match self.output {
            Word::Root => true,
            _ => false
        }
    }
    pub fn has_child(&self, state: char) -> bool {
        self.children.get(&state).is_some()
    }
    pub fn add_child(&mut self, state: char, parent: &NodePtr<DictNode>) {
        let mut child = DictNode::new(state);
        child.parent = Some(parent.try_downgrade().unwrap());
        self.children.insert(state, Some(Rc::new(RefCell::new(child))));
    }
    pub fn get_child(&self, state: char) -> NodePtr<Self> {
        match self.children.get(&state) {
            Some(child) => child.clone(),
            None => None
        }
    }
    pub fn get_children(&self) -> impl Iterator<Item = &NodePtr<DictNode>> {
        self.children.values()
    }
    pub fn get_children_mut(&mut self) -> impl Iterator<Item = &mut NodePtr<DictNode>> {
        self.children.values_mut()
    }
}

impl TryAsRef<DictNode> for NodePtr<DictNode> {
    type Error = ParseNodeError;
    fn try_as_ref(&self) -> Result<Ref<'_, DictNode>, Self::Error> {
        match self {
            Some(node_ptr) => Ok(node_ptr.borrow()),
            None => Err(ParseNodeError)
        }
    }
}

impl TryAsMut<DictNode> for NodePtr<DictNode> {
    type Error = ParseNodeError;
    fn try_as_mut(&self) -> Result<RefMut<'_, DictNode>, Self::Error> {
        match self {
            Some(node_ptr) => Ok(node_ptr.borrow_mut()),
            None => Err(ParseNodeError)
        }
    }
}

impl TryUpgrade<DictNode> for WeakPtr<DictNode> {
    type Error = ParseWeakError;
    fn try_upgrade(&self) -> Result<Rc<RefCell<DictNode>>, Self::Error> {
        match self {
            Some(weak_ptr) => {
                match weak_ptr.upgrade() {
                    Some(strong_ptr) => Ok(strong_ptr),
                    None => Err(ParseWeakError)
                }
            },
            None => Err(ParseWeakError)
        }
    }
}

impl TryDowngrade<DictNode> for NodePtr<DictNode> {
    type Error = ParseNodeError;
    fn try_downgrade(&self) -> Result<Weak<RefCell<DictNode>>, Self::Error> {
        match self {
            Some(node_ptr) => Ok(Rc::downgrade(&node_ptr)),
            None => Err(ParseNodeError)
        }
    }   
}

pub trait TryLink {
    type Error;
    fn try_parent(&self) -> Result<NodePtr<DictNode>, Self::Error>;
    fn try_suffix(&self) -> Result<NodePtr<DictNode>, Self::Error>;
}

impl TryLink for NodePtr<DictNode> {
    type Error = Box<dyn Error>;
    fn try_parent(&self) -> Result<NodePtr<DictNode>, Self::Error> {
        match self.try_as_ref() {
            Ok(node_ptr) => {
                match &node_ptr.parent {
                    Some(parent) => Ok(parent.upgrade()),
                    None => Err(Box::new(ParseWeakError))
                }
            },
            Err(_) => Err(Box::new(ParseNodeError))
        }
    }
    fn try_suffix(&self) -> Result<NodePtr<DictNode>, Self::Error> {
        match self.try_as_ref() {
            Ok(node_ptr) => {
                match &node_ptr.suffix {
                    Some(suffix) => Ok(suffix.upgrade()),
                    None => Err(Box::new(ParseWeakError))
                }
            },
            Err(_) => Err(Box::new(ParseNodeError))
        }
    }
}