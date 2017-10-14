extern crate rand;
extern crate image;
extern crate imageproc;

use self::rand::Rng;
use image::*;
use std::cmp::max;
use super::algorithm::Dimension;

#[derive(Debug)]
pub struct Sample {
  pub d: Dimension,
  pub color: Rgba<u8>,
}

pub fn generate_rectangles(count: usize, min: Dimension, max: Dimension) -> Vec<Sample> {
  let mut rects: Vec<Sample> = Vec::with_capacity(count);
  let mut rng = rand::thread_rng();

  for _ in 0..count {
    let w = rng.gen_range::<u32>(min.w, max.w);
    let h = rng.gen_range::<u32>(min.h, max.h);
    let color = Rgba { data: [rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), 0xFF] };
    rects.push(Sample { d: Dimension { w, h }, color });
  }

  rects
}

pub fn draw_samples(path: &'static str, samples: &[Sample]) {
  let size = samples.iter().fold(Dimension { w: 0, h: 0 }, |acc, s| {
    Dimension { w: acc.w + s.d.w, h: max(acc.h, s.d.h) }
  });

  use imageproc::drawing::draw_filled_rect_mut;

  let mut offset = 0;
  let mut img = RgbaImage::new(size.w, size.h);
  for s in samples {
    draw_filled_rect_mut(
      &mut img,
      imageproc::rect::Rect::at(offset, 0).of_size(s.d.w, s.d.h),
      s.color,
    );
    offset += s.d.w as i32;
  }
  img.save(path).unwrap();

  println!("samples [{}x{}] written to {}", size.w, size.h, path);
}
