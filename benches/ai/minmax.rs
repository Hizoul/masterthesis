extern crate criterion;

use criterion::Criterion;
use rustyblocks::ai::minmax::{MinMaxBot};
use rustyblocks::game_player::{PlaySupervisor};
use rustyblocks::game_logic::field::helper_structs::{GameRules};

pub fn bench_ai_minmax(c: &mut Criterion) {
  c.bench_function("ai_minmax", |b| b.iter(|| {
    let b1 = MinMaxBot::new(0, 2);
    let b2 = MinMaxBot::new(1, 2);
    let mut ps = PlaySupervisor::new_with_rules(vec!(Box::new(b1), Box::new(b2)), GameRules::deterministic());
    ps.play();
    }
  ));
}