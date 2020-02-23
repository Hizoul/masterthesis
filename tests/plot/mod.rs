use rustyblocks::plot::possible_plays::{sample_saver,sample_plotter};
use rustyblocks::plot::reward::{get_values,make_reward_graph,make_hex_graph};
use plotters::prelude::*;
// use rustyblocks::plot::optuna_sqlite::make_optuna_hyperparametersearch_graph;
#[test]
fn basic_plot_test() {
  // sample_saver();
  sample_plotter(&["User Heuristic", "Random Heuristic", "Random"]);
  // make_optuna_hyperparametersearch_graph(
  //   &vec![("./python-rl/ppo2_cnn-False_simple-False.db", "MLP".to_owned()),
  //   ("./python-rl/ppo2_cnn-True_simple-False.db", "CNN".to_owned())
  //   ]).unwrap();
  // make_reward_graph(
  //   &vec![("heuristic-rewards-all.json", "Heuristic".to_owned())],
  //   "heuristic-rewards.png", "RL Heuristic Training", "Episode", "Average Reward"
  // ).unwrap();
  make_hex_graph(
    &vec![
      ("rnd_hex_0.json", "MCTS-UCB".to_owned()),
      ("rnd_hex_1.json", "MCTS-RAVE".to_owned()),
      ("rnd_hex_2.json", "MCTS-PoolRAVE".to_owned()),
    ],
    "hex_random.png", "Win rates against Heuristic", "Field Size", "Win rate", SeriesLabelPosition::MiddleRight
  ).unwrap();
  make_hex_graph(
    &vec![
      ("heu_hex_0.json", "MCTS-UCB".to_owned()),
      ("heu_hex_1.json", "MCTS-RAVE".to_owned()),
      ("heu_hex_2.json", "MCTS-PoolRAVE".to_owned()),
    ],
    "hex_heu.png", "Win rates against Heuristic", "Field Size", "Win rate", SeriesLabelPosition::LowerRight
  ).unwrap();
  // make_hex_graph(
  //   &vec![
  //     ("heu_hex_0.json", "MCTS-UCB-Heuristic".to_owned()),
  //     ("heu_hex_1.json", "MCTS-RAVE-Heuristic".to_owned()),
  //     ("heu_hex_2.json", "MCTS-PoolRAVE-Heuristic".to_owned()),
  //     ("rnd_hex_0.json", "MCTS-UCB-Random".to_owned()),
  //     ("rnd_hex_1.json", "MCTS-RAVE-Random".to_owned()),
  //     ("rnd_hex_2.json", "MCTS-PoolRAVE-Random".to_owned()),
  //   ],
  //   "hex_eval.png", "Win rates against Heuristic", "Field Size", "Win rate"
  // ).unwrap();

  make_reward_graph(
    &vec![("nobleed-2-rewards-per20k.json", "RL-Nobleed-Heuristic#2".to_owned()),("nobleed-3-rewards-per20k.json", "RL-Nobleed-Heuristic#5".to_owned())],
    "nobleed_heu_training.png", "RL-Selfplay-Nobleed Training", "Steps x 20000", "Average Reward"
  ).unwrap();
  // make_reward_graph(
  //   &vec![("longnobleed.json", "RL-Nobleed-Heuristic#5".to_owned())],
  //   "nboleed_heu_long.png", "RL-Selfplay-Nobleed Training", "Steps x 20000", "Average Reward"
  // ).unwrap();
  make_reward_graph(
    &vec![("heuristic-rewards-per20k.json", "RL-Heuristic".to_owned())],
    "heuristic_training.png", "RL-Heuristic Training", "Steps x 50000", "Average Reward"
  ).unwrap();
  make_reward_graph(
    &vec![("0-per100k-rewards.json", "RL-Selfplay#1-LSTM".to_owned()),
    ("1-per100k-rewards.json", "RL-Selfplay#2-LSTM".to_owned()),
    ("2-per100k-rewards.json", "RL-Selfplay#3-LSTM".to_owned()),
    ("3-per100k-rewards.json", "RL-Selfplay#4-LSTM".to_owned()),
    ("4-per100k-rewards.json", "RL-Selfplay#5-LSTM".to_owned()),],
    "selfplay_training_lstm.png", "RL-Selfplay-LSTM Training", "Steps x 20000", "Average Reward"
  ).unwrap();
  // make_reward_graph(
  //   &vec![("0-0-rewards-per20k.json", "RL-Selfplay#1".to_owned()),
  //   ("0-1-rewards-per20k.json", "RL-Selfplay#2".to_owned()),
  //   ("0-2-rewards-per20k.json", "RL-Selfplay#3".to_owned()),
  //   ("2-0-rewards-per20k.json", "RL-Selfplay#4".to_owned()),
  //   ("2-1-rewards-per20k.json", "RL-Selfplay#5".to_owned()),
  //   ("2-2-rewards-per20k.json", "RL-Selfplay#6".to_owned()),],
  //   "selfplay_heu_0.png", "Selfplayed against Heuristic Training", "Steps x 20000", "Average Reward"
  // ).unwrap();
  // make_reward_graph(
  //   &vec![
  //   ("2-0-rewards-per20k.json", "RL-Selfplay#4".to_owned()),
  //   ("2-1-rewards-per20k.json", "RL-Selfplay#5".to_owned()),
  //   ("2-2-rewards-per20k.json", "RL-Selfplay#6".to_owned()),],
  //   "selfplay_heu_2.png", "Selfplayed against Heuristic Training", "Steps x 20000", "Average Reward"
  // ).unwrap();
  make_reward_graph(
    &vec![
    ("long-1-rewards.json", "RL-Selfplay#1-Heuristic".to_owned()),
    ("long-2-rewards.json", "RL-Selfplay#3-Heuristic".to_owned())],
    "selfplay_heu_long.png", "RL-Selfplay-Heuristic Training", "Steps x 20000", "Average Reward"
  ).unwrap();
  // make_reward_graph(
  //   &vec![
  //     ("0-1-rewards-per20k.json", "RL-Selfplay#1-Heuristic#1".to_owned()),
  //   ("0-2-rewards-per20k.json", "RL-Selfplay#3-Heuristic#2".to_owned()),],
  //   "selfplay_heu_training.png", "Selfplayed against Heuristic Training", "Steps x 20000", "Average Reward"
  // ).unwrap();
  // make_reward_graph(
  //   &vec![
  //     ("0-1-rewards-per20k.json", "RL-Selfplay#1-Heuristic#1".to_owned()),
  //   ("0-2-rewards-per20k.json", "RL-Selfplay#3-Heuristic#2".to_owned()),],
  //   "mcts_beta.png", "Pre filled MCTS vs User Heuristic", "Steps x 20000", "Win rate"
  // ).unwrap();

  // make_reward_graph(
  //   &vec![("0-all-rewards.json", "RL-Selfplay#1".to_owned()),
  //   ("1-all-rewards.json", "RL-Selfplay#2".to_owned()),
  //   ("2-all-rewards.json", "RL-Selfplay#3".to_owned()),
  //   ("3-all-rewards.json", "RL-Selfplay#4".to_owned()),
  //   ("4-all-rewards.json", "RL-Selfplay#5".to_owned()),],
  //   "selfplay_training_al.png", "Selfplay Training", "Episode", "Average Reward"
  // ).unwrap();
}