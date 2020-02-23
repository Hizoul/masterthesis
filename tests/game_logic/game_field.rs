use rustyblocks::game_logic::field::{GameField};
use insta::{assert_debug_snapshot,assert_json_snapshot,assert_snapshot};
use rustyblocks::game_logic::field::render::Renderable;

#[test]
fn game_field() {
  let mut g = GameField::new(2);
  let mut scores = Vec::new();
  g.current_player = 1;
  g.current_block = 0;
  g.current_orientation = 0;
  assert_json_snapshot!("g", &g.lowest_ys);
  assert!(g.place_block_using_play(0));
  scores.push(g.scores.clone());
  assert_snapshot!("field_after_first_play", g.to_field_string());
  assert!(g.place_block_using_play(0));
  scores.push(g.scores.clone());
  assert!(g.place_block_using_play(0));
  scores.push(g.scores.clone());
  assert!(g.place_block_using_play(0));
  scores.push(g.scores.clone());
  assert_snapshot!("field_after_four_plays", g.to_field_string());
  assert_snapshot!("log_after_four_plays", serde_json::to_string(&g.log).unwrap());
  assert_debug_snapshot!("score_after_four_plays", scores);
  while !g.game_over {
    g.place_block_using_play(0);
    scores.push(g.scores.clone());
  }
  assert_snapshot!("field_after_game_over", g.to_field_string_with_player(false));
  assert_snapshot!("log_after_game_over", serde_json::to_string(&g.log).unwrap());
  assert_debug_snapshot!("scores", scores);

  g.reset(true);
  assert_snapshot!("after_reset_field", g.to_field_string());
  assert_debug_snapshot!("after_reset_scores", g.scores);
  scores.clear();
  while !g.game_over {
    g.place_block_using_play(0);
    scores.push(g.scores.clone());
  }
  assert_debug_snapshot!("after_reset_and_played_field_reshapeable", g.to_reshapeable_array(false));
  assert_debug_snapshot!("after_reset_and_played_field_reshapeable_as_second", g.to_reshapeable_array(true));
  assert_snapshot!("after_reset_and_played_field", g.to_field_string());
  assert_debug_snapshot!("after_reset_and_played_scores", g.scores);

}