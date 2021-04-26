use crate::game_logic::{field::{GameField, render::Renderable}, log::GameLog, val::MAX_PLAYS};
use crate::game_logic::field::helper_structs::GameRules;
use crate::game_logic::field::rl::LearnHelper;
use plotters::prelude::*;
use crate::plot::{get_color};
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use std::fs::{read_to_string};
use serde_json::{to_string,from_str};

const SAMPLE_AMOUNT: usize = 2;
const TURN_AMOUNT: usize = 51;

pub fn get_possible_points(original_field: &GameField) -> Vec<i16> {
  let possible_plays = &original_field.possible_plays;
  let mut possible_points = Vec::with_capacity(possible_plays.len());
  let current_player = original_field.current_player as usize;
  for possible_play in possible_plays {
    let mut field_clone = original_field.empty_clone();
    field_clone.copy_from(&original_field);
    field_clone.update_possible_plays();
    let placed = field_clone.place_block_using_playinstance(*possible_play);
    assert!(placed, format!("Play is placeable"));
    possible_points.push(field_clone.scores[current_player] - original_field.scores[current_player])
  }
  possible_points
}

pub fn get_play_amounts(use_random_play: bool, use_random_heu: bool) -> Vec<Vec<Vec<i16> >> {
  let mut amounts: Vec<Vec<Vec<i16>>> = Vec::with_capacity(TURN_AMOUNT);
  for _ in 0..TURN_AMOUNT {
    amounts.push(Vec::with_capacity(SAMPLE_AMOUNT));
  }
  let mut field = GameField::new_with_rules(2, GameRules::deterministic());
  let start = Instant::now();
  for i in 0..SAMPLE_AMOUNT {
    field.reset(true);
    let mut current_turn: usize = 0;
    amounts[0].push(get_possible_points(&field));
    while !field.game_over {
      let play = if use_random_play {field.get_random_play()} else {field.get_best_heuristic_play(use_random_heu)};
      field.place_block_using_play(play);
      current_turn += 1;
      let index_to_use = if current_turn >= TURN_AMOUNT {TURN_AMOUNT-1} else {current_turn};
      amounts[index_to_use].push(get_possible_points(&field));
    }
    println!("DONE WITH {}", i);
  }
  let end = start.elapsed().as_millis();
  println!("simulation took {}ms thats {}ms/game", end, end as usize / SAMPLE_AMOUNT);
  amounts
}

pub fn sample_saver() {
  println!("STARTING RANDOM");
  let random_start = Instant::now();
  let mut file = File::create("randomamounts.json").unwrap();
  file.write_all(to_string(&get_play_amounts(true, false)).unwrap().as_bytes()).unwrap();
  let random_end = random_start.elapsed().as_millis();
  println!("STARTING HEURISTIC RANDOM WAS {}ms", random_end);
  let deterministic_start = Instant::now();
  let mut file = File::create("heuristicamounts.json").unwrap();
  file.write_all(to_string(&get_play_amounts(false, false)).unwrap().as_bytes()).unwrap();
  let deterministic_end_nonrand = deterministic_start.elapsed().as_millis();
  let deterministic_rand_start = Instant::now();
  let mut file = File::create("heuristicamounts_rand.json").unwrap();
  file.write_all(to_string(&get_play_amounts(false, true)).unwrap().as_bytes()).unwrap();
  let deterministic_end_rand = deterministic_rand_start.elapsed().as_millis();
  println!("DONE DOING EVERYTHING random took {}ms deterministic took {}ms deterministic with more random took {}ms", random_end,deterministic_end_nonrand, deterministic_end_rand);
}

pub type AllPossiblePoints = Vec<Vec<(i16, i16)>>;

pub fn enumerate_for_depth(field: &GameField, max_depth: usize, enumerated_data: &mut AllPossiblePoints) {
  let current_depth = field.played_indices.len();
  if max_depth <= current_depth {
    return
  }
  let current_player = field.current_player as usize;
  for possible_play in &field.possible_plays {
    let mut field_clone = field.empty_clone();
    field_clone.copy_from(&field);
    field_clone.update_possible_plays();
    let placed = field_clone.place_block_using_playinstance(*possible_play);
    assert!(placed, format!("The following play should have been a possible play {:?}", possible_play));
    enumerated_data[current_depth].push((field_clone.scores[current_player], field_clone.scores[current_player] - field.scores[current_player]));
    enumerate_for_depth(&field_clone, max_depth, enumerated_data);
  }
}

pub fn enumerate_possible_points_for_full_tree() {
  let field = GameField::new_with_rules(2, GameRules::deterministic());
  let max_depth = 3;
  let mut enumerated_data: AllPossiblePoints = Vec::with_capacity(max_depth);
  for depth in 0..max_depth {
    enumerated_data.push(Vec::with_capacity(MAX_PLAYS.pow(depth as u32)))
  }
  enumerate_for_depth(&field, max_depth, &mut enumerated_data);
  let mut file = File::create("all_possible_points.json").unwrap();
  file.write_all(to_string(&enumerated_data).unwrap().as_bytes()).unwrap();
}

use serde::{Serialize, Deserialize};
#[derive(Default, Serialize, Deserialize)]
pub struct PointsAtDepth {
  pub minus_two: u128,
  pub minus_one: u128,
  pub neutral: u128,
  pub plus_one: u128,
  pub plus_two: u128,
  pub plus_three: u128,
  pub plus_four: u128,
  pub plus_five: u128,
  pub plus_six: u128
}
pub type AllPoints = Vec<PointsAtDepth>;
pub type ChannelData = (usize, i64);
use crossbeam_channel::{unbounded, Sender};


pub fn parallel_enumerate_tree() {
  let field = GameField::new_with_rules(2, GameRules::deterministic());
  let max_depth = 6;
  let mut enumerated_data: AllPoints = Vec::with_capacity(max_depth);
  for _ in 0..max_depth {
    enumerated_data.push(PointsAtDepth::default())
  }
  // Create a channel of unbounded capacity.
  let (sender, receiver) = unbounded::<ChannelData>();
  let mut receiver_thread = std::thread::spawn(move || {
    let f_name = format!("all_possible_points_parallel_{}.json", max_depth);
    let mut i: u128 = 0;
    let mut part_num: u128 = 0;
    let start = Instant::now();
    'BL: loop {
      if let Ok(dat) = receiver.recv() {
        let current_depth = dat.0 as usize;
        let points_achieved = dat.1;
        if current_depth == std::usize::MAX {
          break 'BL;
        }
        match points_achieved {
          -2 => {enumerated_data[current_depth].minus_two += 1;},
          -1 => {enumerated_data[current_depth].minus_one += 1;},
          0 => {enumerated_data[current_depth].neutral += 1;},
          1 => {enumerated_data[current_depth].plus_one += 1;},
          2 => {enumerated_data[current_depth].plus_two += 1;},
          3 => {enumerated_data[current_depth].plus_three += 1;},
          4 => {enumerated_data[current_depth].plus_four += 1;},
          5 => {enumerated_data[current_depth].plus_five += 1;},
          _ => {enumerated_data[current_depth].plus_six += 1;}
        }
        if i % 100000000 == 0 {
          part_num += 1;
          let elapsed = start.elapsed().as_secs();
          println!("Already enumerated {} possibilities. Part {} / 180754 done. Took {} seconds", i, part_num, elapsed);
          std::fs::write(&f_name, to_string(&enumerated_data).unwrap().as_bytes()).unwrap();
        }
        i += 1;
      } else {
        println!("Receive Error");
      }
    }
    std::fs::write(&f_name, to_string(&enumerated_data).unwrap().as_bytes()).unwrap();
  });
  println!("starting parallel enumeration");
  parallell_enumerate(&field, max_depth, &sender);
  println!("Done with parallel enumeration");
  sender.send((std::usize::MAX, 0)).unwrap();
  receiver_thread.join().unwrap();
  println!("Receiver thread is done and wrote file");
}


use rayon::prelude::*;
pub fn parallell_enumerate(field: &GameField, max_depth: usize, sender: &Sender<ChannelData>) {
  let current_depth = field.played_indices.len();
  if max_depth <= current_depth {
    return
  }
  let current_player = field.current_player as usize;
  field.possible_plays.par_iter().for_each(|possible_play| {
    let mut field_clone = field.empty_clone();
    field_clone.copy_from(&field);
    field_clone.update_possible_plays();
    let placed = field_clone.place_block_using_playinstance(*possible_play);
    assert!(placed, format!("The following play should have been a possible play {:?}", possible_play));
    let points_achieved = field_clone.scores[current_player] - field.scores[current_player];
    sender.send((current_depth, points_achieved as i64)).unwrap();
    parallell_enumerate(&field_clone, max_depth, sender);
  })
}


pub fn enumerate_possible_points_for_full_tree_improved() {
  let field = GameField::new_with_rules(2, GameRules::deterministic());
  let max_depth = 5;
  let mut enumerated_data: AllPoints = Vec::with_capacity(max_depth);
  for _ in 0..max_depth {
    enumerated_data.push(PointsAtDepth::default())
  }
  enumerate_for_depth_improved(&field, max_depth, &mut enumerated_data);
  let mut file = File::create(format!("all_possible_points_improved_{}.json", max_depth)).unwrap();
  file.write_all(to_string(&enumerated_data).unwrap().as_bytes()).unwrap();
}

pub fn enumerate_for_depth_improved(field: &GameField, max_depth: usize, enumerated_data: &mut AllPoints) {
  let current_depth = field.played_indices.len();
  if max_depth <= current_depth {
    return
  }
  let current_player = field.current_player as usize;
  for possible_play in &field.possible_plays {
    let mut field_clone = field.empty_clone();
    field_clone.copy_from(&field);
    field_clone.update_possible_plays();
    let placed = field_clone.place_block_using_playinstance(*possible_play);
    assert!(placed, format!("The following play should have been a possible play {:?}", possible_play));
    let points_achieved = field_clone.scores[current_player] - field.scores[current_player];
    match points_achieved {
      -2 => {enumerated_data[current_depth].minus_two += 1;},
      -1 => {enumerated_data[current_depth].minus_one += 1;},
      0 => {enumerated_data[current_depth].neutral += 1;},
      1 => {enumerated_data[current_depth].plus_one += 1;},
      2 => {enumerated_data[current_depth].plus_two += 1;},
      3 => {enumerated_data[current_depth].plus_three += 1;},
      4 => {enumerated_data[current_depth].plus_four += 1;},
      5 => {enumerated_data[current_depth].plus_five += 1;},
      _ => {enumerated_data[current_depth].plus_six += 1;}
    }
    enumerate_for_depth_improved(&field_clone, max_depth, enumerated_data);
  }
}


pub fn mean(arr: &[i32]) -> i32 {
  if arr.len() == 0 {
    return 0;
  }
  let mut sum = 0;
  for entry in arr {
    sum += entry;
  }
  sum / arr.len() as i32
}

fn get_avg_linedata(list: &Vec<Vec<i32>>) -> Vec<(i32,i32)> {
  let mut lines = Vec::with_capacity(list.len());
  let mut i: i32 = 0;
  for entry in list {
    lines.push((i, mean(entry.as_slice())));
    i += 1;
  }
  lines
}

fn get_candlestick_data(list: &Vec<Vec<i32>>) -> Vec<(i32, i32, i32, i32, i32)> {
  let mut res = Vec::with_capacity(list.len());
  let mut i: i32 = 0;
  for entry in list {
    let mut high = 0;
    let mut low = if entry.len() == 0 {0} else {162};
    for sub_entry in entry {
      if *sub_entry > high {
        high = *sub_entry;
      }
      if *sub_entry < low {
        low = *sub_entry;
      }
    }
    res.push((i, high,high,low,low));
    i += 1;
  }
  res
}

pub fn sample_plotter(file_names: &[&str]) {
  let mut line_data = Vec::new();
  let mut candle_data = Vec::new();
  let mut total_turns = 0;
  let mut total_possible = 0;
  let mut total_entries = 0;
  for file_name in file_names {
    let list: Vec<Vec<i32>> = from_str(read_to_string(format!("{}{}",file_name,".json").as_str()).unwrap().as_ref()).unwrap();
    line_data.push((file_name.to_string(),get_avg_linedata(&list)));
    candle_data.push((file_name.to_string(),get_candlestick_data(&list)));
    let mut games_ended: Vec<i32> = Vec::new();
    let mut previous_amount = list[0].len() as i32;
    total_entries += previous_amount;
    let mut total_ended = 0;
    let line_data = get_avg_linedata(&list);
    for i in 0..list.len() {
      let current = list[i].len() as i32;
      total_possible += line_data[i].1;
      games_ended.push(previous_amount - current);
      total_ended += previous_amount - current;
      total_turns += (previous_amount - current) * (i as i32 + 1);
      previous_amount = current;
    }
    println!("{} total turns:{} avg turns: {} total possible: {} avg possible: {} total ended: {} {:?}", file_name, total_turns, total_turns / total_entries, total_possible, total_possible / 150, total_ended, games_ended);

  }
  make_line_and_candle_chart!("graphs/turn_amounts_full.png", (640, 480), "Average amount of possible moves per turn".to_owned(),
    0..50, 0..162,
    "Amount of Turns".to_owned(), "Possible Moves".to_owned(), line_data.as_slice(),candle_data.as_slice());
  make_line_chart!(
    "graphs/turn_amounts_lineonly.png", (640, 480), "Average amount of possible moves per turn".to_owned(),
    0..50, 0..162,
    "Amount of Turns".to_owned(), "Possible Moves".to_owned(), line_data.as_slice());
}