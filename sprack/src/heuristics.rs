use super::{Dimension, PackInput};

use std::slice::Iter;
use std::cmp::{Ordering, max};


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PackHeuristic { Area, Perimeter, Side, Width, Height, SquarenessArea, SquarenessPerimeter }

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
