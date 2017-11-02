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
use rayon::prelude::*;
use image::RgbaImage;
use docopt::Docopt;

const PNG_EXT: &str = "png";

const USAGE: &'static str = "
Usage:  sprack [options] ([-w SIDE] [-h SIDE] | [-s SIDE]) [--demo | --help | <files>...]

Options:
    -r, --recursive             Recursively descend into directories.
    -o, --out=DIR               Output dir for results, overwrites existing files [default: ./out].
    -w, --width=SIDE            Atlas width [default: 1024].
    -h, --height=SIDE           Atlas height [default: 1024].
    -s, --size=SIDE             Atlas width and height.
    -f, --flipping              Allow placement of sprites rotated by 90 degrees.
    -t, --trim                  Trim resulting images to minimal size.
    -i, --increments-count=NUM  Allows incremental atlas size growth. 0 means atlas starts at
                                specified size, without increments. If NUM is > 0 starts at
                                INC size and if sprite doesn't fit - grows by another INC, where
                                INC = min(1, SIDE/(NUM+1)). Allowed values are 0..255, the higher
                                the value - the more time packing will take [default: 0].
    -k, --keep-work-dir         Do not delete temporary files after work.
        --demo                  Generate in-memory random colored rectangles with up arrows and
                                compact them into sprite atlases.
    -h, --help                  Show this help message.
";

#[derive(Deserialize, Debug)]
struct Args {
  arg_files: Vec<String>,
  flag_out: String,
  flag_increments_count: u8,
  flag_width: u32,
  flag_height: u32,
  flag_size: Option<u32>,
  flag_trim: bool,
  flag_flipping: bool,
  flag_demo: bool,
  flag_keep_work_dir: bool,
  flag_recursive: bool,
  flag_help: bool,
}

impl<'a> From<&'a Args> for RunOptions<'a> {
  fn from(args: &'a Args) -> Self {
    let bin_size = if let Some(size) = args.flag_size {
      Dimension { w: size, h: size }
    } else {
      Dimension { w: args.flag_width, h: args.flag_height }
    };

    let pack_options = PackOptions {
      bin_size,
      flipping: args.flag_flipping,
      trim: args.flag_trim,
      atlas_compact_steps: args.flag_increments_count,
      ..Default::default()
    };

    RunOptions {
      keep_work_dir: args.flag_keep_work_dir,
      input_paths: args.arg_files.iter().map(|f| Path::new(f.as_str())).collect(),
      output_path: Path::new(args.flag_out.as_str()),
      demo_run: args.flag_demo,
      recursive: args.flag_recursive,
      pack_options,
      ..Default::default()
    }
  }
}

fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  println!("Args: {:?}", args);
  let work_dir = new_work_dir().expect("Failed to create work dir");
  println!("Work dir is {:?}", &work_dir);

  let options = RunOptions::from(&args);
  for path in &options.input_paths {
    println!("> {:?}", path)
  }

  let samples = generate_rectangles(200);
  draw_samples(&work_dir, &samples);

  let input = samples.iter().map(|s| Dimension { w: s.width(), h: s.height() }).collect::<Vec<_>>();
  let solutions = pack(&input, &options.pack_options);

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

  if let Some(best) = best {
    let best_result_dir = Path::new(&work_dir).join(&best.heuristics.name());
    match copy_result_to_out(&best_result_dir, &options) {
      Ok(size) => println!("Best results with {}, {} bytes", &best.heuristics.name(), size),
      Err(e) => eprintln!("Failed to copy results from {:?} to {:?} - {:?}", &best_result_dir, &options.output_path, e),
    }
  }

  if !&options.keep_work_dir { cleanup_work_dir(&work_dir); }
}

fn write_solution(solution: &PackResult, images: &[RgbaImage], options: &RunOptions, work_dir: &AsRef<Path>) -> u64 {
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
