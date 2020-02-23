use rustyblocks::zobrist::{ZobristHasher,U64Hasher};
use std::collections::HashMap;
use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::render::{Renderable};
#[test]
fn zobrist_test() {
  let hash_builder = U64Hasher::new();
  let mut zobrist_hasher = ZobristHasher::new(201, 20);
  let mut my_map: HashMap<u64, u8, U64Hasher> = HashMap::with_hasher(hash_builder);
  let mut field = GameField::new(2);
  my_map.insert(zobrist_hasher.hash(field.to_field_string_with_player_as_u8().as_slice()), 6);
  field.place_block_using_play(0);
  my_map.insert(zobrist_hasher.hash(field.to_field_string_with_player_as_u8().as_slice()), 3);
  field.place_block_using_play(3);
  my_map.insert(zobrist_hasher.hash(field.to_field_string_with_player_as_u8().as_slice()), 9);
  
  println!("is equal {} {} {}",zobrist_hasher.hash(field.to_field_string_with_player_as_u8().as_slice()) == zobrist_hasher.hash_field(&field)
  ,zobrist_hasher.hash(field.to_field_string_with_player_as_u8().as_slice()), zobrist_hasher.hash_field(&field));
}