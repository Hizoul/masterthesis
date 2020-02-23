#[macro_use]
extern crate criterion;
mod game_logic;
mod ai;

criterion_group!(benches,
  game_logic::tetromino::bench_base_shape, game_logic::tetromino::bench_translated_shape,
  game_logic::field::bench_field, ai::minmax::bench_ai_minmax, ai::mcts::bench_ai_mcts_basic,
  game_logic::transposition::bench_transpo, ai::unsafe_mcts::bench_ai_unsafe_mcts_basic);
criterion_main!(benches);