use crate::hex::tournament::{HexBot};
use crate::hex::heuristic::hex_heuristic;
use crate::hex::{HexGame};
use crate::mcts::rewrite::{MctsGame,MCTSBot};
use crate::mcts::tree::{Node,TreeBot};

impl MctsGame for HexGame {
  fn get_depth(&self) -> usize {self.turn_counter}
  fn get_action_amount(&self) -> usize {self.get_possible_plays()}
  fn get_current_player(&self) -> u8 {if self.is_first_players_turn {0} else {1}}
  fn do_action(&mut self, action: usize) {
    self.do_action(self.local_to_global_action_index(action));
  }
  fn get_action(&mut self, _: usize) -> usize {
    // to be changed for hex eval experiment
    self.global_to_local(self.get_random())
    // let answer = hex_heuristic(&self.field, if self.is_first_players_turn {1} else {2});
    // if answer == self.field.len() * 2 {
    //   self.global_to_local(self.get_random())
    // } else {
    //   self.global_to_local(answer)
    // }
    // self.global_to_local(hex_heuristic(&self.field, if self.is_first_players_turn {1} else {2}))
  }
  fn is_game_over(&self) -> bool {self.game_over}
  fn render_field(&self) -> Vec<u8> {self.to_1d_array()}
  fn get_winner(&self) -> i8 {
    if self.winner == 0 {3} else {(self.winner - 1) as i8}}
  fn box_clone(&self) -> Box<dyn MctsGame> {
    Box::new((*self).clone())
  }
}

pub struct HexHeuristicBot {}

impl HexHeuristicBot {
  pub fn new() -> HexHeuristicBot {
    HexHeuristicBot {
    }
  }
}

impl HexBot for HexHeuristicBot {
  fn make_play_hex(&mut self, field: &HexGame, is_second: bool) -> usize {
    let answer = hex_heuristic(&field.field, if is_second {2} else {1});
    if answer == field.field.len() * 2 {
      field.get_random()
    } else {
      answer
    }
  }
  fn get_name_hex(&self) -> String {
    "Heuristic".to_owned()
  }
}
pub struct HexRandomBot {
}

impl HexRandomBot {
  pub fn new() -> HexRandomBot {
    HexRandomBot {
    }
  }
}

impl HexBot for HexRandomBot {
  fn make_play_hex(&mut self, field: &HexGame, _: bool) -> usize {
    field.get_random()
  }
  fn get_name_hex(&self) -> String {
    "Random".to_owned()
  }
}

impl HexBot for MCTSBot {
  fn make_play_hex(&mut self, field: &HexGame, _: bool) -> usize {
    let parent = if field.turn_counter == 0 {
      self.get_root()
    } else {
      // println!("FINDING NEW PARENT ARC {:?}", field.played_indices);
      let mut parent_arc = self.get_root();
      let mut field_findhelper = HexGame::new(field.field.len());
      for i in field.played_indices.iter() {
        let i_usize = *i as usize;
        // println!("doing ACTION {} out of {} {} {}", field_findhelper.local_to_global_action_index(i_usize), field_findhelper.get_possible_plays(), i_usize, field_findhelper);
        field_findhelper.do_action(field_findhelper.local_to_global_action_index(i_usize));
        if !Node::child_is_parent(&parent_arc, i_usize) {
          parent_arc = Node::get_child(&parent_arc, i_usize);
        } else {
          parent_arc = Node::create_child(&parent_arc, i_usize, field_findhelper.get_possible_plays());
        }
      }
      parent_arc
    };
    let to_box = field.clone();
    let mut boxed = Box::new(to_box) as Box<dyn MctsGame>;
    let decision = self.mcts_algo(&mut boxed, &parent) as usize;
    // Node::print(&self.get_root());
    let converted_decision = field.local_to_global_action_index(decision);
    // println!("GOT MCTS DECISION {} converted to {}", decision, converted_decision);
    converted_decision
  }
  fn get_name_hex(&self) -> String {self.name.clone()}
}