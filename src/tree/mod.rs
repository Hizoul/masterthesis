
use std::sync::{Arc,RwLock};
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::{SerializeStruct};

pub type NodePointer = Arc<Node>;
pub type ThreadSafeNode = Arc<RwLock<Node>>;

#[derive(Debug,Clone,Deserialize)]
pub struct Node {
  pub parent: Option<ThreadSafeNode>,
  pub children: Vec<ThreadSafeNode>,
  pub score: Vec<i16>,
  pub played_indices: Vec<u8>,
  pub expanded: bool
}


impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
      let mut children = Vec::with_capacity(self.children.len());
      for child in self.children.iter() {
        let child_is_parent;
        {
          let newchild = child.read().unwrap();
          child_is_parent = *newchild == *self;
        }
        if !child_is_parent {
          children.push(child);
        }
      }
      let mut state = serializer.serialize_struct("Node", 5)?;
      // state.serialize_field("parent", &self.parent)?;
      state.serialize_field("children", &children)?;
      state.serialize_field("score", &self.score)?;
      state.serialize_field("played_indices", &self.played_indices)?;
      state.serialize_field("expanded", &self.expanded)?;
      state.end()
    }
}

impl PartialEq for Node {
  fn eq(&self, other: &Node) -> bool {
    self.played_indices == other.played_indices
  }
}

impl Node {
  pub fn new(parent: Option<ThreadSafeNode>, child_size: Option<usize>, played_indices: Vec<u8>) -> ThreadSafeNode {
    let children = if child_size.is_some() { Vec::with_capacity(child_size.unwrap()) } else { Vec::new() };
    let mut score = Vec::with_capacity(4);
    for _ in 0..4 {
      score.push(0);
    }
    let n = Node {
      parent,
      children,
      score,
      played_indices,
      expanded: false
    };
    let pointer = Arc::new(RwLock::new(n.clone()));
    if child_size.is_some() {
      for _ in 0..child_size.unwrap() {
        let mut un = pointer.write().unwrap();
        un.children.push(pointer.clone());
      }
    }
    return pointer
  }
  pub fn new_safe_score(score: i16, player_index: u8, parent: Option<ThreadSafeNode>, child_size: Option<usize>, played_indices: Vec<u8>) -> ThreadSafeNode {
    let n = Node::new(parent, child_size, played_indices);
    let mut un = n.write().unwrap();
    un.score[player_index as usize] = score;
    n.clone()
  }
}