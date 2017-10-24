use sprack::PackOptions;

use std::path::Path;


pub struct RunOptions<'a> {
  pub pack_options: PackOptions<'a>,
  pub output_path: &'a Path,
  pub keep_work_dir: bool,
}

impl<'a> Default for RunOptions<'a> {
  fn default() -> RunOptions<'a> {
    RunOptions {
      pack_options: PackOptions { ..Default::default() },
      output_path: Path::new("out"),
      keep_work_dir: false,
    }
  }
}
