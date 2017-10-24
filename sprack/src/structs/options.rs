use super::{PackHeuristic, Dimension};

use std::iter::FromIterator;
use std::collections::HashSet;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PackOptions<'a> {
  pub bin_size: Dimension,
  pub atlas_compact_steps: u8,
  pub flipping: bool,
  pub trim: bool,
  pub pack_heuristics: HashSet<&'a PackHeuristic>,
}

impl<'a> Default for PackOptions<'a> {
  fn default() -> PackOptions<'a> {
    PackOptions {
      bin_size: Dimension::new(512, 512),
      atlas_compact_steps: 0,
      flipping: false,
      trim: false,
      pack_heuristics: HashSet::from_iter(PackHeuristic::all()),
    }
  }
}
