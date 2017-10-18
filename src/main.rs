extern crate image;

mod sprack;

use sprack::*;
use std::path::Path;

fn main() {
  let samples = generate_rectangles(200);
  let rectangles = samples.iter().map(|s| Dimension { w: s.width(), h: s.height() }).collect::<Vec<_>>();
  let options = PackOptions { flipping: true, trim: true, bin_size: Dimension { w: 256, h: 256 }, ..Default::default() };

  draw_samples(&options.output_path, &samples);

  let best = match pack(&rectangles, &options) {
    Ok(solutions) => {
      let mut best: Option<(PackResult, u64)> = None;
      for solution in solutions {
        let dir = Path::new(&options.output_path).join(&solution.heuristics.get().1);
        std::fs::remove_dir_all(&dir).unwrap_or(());
        std::fs::create_dir_all(&dir).expect(format!("Failed to create dir {:?}", &dir).as_ref());
        let mut size = 0;
        for (bin_number, bin) in solution.bins.iter().enumerate() {
          size += draw_bin(&dir.join(bin_number.to_string()).with_extension("png"), &samples, bin, options.trim);
        }
        println!("Heuristic {}: {} bins used, total size: {}b", solution.heuristics.get().1, solution.bins.len(), size);
        if best.is_none() || size < best.as_ref().unwrap().1 { best = Some((solution, size)); }
      }
      best
    }
    Err(e) => {
      eprintln!("Error: {:?}", e);
      None
    }
  };

  if let Some(best) = best {
    println!("Best results with {:?}, {} bytes total", (best.0).heuristics, best.1);
    // todo: copy as "best"
  }
}
