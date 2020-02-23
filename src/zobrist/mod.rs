use std::hash::{Hasher, BuildHasher};
use rand::{thread_rng,RngCore};
use std::sync::Arc;
use crate::game_logic::field::{GameField};
use crate::game_logic::val::{GAMEFIELD_MAX_WIDTH_USIZE,GAMEFIELD_MAX_HEIGHT_USIZE};

#[derive(Debug)]
pub struct ZobristHasher {
  array_length: usize,
  unique_items: usize,
  hash_pieces: Arc<Vec<Vec<u64>>>,
  had_array_start: bool,
  hashed_value: u64
}

impl ZobristHasher {
  pub fn new(array_length: usize, unique_items: usize) -> ZobristHasher {
    let mut hash_pieces = Vec::with_capacity(array_length);
    for _ in 0..array_length {
      let mut index_hash = Vec::with_capacity(unique_items);
      for _ in 0..unique_items {
        index_hash.push(thread_rng().next_u64());
      }
      hash_pieces.push(index_hash);
    }
    ZobristHasher {
      array_length, unique_items,
      hash_pieces: Arc::new(hash_pieces),
      had_array_start: false,
      hashed_value: 0
    }
  }
  pub fn hash_field(&self, field: &GameField) -> u64 {
    let mut hashed_value = 0;
    for p in field.tetrominos.iter() {
      let mut i = 0;
      for part in field.shape_cache.get_block_shape(p).iter() {
        let to_set = if i == 0 {i+=1;((p.from + 1) * 2) - 1} else {((p.from + 1) * 2)};
        let field_index = (((GAMEFIELD_MAX_HEIGHT_USIZE-1) - part.y as usize) * GAMEFIELD_MAX_WIDTH_USIZE) + part.x as usize;
        hashed_value = hashed_value ^ self.hash_pieces[field_index][to_set as usize];
      }
    }
    hashed_value = hashed_value ^ self.hash_pieces[200][(field.current_player + 1) as usize];
    hashed_value
  }
  pub fn hash(&self, data: &[u8]) -> u64 {
    let mut hashed_value = 0;
    for i in 0..self.array_length {
      let mut hash_index = data[i] as usize;
      if i == 200 {
        hash_index += 1;
      } 
      if hash_index != 0 {
        hashed_value = hashed_value ^ self.hash_pieces[i][hash_index];
      }
    }
    hashed_value
  }
}

impl Clone for ZobristHasher {
  fn clone(&self) -> Self {
    ZobristHasher {
      array_length: self.array_length,
      unique_items: self.unique_items,
      hash_pieces: self.hash_pieces.clone(),
      had_array_start: false,
      hashed_value: 0
    }
  }
}

impl Hasher for ZobristHasher {
  fn finish(&self) -> u64 {
    self.hashed_value
  }
  fn write(&mut self, data: &[u8]) {
    println!("RECIEVING DATA {:?} len is {}", data, data.len());
    if self.had_array_start {
      self.hash(data);
    } else {
      self.had_array_start = true;
      if data[0] as usize != self.array_length {
        panic!("Zobrist Hash Function received an array of invalid size! Expected {} but got {}.", self.array_length, data[0]);
      }
    }
  }
}

impl BuildHasher for ZobristHasher {
  type Hasher = ZobristHasher;
  fn build_hasher(&self) -> ZobristHasher {
    self.clone()
  }
}



#[derive(Debug)]
pub struct U64Hasher {
  original_value: u64
}

impl U64Hasher {
  pub fn new() -> U64Hasher {
    U64Hasher {
      original_value: 0
    }
  }
}

impl Clone for U64Hasher {
  fn clone(&self) -> Self {
    U64Hasher {
      original_value: 0
    }
  }
}

impl Hasher for U64Hasher {
  fn finish(&self) -> u64 {
    self.original_value
  }
  fn write(&mut self, data: &[u8]) {
    let mut fixed_size: [u8; 8] = Default::default();
    fixed_size.copy_from_slice(data);
    self.original_value = u64::from_le_bytes(fixed_size);
  }
}

impl BuildHasher for U64Hasher {
  type Hasher = U64Hasher;
  fn build_hasher(&self) -> U64Hasher {
    self.clone()
  }
}