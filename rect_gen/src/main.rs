#[macro_use]
extern crate serde_derive;
extern crate image;
extern crate rayon;
extern crate docopt;
extern crate sprack;

mod demo;

use demo::*;
use docopt::Docopt;

const USAGE: &'static str = "
Usage:  rectgen [options]

Options:
    -c, --count=COUNT           Amount of rectangles to generate [default: 10].
    -o, --out=DIR               Output dir for results, overwrites existing files [default: .].
    -p, --prefix=PREFIX         Prefix of generated files [default: rectgen_].
    -h, --help                  Show this help message.
";

#[derive(Deserialize, Debug)]
struct Args {
  flag_count: usize,
  flag_out: String,
  flag_prefix: String,
  flag_help: bool,
}

fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

// todo: println!("Args: {:?}", args);

  let samples = generate_rectangles(args.flag_count);
  draw_samples(&args.flag_out, &args.flag_prefix, &samples);
}
