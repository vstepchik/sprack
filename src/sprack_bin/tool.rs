use sprack::PackOptions;

use std::path::Path;


pub struct RunOptions<'a> {
  pub pack_options: PackOptions<'a>,
  pub input_paths: Vec<&'a Path>,
  pub output_path: &'a Path,
  pub keep_work_dir: bool,
  pub demo_run: bool,
  pub recursive: bool,
}

impl<'a> Default for RunOptions<'a> {
  fn default() -> RunOptions<'a> {
    RunOptions {
      pack_options: PackOptions { ..Default::default() },
      input_paths: vec![],
      output_path: Path::new("out"),
      keep_work_dir: false,
      demo_run: false,
      recursive: false,
    }
  }
}
