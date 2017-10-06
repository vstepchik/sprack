mod algorithm;

pub use self::algorithm::Dimension;
pub use self::algorithm::Rectangle;
pub use self::algorithm::Bin;
pub use self::algorithm::Fit;

#[derive(Debug)]
pub struct PackResult { pub bins: Vec<Bin>, pub sorting_name: &'static str }

#[derive(Debug)]
pub struct PackErr(pub &'static str);

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


pub fn pack(rectangles: &[Dimension], options: PackOptions) -> Result<Vec<PackResult>, PackErr> {
  report_sizes();

  if rectangles.iter().any(|r| options.bin_size.fits(r) == Fit::No) {
    return Err(PackErr("Some pieces do not fit bin size"));
  }

  let mut results = Vec::new();
  for (sorting, name) in Dimension::comparison_modes() {
    let mut cloned = rectangles.to_owned();
    cloned.sort_unstable_by(sorting);

    let bins = pack_sorted(&cloned, options)?;
    results.push(PackResult { sorting_name: name, bins });
  }

  Ok(results)
}

fn pack_sorted(rectangles: &[Dimension], options: PackOptions) -> Result<Vec<Bin>, PackErr> {
  let mut bins: Vec<Bin> = vec![Bin::new(&options.bin_size)];
  for (idx, &rect) in rectangles.iter().enumerate() {
    let mut packed = false;
    for bin in &mut bins {
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
