use rustyblocks::ai::random::{RandomBot};
use rustyblocks::game_player::{PlaySupervisor};
use rustyblocks::game_logic::field::render::Renderable;
use insta::{assert_snapshot};

#[test]
fn ai_random() {
  let b1 = RandomBot::new();
  let b2 = RandomBot::new();
  let mut ps = PlaySupervisor::new(vec!(Box::new(b1), Box::new(b2)));
  ps.play();
  assert_snapshot!("field_after_game", ps.field.to_field_string());
}