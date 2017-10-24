extern crate image;
extern crate rayon;
extern crate sprack;

mod sprack_bin;

use sprack_bin::*;
use sprack::*;
use std::path::Path;
use rayon::prelude::*;
use image::RgbaImage;

const PNG_EXT: &'static str = "png";

fn main() {
  let work_dir = new_work_dir().expect("Failed to create work dir");
  println!("Work dir is {:?}", &work_dir);
  let samples = generate_rectangles(200);
  let options = RunOptions {
    keep_work_dir: true,
    pack_options: PackOptions {
      flipping: true,
      atlas_compact_steps: 3,
      bin_size: Dimension { w: 512, h: 512 },
      ..Default::default()
    },
    ..Default::default()
  };
  draw_samples(&work_dir, &samples);

  let input = samples.iter().map(|s| Dimension { w: s.width(), h: s.height() }).collect::<Vec<_>>();
  let solutions = pack(&input, &options.pack_options);

  let best: Option<&PackResult> = match solutions {
    Ok(ref solutions) => solutions.par_iter()
      .map(|pack_result| (pack_result, write_solution(&pack_result, &samples, &options, &work_dir)))
      .min_by_key(|tuple| tuple.1)
      .map(|tuple| tuple.0),
    Err(e) => {
      eprintln!("Error: {:?}", e);
      None
    }
  };

  if let Some(best) = best {
    let best_result_dir = Path::new(&work_dir).join(&best.heuristics.get().1);
    match copy_result_to_out(&best_result_dir, &options) {
      Ok(size) => println!("Best results with {:?}, {} bytes", best.heuristics, size),
      Err(e) => eprintln!("Failed to copy results from {:?} to {:?} - {:?}", &best_result_dir, &options.output_path, e)
    }
  }

  if !&options.keep_work_dir { cleanup_work_dir(&work_dir); }
}

fn write_solution(solution: &PackResult, images: &[RgbaImage], options: &RunOptions, work_dir: &AsRef<Path>) -> u64 {
  let dir = Path::new(&work_dir.as_ref()).join(&solution.heuristics.get().1);
  std::fs::remove_dir_all(&dir).unwrap_or(());
  std::fs::create_dir_all(&dir).expect(format!("Failed to create dir {:?}", &dir).as_ref());
  let mut size = 0;
  for (i, bin) in solution.bins.iter().enumerate() {
    size += draw_bin(&dir.join(i.to_string()).with_extension(PNG_EXT), &images, bin, options.pack_options.trim);
  }
  println!("Heuristic {}: {} bins used, total size: {}b", solution.heuristics.get().1, solution.bins.len(), size);
  size
}
