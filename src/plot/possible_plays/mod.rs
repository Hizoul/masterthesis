use crate::game_logic::field::GameField;
use crate::game_logic::field::helper_structs::GameRules;
use crate::game_logic::field::rl::LearnHelper;
use plotters::prelude::*;
use crate::plot::{get_color};
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use std::fs::{read_to_string};
use serde_json::{to_string,from_str};

const SAMPLE_AMOUNT: usize = 10000;
const TURN_AMOUNT: usize = 51;

pub fn get_play_amounts(use_random_play: bool) -> Vec<Vec<u8>> {
  let mut amounts: Vec<Vec<u8>> = Vec::with_capacity(TURN_AMOUNT);
  for _ in 0..TURN_AMOUNT {
    amounts.push(Vec::with_capacity(SAMPLE_AMOUNT));
  }
  let mut field = GameField::new_with_rules(2, GameRules::deterministic());
  let start = Instant::now();
  for i in 0..SAMPLE_AMOUNT {
    field.reset(true);
    let mut current_turn: usize = 0;
    amounts[0].push(field.possible_plays.len() as u8);
    while !field.game_over {
      let play = if use_random_play {field.get_random_play()} else {field.get_best_heuristic_play(true)};
      field.place_block_using_play(play);
      current_turn += 1;
      let index_to_use = if current_turn >= TURN_AMOUNT {TURN_AMOUNT-1} else {current_turn};
      amounts[index_to_use].push(field.possible_plays.len() as u8);
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
  // let mut file = File::create("randomamounts.json").unwrap();
  // file.write_all(to_string(&get_play_amounts(true)).unwrap().as_bytes()).unwrap();
  let random_end = random_start.elapsed().as_millis();
  println!("STARTING HEURISTIC RANDOM WAS {}ms", random_end);
  let deterministic_start = Instant::now();
  let mut file = File::create("heuristicamounts.json").unwrap();
  file.write_all(to_string(&get_play_amounts(false)).unwrap().as_bytes()).unwrap();
  println!("DONE DOING EVERYTHING random took {}ms deterministic took {}ms ", random_end,deterministic_start.elapsed().as_millis());
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
  for file_name in file_names {
    let list: Vec<Vec<i32>> = from_str(read_to_string(format!("{}{}",file_name,".json").as_str()).unwrap().as_ref()).unwrap();
    line_data.push((file_name.to_string(),get_avg_linedata(&list)));
    candle_data.push((file_name.to_string(),get_candlestick_data(&list)))
  }
  make_line_and_candle_chart!("graphs/turn_amounts_full.png", (640, 480), "Average amount of possible moves per turn".to_owned(),
    0..50, 0..162,
    "Amount of Turns".to_owned(), "Possible Moves".to_owned(), line_data.as_slice(),candle_data.as_slice());
  make_line_chart!(
    "graphs/turn_amounts_lineonly.png", (640, 480), "Average amount of possible moves per turn".to_owned(),
    0..50, 0..162,
    "Amount of Turns".to_owned(), "Possible Moves".to_owned(), line_data.as_slice());
}