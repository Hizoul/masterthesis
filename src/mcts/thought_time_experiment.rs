
use crate::game_player::tournament::{Tournament, array_to_table};
use crate::ai::heuristic::HeuristicBot;
use crate::mcts::bot::MCTSBot;
use crate::ai::Bot;

pub fn do_thought_time_experiment() {
  let thought_time_increase = 500;
  let amount_of_players = 10;
  // let h1 = HeuristicBot::new(0);
  let mut players: Vec<Box<dyn Bot>> = Vec::new();
  for i in 1..amount_of_players {
    println!("THOUGHT TIME IS {}", i);
    let m1 = MCTSBot::new_with_time(i * thought_time_increase);
    players.push(Box::new(m1));
  }
  let mut t = Tournament::new(players, 10);
  t.run(Option::None);
  t.make_ratings();
  let player_names = t.player_names();
  println!("wins are {:?} {:?}", t.wins, t.wins_against);
  println!("{}", array_to_table(t.wins_against, player_names));
}