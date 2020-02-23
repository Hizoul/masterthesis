//! Wrappers around the Python API of the OpenAI gym.
use crate::game_logic::field::GameField;
use crate::game_logic::field::render::Renderable;
use crate::game_logic::field::rl::LearnHelper;
use crate::game_logic::field::helper_structs::GameRules;
use tch::{Tensor, Device};

/// The return value for a step.
#[derive(Debug)]
pub struct Step {
  pub obs: Tensor,
  pub action: i64,
  pub reward: f64,
  pub is_done: bool,
}

impl Step {
  /// Returns a copy of this step changing the observation tensor.
  pub fn make_clone(&self) -> Step {
    Step {
      obs: self.obs.copy(),
      action: self.action,
      reward: self.reward,
      is_done: self.is_done,
    }
  }
}

/// An OpenAI Gym session.
pub struct GymEnv {
  field: GameField,
  action_space: i64,
  observation_space: i64,
}

impl GymEnv {
  /// Creates a new session of the specified OpenAI Gym environment.
  pub fn new() -> GymEnv {
    let field = GameField::new_with_rules(2, GameRules::deterministic());
    let action_space = 190;
    let observation_space = 420;
    GymEnv {
      field,
      action_space,
      observation_space,
    }
  }

  /// Resets the environment, returning the observation tensor.
  pub fn reset(&mut self, device:Device) -> Tensor {
    self.field.reset(true);
    let obs = self.field.converted();
    Tensor::of_slice(obs.as_slice()).to_device(device)
  }

  pub fn pure_field(&mut self) -> Vec<f32> {self.field.converted()}

  /// Applies an environment step using the specified action.
  pub fn step(&mut self, action: i64, device: Device) -> Step {
    self.field.place_block_using_play_global_index(action as u8);
    if !self.field.game_over {
      let opponent_answer = self.field.get_random_heuristic_play();
      self.field.place_block_using_play(opponent_answer);
    }
    let reward = self.field.update_reward(0, true);
    let obs = self.field.converted();
    Step {
      obs: Tensor::of_slice(obs.as_slice()).to_device(device),
      reward: reward,
      is_done: self.field.game_over,
      action
    }
  }

  /// Returns the number of allowed actions for this environment.
  pub fn action_space(&self) -> i64 {
    self.action_space
  }

  /// Returns the shape of the observation tensors.
  pub fn observation_space(&self) -> i64 {
    self.observation_space
  }
}
