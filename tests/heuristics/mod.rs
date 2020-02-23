use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::render::Renderable;
use rustyblocks::game_logic::log::{GameLog};
use rustyblocks::heuristics::{heuristic_value_width_territory,heuristic_value_height_territory,heuristic_value_all_territory,heuristic_value_enemy_block,heuristic_value_connectability};
use insta::{assert_debug_snapshot,assert_snapshot};
use rustyblocks::game_logic::field::rl::LearnHelper;
use rustyblocks::game_logic::field::helper_structs::{GameRules};

const BLA: &'static str = "{\"log\":[{\"PayloadRolled\":{\"from\":1,\"block\":2}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":0,\"x\":1,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":1,\"x\":2,\"y\":2}},{\"PayloadRolled\":{\"from\":1,\"block\":3}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":0,\"x\":0,\"y\":2}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":2,\"y\":4}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":0,\"x\":5,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":2,\"x\":4,\"y\":0}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadConsidering\":{\"play_index\":0}}]}";

const CONNECTIVITY_TEST: &'static str = "{\"log\":[{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":0,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":8,\"y\":0}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":2,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":3,\"y\":2}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":3,\"y\":6}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadConsidering\":{\"play_index\":0}}]}";

#[test]
fn heuristics_via_replay() {
  let log: GameLog = serde_json::from_str(BLA).unwrap();
  let mut field = GameField::new(2);
  field.restore_from_log(&log, false);
  assert_snapshot!("field_after_restore", &field.to_field_string());
  assert_debug_snapshot!("lowest_ys", field.lowest_ys);
  assert_debug_snapshot!("width", (heuristic_value_width_territory(&field, 0),heuristic_value_width_territory(&field, 1)));
  assert_debug_snapshot!("height", (heuristic_value_height_territory(&field, 0),heuristic_value_height_territory(&field, 1)));
  assert_debug_snapshot!("full_territory", (heuristic_value_all_territory(&field, 0),heuristic_value_all_territory(&field, 1)));
  assert_debug_snapshot!("enemy_block", (heuristic_value_enemy_block(&field, 0),heuristic_value_enemy_block(&field, 1)));


  
  let log2: GameLog = serde_json::from_str(CONNECTIVITY_TEST).unwrap();
  let mut field2 = GameField::new(2);
  field2.restore_from_log(&log2, false);
  assert_snapshot!("field2_after_restore", &field.to_field_string());
  assert_debug_snapshot!("connectability", (heuristic_value_connectability(&field2, 0),heuristic_value_connectability(&field2, 1)));
  assert_debug_snapshot!("width2", (heuristic_value_width_territory(&field2, 0),heuristic_value_width_territory(&field2, 1)));
  assert_debug_snapshot!("height2", (heuristic_value_height_territory(&field2, 0),heuristic_value_height_territory(&field2, 1)));
  assert_debug_snapshot!("full_territory2", (heuristic_value_all_territory(&field2, 0),heuristic_value_all_territory(&field2, 1)));
  assert_debug_snapshot!("enemy_block2", (heuristic_value_enemy_block(&field2, 0),heuristic_value_enemy_block(&field2, 1)));
  assert_debug_snapshot!("rewards", (field2.update_reward(0, false),field2.update_reward(1, false)));
}

#[test]
fn heuristics_battle() {
  let mut field = GameField::new_with_rules(2, GameRules::deterministic());
  while !field.game_over {
    let play = field.get_best_heuristic_play(false);
    field.place_block_using_play(play);
  }
  assert_snapshot!("battle_end", &field.to_field_string());
  assert_debug_snapshot!("battle_scores", field.scores);
}
#[test]
fn random_heuristics_battle() {
  let mut field = GameField::new_with_rules(2, GameRules::deterministic());
  while !field.game_over {
    let play = field.get_random_heuristic_play();
    field.place_block_using_play(play);
  }
  assert_snapshot!("battle_end", &field.to_field_string());
  assert_debug_snapshot!("battle_scores", field.scores);
}