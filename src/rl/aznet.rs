use tch::{nn,Kind, nn::{ModuleT,Module,Linear,linear,ConvConfig,Conv2D,conv2d,BatchNorm,batch_norm2d}, Kind::*, Tensor};
use crate::game_logic::val::{GAMEFIELD_MAX_WIDTH,GAMEFIELD_MAX_HEIGHT};

#[derive(Debug)]
pub struct ConvBlock {
  conv1: Conv2D,
  bn1: BatchNorm
}

impl ConvBlock {
  pub fn new (vs: &nn::Path) -> ConvBlock {
    let conv_config = ConvConfig {
      stride: 1,
      padding: 1,
      ..Default::default()
    };
    let conv1 = conv2d(vs, 3, 128, 3, conv_config);
    let bn1 = batch_norm2d(vs, 128, Default::default());
    ConvBlock {
      conv1,
      bn1
    }
  }
}

impl ModuleT for ConvBlock {
  fn forward_t(&self, xs: &Tensor, train: bool) -> Tensor {
    let mut to_return = xs.view([-1, 3, 32, 32]);
    to_return = self.bn1.forward_t(&self.conv1.forward_t(&to_return, train), train);
    to_return.relu()
  }
}


#[derive(Debug)]
struct ResBlock {
  conv1: Conv2D,
  conv2: Conv2D,
  bn1: BatchNorm,
  bn2: BatchNorm
}

impl ResBlock {
  fn new (vs: &nn::Path) -> ResBlock {
    let conv_config = ConvConfig {
      stride: 1,
      padding: 1,
      bias: false,
      ..Default::default()
    };
    let conv1 = conv2d(vs, 128, 128, 3, conv_config);
    let bn1 = batch_norm2d(vs, 128, Default::default());
    let conv2 = conv2d(vs, 128, 128, 3, conv_config);
    let bn2 = batch_norm2d(vs, 128, Default::default());
    ResBlock {
      conv1,
      bn1,
      conv2,
      bn2
    }
  }
}

impl ModuleT for ResBlock {
  fn forward_t(&self, xs: &Tensor, train: bool) -> Tensor {
    let residual = xs;
    let mut to_return = self.bn1.forward_t(&self.conv1.forward_t(&residual, train), train).relu();
    to_return = self.bn2.forward_t(&self.conv2.forward_t(&to_return, train), train);
    to_return += residual;
    to_return.relu()
  }
}



#[derive(Debug)]
struct OutBlock {
  conv1: Conv2D,
  conv: Conv2D,
  bn1: BatchNorm,
  bn: BatchNorm,
  fc: Linear,
  fc1: Linear,
  fc2: Linear
}

impl OutBlock {
  fn new (vs: &nn::Path) -> OutBlock {
    let conv_config = ConvConfig {
      stride: 1,
      padding: 0,
      bias: false,
      ..Default::default()
    };
    let conv = conv2d(vs, 128, 3, 1, Default::default());
    let bn = batch_norm2d(vs, 3, Default::default());

    let fc1 = linear(vs, 3*32*32, 32, Default::default());
    let fc2 = linear(vs, 32, 1, Default::default());

    let conv1 = conv2d(vs, 128, 32, 1, conv_config);
    let bn1 = batch_norm2d(vs, 32, Default::default());

    let fc = linear(vs, 32*32*32, 190, Default::default());
    OutBlock {
      conv,
      bn,
      conv1,
      bn1,
      fc,
      fc1,
      fc2
    }
  }
}

impl OutBlock {
  fn custom_forward(&self, xs: &Tensor, train: bool) -> (Tensor,Tensor) {
    let mut value_head = self.bn.forward_t(&self.conv.forward_t(&xs, train), train).relu();
    value_head = value_head.view([-1, 3*32*32]);
    value_head = self.fc1.forward_t(&value_head, train).relu();
    value_head = self.fc2.forward_t(&value_head, train).tanh();
    let mut policy_head = self.bn1.forward_t(&self.conv1.forward_t(&xs, train), train).relu();
    policy_head = policy_head.view([-1, 32*32*32]);
    policy_head = self.fc.forward_t(&policy_head, train);
    policy_head = policy_head.log_softmax(1,Kind::Double).exp();
    (value_head,policy_head)
  }
}

impl ModuleT for OutBlock {
  fn forward_t(&self, xs: &Tensor, train: bool) -> Tensor {
    let mut value_head = self.bn.forward_t(&self.conv.forward_t(&xs, train), train).relu();
    value_head = value_head.view([-1, 3*32*32]);
    value_head = self.fc1.forward_t(&value_head, train).relu();
    value_head = self.fc2.forward_t(&value_head, train).tanh();
    let mut policy_head = self.bn1.forward_t(&self.conv1.forward_t(&xs, train), train).relu();
    policy_head = policy_head.view([-1, 32*32*32]);
    policy_head = self.fc.forward_t(&policy_head, train);
    policy_head = policy_head.log_softmax(1,Kind::Double).exp();
    policy_head
  }
}


#[derive(Debug)]
pub struct AlphaZeroNet {
  conv: ConvBlock,
  res_1: ResBlock, // todo add 19
  out: OutBlock
}

impl AlphaZeroNet {
  pub fn new (vs: &nn::Path) -> AlphaZeroNet {
    let conv = ConvBlock::new(vs);
    let res_1 = ResBlock::new(vs);
    let out = OutBlock::new(vs);
    AlphaZeroNet {
      conv,
      res_1,
      out
    }
  }
}

impl AlphaZeroNet {
  pub fn custom_forward(&self, xs: &Tensor, train: bool) -> (Tensor, Tensor) {
    let mut to_return = self.conv.forward_t(&xs, train);
    to_return = self.res_1.forward_t(&to_return, train);
    self.out.custom_forward(&to_return, train)
    // let bla = Tensor::of_slice(&[1,2,3]);
    // (to_return, bla)
  }
}

impl ModuleT for AlphaZeroNet {
  fn forward_t(&self, xs: &Tensor, train: bool) -> Tensor {
    let mut to_return = self.conv.forward_t(&xs, train);
    to_return = self.res_1.forward_t(&to_return, train);
    self.out.forward_t(&to_return, train)
  }
}

pub fn get_loss(expected_value: &Tensor, delievred_value: &Tensor, expected_policy: &Tensor, delivered_policy: &Tensor) -> Tensor {
  let value_error = (delievred_value - expected_value).pow(2);
  let policy_error = (-delivered_policy * (1e-8+expected_policy).log()) + 1;
  let total_error = (value_error.view(-1) + policy_error).mean(Kind::Float);
  total_error
}

pub fn get_loss_policy(expected_policy: &Tensor, delivered_policy: &Tensor) -> Tensor {
  let policy_error = (-delivered_policy * (1e-8+expected_policy).log()) + 1;
  let total_error = policy_error.mean(Kind::Float);
  total_error
}

pub fn get_values(tensor: &Tensor) -> Vec<f64> {
  let siz = tensor.size();
  let max = siz[1];
  let mut vals = Vec::with_capacity(max as usize);
  for i in 0..max {
    vals.push(tensor.double_value(&[0, i]));
  }
  vals
}
pub fn get_values_0d(tensor: &Tensor) -> Vec<f64> {
  let siz = tensor.size();
  let max = siz[0];
  let mut vals = Vec::with_capacity(max as usize);
  for i in 0..max {
    vals.push(tensor.double_value(&[i]));
  }
  vals
}

pub fn get_value(tensor:&Tensor) -> f64 {
  tensor.double_value(&[0])
}
