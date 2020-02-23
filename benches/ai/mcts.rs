extern crate criterion;

use criterion::Criterion;
use rustyblocks::ai::Bot;
use rustyblocks::mcts::rewrite::{MCTSBot};
use rustyblocks::game_logic::log::{GameLog};
use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::rl::LearnHelper;
use rustyblocks::game_logic::field::helper_structs::{GameRules};
const CONNECTIVITY_TEST: &'static str = "{\"log\":[{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":0,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":8,\"y\":0}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":2,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":3,\"y\":2}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":3,\"y\":6}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadConsidering\":{\"play_index\":0}}]}";

pub fn bench_ai_mcts_basic(c: &mut Criterion) {
  let field = GameField::new_with_rules(2, GameRules::deterministic());
  c.bench_function("ai_basic_mcts", move |b| b.iter(|| {
    let mut b1 = MCTSBot::new(162);
    b1.config.use_rave = false;
    b1.config.use_pool_rave = false;
    b1.config.play_strategy = 0;
    b1.make_play(&field, false);
  }));
  let mut score_field = GameField::new_with_rules(2, GameRules::deterministic());
  let log: GameLog = serde_json::from_str(CONNECTIVITY_TEST).unwrap();
  score_field.restore_from_log(&log, false);
  c.bench_function("simple_heuristic", move |b| b.iter(|| {
    score_field.get_best_heuristic_play(true);
  }));
  c.bench_function("full_game_heuristic", move |b| b.iter(|| {
    let mut test_field = GameField::new_with_rules(2, GameRules::deterministic());
    while !test_field.game_over {
      let play_to_use = test_field.get_best_heuristic_play(true);
      test_field.place_block_using_play(play_to_use);
    }
  }));
}