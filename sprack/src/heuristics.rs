use super::{Dimension, PackInput};

use std::fmt::{Debug, Result, Formatter};
use std::cmp::{Ordering, PartialOrd, max};

pub const ALL: [&(SortHeuristic + Sync); 7] = [
  &AreaSort,
  &PerimeterSort,
  &SideSort,
  &WidthSort,
  &HeightSort,
  &SquarenessByAreaSort,
  &SquarenessByPerimeterSort,
];

pub struct AreaSort;

pub struct PerimeterSort;

pub struct SideSort;

pub struct WidthSort;

pub struct HeightSort;

pub struct SquarenessByAreaSort;

pub struct SquarenessByPerimeterSort;

pub trait SortHeuristic: Sync {
  fn name(&self) -> &'static str;
  fn cmp(&self, l: &PackInput, r: &PackInput) -> Ordering;
}

impl SortHeuristic for AreaSort {
  fn name(&self) -> &'static str { "area" }
  fn cmp(&self, l: &PackInput, r: &PackInput) -> Ordering { cmp_by_key(l, r, |d| d.w * d.h) }
}

impl SortHeuristic for PerimeterSort {
  fn name(&self) -> &'static str { "perimeter" }
  fn cmp(&self, l: &PackInput, r: &PackInput) -> Ordering { cmp_by_key(l, r, |d| d.w + d.h) }
}

impl SortHeuristic for SideSort {
  fn name(&self) -> &'static str { "side" }
  fn cmp(&self, l: &PackInput, r: &PackInput) -> Ordering { cmp_by_key(l, r, |d| max(d.w, d.h)) }
}

impl SortHeuristic for WidthSort {
  fn name(&self) -> &'static str { "width" }
  fn cmp(&self, l: &PackInput, r: &PackInput) -> Ordering {
    cmp_by_key(l, r, |d| d.w)
  }
}

impl SortHeuristic for HeightSort {
  fn name(&self) -> &'static str { "height" }
  fn cmp(&self, l: &PackInput, r: &PackInput) -> Ordering {
    cmp_by_key(l, r, |d| d.h)
  }
}

impl SortHeuristic for SquarenessByAreaSort {
  fn name(&self) -> &'static str { "squareness_area" }
  fn cmp(&self, l: &PackInput, r: &PackInput) -> Ordering { cmp_by_key(l, r, |d| sqa(d)) }
}

impl SortHeuristic for SquarenessByPerimeterSort {
  fn name(&self) -> &'static str { "squareness_perimeter" }
  fn cmp(&self, l: &PackInput, r: &PackInput) -> Ordering { cmp_by_key(l, r, |d| sqp(d)) }
}

fn squareness(d: &Dimension) -> f32 {
  if d.w < d.h { d.w as f32 / d.h as f32 } else { d.h as f32 / d.w as f32 }
}

fn sqa(d: &Dimension) -> f32 { squareness(d) * (d.w * d.h) as f32 }

fn sqp(d: &Dimension) -> f32 { squareness(d) * (d.w + d.h) as f32 }

fn cmp_by_key<F, T: PartialOrd>(l: &PackInput, r: &PackInput, key: F) -> Ordering
  where F: Fn(&Dimension) -> T {
  key(&l.dim).partial_cmp(&key(&r.dim)).unwrap_or(Ordering::Equal)
}

impl Debug for SortHeuristic {
  fn fmt(&self, f: &mut Formatter) -> Result { write!(f, "{}", self.name()) }
}
