use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize,Clone,Copy)]
pub struct Play {
    pub x: i8,
    pub rotation: i8,
    pub block: i8,
    pub index: u8,
    pub global_index: u8
}

#[derive(Debug,Serialize,Deserialize,Copy,Clone)]
pub enum Payload {
  PayloadRolled {
    from: i8,
    block: i8
  },
  PayloadPlaced {
    x: i8,
    y: i8,
    block: i8,
    orientation: i8,
    from: i8
  },
  PayloadSkipped {
    from: i8,
    block: i8,
    reason: i8
  },
  PayloadConsidering {
    play_index: u8
  }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct GameLog {
    pub log: Vec<Payload>
}

impl GameLog {
  pub fn new() -> GameLog {
    return GameLog {
      log: Vec::with_capacity(100)
    }
  }
  pub fn add_item(&mut self, item: Payload) {
    self.log.push(item)
  }
  pub fn block_placed(&mut self, user: i8, block_type: i8, block_orientation: i8, x: i8, y: i8) {
    self.add_item(Payload::PayloadPlaced {
      from: user,
      block: block_type,
      orientation: block_orientation,
      x: x,
      y: y
    })
  }
  pub fn rolled_block(&mut self, user: i8, block_type: i8) {
    self.add_item(Payload::PayloadRolled {
      from: user,
      block: block_type
    })
  }
  pub fn player_skipped(&mut self, user: i8, block_type: i8, reason: i8) {
    self.add_item(Payload::PayloadSkipped {
      from: user,
      block: block_type,
      reason: reason
    })
  }
  pub fn add_consideration(&mut self, play_index: u8) {
    self.add_item(Payload::PayloadConsidering{
      play_index
    })
  }
  pub fn get_current_roll(&self) -> (i8, i8) {
    for e in self.log.iter().rev() {
      match e {
        Payload::PayloadRolled {block,from} => {
          return (*from, *block)
        }
        _ => {}
      }
    }
    return (0, 0)
  }
  pub fn copy_from(&mut self, other: &GameLog) {
    self.log.clear();
    for entry in other.log.iter() {
      match entry {
        Payload::PayloadConsidering {play_index} => {
          self.add_item(Payload::PayloadConsidering{
            play_index: *play_index
          })
        }
        Payload::PayloadPlaced {block,from,x,y,orientation} => {
          self.block_placed(*from, *block, *orientation, *x, *y);
        }
        Payload::PayloadSkipped {block,from, reason} => {
          self.player_skipped(*from, *block, *reason);
        }
        Payload::PayloadRolled {block,from} => {
          self.rolled_block(*from, *block);
        }
      }
    }
  }
}