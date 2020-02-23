use crate::ai::Bot;
use crate::game_logic::field::{GameField};
use crate::game_logic::field::helper_structs::{GameRules};
use crate::game_logic::cache::{ShapeCache};
use std::sync::{Arc};
use crate::unsafe_mcts::fake_mutex::FakeMutex;

pub type NodePointer = Arc<Node>;
pub type ThreadSafeNode = Arc<FakeMutex<Node>>;

#[derive(Debug)]
pub struct Node {
  pub parent: Option<ThreadSafeNode>,
  pub children: Vec<ThreadSafeNode>,
  pub visits: f64,
  pub wins: Vec<u64>,
  pub rave_visits: f64,
  pub rave_wins: Vec<u64>,
  pub depth: u8,
  pub index: u8,
  pub expanded: bool
}

impl PartialEq for Node {
  fn eq(&self, other: &Node) -> bool {
    self.depth == other.depth && self.index == other.index
  }
}

impl Node {
  pub fn new(parent: Option<ThreadSafeNode>, depth: u8, index: u8, child_size: usize) -> ThreadSafeNode {
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
    let pointer = Arc::new(FakeMutex::new(n));
    {
      for _ in 0..child_size {
        pointer.lock().children.push(pointer.clone());
      }
    }
    return pointer
  }
  pub fn child_is_parent(node: &ThreadSafeNode, child_index: usize) -> bool {
    let readable_node = node.lock();
    let child = &readable_node.children[child_index];
    *readable_node == *child.lock()
  }
  pub fn get_child(node: &ThreadSafeNode, child_index: usize) -> ThreadSafeNode {
    let readable_leaf = node.lock();
    readable_leaf.children[child_index].clone()
  }
  pub fn set_child(node: &ThreadSafeNode, child_index: usize, new_child: ThreadSafeNode) {
    let mut n = node.lock();
    n.children[child_index] = new_child;
  }
  pub fn create_child(node: &ThreadSafeNode, play_index: u8, child_size: usize) -> ThreadSafeNode {
    let mut writeable_leaf = node.lock();
    let new_child = Node::new(Some(node.clone()), writeable_leaf.depth + 1, play_index, child_size);
    writeable_leaf.children[play_index as usize] = new_child.clone();
    new_child
  }
}

pub trait TreeBot: Send + Sync {
  fn get_rules(&self) -> GameRules;
  fn get_shape_cache(&self) -> Arc<ShapeCache>;
  fn get_player(&self) -> u8;
  fn get_score_type(&self) -> u8;
  fn make_tree_play(&mut self, field: &GameField) -> u8;
  fn get_root(&mut self) -> Arc<FakeMutex<Node>>;
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
        parent_arc = Node::create_child(&parent_arc, *i, field_findhelper.possible_plays.len());
      }
    }
    parent_arc
  }
}

impl Bot for dyn TreeBot {
  fn make_play(&mut self, field: &GameField, _: bool) -> u8 {
    self.make_tree_play(field)
  }
  fn get_name(&self) -> String {"unsafe_tree".to_owned()}
}