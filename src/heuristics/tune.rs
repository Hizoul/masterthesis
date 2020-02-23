use crate::game_logic::field::{GameField};
use crate::game_logic::field::helper_structs::{GameRules};
use crate::game_logic::field::rl::LearnHelper;
use crate::game_logic::cache::ShapeCache;
use std::sync::{Arc,RwLock};
use rayon::prelude::*;

const SAMPLE_SIZE: u64 = 500;
const EVALUATED_PLAYER_NUMBER: i8 = 0;
const ENEMY_CONFIGS: [(f64, f64, f64, f64); 2] = [
  (4.95238700733393,0.115980839099051,0.0603795891552745,19.0148438477953),
  (2.29754304265925,0.714950690441259,0.0203784890483687,4.26915982151196)
];

pub fn heuristic_evaluator(connectability_weight: f64, enemy_block_wheight: f64, touching_weight: f64, score_weight: f64) -> u64 {
  let mut final_score = 0;
  let shape_cache = Arc::new(ShapeCache::new());
  let wins_per_config: Vec<u64> = ENEMY_CONFIGS.par_iter().map(|weight_configuration| {
    let wins: RwLock<u64> = RwLock::new(0);
    (0..SAMPLE_SIZE).into_par_iter().for_each(|_| {
      let mut field = GameField::new_with_cache(2, shape_cache.clone(), GameRules::deterministic());
      while !field.game_over {
        let play_to_do = if field.current_player == EVALUATED_PLAYER_NUMBER {
          field.get_best_heuristic_play_for_params(connectability_weight, enemy_block_wheight, touching_weight, score_weight, true, 0)
        } else {
          field.get_best_heuristic_play_for_params(weight_configuration.0, weight_configuration.1, weight_configuration.2, weight_configuration.3, true, 0)
        };
        field.place_block_using_play(play_to_do);
      }
      if field.get_winning_player() == EVALUATED_PLAYER_NUMBER {
        let mut writeable_wins = wins.write().unwrap();
        *writeable_wins += 1;
      }
    });
    let readable_wins = wins.read().unwrap();
    *readable_wins
  }).collect();
  for win_amount in wins_per_config {
    if win_amount > (SAMPLE_SIZE / 4) {
      final_score += win_amount;
    }
  }
  final_score
}