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

fn draw_bin(path: &AsRef<Path>, samples: &[Sample], bin: &Bin) -> u64 {
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

fn main() {
  let min = Dimension::new(8, 8);
  let max = Dimension::new(64, 64);
  let samples = generate_rectangles(50, min, max);
  let rectangles = samples.iter().map(|s| s.d).collect::<Vec<_>>();
  let options = PackOptions { flipping: true, bin_size: Dimension { w: 256, h: 256 }, ..Default::default() };

  draw_samples("in.png", &samples);

  let best = match pack(&rectangles, &options) {
    Ok(solutions) => {
      let mut best: (Option<PackHeuristic>, u64) = (None, std::u64::MAX);
      for solution in solutions {
        let dir = Path::new(&options.output_path).join(&solution.heuristics.get().1);
        std::fs::remove_dir_all(&dir).unwrap_or(());
        std::fs::create_dir_all(&dir).expect(format!("Failed to create dir {:?}", &dir).as_ref());
        let mut size = 0;
        for (bin_number, bin) in solution.bins.iter().enumerate() {
          size += draw_bin(&dir.join(bin_number.to_string()).with_extension("png"), &samples, bin);
        }
        println!("Heuristic {} -> {} bins used, total size: {}b", solution.heuristics.get().1, solution.bins.len(), size);
        if size < best.1 { best = (Some(solution.heuristics), size); }
      }
      Some(best)
    }
    Err(e) => {
      eprintln!("Error: {:?}", e);
      None
    }
  };

  if let Some(best) = best {
    println!("Best results with {:?}, {} bytes total", best.0, best.1);
    // todo: copy as "best"
  }
}
