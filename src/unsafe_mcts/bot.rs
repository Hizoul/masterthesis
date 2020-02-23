use crate::game_logic::cache::ShapeCache;
use crate::game_logic::field::{GameField};
use crate::game_logic::field::render::Renderable;
use crate::game_logic::field::rl::LearnHelper;
use crate::game_logic::field::helper_structs::{GameRules};
use crate::unsafe_mcts::tree::{TreeBot,Node,ThreadSafeNode};
use crate::ai::Bot;
use rand;
use rand::{Rng,thread_rng};
use std::thread;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

fn timestamp() -> u128 {
  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

fn count_children(root: &ThreadSafeNode) -> u128 {
  let mut count = 1;
  let readable_node = root.lock();
  let mut index = 0;
  for child in readable_node.children.iter() {
    if !Node::child_is_parent(&root, index as usize) {
      count += count_children(child);
    }
    index += 1;
  }
  count
}

const SIMULATION_AMOUNT: u64 = 5;
const THREAD_AMOUNT: u64 = 12;
const SLEEP_DURATION: u64 = 2;

pub struct MCTSBot {
  root: ThreadSafeNode,
  shape_cache: Arc<ShapeCache>,
  pub exploration_constant: f64,
  thought_time: u128,
  pub use_rave: bool,
  pub use_pool_rave: bool,
  pub rave_beta_param: f64, // beta - sim / beta = new node alpha,
  pub play_strategy: u8,
  transposition_table: Arc<RwLock<HashMap<Vec<u8>, ThreadSafeNode>>>
}

impl MCTSBot {
  pub fn new() -> MCTSBot {
    MCTSBot::new_with_time(3000)
  }
  pub fn new_with_time(thought_time : u128) -> MCTSBot {
    let shape_cache = Arc::new(ShapeCache::new());
    let field = GameField::new_with_cache(2, shape_cache.clone(), GameRules::deterministic());
    let bot = MCTSBot {
      thought_time,
      exploration_constant: 2.0_f64.sqrt(),
      shape_cache,
      root: Node::new(None, 0, 0, field.possible_plays.len()),
      use_rave: false,
      use_pool_rave: false,
      rave_beta_param: 5000.0,
      transposition_table: Arc::new(RwLock::new(HashMap::new())), 
      play_strategy: 0
    };
    bot
  }
  fn backpropagation(node: &ThreadSafeNode, player_that_won: u8, backpropagate_rave: bool) {
    let mut parent = None;
    {
      let mut n = node.lock();
      if backpropagate_rave {
        n.rave_visits += 1.0;
        n.rave_wins[player_that_won as usize] += 1;
      } else {
        n.visits += 1.0;
        n.wins[player_that_won as usize] += 1;
      }
      if n.parent.is_some() {
        parent = Some(n.parent.as_ref().unwrap().clone());
      }
    }
    if parent.is_some() {
      MCTSBot::backpropagation(&parent.unwrap(), player_that_won, backpropagate_rave);
    }
  }
  fn simulate(field: &GameField, root: &ThreadSafeNode, player: u8, exploration_constant: f64, use_rave: bool, use_pool_rave: bool, rave_beta_param: f64, transposition_table: Arc<RwLock<HashMap<Vec<u8>, ThreadSafeNode>>>, play_strategy: u8) {
    let mut owned_field = field.clone();
    let current_leaf = MCTSBot::simulate_tree(&mut owned_field, &root, player, exploration_constant, use_rave, rave_beta_param, transposition_table.clone());
    for _ in 0..SIMULATION_AMOUNT {
      let mut default_field = owned_field.clone();
      let player_that_won = MCTSBot::simulate_default(&mut default_field, use_rave, use_pool_rave, current_leaf.clone(), transposition_table.clone(), play_strategy);
      MCTSBot::backpropagation(&current_leaf, player_that_won, false);
    }
  }
  fn simulate_tree(field: &mut GameField, root: &ThreadSafeNode, player: u8, exploration_constant: f64, use_rave: bool, rave_beta_param: f64, transposition_table: Arc<RwLock<HashMap<Vec<u8>, ThreadSafeNode>>>) -> ThreadSafeNode {
    let mut current_leaf = root.clone();
    while !field.game_over {
      let action = MCTSBot::select_move(&current_leaf, &field, player, exploration_constant, use_rave, rave_beta_param);
      field.place_block_using_play(action);
      let is_parent = Node::child_is_parent(&current_leaf, action as usize); // is_parent == node not yet in tree
      if is_parent {
        let depth = field.played_indices.len();
        let rendered_field = field.to_field_string_with_player_as_u8();
        let mut existing_child = Option::None;
        if depth >= 4 {
          let readable_table = transposition_table.read().unwrap();
          let found_child = readable_table.get(&rendered_field);
          if found_child.is_some() {
            existing_child = Option::Some(found_child.unwrap().clone());
          }
        }
        if existing_child.is_some() {
          // println!("FOUND MATCH IN TRANSPOSITION TABLE AND USING IT :)");
          let new_existing_child = existing_child.unwrap();
          Node::set_child(&current_leaf, action as usize, new_existing_child.clone());
          return new_existing_child;
        } else {
          let new_child = Node::create_child(&current_leaf, action, field.possible_plays.len());
          if depth >= 4 {
            let mut write_attempt = transposition_table.try_write();
            while write_attempt.is_err() {
              sleep(Duration::from_nanos(SLEEP_DURATION));
              write_attempt = transposition_table.try_write();
            }
            let mut writeable_table = write_attempt.unwrap();
            writeable_table.insert(rendered_field, new_child.clone());
            // if write_res.is_some() {
            //   println!("CONFLICT IN TRANSPOISTION TABLE DETECTED!");
            // }
          }
          return new_child;
        }
      }
      current_leaf = Node::get_child(&current_leaf, action as usize);
    }
    current_leaf
  }
  fn select_move(node: &ThreadSafeNode, field: &GameField, player: u8, exploration_constant: f64, use_rave: bool, rave_beta_param: f64) -> u8 {
    let mut move_index = 0;
    let player_usize = player as usize;
    let is_maximizing = field.played_indices.len() % 2 == player_usize;
    let mut current = if is_maximizing {std::f64::MIN} else {std::f64::MAX};
    let readable_node = node.lock();
    let mut i = 0;
    for child in readable_node.children.iter() {
      let readable_child = child.lock();
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
      let exploration_component = if use_rave {
        let alpha = if child_rave_visits >= rave_beta_param {0.0} else {(rave_beta_param - child_rave_visits) / rave_beta_param};
        alpha * (child_rave_wins / child_rave_visits) + (readable_node.visits.ln() / child_visits).sqrt()
      } else {
        exploration_constant * (readable_node.visits.ln() / child_visits).sqrt()
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
  fn simulate_default(field: &mut GameField, use_rave: bool, use_pool_rave: bool, start_leaf: ThreadSafeNode, transposition_table: Arc<RwLock<HashMap<Vec<u8>, ThreadSafeNode>>>, play_strategy: u8) -> u8 {
    let mut current_leaf = start_leaf;
    let mut pool_rave_used = false;
    let mut current_choices = Vec::with_capacity(190);
    while !field.game_over && field.possible_plays.len() > 0 {
      let mut action = 0;
      if use_pool_rave  {
        let random_val = thread_rng().gen::<f64>();
        if random_val > 0.5 {
          let mut current_max = 0.0;
          let readable_node = current_leaf.lock();
          let mut i = 0;
          for child in readable_node.children.iter() {
            let readable_child = child.lock();
            if *readable_node != *readable_child {
              if readable_child.rave_visits > 50.0 {
                if readable_child.rave_visits > current_max {
                  current_max = readable_child.rave_visits;
                  current_choices.clear();
                }
                if readable_child.rave_visits == current_max {
                  current_choices.push(i.clone());
                }
              }
            }
            i += 1;
          }
          if current_choices.len() > 0 {
            pool_rave_used = true;
            let random_index = thread_rng().gen_range(0, current_choices.len() - 1);
            action = current_choices[random_index];
            println!("POOL ACTION CHOSEN {} {:?}", action, current_choices);
          }
        }
      }
      if !pool_rave_used {
        action = match play_strategy {
          1 => field.get_best_heuristic_play_for_params(1.0,1.0,1.0,1.0, true, 0),
          2 => field.get_best_heuristic_play_for_params(1.0,1.0,1.0,1.0, true, 1),
          _ => field.get_random_play()
        };
      }
      field.place_block_using_play(action);
      if use_rave && !field.game_over {
        let is_parent = Node::child_is_parent(&current_leaf, action as usize); // is_parent == node not yet in tree
        let depth = field.played_indices.len();
        if is_parent {
          let rendered_field = field.to_field_string_with_player_as_u8();
          let mut existing_child = Option::None;
          if depth >= 4 {
            let readable_table = transposition_table.read().unwrap();
            let found_child = readable_table.get(&rendered_field);
            if found_child.is_some() {
              existing_child = Option::Some(found_child.unwrap().clone());
            }
          }
          if existing_child.is_some() {
            // println!("FOUND MATCH IN TRANSPOSITION TABLE AND USING IT :)");
            let new_existing_child = existing_child.unwrap();
            Node::set_child(&current_leaf, action as usize, new_existing_child.clone());
            current_leaf = new_existing_child;
          } else {
            let new_child = Node::create_child(&current_leaf, action, field.possible_plays.len());
            if depth >= 4 {
              let mut write_attempt = transposition_table.try_write();
              while write_attempt.is_err() {
                sleep(Duration::from_nanos(SLEEP_DURATION));
                write_attempt = transposition_table.try_write();
              }
              let mut writeable_table = write_attempt.unwrap();
              writeable_table.insert(rendered_field, new_child.clone());
              // if write_res.is_some() {
              //   println!("CONFLICT IN TRANSPOISTION TABLE DETECTED!");
              // }
            }
            current_leaf = new_child;
          }
        } else {
          current_leaf = Node::get_child(&current_leaf, action as usize);
        }
      }
    }
    let winner = field.get_winning_player();
    if use_rave {
      MCTSBot::backpropagation(&current_leaf, winner as u8, true);
    }
    winner as u8
  }
  fn mcts_algo(&mut self, field: &GameField, root: &ThreadSafeNode) -> u8 {
    let start_time = timestamp();
    let thought_time = self.thought_time;
    let player = self.get_player();
    let exploration_constant = self.exploration_constant;
    let use_rave = self.use_rave;
    let use_pool_rave = self.use_pool_rave;
    let rave_beta_param = self.rave_beta_param;
    let transtable = &self.transposition_table;
    let play_strategy = self.play_strategy;
    let mut threads = Vec::new();
    for _ in 0..THREAD_AMOUNT {
      let mut f: GameField = field.empty_clone();
      f.restore_from_log(&field.log, false);
      let r = root.clone();
      let table = transtable.clone();
      threads.push(thread::spawn(move || {
        let mut time_left = true;
        while time_left {
          time_left = (timestamp() - start_time) < thought_time;
          MCTSBot::simulate(&f, &r, player, exploration_constant, use_rave, use_pool_rave, rave_beta_param, table.clone(), play_strategy);
        }
        return;
      }));
    }
    for thread in threads.into_iter() {
      thread.join().unwrap();
    }
    let res = self.get_most_visited_child(root);
    let res2 = count_children(root);
    println!("MCTS visited {} nodes thats {} vists/second. Simulation visits are {} in total meaning {} simulation visits/second. In total {} child nodes were discovered thats {} nodes / second", res.1, (res.1 as u128 / ((timestamp() - start_time) / 1000)), res.2, (res.2 as u128 / ((timestamp() - start_time) / 1000)), res2, (res2 as u128 / ((timestamp() - start_time) / 1000)));
    res.0
  }
  fn get_most_visited_child(&self, root: &ThreadSafeNode) -> (u8, f64, f64) {
    let mut max_visits = 0.0;
    let mut max_wins = 0;
    let mut max_index = 0;
    let mut index = 0;
    let mut total_visits = 0.;
    let mut rave_visits = 0.;
    let node = root.lock();
    for child in node.children.iter() {
      if !Node::child_is_parent(&root, index as usize) {
        let readable_child = child.lock();
        total_visits += readable_child.visits;
        let wins = readable_child.wins[self.get_player() as usize];
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
    let decision = self.mcts_algo(&field, &parent);
    println!("GOT MCTS DECISION {}", decision);
    decision as u8
  }
}

impl Bot for MCTSBot {
  fn make_play(&mut self, field: &GameField, _: bool) -> u8 {
    self.make_tree_play(field)
  }
  fn get_name(&self) -> String {"unsafe_mcts".to_owned()}
}