use super::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bin {
  pub size: Dimension,
  pub placements: Vec<Placement>,
  node: Box<Node>,
  last_rejected_size: Dimension,
}

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

    // reinsert all rectangles into bigger node
    let mut new_node = Node::new(&new_size);
    let mut placements = Vec::with_capacity(self.placements.len());
    for placement in &self.placements {
      if let Some(rect) = new_node.insert(&placement.rect.non_flipped_size(), placement.index, flipping_allowed) {
        placements.push(Placement { rect, index: placement.index });
      } else {
        // due to heuristics it sometimes happen that bigger node fails to fit rectangles the smaller one was able to
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
