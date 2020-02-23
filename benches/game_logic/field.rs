extern crate criterion;

use criterion::Criterion;
use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::helper_structs::{GameRules};
use rustyblocks::game_logic::log::{GameLog};
use rustyblocks::game_logic::cache::{ShapeCache};
use std::sync::{Arc};
use rand::prelude::*;

const BLA: &'static str = "{\"log\":[{\"PayloadRolled\":{\"from\":1,\"block\":3}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":0,\"x\":8,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":0,\"x\":6,\"y\":0}},{\"PayloadRolled\":{\"from\":1,\"block\":3}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":0,\"x\":4,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":0,\"x\":7,\"y\":2}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":6,\"y\":2}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":7,\"y\":3}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":6,\"y\":6}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":6,\"y\":10}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":2,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":6,\"y\":14}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":1,\"x\":7,\"y\":5}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":5,\"x\":9,\"y\":3}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":3,\"x\":8,\"y\":6}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":10,\"y\":6}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":2,\"y\":2}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":5,\"x\":8,\"y\":7}},{\"PayloadRolled\":{\"from\":1,\"block\":2}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":2,\"x\":4,\"y\":2}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":1,\"x\":7,\"y\":10}},{\"PayloadRolled\":{\"from\":1,\"block\":2}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":0,\"x\":4,\"y\":3}},{\"PayloadRolled\":{\"from\":0,\"block\":0}},{\"PayloadPlaced\":{\"from\":0,\"block\":0,\"orientation\":1,\"x\":2,\"y\":6}},{\"PayloadRolled\":{\"from\":1,\"block\":2}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":3,\"x\":5,\"y\":5}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":9,\"y\":10}},{\"PayloadRolled\":{\"from\":1,\"block\":3}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":3,\"x\":4,\"y\":6}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":3,\"x\":8,\"y\":11}},{\"PayloadRolled\":{\"from\":1,\"block\":2}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":1,\"x\":4,\"y\":9}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":3,\"x\":7,\"y\":12}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":1,\"x\":0,\"y\":0}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":1,\"x\":4,\"y\":12}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":9,\"y\":12}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":9,\"y\":14}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":3,\"x\":0,\"y\":3}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":5,\"x\":0,\"y\":4}},{\"PayloadRolled\":{\"from\":1,\"block\":3}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":1,\"x\":4,\"y\":14}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":7,\"x\":0,\"y\":5}},{\"PayloadRolled\":{\"from\":1,\"block\":4}},{\"PayloadPlaced\":{\"from\":1,\"block\":4,\"orientation\":7,\"x\":1,\"y\":8}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":9,\"y\":16}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":3,\"y\":4}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":3,\"x\":5,\"y\":15}},{\"PayloadRolled\":{\"from\":1,\"block\":0}},{\"PayloadPlaced\":{\"from\":1,\"block\":0,\"orientation\":1,\"x\":3,\"y\":8}},{\"PayloadRolled\":{\"from\":0,\"block\":1}},{\"PayloadPlaced\":{\"from\":0,\"block\":1,\"orientation\":0,\"x\":9,\"y\":18}},{\"PayloadRolled\":{\"from\":1,\"block\":2}},{\"PayloadPlaced\":{\"from\":1,\"block\":2,\"orientation\":1,\"x\":2,\"y\":12}},{\"PayloadRolled\":{\"from\":0,\"block\":3}},{\"PayloadPlaced\":{\"from\":0,\"block\":3,\"orientation\":1,\"x\":2,\"y\":15}},{\"PayloadSkipped\":{\"from\":1,\"block\":4,\"reason\":1}},{\"PayloadRolled\":{\"from\":0,\"block\":4}},{\"PayloadPlaced\":{\"from\":0,\"block\":4,\"orientation\":5,\"x\":7,\"y\":15}},{\"PayloadSkipped\":{\"from\":1,\"block\":0,\"reason\":1}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":2,\"x\":3,\"y\":16}},{\"PayloadRolled\":{\"from\":1,\"block\":3}},{\"PayloadPlaced\":{\"from\":1,\"block\":3,\"orientation\":3,\"x\":0,\"y\":10}},{\"PayloadSkipped\":{\"from\":0,\"block\":3,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":3,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":1,\"reason\":1}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":2,\"y\":17}},{\"PayloadSkipped\":{\"from\":0,\"block\":4,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":4,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":3,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":4,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":4,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":2,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":4,\"reason\":1}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":4,\"y\":17}},{\"PayloadSkipped\":{\"from\":0,\"block\":3,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":3,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":1,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":2,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":1,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":2,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":1,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":3,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":0,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":0,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":4,\"reason\":1}},{\"PayloadRolled\":{\"from\":1,\"block\":1}},{\"PayloadPlaced\":{\"from\":1,\"block\":1,\"orientation\":0,\"x\":2,\"y\":19}},{\"PayloadSkipped\":{\"from\":0,\"block\":1,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":3,\"reason\":1}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadPlaced\":{\"from\":0,\"block\":2,\"orientation\":2,\"x\":7,\"y\":18}},{\"PayloadSkipped\":{\"from\":1,\"block\":3,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":1,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":0,\"reason\":1}},{\"PayloadSkipped\":{\"from\":0,\"block\":0,\"reason\":1}},{\"PayloadSkipped\":{\"from\":1,\"block\":4,\"reason\":1}},{\"PayloadRolled\":{\"from\":0,\"block\":2}},{\"PayloadConsidering\":{\"play_index\":0}}]}";


struct Foo {
  counter: usize
}

impl Foo {
  pub fn add(&mut self) {self.counter+=1;}
  pub fn get(&self) -> usize {self.counter}
}

trait FooTrait {
  fn t_add(&mut self);
  fn t_get(&self) -> usize;
}

impl FooTrait for Foo {
  fn t_add(&mut self) {self.add();}
  fn t_get(&self) -> usize {self.get()}
}

pub fn bench_field(c: &mut Criterion) {
  let cache = ShapeCache::new();
  let rules = GameRules::default();
  let p = Arc::new(cache);
  let p2 = p.clone();
  let p3 = p.clone();
  let log: GameLog = serde_json::from_str(BLA).unwrap();
  c.bench_function("deterministic_play_until_over", move |b| b.iter(|| {
    let mut f = GameField::new_with_cache(2, p.clone(), rules);
    while !f.game_over {
      f.place_block_using_play(0);
    }
  }));
  c.bench_function("random_play_until_over", move |b| b.iter(|| {
    let mut  f = GameField::new_with_cache(2, p2.clone(), rules);
    while !f.game_over {
      let p = f.possible_plays.len() as u8;
      f.place_block_using_play(rand::thread_rng().gen_range(0,p));
    }
  }));
  c.bench_function("replay_log", move |b| b.iter(|| {
    let mut f = GameField::new_with_cache(2, p3.clone(), rules);
    f.restore_from_log(&log, false);
  }));
  c.bench_function("traitcost_direct", move |b| b.iter(|| {
    let mut f = Foo {counter:0};
    for _ in 0..99999999 {
      while f.get() < 9999999999 {
        f.add();
      }
    }
  }));
  c.bench_function("traitcost_struct", move |b| b.iter(|| {
    let mut f: Box<dyn FooTrait> = Box::new(Foo {counter:0});
    for _ in 0..99999999 {
      while f.t_get() < 9999999999 {
        f.t_add();
      }
    }
  }));
}