use serde::{Deserialize, Serialize};
use crate::game_logic::tetromino::{Tetromino};
use std::sync::{Arc};

#[derive(Debug,Clone)]
pub struct Touching {
  pub block: Arc<Tetromino>,
  pub touching: Vec<Arc<Tetromino>>,
  pub connected: Vec<Arc<Tetromino>>
}

impl PartialEq for Touching {
  fn eq(&self, other: &Touching) -> bool {
    self.block == other.block
  }
}

impl Touching {
  pub fn new(block: Arc<Tetromino>) -> Touching {
    return Touching {
      block, touching: Vec::with_capacity(4), connected: Vec::with_capacity(4)
    }
  }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct OwnedPosition {
    pub x: i8,
    pub y: i8,
    pub from: i8
}

#[derive(Debug,Serialize,Deserialize,Copy,Clone)]
pub struct GameRules {
  pub roll_dice: bool,
  pub limit_minus: bool
}

impl GameRules {
  pub fn original() -> GameRules {
    GameRules {roll_dice:true,limit_minus:true}
  }
  pub fn original_without_limit() -> GameRules {
    GameRules {roll_dice:true,limit_minus:false}
  }
  pub fn deterministic() -> GameRules {
    GameRules {roll_dice:false,limit_minus:true}
  }
  pub fn deterministic_without_limit() -> GameRules {
    GameRules {roll_dice:false,limit_minus:false}
  }
}

impl Default for GameRules {
  fn default() -> GameRules {
    GameRules::original_without_limit()
  }
}