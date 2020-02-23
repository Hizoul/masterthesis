extern crate criterion;

use criterion::Criterion;
use rustyblocks::ai::Bot;
use rustyblocks::unsafe_mcts::bot::{MCTSBot};
use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::helper_structs::{GameRules};

pub fn bench_ai_unsafe_mcts_basic(c: &mut Criterion) {
  let field = GameField::new_with_rules(2, GameRules::deterministic());
  c.bench_function("ai_basic_unsafe_mcts", move |b| b.iter(|| {
    let mut b1 = MCTSBot::new();
    b1.use_rave = false;
    b1.use_pool_rave = false;
    b1.play_strategy = 0;
    b1.make_play(&field, false);
  }));
}