pub mod heuristics;
pub mod game_logic;
pub mod tree;
pub mod mcts;
pub mod unsafe_mcts;
pub mod game_player;
pub mod zobrist;
pub mod ai;
#[cfg(feature = "rl")]
pub mod rl;
pub mod hex;
#[cfg(not(target_arch = "wasm32"))]
pub mod db;
#[cfg(not(target_arch = "wasm32"))]
pub mod plot;
// use game_logic::log::{GameLog};
use hex::HexGame;
use game_logic::field::{GameField};
use game_logic::field::helper_structs::{GameRules};
use game_logic::val::{GAMEFIELD_MAX_HEIGHT};
use game_logic::tetromino::{Tetromino,Position,get_rotation_amount};
use game_logic::field::render::Renderable;
use game_logic::log::GameLog;
use game_logic::field::rl::LearnHelper;
use heuristics::tune::heuristic_evaluator;
extern crate libc;
use libc::{c_char,c_double};
use std::ffi::{CString,CStr};
use std::convert::From;

// A struct that can be passed between C and Rust
#[repr(C)]
pub struct ActionAnswerTuple {
    placed: u8,
    reward: c_double,
    done: u8,
    winner: u8
}

// Conversion functions
impl From<(u8, c_double, u8,u8)> for ActionAnswerTuple {
    fn from(tup: (u8, c_double, u8,u8)) -> ActionAnswerTuple {
        ActionAnswerTuple { placed: tup.0, reward: tup.1, done: tup.2, winner: tup.3 }
    }
}

impl From<ActionAnswerTuple> for (u8, c_double, u8,u8) {
    fn from(tup: ActionAnswerTuple) -> (u8, c_double, u8,u8) {
        (tup.placed, tup.reward, tup.done, tup.winner)
    }
}

#[no_mangle]
pub extern fn field_new() -> *mut GameField {
  Box::into_raw(Box::new(GameField::new_with_rules(2, GameRules::deterministic())))
}

#[no_mangle]
pub extern fn field_free(ptr: *mut GameField) {
  if ptr.is_null() { return }
  unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn field_clone(ptr: *mut GameField) -> *mut GameField {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let mut cloned = field.empty_clone();
  cloned.copy_from(field);
  Box::into_raw(Box::new(cloned))
}

#[no_mangle]
pub extern fn field_do_action(ptr: *mut GameField, user_action: u8) -> u8  {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let placed = field.place_block_using_play_global_index(user_action);
  if placed {
    0
  } else {
    1
  }
}

#[no_mangle]
pub extern fn field_do_action_with_answer(ptr: *mut GameField, user_action: u8, use_heuristic: u8) -> ActionAnswerTuple  {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let placed = field.place_block_using_play_global_index(user_action);
  let placed_answer = if placed {0} else {1};
  let reward = field.update_reward(0, true);
  if placed {
    if !field.game_over {
      let play = if use_heuristic == 1 { 
        field.get_best_heuristic_play_for_params(4.95238700733393,0.115980839099051,0.0603795891552745,19.0148438477953, true, 0)
      } else if use_heuristic == 2 {
        field.get_random_heuristic_play()
      } else { field.get_best_heuristic_play(true) };
      field.place_block_using_play(play);
    }
  }
  let done = field.game_over;
  let done_answer = if done {
    // println!("SCORE IS {:?}", field.scores);
    0
  } else {1};
  let winner = if done {field.get_winning_player() == 0} else {false};
  let winner_answer = if winner {0} else {1};
  return (placed_answer, reward, done_answer, winner_answer).into();
}

#[no_mangle]
pub extern fn field_do_action_with_answer_heu(ptr: *mut GameField, connectability_weight: c_double, enemy_block_wheight: c_double, touching_weight: c_double, score_weight: c_double, use_heuristic: u8) -> ActionAnswerTuple  {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let play = field.get_best_heuristic_play_for_params(connectability_weight, enemy_block_wheight, touching_weight, score_weight, true, 0);
  let placed = field.place_block_using_play(play);
  let placed_answer = if placed {0} else {1};
  let reward = field.update_reward(0, true);
  if placed {
    if !field.game_over {
      let play = if use_heuristic == 1 { 
        field.get_best_heuristic_play_for_params(4.95238700733393,0.115980839099051,0.0603795891552745,19.0148438477953, true, 0)
      } else if use_heuristic == 2 {
        field.get_random_heuristic_play()
      } else { field.get_best_heuristic_play(true) };
      field.place_block_using_play(play);
    }
  }
  let done = field.game_over;
  let done_answer = if done {
    // println!("SCORE IS {:?}", field.scores);
    0
  } else {1};
  let winner = if done {field.get_winning_player() == 0} else {false};
  let winner_answer = if winner {0} else {1};
  return (placed_answer, reward, done_answer, winner_answer).into();
}


#[no_mangle]
pub extern fn field_do_action_self_play(ptr: *mut GameField, user_action: u8, score_for_player: i8) -> ActionAnswerTuple  {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let placed = field.place_block_using_play_global_index(user_action);
  let placed_answer = if placed {0} else {1};
  let reward = field.basic_reward(score_for_player);
  let done = field.game_over;
  let winner = if done {field.get_winning_player() == score_for_player && field.scores[0] != field.scores[1]} else {false};
  let winner_answer = if winner {0} else {1};
  let done_answer = if done {
    // println!("SCORE IS {:?} {}", field.scores, winner);
    0
  } else {1};
  return (placed_answer, reward, done_answer, winner_answer).into();
}

#[no_mangle]
pub extern fn field_get_reward(ptr: *mut GameField, player: i8) -> c_double  {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  field.update_reward(player, true)
}

#[no_mangle]
pub extern fn field_get_full_reward(ptr: *mut GameField, player: i8) -> c_double  {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  field.calculate_full_reward(player, true)
}

#[no_mangle]
pub extern fn field_place_block(ptr: *mut GameField, block: i8, rotation: i8, x: i8) -> i64  {
  let rotation_amount = get_rotation_amount(block);
  if rotation >= rotation_amount {
    return 1
  }
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let player = field.current_player;
  let y = field.highest_y(block, rotation, x);
  if y >= GAMEFIELD_MAX_HEIGHT {
    return 1
  }
  let placed = field.place_block(&Tetromino {
        block,
        rotation,
        from: player,
        position: Position {
          y, x
        }
      });
  if placed {
    0
  } else {
    1
  }
}

#[no_mangle]
pub extern fn field_counter_action(ptr: *mut GameField, doing_for_player_0: i64) -> i64 {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let mut placed = false;
  if !field.game_over {
    let play = if doing_for_player_0 == 1 { field.get_random_play() } else if doing_for_player_0 == -1 {
      field.get_best_heuristic_play_for_params(4.95238700733393,0.115980839099051,0.0603795891552745,19.0148438477953, true, 0)
    } else { field.get_best_heuristic_play(true) };
    placed = field.place_block_using_play(play);
  }
  if placed {
    0
  } else {
    1
  }
}

#[no_mangle]
pub extern fn field_counter_action_index(ptr: *mut GameField) -> u8 {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let play = field.get_best_heuristic_play_for_params(4.95238700733393,0.115980839099051,0.0603795891552745,19.0148438477953, true, 0);
  field.possible_plays[play as usize].global_index
}

#[no_mangle]
pub extern fn field_reset(ptr: *mut GameField) {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  field.reset(true);
}

use std::mem;
#[no_mangle]
pub extern fn field_to_array(ptr: *mut GameField, self_play_is_second_player: u8) ->  *mut i8 {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };

  let reshapeable = field.to_reshapeable_array(self_play_is_second_player == 1);
  let mut boxed_slice: Box<[i8]> = reshapeable.into_boxed_slice();
  let array: *mut i8 = boxed_slice.as_mut_ptr();
  // Prevent the slice from being destroyed (Leak the memory).
  mem::forget(boxed_slice);
  return array
}


#[no_mangle]
pub extern fn free_field_array(ptr: *mut u8) {
  if ptr.is_null() { return }
  unsafe { Vec::<u8>::from_raw_parts(ptr, 420, 420); }
}

#[no_mangle]
pub extern fn free_score_array(ptr: *mut i16) {
  if ptr.is_null() { return }
  unsafe { Vec::<i16>::from_raw_parts(ptr, 2, 2); }
}

#[no_mangle]
pub extern fn field_get_score(ptr: *mut GameField) ->  *mut i16 {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };

  let reshapeable = field.scores.clone();
  let mut boxed_slice: Box<[i16]> = reshapeable.into_boxed_slice();
  let array: *mut i16 = boxed_slice.as_mut_ptr();
  // Prevent the slice from being destroyed (Leak the memory).
  mem::forget(boxed_slice);
  return array
}

#[no_mangle]
pub extern fn field_is_game_over(ptr: *mut GameField) -> u8 {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  if field.game_over {
    // println!("SCORE IS {:?}", field.scores);
    1
  } else {
    0
  }
}

#[no_mangle]
pub extern fn field_get_winner(ptr: *mut GameField) -> u8 {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  field.get_winning_player() as u8
}

#[no_mangle]
pub extern fn field_to_string(ptr: *mut GameField) -> *mut c_char {
    let field = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let c_str = CString::new(field.to_playable_string()).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern fn field_to_json_log(ptr: *mut GameField) -> *mut c_char {
    let field = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let c_str = CString::new(serde_json::to_string(&field.log).unwrap()).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern fn field_restore_log(ptr: *mut GameField, log_to_restore: *mut c_char) {
  let field: &mut GameField = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let c_str = unsafe {
    assert!(!log_to_restore.is_null());
    CStr::from_ptr(log_to_restore)
  };
  let log: GameLog = serde_json::from_str(&c_str.to_str().unwrap()).unwrap();
  field.restore_from_log(&log, true);
}

#[no_mangle]
pub extern fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern fn eval_heuristic_weights(connectability_weight: c_double, enemy_block_wheight: c_double, touching_weight: c_double, score_weight: c_double) -> u64 {
  heuristic_evaluator(connectability_weight, enemy_block_wheight, touching_weight, score_weight)
}

#[no_mangle]
pub extern fn hex_new() -> *mut HexGame {
  Box::into_raw(Box::new(HexGame::new(11)))
}

#[no_mangle]
pub extern fn hex_free(ptr: *mut HexGame) {
  if ptr.is_null() { return }
  unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn hex_do_action_with_answer(ptr: *mut HexGame, x: u8, y: u8) -> ActionAnswerTuple  {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let placed = field.place_point(x as usize, y as usize);
  let placed_answer = if placed {0} else {1};
  let reward;
  if placed {
    field.random_place();
  }
  let done = field.game_over;
  let done_answer = if done {
    0
  } else {1};
  let winner = if done {field.winner == 1} else {false};
  let winner_answer = if winner {
    reward = 1.0;
    0
  } else {
    reward = if done {-1.0} else {0.0};
    1
  };
  return (placed_answer, reward, done_answer, winner_answer).into();
}

#[no_mangle]
pub extern fn hex_to_array(ptr: *mut HexGame) ->  *mut u8 {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  let reshapeable = field.to_1d_array();
  let mut boxed_slice: Box<[u8]> = reshapeable.into_boxed_slice();
  let array: *mut u8 = boxed_slice.as_mut_ptr();
  // Prevent the slice from being destroyed (Leak the memory).
  mem::forget(boxed_slice);
  return array
}

#[no_mangle]
pub extern fn free_hex_array(ptr: *mut u8) {
  if ptr.is_null() { return }
  unsafe { Vec::<u8>::from_raw_parts(ptr, 121, 121); }
}


#[no_mangle]
pub extern fn hex_reset(ptr: *mut HexGame) {
  let field = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
  };
  field.reset();
}

#[allow(dead_code)]
pub extern fn fix_linking_when_not_using_stdlib() { panic!() }