use rustyblocks::game_player::tournament::{Tournament, array_to_table};
use rustyblocks::ai::heuristic::HeuristicBot;
use rustyblocks::game_player::mctsparameval::eval_params;

#[test]
fn tournament_test() {
  let h1 = HeuristicBot::new(0);
  let h2 = HeuristicBot::new(1);
  let h3 = HeuristicBot::new(2);
  let mut t = Tournament::new(vec![Box::new(h1), Box::new(h2)], 2);
  t.run(Option::None);
  assert_eq!(t.results.len(), 2);
  t.run(Option::None);
  assert_eq!(t.results.len(), 2);
  t.players.push(Box::new(h3));
  t.run(Option::None);
  assert_eq!(t.results.len(), 6);
  // let player_names = t.player_names();
  // println!("{}", array_to_table(t.wins_against, player_names));
  // eval_params();
}