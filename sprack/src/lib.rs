extern crate rayon;

mod structs;

use std::cmp::{Ordering, min, max};
use std::path::Path;
use std::slice::Iter;
use std::iter::FromIterator;
use std::collections::HashSet;
use rayon::prelude::*;

pub use self::structs::{Dimension, Rectangle, Bin, Fit};

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

      let bins = pack_sorted(&cloned, options);
      PackResult { heuristics: *h, bins }
    })
    .collect::<Vec<_>>();

  Ok(results)
}

fn pack_sorted(rectangles: &[PackInput], options: &PackOptions) -> Vec<Bin> {
  let mut bins: Vec<Bin> = vec![new_bin(&options)];
  let insert_fn: &'static Fn(&mut Bin, &Dimension, u32, &PackOptions) -> bool =
    if options.atlas_compact_steps == 0 { &try_insert } else { &try_insert_with_growth };

  for &input in rectangles {
    let packed = bins.iter_mut().any(|bin| insert_fn(bin, &input.dim, input.id, &options));
    if !packed {
      let mut new_bin = new_bin(&options);
      new_bin.insert(&input.dim, input.id, options.flipping);
      bins.push(new_bin);
    }
  }
  bins
}

fn new_bin(options: &PackOptions) -> Bin {
  if options.atlas_compact_steps == 0 { return Bin::new(&options.bin_size); }
  let size_divisor = options.atlas_compact_steps as u32 + 1;
  let div_side = |val: u32| { max(1, val / size_divisor) };
  Bin::new(&Dimension::new(div_side(options.bin_size.w), div_side(options.bin_size.h)))
}

fn try_insert(bin: &mut Bin, rect: &Dimension, id: u32, options: &PackOptions) -> bool {
  bin.insert(&rect, id, options.flipping)
}

fn try_insert_with_growth(bin: &mut Bin, rect: &Dimension, id: u32, options: &PackOptions) -> bool {
  let size_inc = |val: u32| { max(1, val / (options.atlas_compact_steps + 1) as u32) };
  let mut current_size = bin.size.clone();
  while !bin.insert(&rect, id, options.flipping) {
    if current_size.w >= options.bin_size.w && current_size.h >= options.bin_size.h { return false; }
    current_size = Dimension::new(
      min(current_size.w + size_inc(options.bin_size.w), options.bin_size.w),
      min(current_size.h + size_inc(options.bin_size.h), options.bin_size.h),
    );
    if !bin.resize(current_size, options.flipping) { continue }
  }
  true
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

  pub fn all() -> Iter<'static, PackHeuristic> {
    use PackHeuristic::*;
    static HEURISTICS: [PackHeuristic; 7] = [Area, Perimeter, Side, Width, Height, SquarenessArea, SquarenessPerimeter];
    HEURISTICS.into_iter()
  }
}
