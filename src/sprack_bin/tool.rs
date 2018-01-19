use sprack::PackOptions;

use std::path::Path;
use std::path::PathBuf;


pub struct RunOptions<'a> {
  pub pack_options: PackOptions<'a>,
  pub input_paths: Vec<PathBuf>,
  pub output_path: PathBuf,
  pub keep_work_dir: bool,
  pub demo_run: bool,
  pub recursive: bool,
}

impl<'a> Default for RunOptions<'a> {
  fn default() -> RunOptions<'a> {
    RunOptions {
      pack_options: PackOptions { ..Default::default() },
      input_paths: vec![],
      output_path: Path::new("out").to_owned(),
      keep_work_dir: false,
      demo_run: false,
      recursive: false,
    }
  }
}
