use super::*;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Node {
  pub id: Option<u32>,
  pub bounds: Rectangle,
  pub child1: Option<Box<Node>>,
  pub child2: Option<Box<Node>>,
}

impl Node {
  pub fn new(size: &Dimension) -> Node {
    Node::from_rect(Rectangle { x: 0, y: 0, size: *size, flipped: false })
  }

  pub fn from_bound_box(l: u32, t: u32, r: u32, b: u32) -> Node {
    Node::from_rect(Rectangle { x: l, y: t, size: Dimension { w: r - l, h: b - t }, flipped: false })
  }

  pub fn from_rect(bounds: Rectangle) -> Node {
    Node { id: None, bounds, child1: None, child2: None }
  }

  pub fn insert(&mut self, rect: &Dimension, id: u32, flipping_allowed: bool) -> Option<Rectangle> {
    // attempt insert 1st child
    if let Some(ref mut child1) = self.child1 {
      let rect = child1.insert(rect, id, flipping_allowed);
      // if inserted - we're done
      if rect.is_some() { return rect; }
    }

    // attempt insert 2nd child
    if let Some(ref mut child2) = self.child2 {
      return child2.insert(rect, id, flipping_allowed);
    }

    // so it is leaf
    // return if there's already id
    if self.id.is_some() { return None; }

    let fit = self.bounds.size.fits(rect);

    match fit {
      // node is too small for rectangle - return nothing
      Fit::No => { return None; }

      // the node can fit the rectangle (maybe if we flip it by 90deg)
      Fit::Yes(flip) => if flip && !flipping_allowed { return None; } else { self.bounds.flipped = flip; }

      // the rectangle perfectly fits the node (maybe if we flip it by 90deg)
      Fit::Exact(flip) => {
        if flip && !flipping_allowed { return None; }
        self.id = Some(id);
        self.bounds.flipped = flip;
        return Some(self.bounds);
      }
    }

    let (w, h) = if self.bounds.flipped { (rect.h, rect.w) } else { (rect.w, rect.h) };

    let b = self.bounds;
    // decide to split node horizontally or vertically
    if self.bounds.size.w - w > self.bounds.size.h - h {
      // split horizontally [|]
      self.child1 = Some(Box::new(Node::from_bound_box(b.l(), b.t(), b.l() + w, b.b())));
      self.child2 = Some(Box::new(Node::from_bound_box(b.l() + w, b.t(), b.r(), b.b())));
    } else {
      // split vertically [-]
      self.child1 = Some(Box::new(Node::from_bound_box(b.l(), b.t(), b.r(), b.t() + h)));
      self.child2 = Some(Box::new(Node::from_bound_box(b.l(), b.t() + h, b.r(), b.b())));
    }

    if let Some(ref mut child1) = self.child1 {
      child1.insert(rect, id, flipping_allowed)
    } else {
      println!("no child1 !?!?!?");
      None
    }
  }
}
