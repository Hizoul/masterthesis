use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use plotters::prelude::*;
use crate::plot::{get_color};

#[derive(Debug)]
pub struct Trial {
  trial_id: i64,
  study_id: i64,
  state: String,
  value: f64
}

pub fn make_optuna_hyperparametersearch_graph(paths: &[(&str, String)]) -> Result<()> {
  let mut value_max = 0;
  let mut max_value = -999.0;
  let mut min_value = 999.0;
  let mut lines = Vec::new();
  for algo in paths {
    let values = get_trial_values(algo.0)?;
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
  let image = image_backend!("hyperparametersearch.png", (860, 645));
  let mut plot = make_chart!(&image, "RL-Heuristic Hyperparameter Search".to_owned(), 0..value_max, min_value..max_value, "Trial".to_owned(), "Reward".to_owned());
  draw_lines!(plot, lines);
  plot.configure_series_labels()
    .position(SeriesLabelPosition::LowerMiddle)
    .label_font(("Arial", 26).into_font())
    .background_style(&WHITE.mix(0.8))
    .border_style(&BLACK)
  .draw().unwrap();
  Ok(())
}

pub fn get_trial_values(path: &str) -> Result<Vec<f64>> {
  let conn = Connection::open(path)?;
  let mut stmt = conn.prepare("SELECT * from trials;")?;
  
  let trials = stmt
  .query_map(NO_PARAMS, |row| 
      Ok( 
          Trial {
              trial_id: row.get(0)?,
              study_id: row.get(1)?,
              state: row.get(2)?,
              value: row.get(3)?
          }
      )
  )?;	
  
  let mut results = Vec::new();
  for trial in trials {
    if trial.is_ok() {
      results.push(trial?.value);
    }
  }
  Ok(results)
}