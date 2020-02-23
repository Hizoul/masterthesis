use crate::hex::HexGame;
use serde::{Deserialize, Serialize};
use std::fs::{write, read_to_string};
use bbt::{Rater, Rating, Outcome};
use std::collections::HashMap;
use std::path::Path;

pub trait HexBot: Send + Sync {
  fn make_play_hex(&mut self, game: &HexGame, is_second: bool) -> usize;
  fn get_name_hex(&self) -> String;
}

fn rating_to_num(rating: &Rating) -> f64 {
  rating.mu() - 3.0 * rating.sigma()
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct GameResult {
  pub players: Vec<String>,
  pub winner: u8
}

pub struct Tournament {
  pub players: Vec<Box<dyn HexBot>>,
  pub matches: u64,
  pub field_size: usize,
  pub results: Vec<GameResult>,
  pub ratings: HashMap<String, Rating>,
  pub wins: Vec<u64>,
  pub wins_against: Vec<Vec<u64>>,
  rater: Rater
}

pub fn array_to_table(arr: Vec<Vec<u64>>, labels: Vec<String>) -> String {
  let mut table = String::new();
  let mut top_index = 0;
  let size = arr.len();
  table.push_str("\\begin{table}[]\n\\begin{tabular}{");
  for _ in 0..size+1 {
    table.push_str("|l");
  }
  table.push_str("|}\n\\hline\n");
  table.push_str(" & ");
  for _ in 0..size {
    table.push_str(labels[top_index].as_str());
    if top_index >= size-1 {
      table.push_str(" \\\\ \\hline\n");
    } else {
      table.push_str(" & ");
    }
    top_index += 1;
  }
  top_index = 0;
  for entry in arr {
    let mut sub_index = 0;
    table.push_str(format!("{} & ", labels[top_index]).as_str());
    for sub in entry {
      table.push_str(sub.to_string().as_str());
      sub_index += 1;
      if sub_index >= size {
        table.push_str(" \\\\ \\hline\n");
      } else {
        table.push_str(" & ");
      }
    }
    top_index += 1;
  }
  table.push_str("\\end{tabular}\n\\end{table}");
  table
}

impl Tournament {
  pub fn new(players: Vec<Box<dyn HexBot>>, matches: u64, field_size: usize) -> Tournament {
    let player_amount = players.len();
    let capacity = matches as usize * player_amount;
    let rater = Rater::new(1500.0/6.0);
    let mut ratings = HashMap::with_capacity(player_amount);
    let mut wins = Vec::with_capacity(player_amount);
    let mut wins_against = Vec::with_capacity(player_amount);
    for player in players.iter() {
      ratings.insert(player.get_name_hex(), Rating::new(1500.0, 1500.0/3.0));
      wins.push(0);
      let mut sub_wins = Vec::with_capacity(player_amount);
      for _ in 0..players.len() {
        sub_wins.push(0);
      }
      wins_against.push(sub_wins);
    }
    Tournament {players,matches, ratings, rater,wins, wins_against,results: Vec::with_capacity(capacity), field_size}
  }
  pub fn persist(&self, file_name: String) -> Result<(), std::io::Error> {
    let file_content = serde_json::to_string(&self.results)?;
    write(file_name, file_content)
  }
  pub fn load(&mut self, file_name: String) -> Result<(), std::io::Error> {
    if Path::new(file_name.as_str()).exists() {
      self.results = serde_json::from_str(read_to_string(file_name)?.as_str())?;
    }
    Ok(())
  }
  pub fn make_ratings(&mut self) {
    for result in self.results.iter() {
      let p1 = self.ratings.get(&result.players[0]).unwrap();
      let p2 = self.ratings.get(&result.players[1]).unwrap();
      let draw = result.winner == 3;
      let outcome = if draw {
        Outcome::Draw
      } else {
        if result.winner == 0 {Outcome::Win} else {Outcome::Loss}
      };
      let (new_p1, new_p2) = self.rater.duel(p1.clone(), p2.clone(), outcome);
      self.ratings.insert(result.players[0].clone(), new_p1);
      self.ratings.insert(result.players[1].clone(), new_p2);
      let winner_index = if result.winner == 3 {0} else {result.winner};
      let looser_name_index = if result.winner == 3 {1} else {if result.winner == 0 {1} else {0}};
      let winner_name = &result.players[winner_index as usize];
      let player_index = self.players.iter().position(|r| &r.get_name_hex() == winner_name).unwrap();
      let looser_name = &result.players[looser_name_index as usize];
      let looser_index = self.players.iter().position(|r| &r.get_name_hex() == looser_name).unwrap();
      self.wins[player_index] += 1;
      self.wins_against[player_index][looser_index] += 1;
    }
  }
  pub fn run(&mut self,  file_name_option: Option<String>) {
    let player_amount = self.players.len();
    let mut match_indices: Vec<(usize, usize)> = Vec::new();
    for x in 0..player_amount {
      for y in 0..player_amount {
        if x != y && !match_indices.contains(&(x, y)) && !match_indices.contains(&(y, x)) {
          match_indices.push((x,y));
        }
      }
    }
    let total = match_indices.len();
    let mut current = 0;
    let file_name = if file_name_option.is_some() {file_name_option.unwrap()} else {String::new()};
    for player_indicies in match_indices.clone() {
      println!("IN MATCHUP {} / {}", current, total);
      current += 1;
      let mut field = HexGame::new(self.field_size);
      let mut player_names = Vec::with_capacity(2);
      {
        let player = &mut self.players[player_indicies.0];
        player_names.push(player.get_name_hex());
      }
      {
        let player = &mut self.players[player_indicies.1];
        player_names.push(player.get_name_hex());
      }
      let mut matches_played = 0;
      for matchup in self.results.iter() {
        if matchup.players == player_names {
          matches_played += 1;
        }
      }
      let match_amount: u64 = self.matches - matches_played;
      let mut swap = false;
      for i in 0..match_amount {
        println!("IN MATCH {} / {}", i, self.matches);
        field.reset();
        while !field.game_over {
          let mut is_first_player = field.is_first_players_turn;
          if swap {
            is_first_player = !is_first_player;
          }
          let current_player = if is_first_player {&mut self.players[player_indicies.0]} else {&mut self.players[player_indicies.1]};
          let play_to_make = current_player.make_play_hex(&field, !is_first_player);
          let placed = field.do_action(play_to_make);
          if !placed {
            panic!("Couldnt place block {} {} {}", current_player.get_name_hex(), play_to_make, field);
          }
        }
        let winner = if field.winner == 0 {3} else {
          let ret = field.winner - 1;
          if swap {
            if ret == 0 {1} else {0}
          } else {
            ret
          }
        };
        println!("ANOTHER GAME DONE {} {} {} {} {:?} {}", winner, swap, player_names[winner as usize], field.game_over, player_names, field);
        self.results.push(GameResult {
          players: player_names.clone(),
          winner
        });
        // swap = !swap;
        if (&file_name).len() > 0 {
          self.persist(file_name.clone()).unwrap();
        }
      }
    }
  }
  pub fn player_names(&self) -> Vec<String> {
    let mut names = Vec::with_capacity(self.players.len());
    for player in self.players.iter() {
      let name = player.get_name_hex();
      names.push(name.replace("_", "\\_"));
    }
    names
  }
  pub fn get_elos(&self) -> Vec<(String, f64)> {
    let mut elos = Vec::with_capacity(self.players.len());
    for player in self.players.iter() {
      let name = player.get_name_hex();
      let rating = rating_to_num(self.ratings.get(&name).unwrap());
      elos.push((name, rating));
    }
    elos
  }
  pub fn get_elo_tuple(&self) -> (f64,f64) {
    if self.players.len() > 2 {
      (0.0,0.0)
    } else {
      let mut elos = Vec::with_capacity(self.players.len());
      for player in self.players.iter() {
        let name = player.get_name_hex();
        let rating = rating_to_num(self.ratings.get(&name).unwrap());
        elos.push((name, rating));
      }
      let elo1 =  rating_to_num(self.ratings.get(&self.players[0].get_name_hex()).unwrap());
      let elo2 =  rating_to_num(self.ratings.get(&self.players[1].get_name_hex()).unwrap());
      (elo1, elo2)
    }
  }
}