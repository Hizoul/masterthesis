use super::val::{GAMEFIELD_MAX_HEIGHT,GAMEFIELD_MAX_WIDTH,AMOUNT_OF_UNIQUE_PIECES_USIZE,AMOUNT_OF_UNIQUE_ORIENTATIONS_USIZE,GAMEFIELD_MAX_WIDTH_USIZE, GAMEFIELD_MAX_HEIGHT_USIZE,
AMOUNT_OF_UNIQUE_PIECES,AMOUNT_ROTATIONS};
use super::tetromino::{Tetromino,Position,BlockShape,get_translated_shape,get_block_shape};

#[derive(Debug,Clone)]
pub struct ShapeCache {
  base: Vec<Vec<BlockShape>>,
  translated: Vec<Vec<Vec<Vec<BlockShape>>>>
}

impl ShapeCache {
  pub fn new() -> ShapeCache {
    let mut base = Vec::with_capacity(AMOUNT_OF_UNIQUE_PIECES_USIZE);
    let mut translated = Vec::with_capacity(AMOUNT_OF_UNIQUE_PIECES_USIZE);
    for block in 0..AMOUNT_OF_UNIQUE_PIECES {
      let mut rotations = Vec::with_capacity(AMOUNT_OF_UNIQUE_ORIENTATIONS_USIZE);
      let mut translated_rotations = Vec::with_capacity(AMOUNT_OF_UNIQUE_ORIENTATIONS_USIZE);
      for rotation in 0..AMOUNT_ROTATIONS[block as usize] {
        rotations.push(get_block_shape(block, rotation));
        let mut xs = Vec::with_capacity(GAMEFIELD_MAX_WIDTH_USIZE+1);
        for x in 0..(GAMEFIELD_MAX_WIDTH) {
          let mut ys = Vec::with_capacity(GAMEFIELD_MAX_HEIGHT_USIZE+1);
          for y in 0..(GAMEFIELD_MAX_HEIGHT) {
            ys.push(get_translated_shape(block, rotation, &Position {x,y}));
          }
          xs.push(ys);
        }
        translated_rotations.push(xs)
      }
      base.push(rotations);
      translated.push(translated_rotations);
    }
    return ShapeCache {
      base, translated
    }
  }
  pub fn get_translated(&self, block: i8, rotation:i8, x: i8, y: i8) -> &BlockShape {
    self.translated.get(block as usize).expect("block available").get(rotation as usize).expect("rotation available").get(x as usize).expect("x available").get(y as usize).expect("y available")
  }
  pub fn get_base(&self, block: i8, rotation:i8) -> &BlockShape {
    self.base.get(block as usize).unwrap().get(rotation as usize).unwrap()
  }
  pub fn get_block_shape(&self, block: &Tetromino) -> &BlockShape {
    self.get_translated(block.block, block.rotation, block.position.x, block.position.y)
  }
}
