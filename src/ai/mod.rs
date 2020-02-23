use crate::game_logic::field::{GameField};
use crate::game_logic::field::render::{Renderable};
use crate::game_logic::field::helper_structs::{GameRules};
use crate::tree::{Node,ThreadSafeNode};
use std::sync::{Arc,RwLock};
pub mod random;
pub mod minmax;
pub mod alphabeta;
pub mod heuristic;
pub mod python;
use std::thread::sleep;
use rayon::prelude::*;
use std::time::Duration;
use std::collections::HashMap;

pub fn get_score(field: &GameField, player: u8) -> i16 {
  field.scores[player as usize]
}

pub trait Bot: Send + Sync {
  fn make_play(&mut self, field: &GameField, is_second: bool) -> u8;
  fn get_name(&self) -> String;
}
const SLEEP_DURATION: u64 = 2;

pub trait TreeBot: Send + Sync {
  fn get_rules(&self) -> GameRules;
  fn get_player(&self) -> u8;
  fn get_score_type(&self) -> u8;
  fn make_tree_play(&mut self, field: &GameField) -> u8;
  fn get_root(&mut self) -> Arc<RwLock<Node>>;
  fn get_transposition_table(&mut self) -> Arc<RwLock<HashMap<Vec<u8>, ThreadSafeNode>>>;
  fn get_played(&mut self) -> &mut Vec<u8>;
  fn get_score(&self, field: &GameField) -> i16;
  fn find_node(&mut self, played_indices: &Vec<u8>) -> ThreadSafeNode {
    let mut parent_arc = self.get_root();
    let rules = self.get_rules();
    let mut played = self.get_played().clone();
    let transposition_table = self.get_transposition_table();
    played.clear();
    let mut depth = 0;
    for i in played_indices.iter() {
      depth += 1;
      played.push(*i);
      let subpointer = parent_arc.clone();
      let child_is_not_parent: bool;
      let c;
      {
        let n = subpointer.read().unwrap();
        c = n.children[*i as usize].clone();
        let newchild = c.read().unwrap();
        child_is_not_parent = *newchild != *n;
      }
      if child_is_not_parent {
        parent_arc = c;
      } else {
        let mut f = GameField::new_with_rules(2, self.get_rules());
        for to_play in played.iter() {
          f.place_block_using_play(*to_play);
        }

        let rendered_field = f.to_field_string_with_player_as_u8();
        let mut existing_child = Option::None;
        if depth >= 4 {
          let readable_table = transposition_table.read().unwrap();
          let found_child = readable_table.get(&rendered_field);
          if found_child.is_some() {
            existing_child = Option::Some(found_child.unwrap().clone());
          }
        }
        if existing_child.is_some() {
          let new_existing_child = existing_child.unwrap();
          {
            let readable = new_existing_child.read().unwrap();
            let mut sfcomp = GameField::new_with_rules(2, rules);
            for to_play in readable.played_indices.iter() {
              sfcomp.place_block_using_play(*to_play);
            }
          }
          {
            let mut attempt = parent_arc.try_write();
            while attempt.is_err() {
              sleep(Duration::from_nanos(10));
              attempt = parent_arc.try_write();
            }
            let mut n = attempt.unwrap();
            n.children[*i as usize] = new_existing_child.clone();
          }
          parent_arc = new_existing_child;
        } else {
          let new_node = Node::new_safe_score(self.get_score(&f), self.get_player(), Some(parent_arc.clone()), Some(f.possible_plays.len()), played.clone());
          // n.children.push(new_node);
          {
            let mut attempt = parent_arc.try_write();
            while attempt.is_err() {
              sleep(Duration::from_nanos(10));
              attempt = parent_arc.try_write();
            }
            let mut n = attempt.unwrap();
            n.children[*i as usize] = new_node.clone();
          }
          if depth >= 4 {
            let mut write_attempt = transposition_table.try_write();
            while write_attempt.is_err() {
              sleep(Duration::from_nanos(SLEEP_DURATION));
              write_attempt = transposition_table.try_write();
            }
            let mut writeable_table = write_attempt.unwrap();
            let write_res = writeable_table.insert(rendered_field, new_node.clone());
            if write_res.is_some() {
              println!("CONFLICT IN TRANSPOISTION TABLE DETECTED!");
            }
          }
          parent_arc = new_node;
        }

        let new_node = Node::new_safe_score(self.get_score(&f), self.get_player(), Some(parent_arc.clone()), Some(f.possible_plays.len()), played.clone());
        {let mut n = subpointer.write().unwrap();
        n.children[*i as usize] = new_node.clone();}
        parent_arc = new_node.clone();
      }
    }
    parent_arc
  }
  fn expand_node(&mut self, played_indices: &Vec<u8>, parent_arc: ThreadSafeNode) {
    let mut f = GameField::new_with_rules(2, self.get_rules());
    for to_play in played_indices.iter() {
      f.place_block_using_play(*to_play);
    }
    let player = self.get_player();
    let rules = self.get_rules();
    let transposition_table = self.get_transposition_table();
    let child_depth = played_indices.len() + 1;
    (0..f.possible_plays.len()).into_par_iter().for_each(|i| {
      let child_is_parent: bool;
      let c;
      {
        let n = parent_arc.read().unwrap();
        c = &n.children[i];
        let newchild = c.read().unwrap();
        child_is_parent = *newchild == *n;
      }
      if child_is_parent {
        let mut sf = GameField::new_with_rules(2, rules);
        for to_play in played_indices.iter() {
          sf.place_block_using_play(*to_play);
        }
        sf.place_block_using_play(i as u8);

        let rendered_field = sf.to_field_string_with_player_as_u8();
        let mut existing_child = Option::None;
        if child_depth >= 4 {
          let readable_table = transposition_table.read().unwrap();
          let found_child = readable_table.get(&rendered_field);
          if found_child.is_some() {
            existing_child = Option::Some(found_child.unwrap().clone());
          }
        }
        if existing_child.is_some() {
          let new_existing_child = existing_child.unwrap();
          // {
          //   let readable = new_existing_child.read().unwrap();
          //   let mut sfcomp = GameField::new_with_rules(2, rules);
          //   for to_play in readable.played_indices.iter() {
          //     sfcomp.place_block_using_play(*to_play);
          //   }
          //   println!("MATCH IN TRANPSOITION TABLE {} {} {:?} {:?}", sfcomp.to_field_string_with_player_as_u8() == sf.to_field_string_with_player_as_u8(), sf.played_indices == readable.played_indices, sf.played_indices, readable.played_indices);
          // }
          {
            let mut attempt = parent_arc.try_write();
            while attempt.is_err() {
              sleep(Duration::from_nanos(10));
              attempt = parent_arc.try_write();
            }
            let mut n = attempt.unwrap();
            n.children[i] = new_existing_child;
          }
        } else {
          let new_node = Node::new_safe_score(get_score(&sf, player), player, Some(parent_arc.clone()), Some(sf.possible_plays.len()), sf.played_indices.clone());
          // n.children.push(new_node);
          {
            let mut attempt = parent_arc.try_write();
            while attempt.is_err() {
              sleep(Duration::from_nanos(10));
              attempt = parent_arc.try_write();
            }
            let mut n = attempt.unwrap();
            n.children[i] = new_node.clone();
          }
          if child_depth >= 4 {
            let mut write_attempt = transposition_table.try_write();
            while write_attempt.is_err() {
              sleep(Duration::from_nanos(SLEEP_DURATION));
              write_attempt = transposition_table.try_write();
            }
            let mut writeable_table = write_attempt.unwrap();
            let write_res = writeable_table.insert(rendered_field, new_node);
            if write_res.is_some() {
              println!("CONFLICT IN TRANSPOISTION TABLE DETECTED!");
            }
          }
        }
      }
    });
    {
      let mut attempt = parent_arc.try_write();
      while attempt.is_err() {
        sleep(Duration::from_nanos(10));
        attempt = parent_arc.try_write();
      }
      let mut n = attempt.unwrap();
      n.expanded = true;
    }
  }
  fn expand_depth(&mut self, played_indices: &Vec<u8>, parent_arc: ThreadSafeNode, depth: u128, leafs: &mut Vec<ThreadSafeNode>) {
    // let start_time = timestamp();
    if depth > 0 {
      self.expand_node(played_indices, parent_arc.clone());
      let children;
      {
        let parent = parent_arc.read().unwrap();
        children = parent.children.clone();
      }
      let mut play_index = 0;
      for child in children.iter() {
        let mut new_indices = played_indices.clone();
        new_indices.push(play_index);
        self.expand_depth(&new_indices.clone(), child.clone(), depth - 1, leafs);
        play_index += 1;
      }
    } else {
      leafs.push(parent_arc.clone());
    }
    // let end_time = timestamp();
    // let transpos = self.get_transposition_table().read().unwrap().len();
    // println!("DEEPENING to {} took {} seconds found transpositions are {}", depth, (end_time - start_time) / 1000, transpos);
  }
}

impl Bot for dyn TreeBot {
  fn make_play(&mut self, field: &GameField, _: bool) -> u8 {
    self.make_tree_play(field)
  }
  fn get_name(&self) -> String {"treebot".to_owned()}
}