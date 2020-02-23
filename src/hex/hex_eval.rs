use crate::hex::tournament::{Tournament, HexBot, array_to_table};
use crate::hex::players::{HexHeuristicBot, HexRandomBot};
use crate::mcts::rewrite::MCTSBot;
use std::fs::{write};

const FILE_NAME: &str = "hex_final_tournament.json";

pub fn do_hex_eval() {
  let mut results: Vec<Vec<f64>> = Vec::with_capacity(3);
  let start = "heu";
  for _ in 0..3 {
    results.push(Vec::with_capacity(8));
  }
  for i in 2..12 {
    for mcts_version in 0..3 {
      let tournament_name = format!("{}_hex_mcts_{}_size_{}.json", start, mcts_version, i);
      let field_size = i;
      let mcts_action_size = field_size * field_size;
      let heu = HexHeuristicBot::new();
      let rand = HexRandomBot::new();
      let name = match mcts_version {
        1 => "MCTS-RAVE",
        2 => "MCTS-PoolRave",
        _ => "MCTS-UCB"
      };
      let mut m1 = MCTSBot::new_with_time(1000, mcts_action_size, name, mcts_action_size);
      if mcts_version > 0 {
        m1.config.use_rave = true;
      }
      if mcts_version > 1 {
        m1.config.use_pool_rave = true;
      }
      let mut t = Tournament::new(vec![
        Box::from(heu),
        Box::new(m1),
      ], 100, field_size);
      t.load(tournament_name.clone()).unwrap();
      t.run(Option::Some(tournament_name));
      t.make_ratings();
      println!("GOT WINS AGAINST {:?} {} {} {}", t.wins_against, mcts_version, t.wins_against[1][0], t.wins_against[1][0] / 100);
      let win_rate = t.wins_against[1][0] as f64;
      results[mcts_version].push(win_rate);
      // let player_names = t.player_names();
      // let mut elos = t.get_elos();
      // println!("{:?}", t.wins_against);
      // println!("{}", array_to_table(t.wins_against, player_names));
      // elos.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
      // println!("{:?}", elos);
    }
  }
  println!("got winrates {:?}", results);
  for mcts_version in 0..3 {
    let file_content = serde_json::to_string(&results[mcts_version]).unwrap();
    write(format!("{}_hex_{}.json", start, mcts_version), file_content).unwrap();
  }
}

pub fn do_hex_regular_tournament() {
  let field_size = 7;
  let mcts_action_size = field_size * field_size;
  let heu = HexHeuristicBot::new();
  let rand = HexRandomBot::new();
  let mut m1 = MCTSBot::new_with_time(1000, mcts_action_size, "MCTS-UCB", mcts_action_size);
  m1.config.simulation_amount = 5;
  let mut m2 = MCTSBot::new_with_time(1000, mcts_action_size, "MCTS-RAVE", mcts_action_size);
  m2.config.use_rave = true;
  let mut m3 = MCTSBot::new_with_time(1000, mcts_action_size, "MCTS-PoolRAVE", mcts_action_size);
  m3.config.use_rave = true;
  m3.config.use_pool_rave = true;
  let mut t = Tournament::new(vec![
    Box::new(m1),
    // Box::new(m2),
    // Box::new(m3)
    Box::from(heu),
    // Box::from(rand),
  ], 100, field_size);
  t.load(String::from(FILE_NAME)).unwrap();
  t.run(Option::Some(String::from(FILE_NAME)));
  t.make_ratings();
  let player_names = t.player_names();
  let mut elos = t.get_elos();
  println!("{:?}", t.wins_against);
  println!("{}", array_to_table(t.wins_against, player_names));
  elos.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
  println!("{:?}", elos);
}