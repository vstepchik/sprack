extern crate image;

use image::*;
use std::path::Path;
use super::algorithm::Bin;

pub fn draw_bin(path: &AsRef<Path>, images: &[RgbaImage], bin: &Bin) -> u64 {
  let mut atlas = RgbaImage::new(bin.size.w, bin.size.h);
  for p in &bin.placements {
    let img: ImageBuffer<_, _> = if p.rect.flipped {
      imageops::rotate270(&images[p.index as usize])
    } else {
      images[p.index as usize].to_owned()
    };
    image::imageops::replace(&mut atlas, &img, p.rect.x, p.rect.y);
  }
  atlas.save(path).unwrap();
  path.as_ref().metadata().unwrap().len()
}
