use bbt::{Rater, Rating, Outcome};

pub fn do_rate() {
  let rater = Rater::new(1500.0/6.0);

  let mut p1 = Rating::new(1500.0, 1500.0/3.0);
  let mut p2 = Rating::new(1500.0, 1500.0/3.0);
  for _ in 0..200 {
    let (new_p1, new_p2) = rater.duel(p1, p2, Outcome::Win);
    p1 = new_p1;
    p2 = new_p2;
  }
}