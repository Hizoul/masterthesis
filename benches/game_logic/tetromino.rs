extern crate criterion;

use criterion::Criterion;
use rustyblocks::game_logic::tetromino::{Position,get_block_shape,get_translated_shape};
use rand::prelude::*;

pub fn bench_base_shape(c: &mut Criterion) {
  c.bench_function("block_shape", |b| b.iter(||
    get_block_shape(
      rand::thread_rng().gen_range(0,5),
      rand::thread_rng().gen_range(0,4)
    )));
}
pub fn bench_translated_shape(c: &mut Criterion) {
  c.bench_function("translate_shape", |b| b.iter(|| {
    let mut rng = rand::thread_rng();
    get_translated_shape(
      rng.gen_range(0,5),
      rng.gen_range(0,4),
      &Position::new(rng.gen_range(0,10),rng.gen_range(0,20))
    )
  }));
}