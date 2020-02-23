use crate::ai::Bot;
use crate::game_logic::field::{GameField};
use crate::game_logic::field::helper_structs::{GameRules};
use crate::game_logic::cache::{ShapeCache};
use std::sync::{Arc,RwLock};
use std::thread::sleep;
use std::time::Duration;

pub type NodePointer = Arc<Node>;
pub type ThreadSafeNode = Arc<RwLock<Node>>;

#[derive(Debug,Clone)]
pub struct Node {
  pub parent: Option<ThreadSafeNode>,
  pub children: Vec<ThreadSafeNode>,
  pub visits: f64,
  pub wins: Vec<u64>,
  pub rave_visits: f64,
  pub rave_wins: Vec<u64>,
  pub depth: u8,
  pub index: usize,
  pub expanded: bool
}

impl PartialEq for Node {
  fn eq(&self, other: &Node) -> bool {
    self.depth == other.depth && self.index == other.index
  }
}

impl Node {
  pub fn new(parent: Option<ThreadSafeNode>, depth: u8, index: usize, child_size: usize) -> ThreadSafeNode {
    let children = Vec::with_capacity(child_size);
    let mut wins = Vec::with_capacity(4);
    let mut rave_wins = Vec::with_capacity(4);
    for _ in 0..4 {
      wins.push(0);
      rave_wins.push(0);
    }
    let n = Node {
      parent,
      children,
      index,
      depth,
      wins,
      rave_wins,
      visits: 1.0,
      rave_visits: 1.0,
      expanded: false
    };
    let pointer = Arc::new(RwLock::new(n.clone()));
    {
      let mut un = pointer.write().unwrap();
      for _ in 0..child_size {
        un.children.push(pointer.clone());
      }
    }
    return pointer
  }
  pub fn child_is_parent(node: &ThreadSafeNode, child_index: usize) -> bool {
    let mut leaf_attempt = node.try_read();
    while leaf_attempt.is_err() {
      sleep(Duration::from_nanos(10));
      leaf_attempt = node.try_read();
    }
    let readable_leaf = leaf_attempt.unwrap();
    let child = &readable_leaf.children[child_index];
    let mut child_attempt = child.try_read();
    while child_attempt.is_err() {
      sleep(Duration::from_nanos(10));
      child_attempt = child.try_read();
    }
    let readable_child = child_attempt.unwrap();
    *readable_leaf == *readable_child
  }
  pub fn get_child(node: &ThreadSafeNode, child_index: usize) -> ThreadSafeNode {
    let mut leaf_attempt = node.try_read();
    while leaf_attempt.is_err() {
      sleep(Duration::from_nanos(10));
      leaf_attempt = node.try_read();
    }
    let readable_leaf = leaf_attempt.unwrap();
    readable_leaf.children[child_index].clone()
  }
  pub fn set_child(node: &ThreadSafeNode, child_index: usize, new_child: ThreadSafeNode) {
    let mut attempt = node.try_write();
    while attempt.is_err() {
      sleep(Duration::from_nanos(10));
      attempt = node.try_write();
    }
    let mut n = attempt.unwrap();
    n.children[child_index] = new_child;
  }
  pub fn create_child(node: &ThreadSafeNode, play_index: usize, child_size: usize) -> ThreadSafeNode {
    let mut attempt = node.try_write();
    while attempt.is_err() {
      sleep(Duration::from_nanos(10));
      attempt = node.try_write();
    }
    let mut writeable_leaf = attempt.unwrap();
    let new_child = Node::new(Some(node.clone()), writeable_leaf.depth + 1, play_index, child_size);
    writeable_leaf.children[play_index] = new_child.clone();
    new_child
  }
  pub fn print(node: &ThreadSafeNode) {
    let mut attempt = node.try_read();
    while attempt.is_err() {
      sleep(Duration::from_nanos(10));
      attempt = node.try_read();
    }
    let n = attempt.unwrap();
    println!("Node {}/{} v{} w{:?}", n.index, n.depth, n.visits, n.wins);
    for i in 0..n.children.len() {
      if !Node::child_is_parent(node, i) {
        Node::print(&n.children[i]);
      }
    }
  }
}

pub trait TreeBot: Send + Sync {
  fn get_rules(&self) -> GameRules;
  fn get_shape_cache(&self) -> Arc<ShapeCache>;
  fn get_player(&self) -> u8;
  fn get_score_type(&self) -> u8;
  fn make_tree_play(&mut self, field: &GameField) -> u8;
  fn get_root(&mut self) -> Arc<RwLock<Node>>;
  fn get_score(&self, field: &GameField) -> i16;
  fn find_node(&mut self, field: &GameField) -> ThreadSafeNode {
    let mut parent_arc = self.get_root();
    let mut field_findhelper = field.empty_clone();
    for i in field.played_indices.iter() {
      let i_usize = *i as usize;
      field_findhelper.place_block_using_play(*i);
      if !Node::child_is_parent(&parent_arc, i_usize) {
        parent_arc = Node::get_child(&parent_arc, i_usize);
      } else {
        parent_arc = Node::create_child(&parent_arc, i_usize, field_findhelper.possible_plays.len());
      }
    }
    parent_arc
  }
}

impl Bot for dyn TreeBot {
  fn make_play(&mut self, field: &GameField, _: bool) -> u8 {
    self.make_tree_play(field)
  }
  fn get_name(&self) -> String {"treebot".to_owned()}
}