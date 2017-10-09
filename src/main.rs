extern crate rand;
extern crate image;
extern crate imageproc;

mod sprack;

use sprack::*;
use rand::Rng;
use image::*;
use std::path::Path;

#[derive(Debug)]
struct Sample {
  d: Dimension,
  color: Rgba<u8>,
}

fn generate_rectangles(count: usize, min: Dimension, max: Dimension) -> Vec<Sample> {
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

fn draw_samples(path: &'static str, samples: &[Sample]) {
  let size = samples.iter().fold(Dimension { w: 0, h: 0 }, |acc, s| {
    Dimension { w: acc.w + s.d.w, h: std::cmp::max(acc.h, s.d.h) }
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

fn draw_bin(path: &AsRef<Path>, samples: &[Sample], bin: &Bin) {
  let mut img = RgbaImage::new(bin.size.w, bin.size.h);
  for p in &bin.placements {
    imageproc::drawing::draw_filled_rect_mut(
      &mut img,
      imageproc::rect::Rect::at(p.rect.x as i32, p.rect.y as i32).of_size(p.rect.size.w, p.rect.size.h),
      samples[p.index as usize].color,
    );
  }
  img.save(path).unwrap();
}

fn main() {
  let min = Dimension::new(8, 8);
  let max = Dimension::new(64, 64);
  let samples = generate_rectangles(50, min, max);
  let rectangles = samples.iter().map(|s| s.d).collect::<Vec<_>>();
  let options = PackOptions { flipping: true, bin_size: Dimension { w: 256, h: 256 }, ..Default::default() };

  draw_samples("in.png", &samples);

  match pack(&rectangles, &options) {
    Ok(solutions) => for solution in solutions {
      println!("Got result sorting by {}, bins used: {}", solution.sorting_name, solution.bins.len());
      let dir = Path::new(&options.output_path).join(&solution.sorting_name);
      std::fs::remove_dir_all(&dir).unwrap_or(());
      std::fs::create_dir_all(&dir).expect(format!("Failed to create dir {:?}", &dir).as_ref());
      for (bin_number, bin) in solution.bins.iter().enumerate() {
        draw_bin(&dir.join(bin_number.to_string()).with_extension("png"), &samples, bin);
      }
    }
    Err(e) => eprintln!("Error: {}", e.0),
  }
}
