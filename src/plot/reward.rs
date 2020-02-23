use plotters::prelude::*;
use crate::plot::{get_color};
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Deserializer, Value};

pub fn get_values(path: &str) -> Result<Vec<f64>, std::io::Error> {
  let mut res = Vec::new();
  let mut file = File::open(path)?;
  let mut content = String::new();
  file.read_to_string(&mut content);
  let json_stream: Vec<f64> = serde_json::from_str(content.as_str()).unwrap();
  for value in json_stream {
    res.push(value);
  }
  Ok(res)
}


pub fn make_reward_graph(paths: &[(&str, String)], file_name: &str, title: &str, xlabel: &str, ylabel: &str) -> Result<(), std::io::Error> {
  let mut value_max = 0;
  let mut max_value = -999.0;
  let mut min_value = 999.0;
  let mut lines = Vec::new();
  for algo in paths {
    let values = get_values(algo.0)?;
    let size_of_values = values.len();
    if size_of_values > value_max {
      value_max = size_of_values;
    }
    for v in values.iter() {
      if *v > max_value {
        max_value = *v;
      }
      if *v < min_value {
        min_value = *v;
      }
    }
    lines.push((algo.1.clone(), to_line_series!(values)));
  }
  let image = image_backend!(file_name, (860, 645));
  let mut plot = make_chart!(&image, title.to_owned(), 0..value_max, min_value..max_value, xlabel.to_owned(), ylabel.to_owned());
  draw_lines!(plot, lines);
  plot.configure_series_labels()
  .position(SeriesLabelPosition::LowerMiddle)
  .label_font(("Arial", 23).into_font())
  .background_style(&WHITE.mix(0.8))
  .border_style(&BLACK)
  .draw().unwrap();
  Ok(())
}

pub fn make_hex_graph(paths: &[(&str, String)], file_name: &str, title: &str, xlabel: &str, ylabel: &str, label_pos: SeriesLabelPosition) -> Result<(), std::io::Error> {
  let mut value_max = 0;
  let mut max_value = -999.0;
  let mut min_value = 999.0;
  let mut lines = Vec::new();
  for algo in paths {
    let values = get_values(algo.0)?;
    let size_of_values = values.len();
    if size_of_values > value_max {
      value_max = size_of_values;
    }
    for v in values.iter() {
      if *v > max_value {
        max_value = *v;
      }
      if *v < min_value {
        min_value = *v;
      }
    }
    
    let mut line = Vec::with_capacity(values.len());
    let mut i = 2;
    for entry in values {
      line.push((i, entry));
      i += 1;
    }
    lines.push((algo.1.clone(), line));
  }
  let image = image_backend!(file_name, (860, 645));
  let mut plot = make_chart!(&image, title.to_owned(), 2..11, min_value..max_value, xlabel.to_owned(), ylabel.to_owned());
  
  let mut i = 0;
  for line in lines {
    let color = get_color(i);
    let sub_i = i;
    plot.draw_series(LineSeries::new(
      line.1.clone(),
      ShapeStyle{color,stroke_width: 1,filled: true})).unwrap()
      .label(line.0.as_str())
      .legend(move |(x, y)| Path::new(vec![(x, y), (x + 20, y)], &get_color(sub_i)));
    i += 1;
  }
  plot.configure_series_labels()
  .position(label_pos)
  .label_font(("Arial", 23).into_font())
  .background_style(&WHITE.mix(0.8))
  .border_style(&BLACK)
  .draw().unwrap();
  Ok(())
}