use crate::game_logic::field::{GameField};
use crate::game_logic::field::helper_structs::{GameRules};
use crate::game_logic::field::rl::{LearnHelper};
use std::fs::write;
use std::fs::read_to_string;
use std::collections::HashSet;

pub type ScoreList = Vec<Vec<i16>>;
pub type GameStartList = Vec<Vec<u8>>;
pub const PLAY_AMOUNT: usize = 10000;
pub fn play(strategy: u8) -> (ScoreList, GameStartList) {
  let mut field = GameField::new_with_rules(2, GameRules::deterministic());
  let mut results: ScoreList = Vec::with_capacity(PLAY_AMOUNT);
  let mut play_history: GameStartList = Vec::with_capacity(PLAY_AMOUNT);
  for i in 0..PLAY_AMOUNT {
    println!("PLAYING GAME {}", i);
    field.reset(true);
    while !field.game_over {
      let play_to_make = match strategy {
        0 => field.get_random_play(),
        1 => field.get_random_heuristic_play(),
        2 => field.get_best_heuristic_play_for_params(4.95238700733393,0.115980839099051,0.0603795891552745,19.0148438477953, true, 0),
        _ => field.get_best_heuristic_play(true)
      };
      field.place_block_using_play(play_to_make);
    }
    results.push(field.scores.clone());
    play_history.push(field.played_indices.clone());
  }
  (results, play_history)
}

pub fn pre_calc() {
  let res_random = play(0);
  write("adv_random_scores.json", serde_json::to_string(&res_random.0).unwrap()).unwrap();
  write("adv_random_playhistory.json", serde_json::to_string(&res_random.1).unwrap()).unwrap();
  let res_random = play(1);
  write("adv_randheu_scores.json", serde_json::to_string(&res_random.0).unwrap()).unwrap();
  write("adv_randheu_playhistory.json", serde_json::to_string(&res_random.1).unwrap()).unwrap();
  let res_random = play(2);
  write("adv_bestheu_scores.json", serde_json::to_string(&res_random.0).unwrap()).unwrap();
  write("adv_bestheu_playhistory.json", serde_json::to_string(&res_random.1).unwrap()).unwrap();
  let res_random = play(3);
  write("adv_defaultheu_scores.json", serde_json::to_string(&res_random.0).unwrap()).unwrap();
  write("adv_defaultheu_playhistory.json", serde_json::to_string(&res_random.1).unwrap()).unwrap();
}

pub fn evaluate_scores(scores: ScoreList) -> (f64, f64) {
  let win_percentage: f64;
  let avg_advantage: f64;
  let mut wins = 0.0;
  let mut total_advantage = 0.0;
  let total = scores.len();
  for score in scores {
    if score[0] > score[1] {
      wins += 1.0;
      total_advantage += (score[0] - score[1]) as f64;
    }
  }
  win_percentage = wins / total as f64;
  avg_advantage = total_advantage / wins;
  (win_percentage, avg_advantage)
}

pub fn data_evaulator() {
  let bla = vec!["bestheu", "defaultheu", "randheu", "random"];
  for name in bla.clone() {
    let file_content = read_to_string(format!("adv_{}_scores.json", name)).unwrap();
    let parsed_file: ScoreList = serde_json::from_str(file_content.as_str()).unwrap();
    let eval_res = evaluate_scores(parsed_file);
    println!("GOT {:?} for {}", eval_res, name);
  }
  for name in bla {
    let file_content = read_to_string(format!("adv_{}_playhistory.json", name)).unwrap();
    let parsed_file: GameStartList = serde_json::from_str(file_content.as_str()).unwrap();
    let mut set = HashSet::new();
    for entry in parsed_file {
      set.insert(entry);
    }
    println!("FOR {} got {} unique plays", name, set.len());
  }
}