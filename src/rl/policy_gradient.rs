
// Policy gradient example.
// This uses OpenAI Gym environment through rust-cpython.
//
// This is adapted from OpenAI Spinning Up series:
// https://spinningup.openai.com/en/latest/spinningup/rl_intro3.html
// A TensorFlow Python implementation can be found here:
// https://github.com/openai/spinningup/blob/master/spinup/examples/pg_math/2_rtg_pg.py
use crate::rl::gym_env::{GymEnv, Step};
use tch::{nn, nn::OptimizerConfig, Kind::*, Tensor};

const MAX_FOR_BLOCKS: i64 = 230;

fn model(p: &nn::Path, nin: i64, nact: i64) -> impl nn::Module {
  nn::seq()
    .add(nn::linear(p / "lin1", nin, 32, Default::default()))
    .add_fn(|xs| xs.tanh())
    .add(nn::linear(p / "lin2", 32, nact, Default::default()))
}

fn accumulate_rewards(steps: &[Step]) -> Vec<f64> {
  let mut rewards: Vec<f64> = steps.iter().map(|s| s.reward).collect();
  let mut acc_reward = 0f64;
  for (i, reward) in rewards.iter_mut().enumerate().rev() {
    if steps[i].is_done {
      acc_reward = 0.0;
    }
    acc_reward += *reward;
    *reward = acc_reward;
  }
  rewards
}

/// Trains an agent using the policy gradient algorithm.
pub fn run()  {
  let mut env = GymEnv::new();
  let action_space = env.action_space();
  println!("action space: {}", env.action_space());
  println!("observation space: {:?}", env.observation_space());

  let device = tch::Device::Cpu;
  let vs = nn::VarStore::new(device);
  let model = model(&vs.root(), env.observation_space(), action_space);
  let mut opt = nn::Adam::default().build(&vs, 1e-2).unwrap();

  for epoch_idx in 0..5 {
    let mut obs = env.reset(device);
    let mut steps: Vec<Step> = vec![];
    // Perform some rollouts with the current model.
    loop {
      let action = tch::no_grad(|| {
        let unsqueezed = obs.unsqueeze(0);
        let applied = unsqueezed.apply(&model);
        let softmaxed = applied.softmax(1, Float);
        let mut chosen_action = 0;
        let mut current_max = 0.0;
        let mut actual_max = 0.0;
        for i in 0..action_space {
          let possible_val: f32 = obs.get(MAX_FOR_BLOCKS + i).into();
          let action_possible = possible_val == 1.0;
          let action_probability: f32 = softmaxed.get(0).get(i).into();
          if action_possible && action_probability > current_max {
          current_max = action_probability;
          chosen_action = i;
          }
          if action_probability > actual_max {
          actual_max = action_probability;
          }
        }
        // valide action auswählen
        chosen_action
      });
      let action = i64::from(action);
      let step = env.step(action, device);
      steps.push(step.make_clone());
      obs = if step.is_done { env.reset(device) } else { step.obs };
      if step.is_done && steps.len() > 5000 {
        break;
      }
    }
    let sum_r: f64 = steps.iter().map(|s| s.reward).sum();
    let episodes: i64 = steps.iter().map(|s| s.is_done as i64).sum();
    println!(
      "epoch: {:<3} episodes: {:<5} avg reward per episode: {:.2} {}",
      epoch_idx,
      episodes,
      sum_r / episodes as f64, sum_r
    );

    // Train the model via policy gradient on the rollout data.
    let batch_size = steps.len() as i64;
    let actions_vec: Vec<i64> = steps.iter().map(|s| s.action).collect();
    let actions = Tensor::of_slice(&actions_vec).unsqueeze(1).to_device(device);
    let rewards = accumulate_rewards(&steps);
    let rewards = Tensor::of_slice(&rewards).to_kind(Float).to_device(device);
    let kind_to_use = if device == tch::Device::Cpu {tch::kind::FLOAT_CPU} else {tch::kind::FLOAT_CUDA};
    let mut to_scatter = Tensor::zeros(&[batch_size, action_space], kind_to_use);
    let action_mask = to_scatter.scatter_(
      1,
      &actions,
      &Tensor::from(1f32),
    );
    let obs: Vec<Tensor> = steps.into_iter().map(|s| s.obs).collect();
    let logitss = Tensor::stack(&obs, 0);
    let logits = logitss.apply(&model);
    let log_probs = (action_mask * logits.log_softmax(1, Float)).sum1(&[1], false, Float);
    let loss = -(rewards * log_probs).mean(Float);
    opt.backward_step(&loss)
  }
}
