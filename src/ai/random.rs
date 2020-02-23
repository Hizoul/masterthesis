use crate::ai::{Bot};
use crate::game_logic::field::{GameField};
use rand;
use rand::Rng;
use rand_hc::Hc128Rng;
use rand::SeedableRng;
use crate::game_logic::val::{RANDOM_SEED};

pub struct RandomBot {
  rng: Hc128Rng
}

impl RandomBot {
  pub fn new() -> RandomBot {
    let rng: Hc128Rng = Hc128Rng::from_seed(RANDOM_SEED);
    RandomBot {rng}
  }
}

impl Bot for RandomBot {
  fn make_play(&mut self, field: &GameField, _: bool) -> u8 {
    self.rng.gen_range(0, field.possible_plays.len()) as u8
  }
  fn get_name(&self) -> String {"random".to_owned()}
}