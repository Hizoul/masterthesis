use crate::ai::{Bot};
use crate::game_logic::field::{GameField};
use crate::game_logic::field::rl::LearnHelper;

pub struct HeuristicBot {
  mode: u8
}

impl HeuristicBot {
  pub fn new(mode: u8) -> HeuristicBot {
    HeuristicBot {
      mode
    }
  }
}

impl Bot for HeuristicBot {
  fn make_play(&mut self, field: &GameField, _: bool) -> u8 {
    let mut cloned = field.clone();
    match self.mode {
      0 => cloned.get_best_heuristic_play_for_params(4.95238700733393,0.115980839099051,0.0603795891552745,19.0148438477953, true, 0),
      1 => cloned.get_random_heuristic_play(),
      _ => cloned.get_best_heuristic_play(true)
    }
  }
  fn get_name(&self) -> String {
    let start = match self.mode {
      0 => "Tuned",
      1 => "Random",
      _ => "User"
    };
    format!("{} Heuristic", start).to_owned()
  }
}