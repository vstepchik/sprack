mod algorithm;

pub use self::algorithm::Dimension;
pub use self::algorithm::Rectangle;
pub use self::algorithm::Bin;
pub use self::algorithm::Fit;

#[derive(Debug)]
pub struct PackErr { pub msg: String }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PackOptions {
  pub bin_size: Dimension,
  pub flipping: bool,
}

impl Default for PackOptions {
  fn default() -> PackOptions {
    PackOptions {
      bin_size: Dimension::new(512, 512),
      flipping: false,
    }
  }
}


pub fn pack(rectangles: &Vec<Dimension>, options: PackOptions) -> Result<Vec<Bin>, PackErr> {
  report_sizes();
  // todo: sort rectangles

  if rectangles.iter().any(|r| options.bin_size.fits(r) == Fit::No) {
    return Err(PackErr { msg: "Some pieces do not fit bin size".to_string() });
  }

  let mut bins: Vec<Bin> = vec![Bin::new(&options.bin_size)];
  for (idx, &rect) in rectangles.iter().enumerate() {
    let mut packed = false;
    for mut bin in &mut bins {
      if bin.insert(&rect, idx as u32, options.flipping) { packed = true; }
    }
    if !packed {
      let mut new_bin = Bin::new(&options.bin_size);
      new_bin.insert(&rect, idx as u32, options.flipping);
      bins.push(new_bin);
    }
  }

  Ok(bins)
}

fn report_sizes() {
  use std::mem::size_of;
  use self::algorithm::Node;
  println!("---- Structure sizes ----");
  println!("Dimension: {}b", size_of::<Dimension>());
  println!("Rectangle: {}b", size_of::<Rectangle>());
  println!("Node: {}b", size_of::<Node>());
  println!("Bin: {}b", size_of::<Bin>());
  println!("Fit: {}b", size_of::<Fit>());
  println!();
}
