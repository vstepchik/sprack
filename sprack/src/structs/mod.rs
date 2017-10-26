mod bin;
mod node;
mod options;

pub use self::bin::*;
pub use self::node::*;
pub use self::options::*;
use super::PackHeuristic;

use std::cmp::max;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PackInput { pub dim: Dimension, pub id: u32 }

#[derive(Debug)]
pub struct PackResult { pub bins: Vec<Bin>, pub heuristics: PackHeuristic }

#[derive(Debug)]
pub struct PackErr(pub &'static str);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dimension { pub w: u32, pub h: u32 }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rectangle {
  pub x: u32,
  pub y: u32,
  pub size: Dimension,
  pub flipped: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Placement {
  pub index: u32,
  pub rect: Rectangle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fit { No, Yes(bool), Exact(bool) } // bool is for `flipped`

// ===============================================================================================

impl Dimension {
  pub fn new(w: u32, h: u32) -> Dimension {
    Dimension { w, h }
  }

  pub fn fits(&self, inner: &Dimension) -> Fit {
    if self.w == inner.w && self.h == inner.h { return Fit::Exact(false); }
    if self.h == inner.w && self.w == inner.h { return Fit::Exact(true); }
    if self.w >= inner.w && self.h >= inner.h { return Fit::Yes(false); }
    if self.h >= inner.w && self.w >= inner.h { return Fit::Yes(true); }
    Fit::No
  }
}

impl Rectangle {
  pub fn t(&self) -> u32 { self.y }
  pub fn l(&self) -> u32 { self.x }
  pub fn b(&self) -> u32 { self.y + self.size.h }
  pub fn r(&self) -> u32 { self.x + self.size.w }
  pub fn non_flipped_size(&self) -> Dimension {
    if self.flipped { Dimension::new(self.size.h, self.size.w) } else { self.size }
  }
}
