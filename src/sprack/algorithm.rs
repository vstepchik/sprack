use std::cmp::max;

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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bin {
  pub size: Dimension,
  pub placements: Vec<Placement>,
  node: Box<Node>,
  last_rejected_size: Dimension,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Node {
  pub id: Option<u32>,
  pub bounds: Rectangle,
  pub child1: Option<Box<Node>>,
  pub child2: Option<Box<Node>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Fit { No, Yes(bool), Exact(bool) } // bool is for `flipped`


// ===============================================================================================

impl Bin {
  pub fn new(size: &Dimension) -> Bin {
    Bin {
      size: *size,
      placements: Vec::new(),
      node: Box::new(Node::new(size)),
      last_rejected_size: *size,
    }
  }

  pub fn insert(&mut self, rect: &Dimension, id: u32, flipping_allowed: bool) -> bool {
    // short-circuit if rect is bigger than last rejected one
    match self.last_rejected_size.fits(rect) {
      Fit::No => { return false; }
      Fit::Yes(flip) | Fit::Exact(flip) => if flip && !flipping_allowed { return false; }
    }

    if let Some(rect) = self.node.insert(rect, id, flipping_allowed) {
      self.placements.push(Placement { rect, index: id });
      true
    } else {
      self.last_rejected_size = *rect;
      false
    }
  }

  pub fn resize(&mut self, new_size: Dimension, flipping_allowed: bool) -> bool {
    let new_size = Dimension { w: max(1, new_size.w), h: max(1, new_size.h) };

    let mut new_node = Node::new(&new_size);
    let mut placements = Vec::with_capacity(self.placements.len());
    for placement in &self.placements {
      if let Some(rect) = new_node.insert(&placement.rect.non_flipped_size(), placement.index, flipping_allowed) {
        placements.push(Placement { rect, index: placement.index });
      } else {
        return false;
      }
    }
    self.node = Box::new(new_node);
    self.placements = placements;
    self.size = new_size;
    self.last_rejected_size = new_size;
    true
  }
}

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

impl Node {
  fn new(size: &Dimension) -> Node {
    Node::from_rect(Rectangle { x: 0, y: 0, size: *size, flipped: false })
  }

  fn from_bound_box(l: u32, t: u32, r: u32, b: u32) -> Node {
    Node::from_rect(Rectangle { x: l, y: t, size: Dimension { w: r - l, h: b - t }, flipped: false })
  }

  fn from_rect(bounds: Rectangle) -> Node {
    Node { id: None, bounds, child1: None, child2: None }
  }

  fn insert(&mut self, rect: &Dimension, id: u32, flipping_allowed: bool) -> Option<Rectangle> {
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
