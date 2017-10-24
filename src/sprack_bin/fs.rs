extern crate rand;

use std::fs::{create_dir_all as mk_dir, remove_dir_all as rm_dir, read_dir, copy, DirEntry};
use std::io::Result;
use std::env::temp_dir;
use std::path::{PathBuf, Path};
use self::rand::{thread_rng, Rng};
use sprack_bin::RunOptions;

const APP_DIR_NAME: &'static str = "sprack";

pub fn new_work_dir() -> Result<PathBuf> {
  let work_dir = gen_work_dir_path();
  mk_dir(&work_dir)?;
  Ok(work_dir)
}

pub fn cleanup_work_dir(path: &AsRef<Path>) {
  rm_dir(path).unwrap_or(());
}

pub fn copy_result_to_out(result_dir: &AsRef<Path>, options: &RunOptions) -> Result<u64> {
  let out = &options.output_path;
  mk_dir(out)?;
  get_png_files(&result_dir)?.iter().map(|f| copy(&f.path(), &out.join(&f.file_name()))).sum()
}

fn gen_work_dir_path() -> PathBuf {
  let rand_name: String = thread_rng().gen_ascii_chars().take(8).collect();
  temp_dir().join(APP_DIR_NAME).join(rand_name)
}

fn get_png_files(path: &AsRef<Path>) -> Result<Vec<DirEntry>> {
  let png_files = read_dir(&path)?.filter(|it| it.is_ok()).map(|it| it.unwrap())
    .filter(|it| it.path().extension() == Some("png".as_ref()))
    .collect::<Vec<_>>();
  Ok(png_files)
}
