use plotters::prelude::*;
use crate::plot::{get_color};

pub fn plot_elos(elo_data: Vec<(String, f64)>) {
  let width = 480;
  let root = image_backend!("elos.png", (640, width));
  let mut prepped_data = Vec::new();
  let mut x = 0;
  let mut range_max: u32 = 0;
  let data_max = elo_data.len() as u32;
  let x_offset: i32 = width as i32 / (data_max+1) as i32;
  for d in elo_data.iter() {
    let elo_num = if d.1 > 0.0 {d.1 as u32}else{0u32};
    println!("ELO_NUM IS {}", elo_num);
    prepped_data.push((x, elo_num));
    x += 1;
    if elo_num > range_max {
      range_max = elo_num;
    }
  }
  println!("RANGE MAX IS {}", range_max);
  let mut chart = ChartBuilder::on(&root)
    .x_label_area_size(50)
    .y_label_area_size(55)
    .margin(12)
    .caption("Elo rating per algorithm", ("Arial", 30.0).into_font())
    .build_ranged(0u32..data_max, 0u32..range_max).unwrap();
  chart.configure_mesh()
    .y_desc("Elo")
    .x_desc("Algorithm")
    .x_label_offset(x_offset)
    .x_label_formatter(&|x| {
      if *x >= data_max {
        format!("")
      } else {
        elo_data[*x as usize ].0.clone()
      }
    })
    .axis_desc_style(("Arial", 25).into_font())
    .draw().unwrap();



  let histogram = Histogram::vertical(&chart)
        .style(BLUE.filled())
        .data(prepped_data.into_iter());
  chart.draw_series(
    histogram,
  ).unwrap();
}