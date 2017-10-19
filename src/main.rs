extern crate image;
extern crate rayon;
extern crate fs_extra;

mod sprack;

use sprack::*;
use std::path::Path;
use rayon::prelude::*;
use image::RgbaImage;
use fs_extra::dir::{copy, CopyOptions};

const PNG_EXT: &'static str = "png";
const FOLDER_NAME_BEST: &'static str = "best";

fn main() {
  let samples = generate_rectangles(200);
  let options = PackOptions {
    flipping: true,
    atlas_compact_attempts: 2,
    bin_size: Dimension { w: 512, h: 512 },
    ..Default::default()
  };
  draw_samples(&options.output_path, &samples);

  let input = samples.iter().map(|s| Dimension { w: s.width(), h: s.height() }).collect::<Vec<_>>();
  let solutions = pack(&input, &options);

  let best: Option<&PackResult> = match solutions {
    Ok(ref solutions) => solutions.par_iter()
      .map(|pack_result| (pack_result, write_solution(&pack_result, &samples, &options)))
      .min_by_key(|tuple| tuple.1)
      .map(|tuple| tuple.0),
    Err(e) => {
      eprintln!("Error: {:?}", e);
      None
    }
  };

  if let Some(best) = best {
    println!("Best results with {:?}", best.heuristics);
    let from = Path::new(&options.output_path).join(&best.heuristics.get().1);
    let to = Path::new(&options.output_path).join(FOLDER_NAME_BEST);
    let opts = {
      let mut opts = CopyOptions::new();
      opts.overwrite = true;
      opts
    };
    std::fs::remove_dir_all(&to).unwrap_or(());
    std::fs::create_dir_all(&to).expect(format!("Failed to create dir {:?}", &to).as_ref());
    copy(&from, &to, &opts).expect(format!("Failed to copy best results into {:?}", to).as_str());
  }
}

fn write_solution(solution: &PackResult, images: &[RgbaImage], options: &PackOptions) -> u64 {
  let dir = Path::new(&options.output_path).join(&solution.heuristics.get().1);
  std::fs::remove_dir_all(&dir).unwrap_or(());
  std::fs::create_dir_all(&dir).expect(format!("Failed to create dir {:?}", &dir).as_ref());
  let mut size = 0;
  for (bin_number, bin) in solution.bins.iter().enumerate() {
    size += draw_bin(&dir.join(bin_number.to_string()).with_extension(PNG_EXT), &images, bin, options.trim);
  }
  println!("Heuristic {}: {} bins used, total size: {}b", solution.heuristics.get().1, solution.bins.len(), size);
  size
}
