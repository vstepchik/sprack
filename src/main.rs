extern crate rand;

mod sprack;

use rand::Rng;
use sprack::*;

fn generate_rectangles(count: usize, min: Dimension, max: Dimension) -> Box<Vec<Dimension>> {
  let mut rects: Vec<Dimension> = Vec::with_capacity(count);
  let mut rng = rand::thread_rng();

  for _ in 0..count {
    let w = rng.gen_range::<u32>(min.w, max.w);
    let h = rng.gen_range::<u32>(min.h, max.h);
    rects.push(Dimension { w, h });
  }

  Box::new(rects)
}

fn main() {
  let min = Dimension::new(16, 16);
  let max = Dimension::new(96, 96);
  let rectangles = generate_rectangles(1_000, min, max);
//  println!("{:?}", rectangles);

  match pack(rectangles.as_ref(), PackOptions { flipping: true, ..Default::default() }) {
    Ok(bins) => println!("Got result: {:?}", bins.len()),
    Err(e) => eprintln!("Error: {}", e.msg),
  }
}
