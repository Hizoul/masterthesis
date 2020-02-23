use super::GameField;
use crate::game_logic::tetromino::{get_rotation_amount};
use crate::game_logic::log::{Play};
use crate::game_logic::val::{GAMEFIELD_MAX_WIDTH,AMOUNT_OF_UNIQUE_PIECES};

pub const ACTION_LEFT: u8 = 0;
pub const ACTION_RIGHT: u8 = 1;
pub const ACTION_ROTATE_LEFT: u8 = 2;
pub const ACTION_ROTATE_RIGHT: u8 = 3;
pub const ACTION_PREV_BLOCK: u8 = 4;
pub const ACTION_NEXT_BLOCK: u8 = 5;
pub const ACTION_PLACE_BLOCK: u8 = 6;

pub trait UserActions {
  fn do_user_action(&mut self, user_action: u8);
  fn find_action(&self, block: i8, orientation: i8, x: i8) -> Option<&Play>;
}

impl UserActions for GameField {
  fn find_action(&self, block: i8, orientation: i8, x: i8) -> Option<&Play> {
    let found_action = self.possible_plays.iter().find(|play| {
      play.x == x && play.block == block && play.rotation == orientation
    });
    found_action
  }
  fn do_user_action(&mut self, user_action: u8) {
    match user_action {
      ACTION_PLACE_BLOCK => {
        let action: Option<&Play> = self.find_action(self.current_block,self.current_orientation,self.current_x);
        if action.is_some() {
          let relevant_index = action.unwrap().index;
          self.place_block_using_play(relevant_index);
        }
      }
      ACTION_LEFT => {
        self.current_x = self.current_x - 1;
        if self.current_x < 0 {
          self.current_x = GAMEFIELD_MAX_WIDTH - 1;
        }
      }
      ACTION_RIGHT => {
        self.current_x = self.current_x + 1;
        if self.current_x >= GAMEFIELD_MAX_WIDTH {
          self.current_x = 0;
        }
      }
      ACTION_ROTATE_LEFT => {
        self.current_orientation = self.current_orientation - 1;
        if self.current_orientation < 0 {
          self.current_orientation = get_rotation_amount(self.current_block) - 1; 
        }
      }
      ACTION_ROTATE_RIGHT => {
        self.current_orientation = self.current_orientation + 1;
        if self.current_orientation >= get_rotation_amount(self.current_block) {
          self.current_orientation = 0; 
        }
      }
      ACTION_PREV_BLOCK => {
        self.current_block = self.current_block + 1;
        if self.current_block >= AMOUNT_OF_UNIQUE_PIECES {
          self.current_block = 0; 
        }
      }
      ACTION_NEXT_BLOCK => {
        self.current_block = self.current_block + 1;
        if self.current_block >= AMOUNT_OF_UNIQUE_PIECES {
          self.current_block = 0; 
        }
      }
      _ => {
      }
    }
  }
}