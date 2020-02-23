use super::tournament::{Tournament, array_to_table};
use crate::ai::heuristic::HeuristicBot;
use crate::ai::python::PythonBot;
use crate::mcts::rewrite::MCTSBot;

const file_name: &str = "final_tournament.json";

pub fn final_eval() {
  let h1 = HeuristicBot::new(0);
  let h2 = HeuristicBot::new(1);
  let h3 = HeuristicBot::new(2);
  let mut rl1 = PythonBot::new(0, "RL-Heuristic");
  rl1.is_heuristic = true;
  let rl2 = PythonBot::new(1, "RL-Selfplay#1");
  let rl3 = PythonBot::new(2, "RL-Selfplay#3");
  let rl4 = PythonBot::new(3, "RL-Selfplay#1-Heuristic");
  let rl5 = PythonBot::new(4, "RL-Selfplay#3-Heuristic");
  let rl6 = PythonBot::new(5, "RL-Nobleed#2");
  let rl7 = PythonBot::new(6, "RL-Nobleed#5");
  let rl8 = PythonBot::new(7, "RL-Nobleed#2-Heuristic");
  let rl9 = PythonBot::new(8, "RL-Nobleed#5-Heuristic");
  let m1 = MCTSBot::new_with_time(1000, 190, "MCTS-UCB", 201);
  let mut m2 = MCTSBot::new_with_time(1000, 190, "MCTS-RAVE", 201);
  m2.config.use_rave = true;
  let mut m3 = MCTSBot::new_with_time(1000, 190, "MCTS-PoolRAVE", 201);
  m3.config.use_rave = true;
  m3.config.use_pool_rave = true;
  let mut t = Tournament::new(vec![
    Box::new(rl1),
    Box::new(rl2),
    Box::new(rl3),
    Box::new(rl4),
    Box::new(rl5),
    Box::new(rl6),
    Box::new(rl7),
    Box::new(rl8),
    Box::new(rl9),
    Box::new(h1),
    Box::new(h2),
    Box::new(h3),
    Box::new(m1),
    Box::new(m2),
    Box::new(m3),
  ], 100);
  t.load(String::from(file_name)).unwrap();
  t.run(Option::Some(String::from(file_name)));
  t.make_ratings();
  let player_names = t.player_names();
  // println!("{:?}", t.wins_against);
  // println!("{}", array_to_table(t.wins_against, player_names));
  let mut elos = t.get_elos();
  elos.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
  println!("{:?}", elos);
}