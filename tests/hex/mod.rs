use rustyblocks::hex::{HexGame,get_neighbors};
use rustyblocks::hex::heuristic::hex_heuristic;
use rustyblocks::hex::hex_eval::{do_hex_eval,do_hex_regular_tournament};
use insta::{assert_snapshot};

#[test]
fn hex_game() {
  let mut game = HexGame::new(5);
  let mut game2 = HexGame::new(5);
  assert_snapshot!("initial_field", &game.to_string());
  for i in 0..5 {
    if i == 0 {
      game.place_point(0,i);
      game.place_point(2,2);
    } else {
      game.place_point(0,i);
      game.place_point(i,0);
    }
    assert_eq!(game.game_over, i == 4);
    assert_eq!(game.winner, if i == 4 {1} else {0});
    if i == 0 {
      game2.place_point(2,2);
      game2.place_point(i,0);
    } else {
      game2.place_point(0,i);
      game2.place_point(i,0);
    }
    assert_eq!(game2.game_over, i == 4);
    assert_eq!(game2.winner, if i == 4 {2} else {0});
  }
  assert_snapshot!("end", &game.to_string());
}

#[test]
fn hex_game_pie_rule() {
  // let mut game = HexGame::new(5);
  // assert_eq!(game.field[2][2], 0);
  // game.place_point(2,2);
  // assert_eq!(game.field[2][2], 1);
  // game.place_point(2,2);
  // assert_eq!(game.field[2][2], 2);
  // assert_eq!(game.place_point(2,2), false);
}

#[test]
fn hex_game_heuristic() {
  let mut game = HexGame::new(4);
  while !game.game_over {
    let action = hex_heuristic(&game.field, if game.is_first_players_turn {1}else{2});
    game.do_action(action);
  }
  assert_ne!(game.winner, 0);
  let mut wins_against_random = HexGame::new(11);
  while !wins_against_random.game_over {
    if wins_against_random.is_first_players_turn {
      let action = hex_heuristic(&wins_against_random.field, 1);
      wins_against_random.do_action(action);
    } else {
      wins_against_random.random_place();
    }
  }
  assert_eq!(wins_against_random.winner, 1);
}

#[test]
fn hex_neighbors_sanity_check() {
  let field_size: usize = 4;
  for x in 0..field_size {
    for y in 0..field_size {
      let neighbors = get_neighbors(field_size, x, y);
      for possible_neighbor in neighbors.iter() {
        if possible_neighbor.is_some() {
          let neighbor = possible_neighbor.unwrap();
          let neighbors_neighbors = get_neighbors(field_size, neighbor.0, neighbor.1);
          assert_eq!(neighbors_neighbors.iter().position(|pos| {
            if pos.is_some() {
              let unwrapped_pos = pos.unwrap();
              return unwrapped_pos.0 == x && unwrapped_pos.1 == y;
            }
            return false;
          }).is_some(), true);
        }
      }
    }
  }
}

#[test]
fn hex_game_do_action() {
  // do_hex_eval();
  // do_hex_regular_tournament();
}