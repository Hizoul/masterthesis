use crate::ai::{Bot,TreeBot};
use crate::tree::{ThreadSafeNode,Node};
use crate::game_logic::field::{GameField};
use crate::game_logic::field::helper_structs::{GameRules};
use crate::game_logic::val::MAX_PLAYS;
use std::cmp::{min,max};
use std::sync::{Arc,RwLock};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AlphaBetaBot {
  depth: u128,
  root: ThreadSafeNode,
  played: Vec<u8>,
  player: u8,
  transposition_table: Arc<RwLock<HashMap<Vec<u8>, ThreadSafeNode>>>
}

impl AlphaBetaBot {
  pub fn new(player: u8, depth: u128) -> AlphaBetaBot {
    AlphaBetaBot {
      player,depth, root: Node::new_safe_score(0, player, None, Some(MAX_PLAYS as usize), Vec::new()),played: Vec::new(),
      transposition_table: Arc::new(RwLock::new(HashMap::new()))  
    }
  }
  pub fn alphabeta_algo(&mut self, node: ThreadSafeNode, depth: u128, mut alpha: i16, mut beta: i16, is_root_node: bool) -> i16 {
    // wenn keine kinder übernimm score von node selbst und ruf minmax auf parent auf
    let play_depth;
    let played_indices;
    let has_been_expanded;
    let node_score;
    {
      let readable_node = node.read().unwrap();
      played_indices = readable_node.played_indices.clone();
      play_depth = played_indices.len();
      has_been_expanded = readable_node.expanded;
      node_score = readable_node.score[self.get_player() as usize];
    }
    if depth == 0 {
      return node_score;
    } else {
      if !has_been_expanded {
        self.expand_node(&played_indices, node.clone());
      }
      // wenn kinder dann für alle über diese funktion value holen und vergleichen
      let mut relevant_index = 0;
      let mut current_index = 0;
      let is_maximizing = play_depth % 2 == self.get_player() as usize;
      let mut new_val = if is_maximizing { i16::min_value() } else { i16::max_value() };
      let mut prev_val = -1;
      let children;
      {
        let readable_node = node.read().unwrap();
        children = readable_node.children.clone();
      }
      'child_iter: for child in children.iter() {
        let child_is_parent;
        {
          let newchild = child.read().unwrap();
          let readable_node = node.read().unwrap();
          child_is_parent = *newchild == *readable_node;
        }
        if !child_is_parent {
          let childs_minmax = self.alphabeta_algo(child.clone(), depth - 1, alpha, beta, false);
          new_val = if is_maximizing { max(new_val, childs_minmax) } else { min(new_val, childs_minmax)}; 

          if new_val != prev_val {
            prev_val = new_val;
            relevant_index = current_index;
          }
          current_index += 1;
          if is_maximizing {
            alpha = max(alpha, new_val);
          } else {
            beta = min(beta, new_val);
          }
          if alpha >= beta {
            break 'child_iter;
          }
        }
      }
      if is_root_node {
        return relevant_index;
      }
      return new_val;
    }
  }
}

impl TreeBot for AlphaBetaBot {
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
    let parent = if field.played_indices.len() == 0 {
      self.get_root()
    } else {
      self.find_node(&field.played_indices)
    };
    let decision = self.alphabeta_algo(parent.clone(), self.depth, i16::min_value(), i16::max_value(), true);
    println!("GOT DECISION {}", decision);
    decision as u8
  }
}


impl Bot for AlphaBetaBot {
  fn make_play(&mut self, field: &GameField, is_second: bool) -> u8 {
    self.make_tree_play(field)
  }
  fn get_name(&self) -> String {"alphabeta".to_owned()}
}