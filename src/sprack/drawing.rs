extern crate image;
extern crate imageproc;

use image::*;
use std::path::Path;
use super::demo::Sample;
use super::algorithm::Bin;

pub fn draw_bin(path: &AsRef<Path>, samples: &[Sample], bin: &Bin) -> u64 {
  let mut img = RgbaImage::new(bin.size.w, bin.size.h);
  for p in &bin.placements {
    imageproc::drawing::draw_filled_rect_mut(
      &mut img,
      imageproc::rect::Rect::at(p.rect.x as i32, p.rect.y as i32).of_size(p.rect.size.w, p.rect.size.h),
      samples[p.index as usize].color,
    );
  }
  img.save(path).unwrap();
  path.as_ref().metadata().unwrap().len()
}
