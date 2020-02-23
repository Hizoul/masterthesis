use super::{Coordinate,get_neighbors,coord_to_num,num_to_coord};

pub fn hex_heuristic(field: &Vec<Vec<u8>>, player: u8) -> usize {
  let field_size = field.len() - 1;
  let mut start_positions = Vec::new();
  let mut end_positions = Vec::new();
  for i in 0..field.len() {
    if player == 2 {
      start_positions.push((i, 0));
      end_positions.push((i, field_size));
    } else {
      start_positions.push((0, i));
      end_positions.push((field_size, i));
    }
  }
  let mut paths = Vec::new();
  for start in start_positions {
    for end in end_positions.iter() {
      paths.push(shortest_path(field, player, &start, end));
    }
  }
  if paths.len() > 0 {
    paths.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    for path in paths.iter() {
      for coord in path.iter() {
        if field[coord.1][coord.0] == 0 {
          return coord_to_num(field.len(), coord);
        }
      }
    }
  }
  // println!("COULDN*T FIND ANYTHING IN {:?}", paths);
  field.len() * 2
}

pub fn min_distance(dist: &mut Vec<usize>, visited: &mut Vec<bool>) -> usize {
    let mut min = std::usize::MAX;
    let mut min_index = 0;
    for i in 0..dist.len() {
      if visited[i] == false && dist[i] <= min {
        min = dist[i];
        min_index = i
      }
    }
    min_index 
}

pub fn shortest_path(field: &Vec<Vec<u8>>, player: u8, start: &Coordinate, end: &Coordinate) -> Vec<Coordinate> {
  let size = field.len() * field.len();
  let mut dist: Vec<usize> = Vec::with_capacity(size);
  let mut prev: Vec<usize> = Vec::with_capacity(size);
  let mut visited: Vec<bool> = Vec::with_capacity(size);
  for _ in 0..size {
    dist.push(std::usize::MAX);
    visited.push(false);
    prev.push(std::usize::MAX);
  }
  let mut already_has_block = false;
  for y in 0..field.len() {
    for x in 0..field.len() {
      if field[y][x] == player {
        already_has_block = true;
        dist[coord_to_num(field.len(), &(x,y))] = 1;
      }
    }
  }
  if !already_has_block {
    dist[coord_to_num(field.len(), start)] = 1;
  }
  let mut end_num = coord_to_num(field.len(), end);
  'SEARCH: for i in 0..size {
    let u = min_distance(&mut dist, &mut visited);
    visited[u] = true;
    if u == end_num {
      break 'SEARCH;
    }
    let coord = num_to_coord(field.len(), u);
    if coord.0 < field.len() && coord.1 < field.len() {
      let neighbors = get_neighbors(field.len(), coord.0, coord.1);
      for neighbor_option in neighbors.iter() {
        if neighbor_option.is_some() {
          let neighbor = neighbor_option.unwrap();
          if neighbor.0 < field.len() && neighbor.1 < field.len() {
            if field[neighbor.1][neighbor.0] == 0 {
              let v = coord_to_num(field.len(), &neighbor);
              if !visited[v] && dist[u] != std::usize::MAX && dist[u]+1 < dist[v] {
                dist[v] = dist[u] + 1;
                prev[v] = u;
              }
            }
          }
        }
      }
    }
  }
  let mut path = Vec::new();
  while end_num != std::usize::MAX {
    path.push(end_num);
    end_num = prev[end_num];
  }
  let mut coord_path = Vec::new();
  for n in path {
    coord_path.push(num_to_coord(field.len(), n));
  }
  return coord_path;
}

pub fn reshape(to_reshape: Vec<usize>) -> Vec<Vec<usize>> {
  let new_size = (to_reshape.len() as f64).sqrt() as usize;
  let mut new_top = Vec::with_capacity(new_size);
  for y in 0..new_size {
    let mut new_sub = Vec::with_capacity(new_size);
    for x in 0..new_size {
      new_sub.push(to_reshape[coord_to_num(new_size, &(x,y))]);
    }
    new_top.push(new_sub);
  }
  new_top
}