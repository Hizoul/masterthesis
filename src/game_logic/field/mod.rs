use super::tetromino::{Tetromino,Position};
use super::log::{GameLog,Payload,Play};
use super::val::{GAMEFIELD_MAX_HEIGHT,GAMEFIELD_MAX_WIDTH,RANDOM_SEED,DEFAULT_PIECE_AMOUNT,AMOUNT_OF_UNIQUE_PIECES_USIZE,GAMEFIELD_MAX_WIDTH_USIZE,GAMEFIELD_MAX_HEIGHT_USIZE,
AMOUNT_OF_UNIQUE_PIECES,SKIPREASON_BLOCKEMPTY,SKIPREASON_NOTHING_FITS,MAX_USER_TETROS,AMOUNT_ROTATIONS,MAX_PLAYS};
use super::cache::{ShapeCache};
use crate::game_logic::field::helper_structs::{GameRules,Touching};
use std::sync::{Arc,RwLock};
use rand;
use rand::{Rng,thread_rng};
use rand_hc::Hc128Rng;
use rand::SeedableRng;
use std::thread::sleep;
use std::time::Duration;

pub mod user_actions;
pub mod rl;
pub mod render;
pub mod helper_structs;


#[derive(Debug,Clone)]
pub struct GameField {
  pub players: i8,
  pub game_over: bool,
  pub roll_index: i8,
  pub tetrominos: Vec<Tetromino>,
  pub piece_counter: Vec<Vec<i8>>,
  pub log: GameLog,
  pub current_player: i8,
  pub current_block: i8,
  pub current_orientation: i8,
  pub current_x: i8,
  pub rng: Hc128Rng,
  pub possible_plays: Vec<Play>,
  pub lowest_ys: Vec<i8>,
  pub lowest_ys_cmp: Vec<i8>,
  pub shape_cache: Arc<ShapeCache>,
  pub positions: Vec<Position>,
  pub scores: Vec<i16>,
  pub touching_blocks: Vec<Vec<Arc<RwLock<Touching>>>>,
  pub already_counted: Vec<Vec<Arc<Tetromino>>>,
  connected_cache: Vec<Arc<Tetromino>>,
  rules: GameRules,
  blocks_to_check: Vec<i8>,
  pub played_indices: Vec<u8>,
  pub heuristic_eval_cache: Vec<u8>,
  pub prev_reward: Vec<f64>
}
 
impl GameField {
  pub fn new(players: i8) -> GameField {
    return GameField::new_with_rules(players, GameRules::default());
  }
  pub fn new_with_rules(players: i8, rules: GameRules) -> GameField {
    return GameField::new_with_cache(players, Arc::new(ShapeCache::new()), rules)
  }
  pub fn new_with_cache(players: i8, shape_cache: Arc<ShapeCache>, rules: GameRules) -> GameField {
    let players_usize = players as usize;
    let mut scores = Vec::with_capacity(players_usize);
    let mut rewards: Vec<f64> = Vec::with_capacity(players_usize);
    let mut touching_blocks = Vec::with_capacity(players_usize);
    let mut already_counted = Vec::with_capacity(players_usize);
    let mut piece_counter = Vec::with_capacity(players_usize);
    for _ in 0..players {
      scores.push(0);
      rewards.push(0.);
      touching_blocks.push(Vec::with_capacity(MAX_USER_TETROS));
      already_counted.push(Vec::with_capacity(MAX_USER_TETROS));
      let mut user_counter = Vec::with_capacity(AMOUNT_OF_UNIQUE_PIECES_USIZE);
      for _ in 0..AMOUNT_OF_UNIQUE_PIECES {
        user_counter.push(DEFAULT_PIECE_AMOUNT)
      }
      piece_counter.push(user_counter)
    }
    let mut lowest_ys = Vec::with_capacity(GAMEFIELD_MAX_WIDTH_USIZE);
    for _ in 0..GAMEFIELD_MAX_WIDTH {
      lowest_ys.push(0);
    }
    let mut srng: Hc128Rng = Hc128Rng::from_seed(RANDOM_SEED);
    let mut blocks_to_check = Vec::with_capacity(AMOUNT_OF_UNIQUE_PIECES_USIZE);
    if !rules.roll_dice {
      for i in 0..AMOUNT_OF_UNIQUE_PIECES {
        blocks_to_check.push(i);
      }
    }
    let mut field = GameField {
      players: players,
      game_over: false,
      roll_index: 0,
      tetrominos: Vec::with_capacity((GAMEFIELD_MAX_WIDTH_USIZE * GAMEFIELD_MAX_HEIGHT_USIZE) / 4),
      piece_counter,
      log: GameLog::new(),
      current_player: if rules.roll_dice { srng.gen_range(0, players) } else { 0 },
      current_block: if rules.roll_dice { srng.gen_range(0, AMOUNT_OF_UNIQUE_PIECES) } else { 0 },
      current_orientation: 0,
      current_x: 0,
      rng: srng,
      possible_plays: Vec::with_capacity(MAX_PLAYS), // found out by printing max, calculation yields ~ 190 max plays but some of these are not placeable
      shape_cache,
      lowest_ys,
      lowest_ys_cmp: Vec::with_capacity(GAMEFIELD_MAX_WIDTH_USIZE),
      positions: Vec::with_capacity(GAMEFIELD_MAX_WIDTH_USIZE * GAMEFIELD_MAX_HEIGHT_USIZE),
      scores,
      touching_blocks,
      already_counted,
      connected_cache: Vec::with_capacity(5),
      rules,
      blocks_to_check,
      played_indices: Vec::with_capacity(50),
      heuristic_eval_cache: Vec::with_capacity(50),
      prev_reward: rewards
    };
    field.update_possible_plays();
    field
  }
  pub fn update_lowest_for_block(&mut self, t: &Tetromino) {
    for s in self.shape_cache.get_block_shape(t).iter() {
      self.positions.push(*s);
      let x = s.x;
      let xu = x as usize;
      if x >= 0 && x < GAMEFIELD_MAX_WIDTH {
        if self.lowest_ys[xu] <= s.y {
          self.lowest_ys[xu] = s.y + 1;
        }
      }
    }
  }
  pub fn highest_y(&mut self, block: i8, rotation: i8, initial_x: i8) -> i8 {
    let mut yval = 0;
    let parts = self.shape_cache.get_translated(block, rotation, initial_x, 0);
    self.lowest_ys_cmp.clear();
    for _ in 0..GAMEFIELD_MAX_WIDTH {
      self.lowest_ys_cmp.push(99);
    }
    for p in parts.iter() {
      if p.x >= 0 && p.x < GAMEFIELD_MAX_WIDTH {
        let v = self.lowest_ys_cmp[p.x as usize];
        if v > p.y {
          self.lowest_ys_cmp[p.x as usize] = p.y;
        }
      }
    }
    let mut sum = 0;
    for p in parts.iter() {
      if p.x >= 0  && p.x < GAMEFIELD_MAX_WIDTH && self.lowest_ys[p.x as usize] != 99 {
        sum = self.lowest_ys[p.x as usize] - self.lowest_ys_cmp[p.x as usize];
      } else if p.x >= 0 && p.x < GAMEFIELD_MAX_WIDTH {
        sum = self.lowest_ys_cmp[p.x as usize];
      }
      if sum > yval {
        yval = sum;
      }
    }
    yval
  }
  pub fn can_place_block(&self, block: &Tetromino) -> bool {
    if !self.rules.roll_dice {
      if self.piece_counter[self.current_player as usize][block.block as usize] <= 0 {
        return false
      }
    }
    let parts = self.shape_cache.get_block_shape(block);
    for part in parts.iter() {
      if part.y >= GAMEFIELD_MAX_HEIGHT || part.y < 0 {
        return false
      }
      if part.x >= GAMEFIELD_MAX_WIDTH || part.x < 0 {
        return false
      }
    }
    for pos in self.positions.iter() {
      for part in parts.iter() {
        if part.x == pos.x && part.y == pos.y {
          return false
        }
      }
    }
    true
  }
  pub fn update_possible_plays(&mut self) {
    self.possible_plays.clear();
    if self.rules.roll_dice {
      self.blocks_to_check.clear();
      self.blocks_to_check.push(self.current_block);
    }
    let mut i: u8 = 0;
    let mut total: u8 = 0;
    for block_type in self.blocks_to_check.clone().iter() {
      let mut tet = Tetromino {
        from: self.current_player,
        block: *block_type,
        rotation: 0,
        position: Position {x: 0, y: 0}
      };
      for x in 0..GAMEFIELD_MAX_WIDTH {
        for rotation in 0..AMOUNT_ROTATIONS[*block_type as usize] {
          let can_skip = if self.rules.roll_dice {
            false
          } else {
            match total {
              14 | 16 | 18 | 29 | 30 | 32 | 33 | 66 | 67 | 68 | 72 | 76 | 102 | 106 | 107 | 109 | 174 | 176 | 178 | 180 | 182 | 183 | 184 | 185 | 186 | 187 | 188 | 189 => true,
              _ => false
            }
          };
          if !can_skip {
            tet.rotation = rotation;
            tet.position.x = x;
            tet.position.y = self.highest_y(*block_type, rotation, x);
            if tet.position.y < GAMEFIELD_MAX_HEIGHT && self.can_place_block(&tet) {
              self.possible_plays.push(Play {x,rotation,block:*block_type,index:i,global_index: total});
              i += 1;
            }
          }
          total += 1;
        }
      }
    }
    if self.possible_plays.len() > 0 {
      let first_play = &self.possible_plays[0];
      self.current_block = first_play.block;
      self.current_orientation = first_play.rotation;
      self.current_x = first_play.x;
    }
  }
  pub fn find_play_using_global_index(&self, play_index: u8) -> Option<Play> {
    let mut play_opt: Option<Play> = None;
    'FOUNDPLAY: for play in self.possible_plays.iter() {
      if play.global_index == play_index {
        play_opt = Some(*play);
        break 'FOUNDPLAY;
      }
    }
    return play_opt
  }
  pub fn place_block_using_play_global_index(&mut self, play_index: u8) -> bool {
    let mut play_opt: Option<Play> = self.find_play_using_global_index(play_index);
    if play_opt.is_some() {
      return self.place_block_using_playinstance(play_opt.unwrap());
    }
    false
  }
  pub fn place_block_using_play(&mut self, play_index: u8) -> bool {
    let play_opt = self.possible_plays.get(play_index as usize);
    if play_opt.is_some() {
      let to_use = *play_opt.unwrap();
      return self.place_block_using_playinstance(to_use);
    }
    false
  }
  pub fn place_block_using_playinstance(&mut self, play: Play) -> bool {
    let y = self.highest_y(play.block, play.rotation, play.x);
    let placed = self.place_block(&Tetromino {
      block: play.block,
      rotation: play.rotation,
      from: self.current_player,
      position: Position {
        y, x: play.x
      }
    });
    if placed {
      self.played_indices.push(play.index);
    }
    return placed
  }
  pub fn place_block(&mut self, block: &Tetromino) -> bool {
    if block.from == self.current_player && self.can_place_block(block) {
      self.tetrominos.push(*block);
      self.decrease_piece_count(self.current_player, block.block);
      self.log.block_placed(self.current_player, block.block, block.rotation, block.position.x, block.position.y);
      self.update_lowest_for_block(block);
      self.update_score_via_block(block);
      self.determine_next_player();
      return true
    }
    false
  }
  pub fn determine_next_player(&mut self) {
    self.current_player += 1;
    if self.current_player >= self.players {
      self.current_player = 0;
    }
    if !self.rules.roll_dice || self.roll_block_for(self.current_player).unwrap() {
      self.update_possible_plays();
      if self.possible_plays.len() <= 0 {
        self.log.player_skipped(self.current_player, self.current_block, SKIPREASON_NOTHING_FITS);
        self.roll_index += 1;
        if self.roll_index >= self.players {
          self.game_over = true;
        } else {
          self.determine_next_player()
        }
      } else {
        self.roll_index = 0;
        self.log.add_consideration(0);
      }
    }
  }
  pub fn roll_block_for(&mut self, player: i8) -> Option<bool> {
    let mut rolled_block: i8 = 0;
    let mut player_has_block = false;
    let mut next = player;
    let mut looped_once = false;
    while !player_has_block {
      if next >= self.players {
        next = 0;
        if looped_once {
          self.game_over = true;
          return Option::Some(false)
        }
        looped_once = true;
      }
      rolled_block = self.roll_block();
      if *(self.piece_counter.get(next as usize)?.get(rolled_block as usize)?) > 0 {
        player_has_block = true;
      } else {
        self.log.player_skipped(next, rolled_block, SKIPREASON_BLOCKEMPTY);
        next += 1;
      }
    }
    self.log.rolled_block(next, rolled_block);
    self.current_player = next;
    self.current_block = rolled_block;
    self.current_orientation = 0;
    return Option::Some(true)
  }
  pub fn roll_block(&mut self) -> i8 {
    return self.rng.gen_range(0, AMOUNT_OF_UNIQUE_PIECES);
  }
  fn decrease_piece_count(&mut self, player: i8, block: i8) -> Option<i8> {
    let r = &mut self.piece_counter[player as usize];
    let current = r.get(block as usize)?;
    let new = *current - 1;
    r[block as usize] = new;
    Option::Some(new)
  } 
  pub fn get_connected(&mut self, origin_block: Arc<RwLock<Touching>>, player: usize) {
    let ob = origin_block.read().unwrap();
    for b in ob.touching.iter() {
      if !self.connected_cache.contains(b) {
        self.connected_cache.push(b.clone());
        let mut touch_for_b = None;
        'search: for turw in self.touching_blocks[player].iter() {
          let tu = turw.read().unwrap();
          if tu.block == *b {
            touch_for_b = Some(turw);
            break 'search;
          }
        }
        if touch_for_b.is_some() {
          let cloned_touch = touch_for_b.unwrap().clone();
          self.get_connected(cloned_touch, player);
        }
      } 
    }
  }

  fn get_touching(&mut self, player: usize, block: Arc<Tetromino>) -> Arc<RwLock<Touching>> {
    for touch in self.touching_blocks[player].iter() {
      let mut is_found = false;
      { 
        let lower_t_try = touch.try_read();
        if lower_t_try.is_ok() {
          let lower_t = lower_t_try.unwrap();
          if lower_t.block == block {
            is_found = true;
          }
        }
      }
      if is_found {
        return touch.clone();
      }
    }
    let t = Arc::from(RwLock::new(Touching::new(block)));
    self.touching_blocks[player].push(t.clone());
    return t;
  }
  pub fn update_score_via_block(&mut self, to_update_for: &Tetromino) {
    let to_update_for_arc = Arc::from(*to_update_for);
    let mut player_blocks: Vec<Arc<Tetromino>> = Vec::new();
    for block in self.tetrominos.iter() {
      if block.from == to_update_for.from {
        player_blocks.push(Arc::from(*block));
      }
    }
    let player_as_usize = to_update_for.from as usize;
    // PLUS POINTS
    {
      let touching = self.get_touching(player_as_usize, to_update_for_arc.clone());
      for compare_block in player_blocks.iter() {
        let contained: bool;
        {
          let rt = touching.read().unwrap();
          contained = rt.touching.contains(&compare_block);
        }
        if to_update_for_arc != *compare_block && !contained {
          'comp: for p1 in self.shape_cache.get_block_shape(to_update_for).iter() {
            for p2 in self.shape_cache.get_block_shape(compare_block).iter() {
              if ((p1.y == p2.y) && (p1.x == (p2.x + 1) ||
              p1.x == (p2.x - 1))) ||
              ((p1.x == p2.x) &&
              (p1.y == (p2.y - 1) || p1.y == (p2.y + 1))) {
                {
                  let mut attempt = touching.try_write();
                  while attempt.is_err() {
                    sleep(Duration::from_nanos(10));
                    attempt = touching.try_write();
                  }
                  let mut rt = attempt.unwrap();
                  rt.touching.push(compare_block.clone());
                }
                {
                  let otherrw = self.get_touching(player_as_usize, compare_block.clone());
                  let mut attempt = otherrw.try_write();
                  while attempt.is_err() {
                    sleep(Duration::from_nanos(10));
                    attempt = otherrw.try_write();
                  }
                  let mut other = attempt.unwrap();
                  other.touching.push(to_update_for_arc.clone());
                }
                break 'comp;
              }
            }
          }
        }
      }
    }
    let c = self.touching_blocks[player_as_usize].clone();
    let iter = c.iter();
    for btr in iter {
      let block = btr.read().unwrap();
      if !self.already_counted[player_as_usize].contains(&block.block) && block.touching.len() > 0 {
        self.connected_cache.clear();
        self.connected_cache.push(block.block.clone());
        self.get_connected(btr.clone(), player_as_usize);
        if self.connected_cache.len() >= 3 {
          for b in self.connected_cache.iter() {
            if !self.already_counted[player_as_usize].contains(b) {
              self.already_counted[player_as_usize].push(b.clone());
              self.scores[b.from as usize] += 1;
            }
          }
        }
      }
    }
    // MINUS POINTS
    let mut to_subtract = 0;
    'SUB: for part in self.shape_cache.get_block_shape(to_update_for).iter() {
      let mut highest_y: i8 = 0;
      let mut found_part = false;
      for compare_part in self.positions.iter() {
        if (part.x == compare_part.x && compare_part.y < part.y) && highest_y <= compare_part.y {
          found_part = true;
          highest_y = compare_part.y + 1;
        }
      }
      if highest_y > 0 || !found_part {
        to_subtract += highest_y as i16 - part.y as i16;
        if self.rules.limit_minus && to_subtract < -2 {
          to_subtract = -2;
          break 'SUB;
        }
      }
    }
    self.scores[to_update_for.from as usize] += to_subtract;
  }
  pub fn restore_from_log(&mut self, log: &GameLog, restore_played_indices: bool) {
    self.log.copy_from(log);
    for entry in log.log.iter() {
      match entry {
        Payload::PayloadPlaced {x,y,block,orientation,from} => {
          let t = Tetromino::new(*block,*orientation,*from,Position::new(*x,*y));
          if restore_played_indices {
            self.update_possible_plays();
            let play_index = self.possible_plays.iter().position(|play| 
              play.x == *x && play.block == *block && play.rotation == *orientation
            );
            if play_index.is_some() {
              self.played_indices.push(play_index.unwrap() as u8);
            }
          }
          self.update_lowest_for_block(&t);
          self.tetrominos.push(t);
          self.decrease_piece_count(*from, *block);
          self.update_score_via_block(&t);
          self.determine_next_player();
        },
        Payload::PayloadSkipped {from, block, reason} => {
          self.determine_next_player();
        },
        _ => {}
      }
    }
    if self.rules.roll_dice {
      let current_roll = self.log.get_current_roll();
      self.current_player = current_roll.0;
      self.current_block = current_roll.1;
      self.current_orientation = 0;
    }
    self.update_possible_plays();
    self.game_over = self.possible_plays.len() == 0;
  }
  pub fn restore_from_played_indices(&mut self, played: &Vec<u8>) {
    self.update_possible_plays();
    for to_play in played.iter() {
      self.place_block_using_play(*to_play);
    }
  }
  pub fn get_winning_player(&self) -> i8 {
    let mut max = i16::min_value();
    let mut index = 0;
    let mut winning_index = 0;
    for score in self.scores.iter() {
      if *score > max {
        max = *score;
        winning_index = index;
      } else if *score == max {
        winning_index = 3;
      }
      index += 1;
    }
    winning_index
  }
  pub fn get_random_play(&self) -> u8 {
    if self.possible_plays.len() <= 1 {
      0
    } else {
      thread_rng().gen_range(0, self.possible_plays.len() - 1) as u8
    }
  }
  pub fn empty_clone(&self) -> GameField {
    let field = GameField::new_with_cache(self.players, self.shape_cache.clone(), self.rules);
    field
  }
  pub fn reset(&mut self, update_possible_plays: bool) {
    self.game_over = false;
    self.tetrominos.clear();
    self.log.log.clear();
    self.roll_index = 0;
    self.current_block = 0;
    self.current_orientation = 0;
    self.current_x = 0;
    self.current_player = 0;
    self.lowest_ys.clear();
    self.positions.clear();
    self.scores.clear();
    self.connected_cache.clear();
    self.blocks_to_check.clear();
    self.played_indices.clear();
    self.prev_reward.clear();
    for p in 0..self.players {
      let pu = p as usize;
      self.scores.push(0);
      self.prev_reward.push(0.);
      self.touching_blocks[pu].clear();
      self.already_counted[pu].clear();
      self.piece_counter[pu].clear();
      for _ in 0..AMOUNT_OF_UNIQUE_PIECES {
        self.piece_counter[pu].push(DEFAULT_PIECE_AMOUNT);
      }
    }
    for _ in 0..GAMEFIELD_MAX_WIDTH {
      self.lowest_ys.push(0);
    }
    if !self.rules.roll_dice {
      for i in 0..AMOUNT_OF_UNIQUE_PIECES {
        self.blocks_to_check.push(i);
      }
    }
    if update_possible_plays {
      self.update_possible_plays();
    }
  }
  pub fn copy_from(&mut self, other: &GameField) {
    self.reset(false);
    for p in other.played_indices.iter() {
      self.played_indices.push(*p);
    }
    self.restore_from_log(&other.log, false);
  }
}