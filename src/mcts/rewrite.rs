use crate::mcts::tree::TreeBot;
use crate::game_logic::cache::ShapeCache;
use crate::game_logic::field::{GameField};
use crate::game_logic::field::render::Renderable;
use crate::game_logic::field::rl::LearnHelper;
use crate::game_logic::field::helper_structs::{GameRules};
use crate::mcts::tree::{Node,ThreadSafeNode};
use crate::ai::Bot;
use rand;
use rand::{Rng,thread_rng};
use std::thread;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use crate::zobrist::{ZobristHasher};
use std::panic;
use std::panic::UnwindSafe;

pub trait MctsGame: Send + Sync + UnwindSafe {
  fn get_depth(&self) -> usize;
  fn get_action_amount(&self) -> usize;
  fn do_action(&mut self, action_index: usize);
  fn get_action(&mut self, strategy: usize) -> usize;
  fn is_game_over(&self) -> bool;
  fn render_field(&self) -> Vec<u8>;
  fn get_winner(&self) -> i8;
  fn get_current_player(&self) -> u8;
  fn box_clone(&self) -> Box<dyn MctsGame>;
}

impl Clone for Box<dyn MctsGame>
{
    fn clone(&self) -> Box<dyn MctsGame> {
        self.box_clone()
    }
}

impl MctsGame for GameField {
  fn get_depth(&self) -> usize {self.played_indices.len()}
  fn get_action_amount(&self) -> usize {self.possible_plays.len()}
  fn get_current_player(&self) -> u8 {self.current_player as u8}
  fn do_action(&mut self, action: usize) {self.place_block_using_play(action as u8);}
  fn get_action(&mut self, strategy: usize) -> usize {
    let action = match strategy {
      0 => self.get_best_heuristic_play_for_params(4.95238700733393,0.115980839099051,0.0603795891552745,19.0148438477953, true, 0),
      1 => self.get_random_heuristic_play(),
      2 => self.get_best_heuristic_play(true),
      _ => self.get_random_play()
    };
    action as usize
  }
  fn is_game_over(&self) -> bool {self.game_over}
  fn render_field(&self) -> Vec<u8> {self.to_field_string_with_player_as_u8()}
  fn get_winner(&self) -> i8 {self.get_winning_player()}
  fn box_clone(&self) -> Box<dyn MctsGame> {
    Box::new((*self).clone())
  }
}

fn timestamp() -> u128 {
  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

fn count_children(root: &ThreadSafeNode) -> u128 {
  let mut count = 1;
  let readable_node = root.read().unwrap();
  let mut index = 0;
  for child in readable_node.children.iter() {
    if !Node::child_is_parent(&root, index as usize) {
      count += count_children(child);
    }
    index += 1;
  }
  count
}

const SLEEP_DURATION: u64 = 2;

pub struct MCTSBot {
  pub root: ThreadSafeNode,
  shape_cache: Arc<ShapeCache>,
  pub config: MctsConfig,
  pub name: String
}

#[derive(Debug,Clone)]
pub struct MctsConfig {
  pub exploration_constant: f64,
  pub use_rave: bool,
  pub use_pool_rave: bool,
  pub rave_beta_param: f64, // beta - sim / beta = new node alpha,
  pub play_strategy: u8,
  pub transposition_table: Arc<RwLock<HashMap<u64, ThreadSafeNode>>>,
  pub zobrist_hasher: ZobristHasher,
  pub lookahead_limit: usize,
  pub thought_time: u128,
  pub simulation_amount: u64
}

impl MCTSBot {
  pub fn new(action_amount: usize, zobrist_hasher_size: usize) -> MCTSBot {
    MCTSBot::new_with_time(3000, action_amount, "mcts", zobrist_hasher_size)
  }
  pub fn new_with_time(thought_time : u128, action_amount: usize, name: &str, zobrist_hasher_size: usize) -> MCTSBot {
    let shape_cache = Arc::new(ShapeCache::new());
    let config = MctsConfig {
      exploration_constant: 2.0_f64.sqrt(),
      use_rave: false,
      use_pool_rave: false,
      rave_beta_param: 1000.0,
      transposition_table: Arc::new(RwLock::new(HashMap::new())), 
      play_strategy: 5,
      lookahead_limit: 9999,
      thought_time,
      zobrist_hasher: ZobristHasher::new(zobrist_hasher_size, 20),
      simulation_amount: 50
    };
    let bot = MCTSBot {
      shape_cache,
      root: Node::new(None, 0, 0, action_amount),
      config,
      name: name.to_owned()
    };
    bot
  }
  fn backpropagation(node: &ThreadSafeNode, player_that_won: u8, backpropagate_rave: bool) {
    let mut parent = None;
    {
      let mut attempt = node.try_write();
      while attempt.is_err() {
        sleep(Duration::from_nanos(SLEEP_DURATION));
        attempt = node.try_write();
      }
      let mut n = attempt.unwrap();
      if backpropagate_rave {
        n.rave_visits += 1.0;
        if player_that_won == 3 {
          n.rave_wins[0] += 1;
          n.rave_wins[1] += 1;
        } else {
          n.rave_wins[player_that_won as usize] += 1;
        }
      } else {
        n.visits += 1.0;
        if player_that_won == 3 {
          n.wins[0] += 1;
          n.wins[1] += 1;
        } else {
          n.wins[player_that_won as usize] += 1;
        }
      }
      if n.parent.is_some() {
        parent = Some(n.parent.as_ref().unwrap().clone());
      }
    }
    if parent.is_some() {
      MCTSBot::backpropagation(&parent.unwrap(), player_that_won, backpropagate_rave);
    }
  }
  fn simulate(field: &mut Box<dyn MctsGame>, root: &ThreadSafeNode, player: u8, config: &MctsConfig) {
    let mut owned_field = field.box_clone();
    let current_leaf = MCTSBot::simulate_tree(&mut owned_field, &root, player, config);
    for _ in 0..config.simulation_amount {
      let mut default_field = owned_field.box_clone();
      let player_that_won = MCTSBot::simulate_default(&mut default_field, current_leaf.clone(), config);
      MCTSBot::backpropagation(&current_leaf, player_that_won, false);
    }
  }
  fn simulate_tree(field: &mut Box<dyn MctsGame>, root: &ThreadSafeNode, player: u8, config: &MctsConfig) -> ThreadSafeNode {
    let mut current_leaf = root.clone();
    let start_depth = field.get_depth();
    while !field.is_game_over() && field.get_depth() - start_depth < config.lookahead_limit {
      let action = MCTSBot::select_move(&current_leaf, field, player, config);
      field.do_action(action as usize);
      let is_parent = Node::child_is_parent(&current_leaf, action as usize); // is_parent == node not yet in tree
      if is_parent {
        let depth = field.get_depth();
        let rendered_field = field.render_field();
        let zobrist_hash = config.zobrist_hasher.hash(rendered_field.as_ref());
        let mut existing_child = Option::None;
        let mut write_attempt = config.transposition_table.try_write();
        while write_attempt.is_err() {
          sleep(Duration::from_nanos(SLEEP_DURATION));
          write_attempt = config.transposition_table.try_write();
        }
        let mut writeable_table = write_attempt.unwrap();
        let mut insert_into_table = true;
        if depth >= 4 && depth % 2 == 0 {
          let found_child = writeable_table.get(&zobrist_hash);
          if found_child.is_some() {
              let child = found_child.unwrap().clone(); 
              let mut attempt = child.try_read();
              while attempt.is_err() {
                sleep(Duration::from_nanos(SLEEP_DURATION));
                attempt = child.try_read();
              }
              let readable_child = attempt.unwrap();
              if readable_child.depth == depth as u8 && readable_child.children.len() == field.get_action_amount() {
                existing_child = Option::Some(child.clone());
              } else {
                insert_into_table = false;
              }
          }
        }
        if existing_child.is_some() {
          // println!("FOUND MATCH IN TRANSPOSITION TABLE AND USING IT :)");
          let new_existing_child = existing_child.unwrap();
          Node::set_child(&current_leaf, action as usize, new_existing_child.clone());
          return new_existing_child;
        } else {
          let new_child = Node::create_child(&current_leaf, action, field.get_action_amount());
          if insert_into_table && depth >= 4 && depth % 2 == 0 {
            let write_res = writeable_table.insert(zobrist_hash, new_child.clone());
            if write_res.is_some() {
              println!("CONFLICT IN TRANSPOISTION TABLE DETECTED!");
            }
          }
          return new_child;
        }
      }
      current_leaf = Node::get_child(&current_leaf, action as usize);
    }
    current_leaf
  }
  fn select_move(node: &ThreadSafeNode, field: &mut Box<dyn MctsGame>, player: u8, config: &MctsConfig) -> usize {
    let mut move_index = 0;
    let player_usize = player as usize;
    let is_maximizing = field.get_depth() % 2 == player_usize;
    let mut current = if is_maximizing {std::f64::MIN} else {std::f64::MAX};
    
    let mut attempt = node.try_read();
    while attempt.is_err() {
      sleep(Duration::from_nanos(SLEEP_DURATION));
      attempt = node.try_read();
    }
    let readable_node = attempt.unwrap();
    let mut i = 0;
    for child in readable_node.children.iter() {
      let mut child_attempt = child.try_read();
      while child_attempt.is_err() {
        sleep(Duration::from_nanos(SLEEP_DURATION));
        child_attempt = child.try_read();
      }
      let readable_child = child_attempt.unwrap();
      let child_visits;
      let child_rave_visits;
      let child_wins;
      let child_rave_wins;
      if *readable_node != *readable_child {
        child_visits = readable_child.visits;
        child_wins = readable_child.wins[player_usize] as f64;
        child_rave_visits = readable_child.rave_visits;
        child_rave_wins = readable_child.rave_wins[player_usize] as f64;
      } else {
        child_visits = 1.0;
        child_wins = 0.0;
        child_rave_visits = 1.0;
        child_rave_wins = 0.0;
      }
      let exploration_component = if config.use_rave {
        let alpha = if child_rave_visits >= config.rave_beta_param {0.0} else {(config.rave_beta_param - child_rave_visits) / config.rave_beta_param};
        alpha * (child_rave_wins / child_rave_visits) + (readable_node.visits.ln() / child_visits).sqrt()
      } else {
        config.exploration_constant * (readable_node.visits.ln() / child_visits).sqrt()
      };
      let exploitation_component =  child_wins / child_visits;
      let new_val = if is_maximizing {exploitation_component+exploration_component} else {exploitation_component-exploration_component};
      if is_maximizing && new_val > current || !is_maximizing && new_val < current {
        current = new_val;
        move_index = i;
      }
      i += 1;
    }
    move_index
  }
  fn simulate_default(field: &mut Box<dyn MctsGame>, start_leaf: ThreadSafeNode, config: &MctsConfig) -> u8 {
    let mut current_leaf = start_leaf;
    let mut pool_rave_used = false;
    let mut current_choices = Vec::with_capacity(190);
    let start_depth = field.get_depth();
    while !field.is_game_over() && field.get_action_amount() > 0 && field.get_depth() - start_depth < config.lookahead_limit {
      let mut action = 0;
      if config.use_pool_rave  {
        let random_val = thread_rng().gen::<f64>();
        if random_val > 0.5 {
          let mut current_max = 0.0;
          let mut attempt = current_leaf.try_read();
          while attempt.is_err() {
            sleep(Duration::from_nanos(SLEEP_DURATION));
            attempt = current_leaf.try_read();
          }
          let readable_node = attempt.unwrap();
          let mut i = 0;
          for child in readable_node.children.iter() {
            let mut child_attempt = child.try_read();
            while child_attempt.is_err() {
              sleep(Duration::from_nanos(SLEEP_DURATION));
              child_attempt = child.try_read();
            }
            let readable_child = child_attempt.unwrap();
            if *readable_node != *readable_child {
              if readable_child.rave_visits > 50.0 {
                if readable_child.rave_visits > current_max {
                  current_max = readable_child.rave_visits;
                  current_choices.clear();
                }
                if readable_child.rave_visits == current_max {
                  current_choices.push(i);
                }
              }
            }
            i += 1;
          }
          if current_choices.len() > 1 {
            pool_rave_used = true;
            let random_index = thread_rng().gen_range(0, current_choices.len() - 1);
            action = current_choices[random_index];
            current_choices.clear();
          } else if current_choices.len() == 1 {
            pool_rave_used = true;
            action = current_choices[0];
            current_choices.clear();
          }
        }
      }
      if !pool_rave_used {
        action = field.get_action(config.play_strategy as usize);
      }
      field.do_action(action);
      if config.use_rave && !field.is_game_over() {
        let is_parent = Node::child_is_parent(&current_leaf, action as usize); // is_parent == node not yet in tree
        let depth = field.get_depth();
        if is_parent {
          let rendered_field = field.render_field();
          let zobrist_hash = config.zobrist_hasher.hash(rendered_field.as_ref());
          let mut existing_child = Option::None;

          let mut write_attempt = config.transposition_table.try_write();
          while write_attempt.is_err() {
            sleep(Duration::from_nanos(SLEEP_DURATION));
            write_attempt = config.transposition_table.try_write();
          }
          let mut writeable_table = write_attempt.unwrap();
          let mut insert_into_table = true;
          if depth >= 4 && depth % 2 == 0 {
            let found_child = writeable_table.get(&zobrist_hash);
            if found_child.is_some() {
              let child = found_child.unwrap().clone(); 
              let mut attempt = child.try_read();
              while attempt.is_err() {
                sleep(Duration::from_nanos(SLEEP_DURATION));
                attempt = child.try_read();
              }
              let readable_child = attempt.unwrap();
              if readable_child.depth == depth as u8 && readable_child.children.len() == field.get_action_amount() {
                existing_child = Option::Some(child.clone());
              } else {
                insert_into_table = false;
              }
            }
          }
          if existing_child.is_some() {
            // println!("FOUND MATCH IN TRANSPOSITION TABLE AND USING IT :)");
            let new_existing_child = existing_child.unwrap();
            Node::set_child(&current_leaf, action as usize, new_existing_child.clone());
            current_leaf = new_existing_child;
          } else {
            let new_child = Node::create_child(&current_leaf, action, field.get_action_amount());
            if insert_into_table && depth >= 4 && depth % 2 == 0 {
              let write_res = writeable_table.insert(zobrist_hash, new_child.clone());
              if write_res.is_some() {
                println!("CONFLICT IN TRANSPOISTION TABLE DETECTED!");
              }
            }
            current_leaf = new_child;
          }
        } else {
          current_leaf = Node::get_child(&current_leaf, action as usize);
        }
      }
    }
    let winner = field.get_winner();
    if config.use_rave {
      MCTSBot::backpropagation(&current_leaf, winner as u8, true);
    }
    winner as u8
  }
  pub fn mcts_algo(&mut self, field: &mut Box<dyn MctsGame>, root: &ThreadSafeNode) -> u8 {
    let start_time = timestamp();
    let thought_time = self.config.thought_time;
    let player = field.get_current_player();
    let mut threads = Vec::new();
    for _ in 0..num_cpus::get() {
      let mut f = field.box_clone();
      let r = root.clone();
      let config = self.config.clone();
      threads.push(thread::spawn(move || {
        let mut time_left = true;
        while time_left {
          let subf = f.box_clone();
          let panic_res = panic::catch_unwind(|| {
            let mut boxed = subf;
            MCTSBot::simulate(&mut boxed, &r, player, &config);
          });
          time_left = (timestamp() - start_time) < thought_time;
          if panic_res.is_err() {
            println!("caught error in thread");
          }
        }
        return;
      }));
    }
    for thread in threads.into_iter() {
      thread.join().unwrap();
    }

    // single thread
    // let mut f: GameField = field.empty_clone();
    // f.copy_from(&field);
    // let r = root.clone();
    // let config = self.config.clone();
    // let subf = f.clone();
    // let mut time_left = true;
    // while time_left {
    //   let subsubf = subf.clone();
    //   time_left = (timestamp() - start_time) < thought_time;
    //   let mut boxed = Box::new(subsubf) as Box<dyn MctsGame>;
    //   MCTSBot::simulate(&mut boxed, &r, player, &config);
    // }
    let res = self.get_most_visited_child(root, player);
    // let res2 = count_children(root);
    // let time_spent =  timestamp() - start_time;
    // let elapsed_seconds = time_spent as f64 / 1000.0;
    // if elapsed_seconds > 1.0 {
    //   println!("MCTS visited {} nodes thats {} vists/second. Simulation visits are {} in total meaning {} simulation visits/second. In total {} child nodes were discovered thats {} nodes / second", res.1, (res.1 as f64 / elapsed_seconds), res.2, (res.2 as f64 / elapsed_seconds), res2, (res2 as f64 / elapsed_seconds));
    // } else {
    //   println!("MCTS visited {} nodes thats {} vists/second. Simulation visits are {} in total meaning {} simulation visits/second. In total {} child nodes were discovered thats {} nodes / second", res.1, (res.1 as f64 * elapsed_seconds), res.2, (res.2 as f64 * elapsed_seconds), res2, (res2 as f64 * elapsed_seconds));
    // }
    res.0
  }
  fn get_most_visited_child(&self, root: &ThreadSafeNode, player: u8) -> (u8, f64, f64) {
    let mut max_visits = 0.0;
    let mut max_wins = 0;
    let mut max_index = 0;
    let mut index = 0;
    let mut total_visits = 0.;
    let mut rave_visits = 0.;
    let node = root.read().unwrap();
    for child in node.children.iter() {
      if !Node::child_is_parent(&root, index as usize) {
        let readable_child = child.read().unwrap();
        total_visits += readable_child.visits;
        let wins = readable_child.wins[player as usize];
        // println!("comparing prev {} {} new {} {}", max_visits, max_wins, readable_child.visits, wins);
        if readable_child.visits >= max_visits && wins > max_wins {
          max_index = index;
          max_wins = wins;
          max_visits = readable_child.visits;
        }
        rave_visits += readable_child.rave_visits;
      }
      index += 1;
    }
    (max_index, total_visits, rave_visits)
  }
  pub fn prefill_rave(&self, amount_of_matches: u64) {
    let field = GameField::new_with_rules(2, self.get_rules());
    let current_leaf = self.root.clone();
    let config = self.config.clone();
    for _ in 0..amount_of_matches {
      let mut default_field = field.empty_clone();
      default_field.copy_from(&field);
      let mut boxed = Box::new(default_field) as Box<dyn MctsGame>;
      MCTSBot::simulate_default(&mut boxed, current_leaf.clone(), &config);
    }
  }
}

impl TreeBot for MCTSBot {
  fn get_rules(&self) -> GameRules {GameRules::deterministic()}
  fn get_shape_cache(&self) -> Arc<ShapeCache> {self.shape_cache.clone()}
  fn get_player(&self) -> u8 {0}
  fn get_score_type(&self) -> u8 {0}
  fn get_root(&mut self) -> ThreadSafeNode {
    self.root.clone()
  }
  fn get_score(&self, _field: &GameField) -> i16 {
    0
  }
  fn make_tree_play(&mut self, field: &GameField) -> u8 {
    let parent = if field.played_indices.len() == 0 {
      self.get_root()
    } else {
      self.find_node(&field)
    };
    let mut to_box = field.empty_clone();
    to_box.copy_from(&field);
    let mut boxed = Box::new(to_box) as Box<dyn MctsGame>;
    let decision = self.mcts_algo(&mut boxed, &parent);
    // println!("GOT MCTS DECISION {}", decision);
    decision as u8
  }
}

impl Bot for MCTSBot {
  fn make_play(&mut self, field: &GameField, _: bool) -> u8 {
    self.make_tree_play(field)
  }
  fn get_name(&self) -> String {self.name.clone()}
}