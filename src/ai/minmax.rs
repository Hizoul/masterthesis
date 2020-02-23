use crate::ai::{Bot,TreeBot};
use crate::tree::{ThreadSafeNode,Node};
use crate::game_logic::field::{GameField};
use crate::game_logic::field::helper_structs::{GameRules};
use crate::game_logic::val::MAX_PLAYS;
use std::cmp::{min,max};
use std::sync::{Arc,RwLock};
use std::collections::HashMap;

pub struct MinMaxBot {
  depth: u128,
  root: ThreadSafeNode,
  played: Vec<u8>,
  player: u8,
  transposition_table: Arc<RwLock<HashMap<Vec<u8>, ThreadSafeNode>>>
}

impl MinMaxBot {
  pub fn new(player: u8, depth: u128) -> MinMaxBot {
    MinMaxBot {player,depth, root: Node::new_safe_score(0, player, None, Some(MAX_PLAYS as usize), Vec::new()),played: Vec::new(),
      transposition_table: Arc::new(RwLock::new(HashMap::new()))}
  }
  pub fn minmax_algo(&self, node: ThreadSafeNode, parent_to_stop_at: ThreadSafeNode) -> i16 {
    // wenn keine kinder übernimm score von node selbst und ruf minmax auf parent auf
    let readable_node = node.read().unwrap();
    let amount_of_children = readable_node.children.len();
    let depth = readable_node.played_indices.len();
    let has_been_expanded = readable_node.expanded;
    let node_score = readable_node.score[self.get_player() as usize];
    if !has_been_expanded || amount_of_children == 0 {
      return node_score;
    } else {
      // wenn kinder dann für alle über diese funktion value holen und vergleichen
      let mut relevant_index = 0;
      let mut current_index = 0;
      let is_maximizing = depth % 2 == self.get_player() as usize;
      let mut new_val = if is_maximizing { i16::min_value() } else { i16::max_value() };
      let mut prev_val = -1;
      for child in readable_node.children.iter() {
        let child_is_parent;
        {
          let newchild = child.read().unwrap();
          child_is_parent = *newchild == *readable_node;
        }
        if !child_is_parent {
          let childs_minmax = self.minmax_algo(child.clone(), parent_to_stop_at.clone());
          new_val = if is_maximizing { max(new_val, childs_minmax) } else { min(new_val, childs_minmax)}; 
          if new_val != prev_val {
            prev_val = new_val;
            relevant_index = current_index;
          }
          current_index += 1;
        }
      }
      
      let is_terminal_node;
      {
        let current = node.read().unwrap();
        let terminal = parent_to_stop_at.read().unwrap();
        is_terminal_node = *current == *terminal;
      }
      if is_terminal_node {
        return relevant_index;
      }
      return new_val;
    }
  }
}

impl TreeBot for MinMaxBot {
  fn get_rules(&self) -> GameRules {GameRules::deterministic()}
  fn get_player(&self) -> u8 {self.player}
  fn get_score_type(&self) -> u8 {0}
  fn get_root(&mut self) -> ThreadSafeNode {
    self.root.clone()
  }
  fn get_transposition_table(&mut self) -> Arc<RwLock<HashMap<Vec<u8>, ThreadSafeNode>>> {
    self.transposition_table.clone()
  }
  fn get_played(&mut self) -> &mut Vec<u8> {
    &mut self.played
  }
  fn get_score(&self, field: &GameField) -> i16 {
    let score = field.scores[self.get_player() as usize];
    score
  }
  fn make_tree_play(&mut self, field: &GameField) -> u8 {
    let mut leafs = Vec::new();
    let parent = if field.played_indices.len() == 0 {
      self.get_root()
    } else {
      self.find_node(&field.played_indices)
    };
    self.expand_depth(&field.played_indices, parent.clone(), self.depth, &mut leafs);
    let decision = self.minmax_algo(parent.clone(), parent.clone());
    decision as u8
  }
}


impl Bot for MinMaxBot {
  fn make_play(&mut self, field: &GameField, _: bool) -> u8 {
    self.make_tree_play(field)
  }
  fn get_name(&self) -> String {"minmax".to_owned()}
}