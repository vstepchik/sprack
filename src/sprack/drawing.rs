extern crate image;

use image::RgbaImage;
use image::imageops::{rotate270, replace as draw_img};
use std::path::Path;
use super::algorithm::Bin;

pub fn draw_bin(path: &AsRef<Path>, images: &[RgbaImage], bin: &Bin) -> u64 {
  let mut atlas = RgbaImage::new(bin.size.w, bin.size.h);
  for p in &bin.placements {
    if p.rect.flipped {
      draw_img(&mut atlas, &rotate270(&images[p.index as usize]), p.rect.x, p.rect.y);
    } else {
      draw_img(&mut atlas, &images[p.index as usize], p.rect.x, p.rect.y);
    };
  }
  atlas.save(path).expect(format!("Failed to save atlas {:?}", path.as_ref()).as_ref());
  path.as_ref().metadata().unwrap().len()
}
