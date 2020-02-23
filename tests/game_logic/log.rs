use rustyblocks::game_logic::log::{GameLog,Payload};
use insta::{assert_json_snapshot};

#[test]
fn gamelog_test() {
  let mut g: GameLog = GameLog::new();
  assert_json_snapshot!("before anything", &g);
  g.rolled_block(0, 2);
  assert_json_snapshot!("after roll", &g);
  g.block_placed(0, 1, 2, 3, 4);
  assert_json_snapshot!("after roll, place and roll", &g);
  g.rolled_block(1, 5);
  g.add_consideration(0);
  assert_json_snapshot!("created consideration", &g);
  for c in g.log.iter() {
    match &c {
      Payload::PayloadPlaced {from,x,y,block,orientation} => {
        assert_json_snapshot!("polaced", (from,x,y,block,orientation));
      }
      _ => {}
    }
  }
}