extern crate rand;

mod sprack;

use rand::Rng;
use sprack::*;

fn generate_rectangles(count: usize, min: Dimension, max: Dimension) -> Box<Vec<Rect>> {
    let mut rects: Vec<Rect> = Vec::with_capacity(count);
    let mut rng = rand::thread_rng();

    for _ in 0..count {
        let w = rng.gen_range::<u32>(min.w, max.w);
        let h = rng.gen_range::<u32>(min.h, max.h);
        let rect = Rect {
            x: 0,
            y: 0,
            size: Dimension { w, h },
            flipped: false,
        };
        rects.push(rect);
    }

    Box::new(rects)
}

fn main() {
    let min = Dimension::new(16, 16);
    let max = Dimension::new(64, 64);
    let rectangles = generate_rectangles(9, min, max);

    match pack(rectangles, PackOptions { ..Default::default() }) {
        Ok(bins) => println!("Got result: {:?}", bins),
        Err(e) => eprintln!("Error: {}", e.msg),
    }
}
