use crate::ai::{Bot};
use crate::game_logic::field::{GameField};
use serde::{Deserialize, Serialize};
use crate::game_logic::field::render::Renderable;
use reqwest::Client;
use crate::game_logic::field::rl::LearnHelper;

#[derive(Serialize,Deserialize)]
pub struct PythonRequest {
  pub algo: u8,
  pub obs: Vec<i8>
}
#[derive(Serialize,Deserialize)]
pub struct PythonResponse {
  pub play: u8
}
#[derive(Serialize,Deserialize)]
pub struct PythonHeuResponse {
  pub play: [f64;4]
}

pub struct PythonBot {
  pub algo: u8,
  pub name: String,
  pub is_heuristic: bool,
  client: Client
}

impl PythonBot {
  pub fn new(algo: u8, name: &str) -> PythonBot {
    PythonBot {
      algo,
      name: name.to_owned(),
      client: Client::new(),
      is_heuristic: false
    }
  }
}

impl Bot for PythonBot {
  fn make_play(&mut self, field: &GameField, is_second: bool) -> u8 {
    let mut cloned = field.clone();
    let request_payload = PythonRequest {
      algo: self.algo,
      obs: cloned.to_reshapeable_array(is_second)
    };
    let mut res = self.client.post("http://localhost:8080")
        .json(&request_payload)
        .send().unwrap();
    let bot_play = if self.is_heuristic {
      let result: PythonHeuResponse = res.json().unwrap();
      cloned.get_best_heuristic_play_for_params(result.play[0],result.play[1],result.play[2],result.play[3], true, 0)
    } else {
      let result: PythonResponse = res.json().unwrap();
      let global_play = result.play;
      let play = cloned.find_play_using_global_index(global_play);
      if play.is_some() {
        let found = play.unwrap();
        found.index
      } else {
        200
      }
    };
    bot_play
  }
  fn get_name(&self) -> String {format!("python_{}", self.algo).to_owned()}
}