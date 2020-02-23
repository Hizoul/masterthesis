pub mod heuristic;
use rand::{Rng,thread_rng};

pub type Coordinate = (usize,usize);

fn num_to_coord(size: usize, action: usize) -> Coordinate {
  let y = action / size;
  let x = action % size;
  (x,y)
}

fn coord_to_num(size: usize, coord: &Coordinate) -> usize {
  coord.1 * size + coord.0
}

pub fn get_neighbors(field_size: usize, x: usize, y: usize) -> [Option<Coordinate>; 6] {
  [
    if x + 1 < field_size { Some((x + 1, y)) } else {None},
    if y + 1 < field_size { Some((x, y + 1)) } else {None},
    if x > 0 && y + 1 < field_size { Some((x - 1, y + 1)) } else {None},
    if x > 0 { Some((x - 1, y)) } else {None},
    if y > 0 { Some((x, y - 1)) } else {None},
    if x + 1 < field_size && y > 0 { Some((x + 1, y - 1)) } else {None}
  ]
}

#[derive(Clone)]
pub struct HexGame {
  pub field: Vec<Vec<u8>>,
  pub played_indices: Vec<usize>,
  pub is_first_players_turn: bool,
  pub game_over: bool,
  pub winner: u8,
  already_checked: Vec<Coordinate>,
  turn_counter: usize
}

impl HexGame {
  pub fn new(grid_size: usize) -> HexGame {
    let mut field = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
      let mut sub = Vec::with_capacity(grid_size);
      for _ in 0..grid_size {
        sub.push(0u8);
      }
      field.push(sub);
    }
    HexGame {
      field,
      is_first_players_turn: true,
      game_over: false,
      winner: 0,
      already_checked: Vec::with_capacity(grid_size * 3),
      turn_counter: 0,
      played_indices: Vec::with_capacity(grid_size*grid_size)
    }
  }
  pub fn place_point(&mut self, x: usize, y: usize) -> bool {
    // pie rule against first turn advantage. disabled so mcts has to outplay heuristic
    // if self.turn_counter == 1 {
    //   let placed_num = if self.is_first_players_turn {1} else {2};
    //   let opponent_num = if self.is_first_players_turn {2} else {1};
    //   if self.field[y][x] == opponent_num {
    //     self.turn_counter += 1;
    //     self.field[y][x] = placed_num;
    //     self.played_indices.push(self.global_to_local(coord_to_num(self.field.len(), &(x, y))));
    //     self.is_first_players_turn = !self.is_first_players_turn;
    //     self.check_game_over();
    //     return true;
    //   }
    // }
    if self.field[y][x] == 0 {
      self.turn_counter += 1;
      let placed_num = if self.is_first_players_turn {1} else {2};
      self.field[y][x] = placed_num;
      self.played_indices.push(self.global_to_local(coord_to_num(self.field.len(), &(x, y))));
      self.is_first_players_turn = !self.is_first_players_turn;
      self.check_game_over();
      return true;
    }
    false
  }
  pub fn do_action(&mut self, action: usize) -> bool {
    let size = self.field.len();
    let y = action / size;
    let x = action % size;
    self.place_point(x,y)
  }
  pub fn check_line(field: &Vec<Vec<u8>>, x: usize, y: usize, first_player: bool, already_checked: &mut Vec<Coordinate>) -> bool {
    let field_size = field.len() - 1;
    if x <= field_size && y <= field_size {
      let find_res = already_checked.iter().position(|pos| pos.0 == x && pos.1 == y);
      if find_res.is_none() {
        already_checked.push((x,y));
        let player_num = if first_player {1} else {2};
        if field[y][x] == player_num {
          if first_player && y == field_size || !first_player && x == field_size {
            return true;
          }
          let neighbors = get_neighbors(field.len(), x, y);
          for neighbor_option in neighbors.iter() {
            if neighbor_option.is_some() {
              let neighbor = neighbor_option.unwrap();
              if HexGame::check_line(field, neighbor.0, neighbor.1, first_player, already_checked) {
                return true;
              }
            }
          }
        }
      }
    }
    false
  }
  pub fn check_game_over(&mut self) {
    let field_size = self.field.len();
    let mut already_checked = &mut self.already_checked;
    for i in 0..(field_size*2) {
      let first_player = i < field_size;
      if (i % field_size) == 0 {already_checked.clear()}
      let x;
      let y;
      if first_player {
        x = i;
        y = 0;
      } else {
        x = 0;
        y = i % field_size;
      };
      let is_winning_line = HexGame::check_line(&self.field, x, y, first_player, &mut already_checked);
      if is_winning_line {
        self.game_over = true;
        self.winner = if first_player {1} else {2};
        return;
      }
    }
    self.game_over = true;
    let grid_size = self.field.len();
    for y in 0..grid_size {
      for x in 0..grid_size {
        if self.field[y][x] == 0 {
          self.game_over = false;
          return;
        }
      }
    }
  }
  pub fn get_random(&self) -> usize {
    if !self.game_over {
      let mut counter = 0;
      for _ in 0..50 {
        let x = thread_rng().gen_range(0, self.field.len() - 1) as usize;
        let y = thread_rng().gen_range(0, self.field.len() - 1) as usize;
        counter += 1;
        if self.field[y][x] == 0 {
          return coord_to_num(self.field.len(), &(x, y));
        }
        if counter > 50 {
        }
      }
    }
    for y in 0..self.field.len() {
      for x in 0..self.field.len() {
        if self.field[y][x] == 0 {
          return coord_to_num(self.field.len(), &(x,y));
        }
      }
    }
    0
  }
  pub fn random_place(&mut self) {
    if !self.game_over {
      let mut placed = false;
      while !placed {
        let x = thread_rng().gen_range(0, self.field.len() - 1) as usize;
        let y = thread_rng().gen_range(0, self.field.len() - 1) as usize;
        placed = self.place_point(x, y);
      }
    }
  }
  pub fn get_possible_plays(&self) -> usize {
    let mut play_amount = 0;
    let grid_size = self.field.len();
    for y in 0..grid_size {
      for x in 0..grid_size {
        if self.field[y][x] == 0 {
          play_amount += 1;
        }
      }
    }
    play_amount
  }
  pub fn local_to_global_action_index(&self, decision: usize) -> usize {
    let mut converted_decision = decision;
    let mut amount_free = 0;
    let grid_size = self.field.len();
    let max_action = (grid_size * grid_size)-1;
    'DONE_INCREASING: for y in 0..grid_size {
      for x in 0..grid_size {
        if self.field[y][x] != 0 {
          converted_decision += 1;
        } else {
          amount_free += 1;
        }
        // println!("AMOUNT FREE {} decision {} {}", amount_free, decision, converted_decision);
        if amount_free > decision || converted_decision >= max_action {
          break 'DONE_INCREASING;
        }
      }
    }
    // println!("converted {} {} {} {}", self.get_possible_plays(), decision, converted_decision, self);
    converted_decision
  }
  pub fn global_to_local(&self, action: usize) -> usize {
    let mut converted_decision = 0;
    for i in 0..action {
      let (x,y) = num_to_coord(self.field.len(), i);
      if self.field[y][x] == 0 {
        converted_decision += 1;
      }
    }
    converted_decision
  }
  pub fn to_1d_array(&self) -> Vec<u8> {
    let mut new_array = Vec::with_capacity(self.field.len()*self.field.len());
    for row in self.field.iter() {
      for val in row {
        new_array.push(*val);
      }
    }
    new_array
  }
  pub fn reset(&mut self) {
    self.is_first_players_turn = true;
    self.game_over = false;
    self.winner = 0;
    self.already_checked.clear();
    self.played_indices.clear();
    self.turn_counter = 0;
    let grid_size = self.field.len();
    for y in 0..grid_size {
      for x in 0..grid_size {
        self.field[y][x] = 0;
      }
    }
  }
}

impl std::fmt::Display for HexGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let grid_size = self.field.len();
      let mut res = String::new();
      for y in 0..grid_size {
        for _ in 0..y {
          res.push_str(" ");
        }
        for x in 0..grid_size {
          res.push_str(self.field[y][x].to_string().as_str())
        }
        res.push_str("\n");
      }
      write!(f, "{}", res)
    }
}

pub mod tournament;
pub mod players;
pub mod hex_eval;