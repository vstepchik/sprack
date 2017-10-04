#[derive(Debug)]
pub struct Dimension { pub w: u32, pub h: u32 }

impl Dimension {
  pub fn new(w: u32, h: u32) -> Dimension {
    Dimension { w, h }
  }
  fn of_bounds(bounds: &Boundaries) -> Dimension {
    Dimension { w: bounds.r - bounds.l, h: bounds.b - bounds.t }
  }

  pub fn area(&self) -> u32 { self.w * self.h }
  pub fn perimeter(&self) -> u32 { self.w * 2 + self.h * 2 }

  fn fits(&self, dim: &Dimension) -> Fit {
    if self.w == dim.w && self.h == dim.h { return Fit::Perfectly; }
    if self.h == dim.w && self.w == dim.h { return Fit::PerfectlyFlipped; }
    if self.w <= dim.w && self.h <= dim.h { return Fit::Yes; }
    if self.h <= dim.w && self.w <= dim.h { return Fit::Flipped; }
    return Fit::No;
  }
}

#[derive(Debug)]
pub struct Rect {
  pub x: u32,
  pub y: u32,
  pub size: Dimension,
  pub flipped: bool,
}

impl Rect {
  pub fn area(&self) -> u32 { self.size.area() }
  pub fn perimeter(&self) -> u32 { self.size.perimeter() }
}

#[derive(Debug)]
pub struct Bin { pub size: Dimension, pub rectangles: Box<Vec<Rect>> }

#[derive(Debug)]
pub struct PackErr { pub msg: String }

#[derive(Debug)]
pub struct PackOptions {
  pub bin_size: Dimension,
  pub discard_step: u16,
}

impl Default for PackOptions {
  fn default() -> PackOptions {
    PackOptions {
      bin_size: Dimension::new(256, 256),
      discard_step: 128,
    }
  }
}

pub fn pack(rectangles: Box<Vec<Rect>>, options: PackOptions) -> Result<Vec<Bin>, PackErr> {
  println!("Packing rectangles {:?} with options {:?}", rectangles, options);
  Err(PackErr { msg: "Not implemented".to_string() })
}

/**** PRIVATE ****/


#[derive(Debug)]
struct Boundaries { l: u32, t: u32, r: u32, b: u32 }

enum Fit { No, Yes, Flipped, Perfectly, PerfectlyFlipped }

struct PNode { node: Option<Box<Node>>, fill: bool }

impl PNode {
  fn set(&mut self, l: u32, t: u32, r: u32, b: u32) {
    if let Some(ref mut node) = self.node {
      node.bounds = Boundaries { l, t, r, b };
      node.id = false;
    } else {
      self.node = Some(Box::new(Node::from_boundaries(l, t, r, b)))
    }
    self.fill = true;
  }
}

struct Node {
  children: (Option<Box<PNode>>, Option<Box<PNode>>),
  bounds: Boundaries,
  id: bool,
}

impl Node {
  fn from_boundaries(l: u32, t: u32, r: u32, b: u32) -> Node {
    Node { children: (None, None), id: false, bounds: Boundaries { l, t, r, b } }
  }

  fn insert(&mut self, rect: &mut Rect) -> Option<Node> {
    if let Some(ref mut child) = self.children.0 {
      if child.fill && child.node.is_some() {
        if let Some(ref mut node) = child.node {
          let result_node = node.insert(rect);
          if result_node.is_some() { return result_node; }
        }
        if let Some(ref mut child) = self.children.1 {
          if let Some(ref mut node) = child.node {
            return node.insert(rect);
          }
        }
      }
    }

    if self.id { return None; }

    let self_dim = Dimension::of_bounds(&self.bounds);
    match rect.size.fits(&self_dim) {
      Fit::No => { return None; }
      Fit::Yes => { rect.flipped = false; }
      Fit::Flipped => { rect.flipped = true; }
      Fit::Perfectly => {
        rect.flipped = false;
        return Some(self);
      }
      Fit::PerfectlyFlipped => {
        rect.flipped = true;
        return Some(self);
      }
    }

    let rect_w = if rect.flipped { rect.size.h } else { rect.size.w };
    let rect_h = if rect.flipped { rect.size.w } else { rect.size.h };

    if self_dim.w - rect_w > self_dim.h - rect_h {
      if let Some(ref mut child) = self.children.0 {
        child.set(self.bounds.l, self.bounds.t, self.bounds.l + rect_w, self.bounds.b);
      }
      if let Some(ref mut child) = self.children.1 {
        child.set(self.bounds.l + rect_w, self.bounds.t, self.bounds.l, self.bounds.b);
      }
    } else {
      if let Some(ref mut child) = self.children.0 {
        child.set(self.bounds.l, self.bounds.t, self.bounds.l, self.bounds.b + rect_h);
      }
      if let Some(ref mut child) = self.children.1 {
        child.set(self.bounds.l, self.bounds.t + rect_h, self.bounds.l, self.bounds.b);
      }
    }

    if let Some(ref mut child) = self.children.0 {
      if let Some(ref mut node) = child.node {
        return node.insert(rect);
      }
    }
    None
  }

  fn reset(&mut self, dim: &Dimension) {
    self.id = false;
    self.bounds = Boundaries { l: 0, t: 0, r: dim.w, b: dim.h };
    self.clear()
  }

  fn clear(&mut self) {
    fn clear_child(child: &mut Box<PNode>) {
      child.fill = false;
      if let Some(ref mut node) = child.node { node.clear() }
    }
    if let Some(ref mut child) = self.children.0 { clear_child(child); }
    if let Some(ref mut child) = self.children.1 { clear_child(child); }
  }
}
