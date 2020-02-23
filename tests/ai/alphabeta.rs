use rustyblocks::ai::alphabeta::{AlphaBetaBot};
use rustyblocks::game_player::{PlaySupervisor};
use rustyblocks::game_logic::field::helper_structs::{GameRules};
use rustyblocks::game_logic::field::render::Renderable;
use insta::{assert_snapshot};

#[test]
fn ai_alphabeta() {
  let b1 = AlphaBetaBot::new(0, 4);
  let b2 = AlphaBetaBot::new(1, 1);
  let mut ps = PlaySupervisor::new_with_rules(vec!(Box::new(b1), Box::new(b2)), GameRules::deterministic());
  ps.play();
  assert_snapshot!("field_after_game", ps.field.to_field_string());
  assert_snapshot!("log_after_game", serde_json::to_string(&ps.field.log).unwrap());
}