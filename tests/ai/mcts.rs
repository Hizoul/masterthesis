use rustyblocks::ai::heuristic::{HeuristicBot};
use rustyblocks::mcts::bot::{MCTSBot};
use rustyblocks::game_player::{PlaySupervisor};
use rustyblocks::game_logic::field::helper_structs::{GameRules};
use rustyblocks::game_logic::field::render::Renderable;

#[test]
fn ai_basic_mcts() {
  let b1 = MCTSBot::new();
  let b2 = HeuristicBot::new(2);
  let mut ps = PlaySupervisor::new_with_rules(vec!(Box::new(b1), Box::new(b2)), GameRules::deterministic());
  ps.play();
  // assert_snapshot!("field_after_game", ps.field.to_field_string());
  println!("SCORES FOR MCTS IS {:?} {} {}", ps.field.scores, ps.field.to_field_string(), serde_json::to_string(&ps.field.log).unwrap());
  assert!(ps.field.game_over, true);
}