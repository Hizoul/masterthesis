use super::GameField;
use crate::heuristics::{heuristic_value_enemy_block,heuristic_touching,heuristic_value_connectability};
use rayon::prelude::*;
use rand::Rng;
use std::sync::RwLock;
use std::f64::MIN;
use rand::{thread_rng};

pub trait LearnHelper {
  fn update_reward(&mut self, player: i8, include_difference: bool) -> f64;
  fn basic_reward(&mut self, player: i8) -> f64;
  fn calculate_full_reward(&mut self, player: i8, include_difference: bool) -> f64;
  fn get_random_heuristic_play(&mut self) -> u8;
  fn get_best_heuristic_play(&mut self, true_randomness: bool) -> u8;
  fn get_best_heuristic_play_for_params(&mut self, connectability_weight: f64, enemy_block_wheight: f64, touching_weight: f64, score_weight: f64, true_randomness: bool, heuristic_type: u8) -> u8;
  fn get_other_player(&self, current_player: i8) -> i8;
}

impl LearnHelper for GameField {
  fn get_other_player(&self, current_player: i8) -> i8 {
    let next = current_player + 1;
    if next >= self.players {
      0
    } else {
      next
    }
  }
  fn update_reward(&mut self, player: i8, include_difference: bool) -> f64 {
    let mut new_reward = self.scores[player as usize] as f64; //(self.scores[player as usize] - self.scores[other as usize]) as f64;
    if include_difference {
      let other = self.get_other_player(player);
      new_reward -= self.scores[other as usize] as f64;
    }
    new_reward += heuristic_touching(self, player) * 0.3;
    new_reward /= 100.;
    new_reward
  }
  fn basic_reward(&mut self, player: i8) -> f64 {
    self.scores[player as usize] as f64
  }
  fn calculate_full_reward(&mut self, player: i8, include_difference: bool) -> f64 {
    let mut reward = 0.0;
    let indices = self.played_indices.clone();
    let mut shadow_field = self.empty_clone();
    for index in indices {
      let is_players_turn = shadow_field.current_player == player;
      shadow_field.place_block_using_play(index);
      if is_players_turn {
        reward += shadow_field.update_reward(player, include_difference);
      }
    }
    reward
  }
  fn get_random_heuristic_play(&mut self) -> u8 {
    let mut rng = thread_rng();
    self.get_best_heuristic_play_for_params(
      rng.gen_range(-1.0, 10.0),
      rng.gen_range(-1.0, 10.0),
      rng.gen_range(-1.0, 5.0),
      rng.gen_range(0.1, 25.0), true, 0)
  }
  fn get_best_heuristic_play(&mut self, true_randomness: bool) -> u8 {
    self.get_best_heuristic_play_for_params(0.3, 0.5, 1.0, 20.0, true_randomness, 0)
  }
  fn get_best_heuristic_play_for_params(&mut self, connectability_weight: f64, enemy_block_wheight: f64, touching_weight: f64, score_weight: f64, true_randomness: bool, heuristic_type: u8) -> u8 {
    let current_max_rwlock = RwLock::new(MIN);
    let player = self.current_player;
    let to_copy_from = &self;
    let best_index_rwlock: RwLock<Vec<u8>> = RwLock::from(Vec::with_capacity(175));
    (0..self.possible_plays.len()).into_par_iter().for_each(|current_index| {
      let mut field = to_copy_from.empty_clone();
      field.copy_from(to_copy_from);
      field.place_block_using_play(current_index as u8);
      let heuristic_value: f64 = match heuristic_type {
        1 => heuristic_touching(&field, player) + field.scores[player as usize] as f64,
        _ => (heuristic_value_connectability(&field, player) * connectability_weight)
      + (heuristic_value_enemy_block(&field, player) * enemy_block_wheight) + (heuristic_touching(&field, player) * touching_weight) +
      (field.scores[player as usize] as f64 * score_weight)
      };
      let mut current_max_writeable = current_max_rwlock.write().unwrap();
      let currmaxcmp = *current_max_writeable;
      if heuristic_value > currmaxcmp {
        let mut best_index_writeable = best_index_rwlock.write().unwrap();
        best_index_writeable.clear();
        best_index_writeable.push(current_index as u8);
        *current_max_writeable = heuristic_value;
      } else if heuristic_value == currmaxcmp {
        let mut best_index_writeable = best_index_rwlock.write().unwrap();
        best_index_writeable.push(current_index as u8);
      }
    });
    let mut best_index_writeable = best_index_rwlock.write().unwrap();
    best_index_writeable.sort();
    if best_index_writeable.len() == 0 {
      0 
    } else if best_index_writeable.len() == 1 {
      best_index_writeable[0]
    } else {
      best_index_writeable[if true_randomness {rand::thread_rng().gen_range(0, best_index_writeable.len()-1)} else {self.rng.gen_range(0, best_index_writeable.len()-1)}]
    }
  }
}