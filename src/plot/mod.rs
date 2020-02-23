use plotters::prelude::*;
use plotters::style::{RGBAColor};

pub fn get_color(i: usize) -> RGBAColor {
  let colors: Vec<Box<dyn Color>> = vec![
    Box::new(RED), Box::new(GREEN), Box::new(BLUE), Box::new(CYAN), Box::new(BLACK), Box::new(YELLOW)
  ];
  colors[i % colors.len()].to_rgba()
}
#[macro_export]
macro_rules! to_line_series {
    ($data:expr) => {
      {
        let mut lines = Vec::with_capacity($data.len());
        let mut i = 0;
        for entry in $data {
          lines.push((i, entry));
          i += 1;
        }
        lines
      }
    };
}
#[macro_export]
macro_rules! to_line_series_float {
    ($data:expr) => {
      {
        let mut lines = Vec::with_capacity($data.len());
        let mut i = 0.0;
        for entry in $data {
          lines.push((i, entry));
          i += 1.0;
        }
        lines
      }
    };
}

#[macro_export]
macro_rules! make_chart {
    ($root:expr, $name:expr, $x_range:expr, $y_range:expr, $x_desc:expr, $y_desc:expr) => {
      {
        let mut chart = ChartBuilder::on($root)
          .x_label_area_size(43)
          .y_label_area_size(53)
          .margin(12)
          .caption($name, ("Arial", 30.0).into_font())
          .build_ranged($x_range, $y_range).unwrap();
        chart.configure_mesh()
          .y_desc($y_desc)
          .x_desc($x_desc)
          .axis_desc_style(("Arial", 20).into_font())
          .draw().unwrap();
        chart
      }
    };
}
#[macro_export]
macro_rules! draw_lines {
    ($chart:expr, $line_data:expr) => {
      let mut i = 0;
      for line in $line_data {
        let color = get_color(i);
        let sub_i = i;
        $chart.draw_series(LineSeries::new(
          line.1.clone(),
          ShapeStyle{color,stroke_width: 1,filled: true})).unwrap()
          .label(line.0.as_str())
          .legend(move |(x, y)| Path::new(vec![(x, y), (x + 20, y)], &get_color(sub_i)));
        i += 1;
      }
    };
}

#[macro_export]
macro_rules! draw_candles {
    ($chart:expr, $candles:expr) => {
      let mut i = 0;
      for line in $candles {
        let color = get_color(i);
        let sub_i = i;
        $chart.draw_series(
          line.1.iter().map(|x| CandleStick::new(x.0, x.1, x.2, x.3, x.4, &color, &color, 9))
        ).unwrap()
          .label(line.0.as_str())
          .legend(move |(x, y)| Path::new(vec![(x, y), (x + 20, y)], &get_color(sub_i)));
        i += 1;
      }
    };
}

#[macro_export]
macro_rules! image_backend {
  ($target:expr, $size:expr) => {
    {
      let root = BitMapBackend::new($target, $size).into_drawing_area();
      root.fill(&WHITE).unwrap();
      root
    }
  }
}

#[macro_export]
macro_rules! make_line_chart {
    ($path:expr, $size:expr, $name:expr, $x_range:expr, $y_range:expr, $x_desc:expr, $y_desc:expr, $line_data:expr) => {
      {
        let root = image_backend!($path, $size);
        let mut chart = make_chart!(&root, $name, $x_range, $y_range, $x_desc, $y_desc);
        draw_lines!(chart, $line_data);
        chart
          .configure_series_labels()
          .position(SeriesLabelPosition::UpperRight)
          .label_font(("Arial", 18).into_font())
          .background_style(&WHITE.mix(0.8))
          .border_style(&BLACK)
          .draw().unwrap();
      }
    };
}
#[macro_export]
macro_rules! make_candle_chart {
    ($path:expr, $size:expr, $name:expr, $x_range:expr, $y_range:expr, $x_desc:expr, $y_desc:expr, $line_data:expr) => {
      {
        let root = image_backend!($path, $size);
        let mut chart = make_chart!(&root, $name, $x_range, $y_range, $x_desc, $y_desc);
        draw_candles!(chart, $line_data);
        chart
          .configure_series_labels()
          .background_style(&WHITE.mix(0.8))
          .border_style(&BLACK)
          .draw().unwrap()
      }
    };
}
#[macro_export]
macro_rules! make_line_and_candle_chart {
    ($path:expr, $size:expr, $name:expr, $x_range:expr, $y_range:expr, $x_desc:expr, $y_desc:expr, $line_data:expr, $candle_data:expr) => {
      {
        let root = image_backend!($path, $size);
        let mut chart = make_chart!(&root, $name, $x_range, $y_range, $x_desc, $y_desc);
        draw_lines!(chart, $line_data);
        let mut i = 0;
        for line in $candle_data {
          let color = get_color(i);
          let sub_i = i;
          chart.draw_series(
            line.1.iter().map(|x| CandleStick::new(x.0, x.1, x.2, x.3, x.4, &color, &color, 9))
          ).unwrap();
          i += 1;
        }
        chart
          .configure_series_labels()
          .position(SeriesLabelPosition::UpperRight)
          .label_font(("Arial", 18).into_font())
          .background_style(&WHITE.mix(0.8))
          .border_style(&BLACK)
          .draw().unwrap()
      }
    };
}


#[macro_use]
#[cfg(feature = "sqlite")]
pub mod optuna_sqlite;
#[macro_use]
pub mod possible_plays;
#[macro_use]
pub mod elo;
#[macro_use]
pub mod reward;