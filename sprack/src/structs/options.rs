use super::{SortHeuristic, Dimension};

pub struct PackOptions<'a> {
  pub bin_size: Dimension,
  pub atlas_compact_steps: u8,
  pub flipping: bool,
  pub trim: bool,
  pub sort_heuristics: &'a [&'a (SortHeuristic + Sync)],
}

impl<'a> Default for PackOptions<'a> {
  fn default() -> PackOptions<'a> {
    PackOptions {
      bin_size: Dimension::new(512, 512),
      atlas_compact_steps: 0,
      flipping: false,
      trim: false,
      sort_heuristics: &super::DEFAULT_HEURISTICS,
    }
  }
}
