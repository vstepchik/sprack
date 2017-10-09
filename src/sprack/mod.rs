mod algorithm;

use std::cmp::{Ordering, min, max};
use std::path::Path;
use std::collections::HashSet;

pub use self::algorithm::Dimension;
pub use self::algorithm::Rectangle;
pub use self::algorithm::Bin;
pub use self::algorithm::Fit;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PackInput { pub dim: Dimension, pub id: u32 }

#[derive(Debug)]
pub struct PackResult { pub bins: Vec<Bin>, pub sorting_name: &'static str }

#[derive(Debug)]
pub struct PackErr(pub &'static str);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PackHeuristic { Area, Perimeter, Side, Width, Height, SquarenessArea, SquarenessPerimeter }

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PackOptions<'a> {
  pub output_path: &'a Path,
  pub bin_size: Dimension,
  pub flipping: bool,
  pub pack_heuristics: HashSet<PackHeuristic>,
}

impl<'a> Default for PackOptions<'a> {
  fn default() -> PackOptions<'a> {
    PackOptions {
      output_path: Path::new("out"),
      bin_size: Dimension::new(512, 512),
      flipping: false,
      pack_heuristics: HashSet::new(),
    }
  }
}


pub fn pack(rectangles: &[Dimension], options: &PackOptions) -> Result<Vec<PackResult>, PackErr> {
  let dimension_bigger_than_bin = |r: &Dimension| match options.bin_size.fits(r) {
    Fit::No => { true }
    Fit::Yes(flip) | Fit::Exact(flip) => flip && !options.flipping
  };

  if rectangles.iter().any(dimension_bigger_than_bin) {
    return Err(PackErr("Some pieces do not fit bin size"));
  }

  let inputs = rectangles.iter().enumerate()
    .map(|(idx, dim)| { PackInput { id: idx as u32, dim: *dim } }).collect::<Vec<_>>();
  let mut results = Vec::new();
  for (sorting, name) in PackInput::comparison_modes() {
    let mut cloned = inputs.to_owned();
    cloned.sort_unstable_by(sorting);

    let bins = pack_sorted(&cloned, options)?;
    results.push(PackResult { sorting_name: name, bins });
  }

  Ok(results)
}

fn pack_sorted(rectangles: &[PackInput], options: &PackOptions) -> Result<Vec<Bin>, PackErr> {
  let mut bins: Vec<Bin> = vec![Bin::new(&options.bin_size)];
  for &input in rectangles {
    let mut packed = false;
    for bin in &mut bins {
      if bin.insert(&input.dim, input.id, options.flipping) { packed = true; }
    }
    if !packed {
      let mut new_bin = Bin::new(&options.bin_size);
      new_bin.insert(&input.dim, input.id, options.flipping);
      bins.push(new_bin);
    }
  }

  Ok(bins)
}


pub type FunctionReference = (fn(&PackInput, &PackInput) -> Ordering, &'static str);

impl PackInput {
  fn cmp_by_area(l: &PackInput, r: &PackInput) -> Ordering { (r.dim.w * r.dim.h).cmp(&(l.dim.w * l.dim.h)) }

  fn cmp_by_perimeter(l: &PackInput, r: &PackInput) -> Ordering { (r.dim.w + r.dim.h).cmp(&(l.dim.w + l.dim.h)) }

  fn cmp_by_max_side(l: &PackInput, r: &PackInput) -> Ordering { max(r.dim.w, r.dim.h).cmp(&max(l.dim.w, l.dim.h)) }

  fn cmp_by_w(l: &PackInput, r: &PackInput) -> Ordering { r.dim.w.cmp(&l.dim.w) }

  fn cmp_by_h(l: &PackInput, r: &PackInput) -> Ordering { r.dim.h.cmp(&l.dim.h) }

  fn cmp_by_squareness_area(l: &PackInput, r: &PackInput) -> Ordering {
    PackInput::sqa(&r.dim).partial_cmp(&PackInput::sqa(&l.dim)).unwrap_or(Ordering::Equal)
  }

  fn cmp_by_squareness_perimeter(l: &PackInput, r: &PackInput) -> Ordering {
    PackInput::sqp(&r.dim).partial_cmp(&PackInput::sqp(&l.dim)).unwrap_or(Ordering::Equal)
  }

  fn sq(d: &Dimension) -> f32 { min(d.w, d.h) as f32 / max(d.w, d.h) as f32 }

  fn sqa(d: &Dimension) -> f32 { PackInput::sq(d) * (d.w * d.h) as f32 }

  fn sqp(d: &Dimension) -> f32 { PackInput::sq(d) * (d.w + d.h) as f32 }

  fn comparison_modes() -> Vec<FunctionReference> {
    vec![
      (PackInput::cmp_by_area, "area"),
      (PackInput::cmp_by_perimeter, "perimeter"),
      (PackInput::cmp_by_max_side, "max_side"),
      (PackInput::cmp_by_w, "width"),
      (PackInput::cmp_by_h, "height"),
      (PackInput::cmp_by_squareness_area, "squareness_area"),
      (PackInput::cmp_by_squareness_perimeter, "squareness_perimeter"),
    ]
  }
}
