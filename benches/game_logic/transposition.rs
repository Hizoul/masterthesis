extern crate criterion;

use criterion::Criterion;
use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::helper_structs::{GameRules};
use rustyblocks::game_logic::field::render::{Renderable};
use rustyblocks::zobrist::{ZobristHasher,U64Hasher};
use std::collections::HashMap;

pub fn bench_transpo(c: &mut Criterion) {
  c.bench_function("hash_u8", move |b| b.iter(|| {
    let mut set = HashMap::new();
    for _ in 0..3 {
      let mut f = GameField::new_with_rules(2, GameRules::deterministic());
      while !f.game_over {
        f.place_block_using_play(0);
        set.insert(f.to_field_string_with_player_as_u8(), 1);
      }
    }
  }));
  c.bench_function("hash_string", move |b| b.iter(|| {
    let mut set = HashMap::new();
    for _ in 0..3 {
      let mut f = GameField::new_with_rules(2, GameRules::deterministic());
      while !f.game_over {
        f.place_block_using_play(0);
        set.insert(f.to_field_string_with_player(true), 1);
      }
    }
  }));
  let mut zobrist_hasher2 = ZobristHasher::new(201, 20);
  c.bench_function("hash_zobrist_unopt", move |b| b.iter(|| {
    let hash_builder = U64Hasher::new();
    let mut set = HashMap::with_hasher(hash_builder);
    for _ in 0..3 {
      let mut f = GameField::new_with_rules(2, GameRules::deterministic());
      while !f.game_over {
        f.place_block_using_play(0);
        set.insert(zobrist_hasher2.hash(&f.to_field_string_with_player_as_u8()), 1);
      }
    }
  }));
  let mut zobrist_hasher3 = ZobristHasher::new(201, 20);
  c.bench_function("hash_zobrist_unopt_nou64hasher", move |b| b.iter(|| {
    let mut set = HashMap::new();
    for _ in 0..3 {
      let mut f = GameField::new_with_rules(2, GameRules::deterministic());
      while !f.game_over {
        f.place_block_using_play(0);
        set.insert(zobrist_hasher3.hash(&f.to_field_string_with_player_as_u8()), 1);
      }
    }
  }));
  let mut zobrist_hasher = ZobristHasher::new(201, 20);
  c.bench_function("hash_zobrist_field", move |b| b.iter(|| {
    let mut set = HashMap::new();
    for _ in 0..3 {
      let mut f = GameField::new_with_rules(2, GameRules::deterministic());
      while !f.game_over {
        f.place_block_using_play(0);
        set.insert(zobrist_hasher.hash_field(&f), 1);
      }
    }
  }));
}