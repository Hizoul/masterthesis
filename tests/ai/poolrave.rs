use rustyblocks::ai::heuristic::{HeuristicBot};
use rustyblocks::mcts::bot::{MCTSBot};
use rustyblocks::game_player::{PlaySupervisor};
use rustyblocks::game_logic::field::helper_structs::{GameRules};
use rustyblocks::game_logic::field::render::Renderable;

#[test]
fn ai_poolrave_mcts() {
  let mut b1 = MCTSBot::new();
  b1.use_rave = true;
  b1.use_pool_rave = false;
  let b2 = HeuristicBot::new(2);
  let mut ps = PlaySupervisor::new_with_rules(vec!(Box::new(b1), Box::new(b2)), GameRules::deterministic());
  ps.play();
  
  println!("SCORES FOR MCTS IS {:?} {} {}", ps.field.scores, ps.field.to_field_string(), serde_json::to_string(&ps.field.log).unwrap());
  assert!(ps.field.game_over, true);
}