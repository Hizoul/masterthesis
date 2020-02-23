use super::azmcts::AZMCTS;
use super::aznet::{AlphaZeroNet,get_value,get_values};
use tch::{
  Device, Reduction,Kind,
  nn::{ModuleT,Module,VarStore,sgd,Sgd,Optimizer,OptimizerConfig},
  Tensor};
use crate::game_logic::field::{GameField};
use crate::game_logic::field::helper_structs::GameRules;
use crate::game_logic::field::render::{Renderable};

pub struct AZTrainer {
  pub mcts: AZMCTS,
  pub field: GameField,
  pub states: Vec<Vec<f32>>,
  pub probs: Vec<Vec<f64>>,
  pub values: Vec<f32>,
  pub states_update: Vec<Vec<f32>>,
  pub probs_update: Vec<Vec<f64>>,
  pub values_update: Vec<f32>,
  pub current_players: Vec<i8>,
  pub opt: Optimizer<Sgd>,
  pub learn_rate: f64,
  pub learn_rate_mult: f64
}

impl AZTrainer {
  pub fn new() -> AZTrainer {
    let device = Device::Cpu;
    let vs = VarStore::new(device);
    let zero = AlphaZeroNet::new(&vs.root());
    let mcts = AZMCTS::new(zero);
    let field = GameField::new_with_rules(2, GameRules::deterministic());
    let states = Vec::new();
    let probs = Vec::new();
    let values = Vec::new();
    let states_update = Vec::new();
    let probs_update = Vec::new();
    let values_update = Vec::new();
    let current_players = Vec::new();
    let opt = Sgd::default().build(&vs, 1e-4).unwrap();
    AZTrainer {
      mcts,
      field,
      states,
      probs,
      values,
      states_update,
      values_update,
      probs_update,
      current_players,
      opt,
      learn_rate: 2e-3,
      learn_rate_mult: 1.0
    }
  }
  pub fn collect_selfplay_data(&mut self, game_amount: usize) {
    self.states_update.clear();
    self.probs_update.clear();
    self.values_update.clear();
    for _ in 0..game_amount {
      self.do_self_play_episode();
      self.states_update.append(&mut self.states);
      self.probs_update.append(&mut self.probs);
      self.values_update.append(&mut self.values);
    }
  }
  pub fn do_self_play_episode(&mut self) {
    self.field.reset(true);
    self.mcts.update_with_move(-1);
    self.states.clear();
    self.probs.clear();
    self.values.clear();
    self.current_players.clear();
    while !self.field.game_over {
      let current_player = self.field.current_player;
      let (action, probs) = self.mcts.get_action_with_probs(&mut self.field, true);
      self.field.place_block_using_play_global_index(action);
      self.states.push(self.field.to_image(current_player == 1));
      self.probs.push(probs);
      self.current_players.push(current_player);
    }
    let winner = self.field.get_winning_player();
    for player in self.current_players.iter() {
      if winner == 3 {
        self.values.push(0.0);
      } else if winner == *player {
        self.values.push(1.0);
      } else {
        self.values.push(-1.0);
      }
    }
  }
  pub fn train_step(&mut self, lr: f64) {
    self.opt.zero_grad();
    self.opt.set_lr(lr);
    let mut i = 0;
    for state in self.states_update.iter() {
      let state_tensor = Tensor::of_slice(state.as_slice());
      let (value, probs) = self.mcts.net.custom_forward(&state_tensor, true);
      let actual_value = Tensor::of_slice(&[self.values_update[i]]);
      let value_loss = value.view(-1).mse_loss(&actual_value, Reduction::None);
      let actual_probs = Tensor::of_slice(self.probs_update[i].as_slice());
      let policy_loss = (probs * actual_probs).sum(Kind::Float).mean(Kind::Float);
      let loss = value_loss + policy_loss;
      loss.backward();
      self.opt.step();
      i += 1;
    }
  }
  pub fn update_network(&mut self) {
    self.gather_answers();
    let old_probs = self.probs.clone();
    let old_values = self.values.clone();
    for i in 0..10 { // todo: configurable epochs
      self.train_step(self.learn_rate*self.learn_rate_mult);
      self.gather_answers();
      let new_probs = self.probs.clone();
      let new_values = self.values.clone();
      // todo : adaptive learnrate multiplier
    }
  }
  pub fn gather_answers(&mut self) {
    self.values.clear();
    self.probs.clear();
    for state in self.states_update.iter() {
      let state_tensor = Tensor::of_slice(state.as_slice());
      let (value, prob) = self.mcts.net.custom_forward(&state_tensor, true);
      self.values.push(get_value(&value) as f32);
      self.probs.push(get_values(&prob));
    }
  }
  pub fn eval(&mut self) -> f64 {
    0.0
  }
  pub fn run(&mut self, train_amount: usize, check_frequency: usize) {
    self.mcts.playout_amount = 2;
    let best_win_ratio = 0.0;
    for i in 0..train_amount {
      self.collect_selfplay_data(2);
      println!("UPDATING NETWORK");
      self.update_network();
      if i % check_frequency == 0 {
        let win_ratio = self.eval();
        // self.mcts.net.save("current.model");
      }
    }
  }
}