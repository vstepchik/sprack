extern crate rand;
extern crate image;

use std::path::Path;
use std::fs;
use self::rand::Rng;
use image::*;
use std::cmp::max;
use super::algorithm::Dimension;

pub fn generate_rectangles(count: usize) -> Vec<RgbaImage> {
  let min = Dimension::new(8, 8);
  let max = Dimension::new(64, 64);
  let mut rects: Vec<RgbaImage> = Vec::with_capacity(count);
  let mut rng = rand::thread_rng();

  for _ in 0..count {
    let w = rng.gen_range::<u32>(min.w, max.w);
    let h = rng.gen_range::<u32>(min.h, max.h);
    let color = Rgba([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), 0xFF]);
    rects.push(new_rect_image(w, h, color));
  }

  rects
}

fn new_rect_image(width: u32, height: u32, color: Rgba<u8>) -> RgbaImage {
  let mut img = RgbaImage::from_pixel(width, height, color);

  let mark_color = Rgba([0x00d, 0x00, 0x00, 0xFF]);
  let x = width / 2;
  img.put_pixel(x - 1, 3, mark_color);
  img.put_pixel(x + 1, 3, mark_color);
  for y in 2..height-2 {
    img.put_pixel(x, y, mark_color);
  }
  img
}

pub fn draw_samples(path: &AsRef<Path>, samples: &[RgbaImage]) {
  fs::create_dir_all(&path).expect(format!("Failed to create dir {:?}", &path.as_ref()).as_ref());

  let size = samples.iter().fold(Dimension { w: 0, h: 0 }, |acc, s| {
    Dimension { w: acc.w + s.width(), h: max(acc.h, s.height()) }
  });

  let mut offset = 0;
  let mut img = RgbaImage::new(size.w, size.h);
  for s in samples {
    image::imageops::replace(&mut img, s, offset, 0);
    offset += s.width();
  }
  img.save(Path::new(&path.as_ref()).join("samples.png")).unwrap();

  println!("samples [{}x{}] written", size.w, size.h);
}
