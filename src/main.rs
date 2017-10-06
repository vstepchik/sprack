extern crate rand;

mod sprack;

use rand::Rng;
use sprack::*;

fn generate_rectangles(count: usize, min: Dimension, max: Dimension) -> Vec<Dimension> {
  let mut rects: Vec<Dimension> = Vec::with_capacity(count);
  let mut rng = rand::thread_rng();

  for _ in 0..count {
    let w = rng.gen_range::<u32>(min.w, max.w);
    let h = rng.gen_range::<u32>(min.h, max.h);
    rects.push(Dimension { w, h });
  }

  rects
}

fn main() {
  let min = Dimension::new(8, 8);
  let max = Dimension::new(64, 64);
  let rectangles = generate_rectangles(1_000, min, max);

  match pack(rectangles.as_ref(), PackOptions { flipping: true, ..Default::default() }) {
    Ok(solutions) => for solution in solutions {
      println!("Got result sorting by {}, bins used: {}", solution.sorting_name, solution.bins.len());
    }
    Err(e) => eprintln!("Error: {}", e.0),
  }
}
