use rustyblocks::game_logic::field::{GameField};
use rustyblocks::game_logic::field::helper_structs::GameRules;
use rustyblocks::game_logic::field::render::{Renderable};
use insta::{assert_debug_snapshot,assert_json_snapshot,assert_snapshot};
use rustyblocks::rl::aznet::{AlphaZeroNet,get_loss,get_loss_policy, get_value, get_values};
use tch::{Device, nn::{ModuleT,VarStore,sgd,Sgd,OptimizerConfig},Tensor};
use rustyblocks::rl::azmcts::MCTSPlayer;
// use image::{ImageBuffer,Rgb};

// fn save_image(input: Vec<f32>, name: &str) {
//   let mut imgbuf = ImageBuffer::from_fn(32, 32, |x, y| {
//     let i = (((y * 32)*3 + x*3)) as usize;
//     if x == 31 && y == 31 {
//       println!("END IS {}", i);
//     }
//     Rgb([input[i] as u8, input[i+1] as u8, input[i+2] as u8])
//   });
//   imgbuf.save_with_format(name, image::ImageFormat::PNG).unwrap();
// }

#[test]
fn alphazero() {
  let mut g = GameField::new_with_rules(2, GameRules::deterministic());
  let device = Device::Cpu;
  let vs = VarStore::new(device);
  let mut zero = AlphaZeroNet::new(&vs.root());
  let mut opt = Sgd::default().build(&vs, 1e-4).unwrap();
  // let input = Tensor::of_slice(start_state.as_slice()).to_device(device).view([3, 32, 32]);
  // let expected = Tensor::of_slice(&[0.8]);
  // for _ in 0..10 {
  //   let policy = zero.forward_t(&input, false);
  //   let loss = get_loss_policy(&policy, &policy);
  //   opt.backward_step(&loss);
  //   println!("VALUES ARE {:?}", get_values(&policy));
  // }
  let mut mctsplayer = MCTSPlayer::new(zero);
  let m = mctsplayer.get_action(&mut g);
  println!("CHOSEN MOVE Is {}", m);
  while !g.game_over {
    g.place_block_using_play(g.get_random_play());
  }
}

// TODO:
// x - render2image
// possible actions masking
// mctschild idnex to global index
// collect + train on data
// x - mcts select move via network