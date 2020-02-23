use crate::game_logic::field::{GameField};
use crate::game_logic::field::helper_structs::{GameRules};
use crate::game_logic::field::render::{Renderable};
use std::collections::HashSet;

pub fn calculate(current_depth: u8, max_depth: u8, set: &mut HashSet<String>) -> usize {
  if current_depth > current_depth {
    calculate(current_depth + 1, max_depth, set);
  } 
  set.len()
}

pub fn get_amount_of_transpositions(max_depth: usize) -> usize {
  let mut field = GameField::new_with_rules(2, GameRules::deterministic());
  let mut set = HashSet::new();
  let mut plays: Vec<u8> = Vec::with_capacity(max_depth);
  for _ in 0..max_depth {
    plays.push(0);
  }
  let mut is_over = false;
  let loop_max = max_depth -1;
  while !is_over {
    field.reset(true);
    let mut is_possible = true;
    plays[loop_max] += 1;
    for i in 0..max_depth {
      if plays[loop_max - i] > 190 {
        plays[loop_max - i] = 0;
        if i == loop_max {
          is_over = true
        } else {
          plays[loop_max - (i+1)] += 1
        }
      }
    }
    for play in &plays {
      if !field.place_block_using_play(*play) {
        is_possible = false;
      }
    }
    if is_possible {
      set.insert(field.to_field_string_with_player_as_u8());
    }
  }
  set.len()
}