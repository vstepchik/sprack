extern crate image;

use std::cmp::max;
use std::path::Path;
use image::{RgbaImage, DynamicImage};
use image::imageops::{rotate270, replace as draw_img};
use sprack::Bin;

pub fn draw_bin(path: &AsRef<Path>, images: &[DynamicImage], bin: &Bin, trim: bool) -> u64 {
  let (width, height) = compute_atlas_size(bin, trim);
  let mut atlas = RgbaImage::new(width, height);
  for p in &bin.placements {
    if p.rect.flipped {
      draw_img(&mut atlas, &rotate270(&images[p.index as usize]), p.rect.x, p.rect.y);
    } else {
      // fixme: avoid copying (to_rgba()) - it seems unnecessary
      draw_img(&mut atlas, &images[p.index as usize].to_rgba(), p.rect.x, p.rect.y);
    };
  }
  atlas.save(path).expect(format!("Failed to save atlas {:?}", path.as_ref()).as_ref());
  path.as_ref().metadata().unwrap().len()
}

fn compute_atlas_size(bin: &Bin, trim: bool) -> (u32, u32) {
  if trim {
    bin.placements.iter().fold((1, 1), |acc, p| (max(acc.0, p.rect.r()), max(acc.1, p.rect.b())))
  } else {
    (bin.size.w, bin.size.h)
  }
}
