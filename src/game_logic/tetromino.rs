use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize,Copy,Clone)]
pub struct Position {pub x: i8, pub y: i8}
impl Position {
  pub fn new(x: i8, y:i8) -> Position {
    return Position {x,y}
  }
}
impl PartialEq for Position {
  fn eq(&self, other: &Position) -> bool {
    self.x == other.x && self.y == other.y
  }
}
pub type BlockShape = Vec<Position>;

pub fn get_block_shape(block: i8, rotation: i8) -> BlockShape {
  match block {
    0 => {
      match rotation {
        0 => return vec!(Position::new(0,0),Position::new(1,0),Position::new(2,0),Position::new(3,0)),
        _ => return vec!(Position::new(0,0),Position::new(0,1),Position::new(0,2),Position::new(0,3))
      }
    }
    1 => {
      return vec!(Position::new(0,0),Position::new(1,0),Position::new(0,1),Position::new(1,1))
    }
    2 => {
      match rotation {
        0 => return vec!(Position::new(0,0),Position::new(-1,0),Position::new(1,0),Position::new(0,1)),
        1 => return vec!(Position::new(0,0),Position::new(0,1),Position::new(1,0),Position::new(0,-1)),
        2 => return vec!(Position::new(0,0),Position::new(1,0),Position::new(-1,0),Position::new(0,-1)),
        _ => return vec!(Position::new(0,0),Position::new(0,1),Position::new(-1,0),Position::new(0,-1))
      }
    }
    3 => {
      match rotation {
        0 => return vec!(Position::new(0,0),Position::new(1,0),Position::new(1,1),Position::new(2,1)),
        1 => return vec!(Position::new(0,0),Position::new(0,-1),Position::new(1,-1),Position::new(1,-2)),
        2 => return vec!(Position::new(0,0),Position::new(-1,0),Position::new(-1,1),Position::new(-2,1)),
        _ => return vec!(Position::new(0,0),Position::new(0,1),Position::new(1,1),Position::new(1,2))
      }
    }
    _ => {
      match rotation {
        0 => return vec!(Position::new(0,0),Position::new(1,0),Position::new(2,0),Position::new(2,1)),
        1 => return vec!(Position::new(0,0),Position::new(1,0),Position::new(0,1),Position::new(0,2)),
        2 => return vec!(Position::new(0,0),Position::new(0,1),Position::new(1,1),Position::new(2,1)),
        3 => return vec!(Position::new(0,0),Position::new(1,0),Position::new(1,-1),Position::new(1,-2)),
        4 => return vec!(Position::new(0,0),Position::new(0,1),Position::new(1,0),Position::new(2,0)),
        5 => return vec!(Position::new(0,0),Position::new(1,0),Position::new(1,1),Position::new(1,2)),
        6 => return vec!(Position::new(0,0),Position::new(1,0),Position::new(2,0),Position::new(2,-1)),
        _ => return vec!(Position::new(0,0),Position::new(0,1),Position::new(0,2),Position::new(1,2))
      }
    }
  }
}

pub fn get_rotation_amount(block: i8) -> i8 {
  match block {
    0 => {
      2
    }
    1 => {
      1
    }
    2 => {
      4
    }
    3 => {
      4
    }
    _ => {
      8
    }
  }
}

pub fn get_translated_shape(block: i8, rotation: i8, position: &Position) -> BlockShape {
  let mut base_shape = get_block_shape(block, rotation);
  for mut p in &mut base_shape {
    p.x += position.x;
    p.y += position.y;
  }
  return base_shape
}

#[derive(Debug,Serialize,Deserialize,Copy,Clone)]
pub struct Tetromino {
  pub position: Position,
  pub block: i8,
  pub rotation: i8,
  pub from: i8
}

pub fn get_translated_shape_from_block(block: &Tetromino) -> BlockShape {
  get_translated_shape(block.block, block.rotation, &block.position)
}


impl PartialEq for Tetromino {
  fn eq(&self, other: &Tetromino) -> bool {
    self.position == other.position && self.rotation == other.rotation && self.block == other.block
  }
}

impl Tetromino {
  pub fn new(block: i8, rotation: i8, from: i8, position: Position) -> Tetromino {
    return Tetromino {
      block: block,
      position: position,
      rotation: rotation,
      from: from
    }
  }
}