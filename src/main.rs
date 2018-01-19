#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate rayon;
extern crate docopt;
extern crate sprack;

mod sprack_bin;

use sprack_bin::*;
use sprack::*;
use std::path::Path;
use std::ffi::OsStr;
use rayon::prelude::*;
use image::{DynamicImage, GenericImage};

const PNG_EXT: &str = "png";



fn main() {
  let work_dir = new_work_dir().expect("Failed to create work dir");
  println!("Work dir is {:?}", &work_dir);

  let options = sprack_bin::read_options();
  for path in &options.input_paths {
    if path.is_dir() {
      println!("> [{:?}]", path)
    } else {
      println!("> {:?}", path)
    }
  }

  println!("filtering and reading samples");
  let samples = options.input_paths.iter()
    .filter(|path| is_supported_format(path))
    .map(|path| image::open(path).unwrap())
    .collect::<Vec<_>>();

  println!("converting samples into rectangles");
  let input = samples.iter().map(|s| Dimension { w: s.width(), h: s.height() }).collect::<Vec<_>>();
  println!("finding solutions");
  let solutions = pack(&input, &options.pack_options);

  println!("writing solutions & picking best");
  let best: Option<&PackResult> = match solutions {
    Ok(ref solutions) => solutions.par_iter()
      .map(|pack_result| (pack_result, write_solution(pack_result, &samples, &options, &work_dir)))
      .min_by_key(|tuple| tuple.1)
      .map(|tuple| tuple.0),
    Err(e) => {
      eprintln!("Error: {:?}", e);
      None
    }
  };

  println!("copying best solution");
  if let Some(best) = best {
    let best_result_dir = Path::new(&work_dir).join(&best.heuristics.name());
    match copy_result_to_out(&best_result_dir, &options) {
      Ok(size) => println!("Best results with {}, {} bytes", &best.heuristics.name(), size),
      Err(e) => eprintln!("Failed to copy results from {:?} to {:?} - {:?}", &best_result_dir, &options.output_path, e),
    }
  }

  println!("cleanup work");
  if !&options.keep_work_dir { cleanup_work_dir(&work_dir); }
}

fn is_supported_format(path: &Path) -> bool {
  if let Some(ext) = path.extension().map(OsStr::to_string_lossy).map(|e| e.to_lowercase()) {
    let ext = ext.as_str();
    match ext {
      "png" | "bmp" | "gif" | "jpg" | "jpeg" | "ico" | "tiff" | "webp" | "ppm" => true,
      _ => false
    }
  } else { false }
}

fn write_solution(solution: &PackResult, images: &[DynamicImage], options: &RunOptions, work_dir: &AsRef<Path>) -> u64 {
  let dir = Path::new(&work_dir.as_ref()).join(&solution.heuristics.name());
  std::fs::remove_dir_all(&dir).unwrap_or(());
  std::fs::create_dir_all(&dir).expect(format!("Failed to create dir {:?}", &dir).as_ref());
  let mut size = 0;
  for (i, bin) in solution.bins.iter().enumerate() {
    size += draw_bin(&dir.join(i.to_string()).with_extension(PNG_EXT), images, bin, options.pack_options.trim);
  }
  println!("Heuristic {}: {} bins used, total size: {}b", solution.heuristics.name(), solution.bins.len(), size);
  size
}
