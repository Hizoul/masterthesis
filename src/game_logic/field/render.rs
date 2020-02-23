use super::GameField;
use crate::game_logic::val::{GAMEFIELD_MAX_WIDTH,GAMEFIELD_MAX_WIDTH_USIZE,GAMEFIELD_MAX_HEIGHT_USIZE,GAMEFIELD_MAX_HEIGHT,AMOUNT_OF_UNIQUE_PIECES_USIZE};

pub trait Renderable {
  fn to_field_string(&self) -> String;
  fn to_field_string_with_player(&self, include_current_player: bool) -> String;
  fn to_field_string_with_player_as_u8(&self) -> Vec<u8>;
  fn to_playable_string(&mut self) -> String;
  fn to_reshapeable_array(&mut self, second_player_as_first: bool) -> Vec<i8>;
  fn to_reshaped_array(&mut self) -> Vec<Vec<i8>>;
  fn to_image(&mut self, second_player_as_first: bool) -> Vec<f32>;
  fn converted(&mut self) -> Vec<f32>;
}
pub const MAX: usize = GAMEFIELD_MAX_HEIGHT_USIZE*GAMEFIELD_MAX_WIDTH_USIZE;
pub const MAX_FOR_SCORE: usize = (GAMEFIELD_MAX_HEIGHT_USIZE+1)*GAMEFIELD_MAX_WIDTH_USIZE;
pub const MAX_FOR_BLOCKS: usize = (GAMEFIELD_MAX_HEIGHT_USIZE+3)*GAMEFIELD_MAX_WIDTH_USIZE;
// 1 for scores, 2 and 3 for block amounts of players, 19 more for valid play indicators
pub const AMOUNT_OF_ADDITIONAL_LINES: usize = 22;

impl Renderable for GameField {
  fn to_field_string(&self) -> String {
    self.to_field_string_with_player(false)
  }
  fn to_field_string_with_player(&self, include_current_player: bool) -> String {
    let mut field = String::with_capacity((GAMEFIELD_MAX_WIDTH_USIZE * GAMEFIELD_MAX_HEIGHT_USIZE) +
        GAMEFIELD_MAX_WIDTH_USIZE + 1);
    field.push('\n');
    for ys in 0..GAMEFIELD_MAX_HEIGHT {
      let y = (GAMEFIELD_MAX_HEIGHT - 1) - ys;
      for x in 0..GAMEFIELD_MAX_WIDTH {
        let mut player = "0".to_string();
        'search: for p in self.tetrominos.iter() {
          let mut i = 0;
          for part in self.shape_cache.get_block_shape(p).iter() {
            if part.x == x && part.y == y {
              let to_set = if i == 0 {((p.from + 1) * 2) - 1} else {((p.from + 1) * 2)};
              player = to_set.to_string();
              break 'search;
            }
            i += 1;
          }
        }
        field.push_str(player.as_str());
      }
      field.push('\n');
    }
    if include_current_player {
      field.push_str(self.current_player.to_string().as_str());
    }
    field
  }
  fn to_field_string_with_player_as_u8(&self) -> Vec<u8> {
    let mut field: [u8; 201] = [0; 201];
    for p in self.tetrominos.iter() {
      let mut i = 0;
      for part in self.shape_cache.get_block_shape(p).iter() {
        let to_set = if i == 0 {i+=1;((p.from + 1) * 2) - 1} else {((p.from + 1) * 2)};
        field[((((GAMEFIELD_MAX_HEIGHT_USIZE-1) - part.y as usize) * GAMEFIELD_MAX_WIDTH_USIZE) + part.x as usize)] = to_set as u8;
      }
    }
    field[200] = self.current_player as u8;
    field.to_vec()
  }
  fn to_reshapeable_array(&mut self, second_player_as_first: bool) -> Vec<i8> {
    // first additional line = scores
    // second and third additional lines = blocks per players
    // 190 additional entries for possible combinations
    let mut field_nums: Vec<i8> = Vec::with_capacity((GAMEFIELD_MAX_HEIGHT_USIZE+AMOUNT_OF_ADDITIONAL_LINES) * GAMEFIELD_MAX_WIDTH_USIZE);
    for index in 0..(GAMEFIELD_MAX_HEIGHT_USIZE+AMOUNT_OF_ADDITIONAL_LINES)*GAMEFIELD_MAX_WIDTH_USIZE {
      if index >= MAX {
        if index < MAX_FOR_SCORE {
          let mut corrected_index = index % MAX;
          if corrected_index < (self.players as usize) {
            if second_player_as_first {
              corrected_index = if corrected_index == 0 {1} else {0};
            }
            field_nums.push(self.scores[corrected_index] as i8);
          } else {
            field_nums.push(-1);
          }
        } else if index < MAX_FOR_BLOCKS {
          let mut player = (index % MAX_FOR_SCORE) / AMOUNT_OF_UNIQUE_PIECES_USIZE;
          if player >= (self.players as usize) {
            field_nums.push(-1);
          } else {
            let block = (index % MAX_FOR_SCORE) % AMOUNT_OF_UNIQUE_PIECES_USIZE;
            if second_player_as_first {
              player = if player == 0 {1} else {0};
            }
            field_nums.push(self.piece_counter[player][block]);
          }
        } else {
          field_nums.push(-1);
        }
      } else {
        field_nums.push(0);
      }
    }
    for play in self.possible_plays.iter() {
      field_nums[(MAX_FOR_BLOCKS)+play.global_index as usize] = 1;
    }
    for p in self.tetrominos.iter() {
      let mut i = 0;
      for part in self.shape_cache.get_block_shape(p).iter() {
        let mut from = p.from;
        if second_player_as_first {
          from = if from == 0 {1} else {0};
        }
        let to_set = if i == 0 {
          i += 1;
          ((from + 1) * 2) - 1
        } else {
          ((from + 1) * 2)
        };
        field_nums[((((GAMEFIELD_MAX_HEIGHT_USIZE-1) - part.y as usize) * GAMEFIELD_MAX_WIDTH_USIZE) + part.x as usize)] = to_set;
      }
    }
    // let b = self.current_block;
    // let o = self.current_orientation;
    // let x = self.current_x;
    // let y = self.highest_y(b, o, x);
    // if !self.game_over {
    //   let hover_block = self.shape_cache.get_translated(b, o, x, y);
    //   for part in hover_block.iter() {
    //     field_nums[((((GAMEFIELD_MAX_HEIGHT_USIZE-1) - part.y as usize) * GAMEFIELD_MAX_WIDTH_USIZE) + part.x as usize)] = (self.current_player as u8 + 1) * 2;
    //   }
    // }
    field_nums
  }
  fn to_image(&mut self, second_player_as_first: bool) -> Vec<f32> {
    let mut to_return: Vec<f32> = vec![0.0; 3072];
    let orig = self.to_reshapeable_array(second_player_as_first);
    for i in 0..200 {
      let y = (i as f32 / 10.0).floor() as usize;
      let x = i % 10;
      let mut current = 0;
      let mut val = 0.0;
      if orig[i] == 1 || orig[i] == 2 {
        current = 0;
        val = if orig[i] == 1 {128.0} else {255.0};
      } else if orig[i] == 3 || orig[i] == 4 {
        current = 2;
        val = if orig[i] == 3 {128.0} else {255.0};
      }
      to_return[((y * 32*3) + x*3) + current] = val;
    }
    for i in 200..210 {
      let y = ((i - 200) as f32 / 10.0).floor() as usize;
      let x = (i % 10) + 10;
      to_return[(y*32*3)+x*3+1] = orig[i] as f32 + 124.0;
    }
    for i in 210..230 {
      let y = ((i - 200) as f32 / 10.0).floor() as usize;
      let x = (i % 10) + 10;
      let val = if orig[i] <= 0 {0.0} else {orig[i] as f32 * 50.0};
      to_return[(y*32*3)+x*3+1] = val;
    }
    for i in 230..420 {
      let y = ((i - 200) as f32 / 10.0).floor() as usize;
      let x = (i % 10) + 10;
      let val = if orig[i] == -1 {
         0.0
      } else {
        255.0
      };
      to_return[(y*32*3)+x*3+1] = val;
    }
    to_return
  }
  fn converted(&mut self) -> Vec<f32> {
    let not_yet_reshaped = self.to_reshapeable_array(false);
    let mut converted: Vec<f32> = Vec::new();
    for num in not_yet_reshaped {
      converted.push(num as f32);
    }
    converted
  }
  fn to_reshaped_array(&mut self) -> Vec<Vec<i8>> {
    let not_yet_reshaped = self.to_reshapeable_array(false);
    let mut reshaped = Vec::new();
    let mut current_sub = Vec::new();
    let i = 0;
    for num in not_yet_reshaped {
      current_sub.push(num);
      if i > 0 && i % 10 == 0 {
        reshaped.push(current_sub.clone());
        current_sub.clear();
      }
    }
    reshaped
  }
  fn to_playable_string(&mut self) -> String {
    let mut field_nums: Vec<Vec<i8>> = Vec::with_capacity(GAMEFIELD_MAX_HEIGHT_USIZE);
    for _ in 0..GAMEFIELD_MAX_HEIGHT {
      let mut new_line = Vec::with_capacity(GAMEFIELD_MAX_WIDTH_USIZE);
      for _ in 0..GAMEFIELD_MAX_WIDTH {
        new_line.push(0);
      }
      field_nums.push(new_line);
    }
    for p in self.tetrominos.iter() {
      for part in self.shape_cache.get_block_shape(p).iter() {
        field_nums[(GAMEFIELD_MAX_HEIGHT_USIZE-1) - part.y as usize][part.x as usize] = ((p.from + 1) * 2) - 1;
      }
    }
    let b = self.current_block;
    let o = self.current_orientation;
    let x = self.current_x;
    let y = self.highest_y(b, o, x);
    if !self.game_over {
      let hover_block = self.shape_cache.get_translated(b, o, x, y);
      for part in hover_block.iter() {
        let new_y = (GAMEFIELD_MAX_HEIGHT_USIZE-1) - part.y as usize;
        let new_x = part.x as usize;
        if new_x < GAMEFIELD_MAX_WIDTH_USIZE && new_y < GAMEFIELD_MAX_HEIGHT_USIZE {
          field_nums[new_y][new_x] = (self.current_player + 1) * 2;
        }
      }
    }
    let mut field = String::with_capacity((GAMEFIELD_MAX_WIDTH_USIZE * GAMEFIELD_MAX_HEIGHT_USIZE) +
        GAMEFIELD_MAX_WIDTH_USIZE + 1);
    for top in field_nums.iter() {
      for ele in top.iter() {
        field.push_str(ele.to_string().as_str());
      }
      field.push('\n');
    }
    field
  }
}