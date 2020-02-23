use crate::game_logic::field::{GameField};
use crate::game_logic::field::helper_structs::{GameRules};
use crate::ai::Bot;
pub mod advantage_checker;
pub mod transposition_amount;
pub mod rating;
pub mod tournament;
pub mod mctsparameval;
pub mod final_eval;

pub struct PlaySupervisor {
  pub players: Vec<Box<dyn Bot>>,
  pub field: GameField
}

impl PlaySupervisor {
  pub fn new(players: Vec<Box<dyn Bot>>) -> PlaySupervisor {
    let field = GameField::new_with_rules(players.len() as i8, GameRules::deterministic());
    PlaySupervisor {players,field}
  }
  pub fn new_with_rules(players: Vec<Box<dyn Bot>>, rules: GameRules) -> PlaySupervisor {
    let field = GameField::new_with_rules(players.len() as i8, rules);
    PlaySupervisor {players,field}
  }
  pub fn play(&mut self) {
    while !self.field.game_over {
      for p in self.players.iter_mut() {
        if !self.field.game_over {
          let play_to_make = p.make_play(&self.field, false);
          self.field.place_block_using_play(play_to_make);
        }
      }
    }
  }
}