use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::helper_structs::{GameRules};
use rustyblocks::game_logic::field::render::Renderable;
use insta::{assert_json_snapshot,assert_snapshot};

#[test]
fn game_field_rules() {
  let mut g = GameField::new_with_rules(2, GameRules::deterministic());
  while !g.game_over {
    g.place_block_using_play(0);
  }
  assert_snapshot!("field_after_game_over", g.to_field_string());
  assert_json_snapshot!("scores", g.scores);
}