extern crate rayon;

mod structs;
mod heuristics;

pub use structs::{Dimension, PackResult, PackErr, PackOptions, Bin};

use self::structs::*;
use self::heuristics::*;

use std::cmp::{min, max};
use rayon::prelude::*;


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
