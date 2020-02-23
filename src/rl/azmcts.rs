use crate::ai::Bot;
use crate::game_logic::field::{GameField};
use crate::game_logic::field::helper_structs::{GameRules};
use crate::game_logic::field::render::{Renderable};
use crate::game_logic::cache::{ShapeCache};
use std::sync::{Arc,RwLock};
use std::thread::sleep;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tch::{nn::{Module},Tensor,Kind};
use crate::rl::aznet::{AlphaZeroNet, get_value, get_values, get_values_0d};
use random_choice::random_choice;

pub type NodePointer = Arc<Node>;
pub type ThreadSafeNode = Arc<RwLock<Node>>;

#[derive(Debug,Clone)]
pub struct Node {
  pub parent: Option<ThreadSafeNode>,
  pub children: Vec<ThreadSafeNode>,
  pub visits: f64,
  pub wins: Vec<u64>,
  pub Q: f64,
  pub u: f64,
  pub prior: f64,
  pub depth: u8,
  pub index: usize,
  pub expanded: bool
}

impl PartialEq for Node {
  fn eq(&self, other: &Node) -> bool {
    self.depth == other.depth && self.index == other.index
  }
}

#[macro_export]
macro_rules! try_read {
    ($obj:expr) => {{
      let mut attempt = $obj.try_read();
      while attempt.is_err() {
        sleep(Duration::from_nanos(10));
        attempt = $obj.try_read();
      }
      attempt.unwrap()}
    };
}

#[macro_export]
macro_rules! try_write {
    ($obj:expr) => {{
      let mut attempt = $obj.try_write();
      while attempt.is_err() {
        sleep(Duration::from_nanos(10));
        attempt = $obj.try_write();
      }
      attempt.unwrap()}
    };
}

impl Node {
  pub fn new(parent: Option<ThreadSafeNode>, depth: u8, index: usize, child_size: usize) -> ThreadSafeNode {
    let children = Vec::with_capacity(child_size);
    let mut wins = Vec::with_capacity(4);
    for _ in 0..4 {
      wins.push(0);
    }
    let n = Node {
      parent,
      children,
      index,
      depth,
      wins,
      visits: 1.0,
      expanded: false,
      Q: 0.0,
      u: 0.0,
      prior: 1.0
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
    let readable_leaf = try_read!(node);
    let child = &readable_leaf.children[child_index];
    let readable_child = try_read!(child);
    *readable_leaf == *readable_child
  }
  pub fn get_child(node: &ThreadSafeNode, child_index: usize) -> ThreadSafeNode {
    let readable_leaf = try_read!(node);
    readable_leaf.children[child_index].clone()
  }
  pub fn set_child(node: &ThreadSafeNode, child_index: usize, new_child: ThreadSafeNode) {
    let mut n = try_write!(node);
    n.children[child_index] = new_child;
  }
  pub fn create_child(node: &ThreadSafeNode, play_index: usize, child_size: usize) -> ThreadSafeNode {
    let mut writeable_leaf = try_write!(node);
    let new_child = Node::new(Some(node.clone()), writeable_leaf.depth + 1, play_index, child_size);
    writeable_leaf.children[play_index] = new_child.clone();
    new_child
  }
  pub fn expand(node: &ThreadSafeNode, priors: Vec<f64>) {
    {
      let mut wn = try_write!(node);
      wn.expanded = true;
    }
    let mut i = 0;
    for prior_value in priors {
      if prior_value != 0.0 && Node::child_is_parent(node, i) {
        let new_child = Node::create_child(node, i, 190);
        let mut writeable_child = try_write!(new_child);
        writeable_child.prior = prior_value;
      }
      i += 1;
    }
  }
  pub fn update(node: &ThreadSafeNode, leaf_value: f64) {
    let mut writeable_node = try_write!(node);
    writeable_node.visits += 1.0;
    writeable_node.Q += 1.0*(leaf_value-writeable_node.Q) / writeable_node.visits;
  }
  pub fn update_recursive(node: &ThreadSafeNode, leaf_value: f64) {
    let parent;
    {
      let readable_node = try_read!(node);
      parent = readable_node.parent.clone();
    }
    if parent.is_some() {
      Node::update_recursive(&parent.unwrap(), -leaf_value); // todo verify minus is correct here
    }
    Node::update(node, leaf_value);
  }
  pub fn select(node: &ThreadSafeNode, c_puct: f64) -> u8 {
    let mut action: u8 = 0;
    let mut action_max = -1.0;
    for i in 0..190 {
      if !Node::child_is_parent(node, i) {
        let child_value = Node::get_value(&Node::get_child(node, i), c_puct);
        if child_value > action_max {
          action_max = child_value;
          action = i as u8;
        }
      }
    }
    if action_max == -1.0 {
      let rn = try_read!(node);
      panic!("Tried to select from non expanded node {}", rn.expanded);
    }
    action
  }
  pub fn get_value(node: &ThreadSafeNode, c_puct: f64) -> f64 {
    let mut rn = try_write!(node);
    let mut parent_visits = 1.0;
    if rn.parent.is_some() {
      let parent = rn.parent.clone().unwrap();
      let rp = try_read!(parent);
      parent_visits = rp.visits;
    }
    rn.u = (c_puct * rn.prior * parent_visits) / (1.0+rn.visits);
    rn.Q + rn.u
  }
}

pub struct AZMCTS {
  pub root: ThreadSafeNode,
  pub policy: u8,
  pub c_puct: f64,
  pub playout_amount: u128,
  pub net: AlphaZeroNet,
  pub player_num: i8,
  pub exploration_constant: f64
}

fn filter_invalid_moves(policy_values: Vec<f64>, field: &GameField) -> Vec<f64> {
  let mut filtered_policy_values = vec![0.0;policy_values.len()];
  for play in field.possible_plays.iter() {
    filtered_policy_values[play.global_index as usize] = policy_values[play.global_index as usize];
  }
  filtered_policy_values
}

fn timestamp() -> u128 {
  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}
impl AZMCTS {
  pub fn new(net: AlphaZeroNet) -> AZMCTS {
    let root = Node::new(Option::None, 0, 0, 190);
    AZMCTS {
      root,
      policy: 0,
      c_puct: 5.0,
      playout_amount: 10,
      net,
      player_num: 0,
      exploration_constant: 1e-3
    }
  }
  pub fn playout(&mut self, field: &mut GameField) {
    let mut node = self.root.clone();
    let mut is_leaf;
    {
      let rn = try_read!(node);
      is_leaf = !rn.expanded;
    }
    while !is_leaf && !field.game_over { // TODO: verify this works
      let action = Node::select(&node, self.c_puct);
      let placed = field.place_block_using_play_global_index(action);
      if !placed {
        let mut plays = Vec::new();
        for play in field.possible_plays.iter() {
          plays.push(play.global_index);
        }
        println!("ACTION IS {}, plays are {:?}", action, plays);
        panic!("leaf that should be placeable wasnt placeable {} {}", action, field.possible_plays.len());
      }
      if Node::child_is_parent(&node, action as usize) {
        panic!("Choosing parent again for something that should be the next move!");
      }
      node = Node::get_child(&node, action as usize);
      {
        let rn = try_read!(node);
        is_leaf = !rn.expanded;
      }
    }
    let img = field.to_image(field.current_player == 1);
    let input = Tensor::of_slice(img.as_slice()).view([3,32,32]);
    let (value, policy) = self.net.custom_forward(&input, false);
    let mut leaf_value = get_value(&value);
    if field.game_over {
      let winner = field.get_winning_player();
      if winner == 3 {
        leaf_value = 0.0;
      } else {
        leaf_value = if winner == self.player_num {1.0} else {-1.0};
      }
    } else {
      Node::expand(&node, filter_invalid_moves(get_values(&policy), field));
    }
    Node::update_recursive(&node, -leaf_value); // TODO: verify minus is correct here
  }
  pub fn get_move_probs(&mut self, field: &mut GameField) -> (Vec<i64>, Vec<f64>)  {
    // let start = timestamp();
    for _ in 0..self.playout_amount {
      let mut cloned_field = field.empty_clone();
      cloned_field.copy_from(&field);
      self.playout(&mut cloned_field);
    }
    // let end = timestamp();
    // println!("PLAYOUTS TOOK {} ms ", end - start);
    let mut actions = Vec::with_capacity(190);
    let mut visits = Vec::with_capacity(190);
    for i in 0..190 {
      if !Node::child_is_parent(&self.root, i) {
        actions.push(i as i64);
        let child = Node::get_child(&self.root, i);
        let readable_child = try_read!(child);
        visits.push(readable_child.visits);
      } else {
        visits.push(0.0);
        actions.push(-1);
      }
    }
    let mut action_probabilities = Vec::with_capacity(visits.len());
    for action_index in actions.iter() {
      if *action_index == -1 || visits[*action_index as usize] == 0.0 {
        action_probabilities.push(0.0);
      } else {
        action_probabilities.push((1.0 / self.exploration_constant) * (visits[*action_index as usize]+1e-10).ln());
      }
    }
    let prob_tens = Tensor::of_slice(action_probabilities.as_slice()).softmax(0, Kind::Double);
    (actions, filter_invalid_moves(get_values_0d(&prob_tens), field))
  }
  pub fn update_with_move(&mut self, last_move: i64) {
    if last_move == -1 {
      self.root = Node::new(Option::None, 0, 0, 190);
    } else {
      if !Node::child_is_parent(&self.root, last_move as usize) {
        self.root = Node::get_child(&self.root, last_move as usize);
        let mut writeable_root = try_write!(self.root);
        writeable_root.parent = Option::None;
      } else {
        panic!("ERROR MOVING TREE!");
      }
    }
  }
  pub fn get_action(&mut self, field: &mut GameField, is_self_play: bool) -> u8 {
    let res = self.get_action_with_probs(field, is_self_play);
    return res.0;
  }
  pub fn get_action_with_probs(&mut self, field: &mut GameField, is_self_play: bool) -> (u8, Vec<f64>) {
    let mut move_probs: Vec<f64> = vec![0.0; 190];
    if !field.game_over {
      let (actions, probabilities) = self.get_move_probs(field);
      for action in actions.iter() {
        if *action >= 0 {
          move_probs[*action as usize] = probabilities[*action as usize];
        }
      }
      if is_self_play {
        // todo: add dirichlet noise to weights
        let chosen_moves: Vec<&i64> = random_choice().random_choice_f64(&actions, &move_probs, 1);
        let chosen_move = chosen_moves[0];
        self.update_with_move(*chosen_move);
        return (*chosen_move as u8, move_probs);
      } else {
        let chosen_moves: Vec<&i64> = random_choice().random_choice_f64(&actions, &move_probs, 1);
        let chosen_move = chosen_moves[0];
        self.update_with_move(*chosen_move);
        return (*chosen_move as u8, move_probs);
      }
    }
    return (0, move_probs);
  }
}
