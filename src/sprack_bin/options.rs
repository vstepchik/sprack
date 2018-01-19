use super::tool::RunOptions;
use sprack::{Dimension, PackOptions};
use std::path::{Path, PathBuf};
use docopt::Docopt;

const USAGE: &'static str = "
Usage:  sprack [options] ([-w SIDE] [-h SIDE] | [-s SIDE]) [--help | <files>...]

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

    -h, --help                  Show this help message.
        --debug                 Show more debug during program execution.
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
  flag_keep_work_dir: bool,
  flag_recursive: bool,
  flag_debug: bool,
  flag_help: bool,
}

impl<'a> From<Args> for RunOptions<'a> {
  fn from(args: Args) -> Self {
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
      input_paths: args.arg_files.iter()
        .map(|f| f.clone())
        .map(|f| PathBuf::from(f))
        .collect(),
      output_path: Path::new(&args.flag_out.clone()).to_owned(),
      recursive: args.flag_recursive,
      pack_options,
      ..Default::default()
    }
  }
}

pub fn read_options<'a>() -> RunOptions<'a> {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  if args.flag_debug { println!("Debug: {:?}", args); }

  RunOptions::from(args)
}
