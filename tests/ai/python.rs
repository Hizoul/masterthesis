use rustyblocks::ai::python::{PythonBot};
use rustyblocks::game_logic::field::{GameField};
use rustyblocks::ai::Bot;
use rustyblocks::game_logic::field::helper_structs::{GameRules};

#[test]
fn ai_python() {
  let mut b1 = PythonBot::new(0, "bla");
  let field = GameField::new_with_rules(2, GameRules::deterministic());
  let answer = b1.make_play(&field, false);
  assert!(answer != 0);
}