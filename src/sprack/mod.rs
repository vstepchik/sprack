mod algorithm;
mod drawing;
mod demo;

use std::cmp::{Ordering, max};
use std::path::Path;
use std::slice::Iter;
use std::iter::FromIterator;
use std::collections::HashSet;
use rayon::prelude::*;

pub use self::algorithm::{Dimension, Rectangle, Bin, Fit};
pub use self::drawing::draw_bin;
pub use self::demo::{draw_samples, generate_rectangles};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PackInput { pub dim: Dimension, pub id: u32 }

#[derive(Debug)]
pub struct PackResult { pub bins: Vec<Bin>, pub heuristics: PackHeuristic }

#[derive(Debug)]
pub struct PackErr(pub &'static str);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PackHeuristic { Area, Perimeter, Side, Width, Height, SquarenessArea, SquarenessPerimeter }

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PackOptions<'a> {
  pub output_path: &'a Path,
  pub bin_size: Dimension,
  pub atlas_compact_attempts: u8,
  pub flipping: bool,
  pub trim: bool,
  pub pack_heuristics: HashSet<&'a PackHeuristic>,
}

impl<'a> Default for PackOptions<'a> {
  fn default() -> PackOptions<'a> {
    PackOptions {
      output_path: Path::new("out"),
      bin_size: Dimension::new(512, 512),
      atlas_compact_attempts: 0,
      flipping: false,
      trim: false,
      pack_heuristics: HashSet::from_iter(PackHeuristic::all()),
    }
  }
}


pub fn pack(rectangles: &[Dimension], options: &PackOptions) -> Result<Vec<PackResult>, PackErr> {
  if options.pack_heuristics.is_empty() { return Err(PackErr("No heuristics supplied")); };

  let dimension_bigger_than_bin = |r: &Dimension| match options.bin_size.fits(r) {
    Fit::No => { true }
    Fit::Yes(flip) | Fit::Exact(flip) => flip && !options.flipping
  };

  if rectangles.iter().any(dimension_bigger_than_bin) {
    return Err(PackErr("Some pieces do not fit bin size"));
  }

  let inputs = rectangles.iter().enumerate()
    .map(|(idx, dim)| { PackInput { id: idx as u32, dim: *dim } }).collect::<Vec<_>>();

  let results: Vec<PackResult> = options.pack_heuristics.par_iter()
    .map(|&h| {
      let mut cloned = inputs.to_owned();
      cloned.sort_unstable_by(h.get().0);

      let bins = pack_sorted(&cloned, options).unwrap();
      PackResult { heuristics: *h, bins }
    })
    .collect::<Vec<_>>();

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


pub type HeuristicReference = (fn(&PackInput, &PackInput) -> Ordering, &'static str);

impl PackHeuristic {
  fn cmp_by_area(l: &PackInput, r: &PackInput) -> Ordering { (r.dim.w * r.dim.h).cmp(&(l.dim.w * l.dim.h)) }

  fn cmp_by_perimeter(l: &PackInput, r: &PackInput) -> Ordering { (r.dim.w + r.dim.h).cmp(&(l.dim.w + l.dim.h)) }

  fn cmp_by_max_side(l: &PackInput, r: &PackInput) -> Ordering { max(r.dim.w, r.dim.h).cmp(&max(l.dim.w, l.dim.h)) }

  fn cmp_by_w(l: &PackInput, r: &PackInput) -> Ordering { r.dim.w.cmp(&l.dim.w) }

  fn cmp_by_h(l: &PackInput, r: &PackInput) -> Ordering { r.dim.h.cmp(&l.dim.h) }

  fn cmp_by_squareness_area(l: &PackInput, r: &PackInput) -> Ordering {
    PackHeuristic::sqa(&r.dim).partial_cmp(&PackHeuristic::sqa(&l.dim)).unwrap_or(Ordering::Equal)
  }

  fn cmp_by_squareness_perimeter(l: &PackInput, r: &PackInput) -> Ordering {
    PackHeuristic::sqp(&r.dim).partial_cmp(&PackHeuristic::sqp(&l.dim)).unwrap_or(Ordering::Equal)
  }

  fn sq(d: &Dimension) -> f32 { if d.w < d.h { d.w as f32 / d.h as f32 } else { d.h as f32 / d.w as f32 } }

  fn sqa(d: &Dimension) -> f32 { PackHeuristic::sq(d) * (d.w * d.h) as f32 }

  fn sqp(d: &Dimension) -> f32 { PackHeuristic::sq(d) * (d.w + d.h) as f32 }

  pub fn get(&self) -> HeuristicReference {
    use PackHeuristic::*;
    match *self {
      Area => (PackHeuristic::cmp_by_area, "area"),
      Perimeter => (PackHeuristic::cmp_by_perimeter, "perimeter"),
      Side => (PackHeuristic::cmp_by_max_side, "side"),
      Width => (PackHeuristic::cmp_by_w, "width"),
      Height => (PackHeuristic::cmp_by_h, "height"),
      SquarenessArea => (PackHeuristic::cmp_by_squareness_area, "squareness_area"),
      SquarenessPerimeter => (PackHeuristic::cmp_by_squareness_perimeter, "squareness_perimeter"),
    }
  }

  fn all() -> Iter<'static, PackHeuristic> {
    use PackHeuristic::*;
    static HEURISTICS: [PackHeuristic; 7] = [Area, Perimeter, Side, Width, Height, SquarenessArea, SquarenessPerimeter];
    HEURISTICS.into_iter()
  }
}
