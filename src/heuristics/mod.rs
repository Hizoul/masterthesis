use crate::game_logic::field::{GameField};
use crate::game_logic::val::{GAMEFIELD_MAX_WIDTH,GAMEFIELD_MAX_HEIGHT};
use std::cmp::{min, max};
pub mod tune;
const SCORE_MODIFIER: f64 = 0.01;

pub fn heuristic_default_reward(field: &GameField, player: i8) -> f64 {
  heuristic_value_connectability(field, player) + heuristic_value_enemy_block(field, player) + (field.scores[player as usize] * 10) as f64
}

pub fn heuristic_value_width_territory(field: &GameField, player: i8) -> f64 {
  let mut score = 0.;
  for block in field.tetrominos.iter() {
    if block.from == player {
      for pos in field.shape_cache.get_block_shape(block) {
        if field.lowest_ys[pos.x as usize] == (pos.y + 1) {
          score += SCORE_MODIFIER;
        }
      }
    }
  }
  score
}

pub fn heuristic_value_height_territory(field: &GameField, player: i8) -> f64 {
  let mut score: f64 = 0.;
  for block in field.tetrominos.iter() {
    if block.from == player {
      for pos in field.shape_cache.get_block_shape(block) {
        let current_y = field.lowest_ys[pos.x as usize];
        if current_y == (pos.y + 1) {
          let current_cmp = current_y - 1;
          let left_y = field.lowest_ys[max(0, pos.x - 1) as usize];
          let right_y = field.lowest_ys[min(pos.x + 1, GAMEFIELD_MAX_WIDTH - 1) as usize];
          if left_y < current_cmp {
            score += ((current_cmp - left_y) as f64) / GAMEFIELD_MAX_HEIGHT as f64;
          }
          if right_y < current_cmp {
            score += ((current_cmp - right_y) as f64) / GAMEFIELD_MAX_HEIGHT as f64;
          }
        }
      }
    }
  }
  score
}

// Combines height and width check in one function
pub fn heuristic_value_all_territory(field: &GameField, player: i8) -> f64 {
  let mut score: f64 = 0.;
  for block in field.tetrominos.iter() {
    if block.from == player {
      for pos in field.shape_cache.get_block_shape(block) {
        let current_y = field.lowest_ys[pos.x as usize];
        if current_y == (pos.y + 1) {
          score += SCORE_MODIFIER;
          let left_y = field.lowest_ys[max(0, pos.x - 1) as usize];
          let right_y = field.lowest_ys[min(pos.x + 1, GAMEFIELD_MAX_WIDTH - 1) as usize];
          if left_y < current_y {
            score += (current_y - left_y) as f64;
          }
          if right_y < current_y {
            score += (current_y - right_y) as f64;
          }
        }
      }
    }
  }
  score
}

pub fn heuristic_value_connectability(field: &GameField, player: i8) -> f64 {
  let mut score: f64 = 0.;
  for block in field.tetrominos.iter() {
    if block.from == player {
      for pos in field.shape_cache.get_block_shape(block) {
        let current_y = field.lowest_ys[pos.x as usize];
        let y_cmp = pos.y + 1;
        if current_y == y_cmp {
          score += SCORE_MODIFIER;
        }
        if pos.x - 1 >= 0 {
          let left_y = field.lowest_ys[(pos.x - 1) as usize];
          if left_y <= pos.y {
            score += SCORE_MODIFIER;
          }
        }
        if pos.x + 1 < GAMEFIELD_MAX_WIDTH {
          let right_y = field.lowest_ys[(pos.x + 1) as usize];
          if right_y <= pos.y {
            score += SCORE_MODIFIER;
          }
        }
      }
    }
  }
  score
}

pub fn heuristic_touching(field: &GameField, player: i8) -> f64 {
  let mut score: f64 = 0.;
  for touch in field.touching_blocks[player as usize].iter() {
    let readable_touch = touch.read().unwrap();
    score += readable_touch.touching.len() as f64;
  }
  score
}

pub fn heuristic_value_enemy_block(field: &GameField, player: i8) -> f64 {
  let mut score = 0.;
  for b1 in field.tetrominos.iter() {
    if b1.from == player {
      for b2 in field.tetrominos.iter() {
        if b2.from != player {
          for p1 in field.shape_cache.get_block_shape(b1).iter() {
            for p2 in field.shape_cache.get_block_shape(b2).iter() {
              if ((p1.y == p2.y) && (p1.x == (p2.x + 1) ||
              p1.x == (p2.x - 1))) ||
              ((p1.x == p2.x) && p1.y == (p2.y + 1)) {
                score += SCORE_MODIFIER;
              }
            }
          }
        }
      }
    }
  }
  score
}

pub fn heuristic_all(field: &GameField, player: i8) -> (f64, f64, f64)  {
  let mut enemy_block = 0.;
  let mut connectability = 0.;
  let mut touching: f64 = 0.;
  for touch in field.touching_blocks[player as usize].iter() {
    let readable_touch = touch.read().unwrap();
    touching += readable_touch.touching.len() as f64;
  }
  for b1 in field.tetrominos.iter() {
    if b1.from == player {
      for pos in field.shape_cache.get_block_shape(b1).iter() {
        let current_y = field.lowest_ys[pos.x as usize];
        let y_cmp = pos.y + 1;
        if current_y == y_cmp {
          connectability += SCORE_MODIFIER;
        }
        if pos.x - 1 >= 0 {
          let left_y = field.lowest_ys[(pos.x - 1) as usize];
          if left_y <= pos.y {
            connectability += SCORE_MODIFIER;
          }
        }
        if pos.x + 1 < GAMEFIELD_MAX_WIDTH {
          let right_y = field.lowest_ys[(pos.x + 1) as usize];
          if right_y <= pos.y {
            connectability += SCORE_MODIFIER;
          }
        }
      }
      for b2 in field.tetrominos.iter() {
        if b2.from != player {
          for p1 in field.shape_cache.get_block_shape(b1).iter() {
            for p2 in field.shape_cache.get_block_shape(b2).iter() {
              if ((p1.y == p2.y) && (p1.x == (p2.x + 1) ||
              p1.x == (p2.x - 1))) ||
              ((p1.x == p2.x) && p1.y == (p2.y + 1)) {
                enemy_block += SCORE_MODIFIER;
              }
            }
          }
        }
      }
    }
  }
  (enemy_block, connectability, touching)
}
