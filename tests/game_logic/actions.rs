use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::render::Renderable;
use rustyblocks::game_logic::field::helper_structs::{GameRules};
use rustyblocks::game_logic::field::user_actions::UserActions;
use rustyblocks::game_logic::field::user_actions::{ACTION_LEFT,ACTION_RIGHT,ACTION_ROTATE_LEFT,ACTION_ROTATE_RIGHT,ACTION_NEXT_BLOCK,ACTION_PLACE_BLOCK};
use insta::{assert_snapshot};

#[test]
fn game_field_actions() {
  let mut g = GameField::new_with_rules(2, GameRules::deterministic());
  assert_snapshot!("field_before_anything", g.to_playable_string());
  g.do_user_action(ACTION_LEFT);
  assert_snapshot!("field_after_move_left_over_bounds", g.to_playable_string());
  g.do_user_action(ACTION_RIGHT);
  assert_snapshot!("field_after_move_right_over_bounds", g.to_playable_string());
  g.do_user_action(ACTION_ROTATE_LEFT);
  assert_snapshot!("field_after_rotate_left_over_bounds", g.to_playable_string());
  g.do_user_action(ACTION_ROTATE_RIGHT);
  assert_snapshot!("field_after_rotate_right_over_bounds", g.to_playable_string());
  g.do_user_action(ACTION_PLACE_BLOCK);
  assert_snapshot!("field_after_place_block", g.to_playable_string());
  g.do_user_action(ACTION_RIGHT);
  g.do_user_action(ACTION_NEXT_BLOCK);
  assert_snapshot!("field_after_switch_block_to_square", g.to_playable_string());
  g.do_user_action(ACTION_NEXT_BLOCK);
  assert_snapshot!("field_after_switch_block_to_t", g.to_playable_string());
  g.do_user_action(ACTION_NEXT_BLOCK);
  assert_snapshot!("field_after_switch_block_to_s", g.to_playable_string());
  g.do_user_action(ACTION_NEXT_BLOCK);
  assert_snapshot!("field_after_switch_block_to_l", g.to_playable_string());
  g.do_user_action(ACTION_NEXT_BLOCK);
  assert_snapshot!("field_after_switch_block_back_to_i", g.to_playable_string());
}