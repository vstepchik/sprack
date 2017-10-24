use super::{PackHeuristic, Dimension};

use std::path::Path;
use std::iter::FromIterator;
use std::collections::HashSet;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PackOptions<'a> {
  pub output_path: &'a Path,
  pub bin_size: Dimension,
  pub atlas_compact_steps: u8,
  pub flipping: bool,
  pub trim: bool,
  pub keep_work_dir: bool,
  pub pack_heuristics: HashSet<&'a PackHeuristic>,
}

impl<'a> Default for PackOptions<'a> {
  fn default() -> PackOptions<'a> {
    PackOptions {
      output_path: Path::new("out"),
      bin_size: Dimension::new(512, 512),
      atlas_compact_steps: 0,
      flipping: false,
      trim: false,
      keep_work_dir: false,
      pack_heuristics: HashSet::from_iter(PackHeuristic::all()),
    }
  }
}
