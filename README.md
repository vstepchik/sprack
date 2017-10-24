## Sprack

A tool for packing sprites/lightmaps/whatever textures into atlases.

Written while learning Rust. Inspired by [TeamHypersomnia/rectpack2D](https://github.com/TeamHypersomnia/rectpack2D)
and [@blackpawn's Packing Lightmaps](http://blackpawn.com/texts/lightmaps/default.html) text.

Suggestions are welcome!

##### TODO
- [x] Base algorithm
- [x] Sprite flipping support (attempt fit rotated by 90°)
- [x] Heuristics
  - [x] Sort by area
  - [x] Sort by perimeter
  - [x] Sort by max side
  - [x] Sort by width
  - [x] Sort by height
  - [x] Sort by squareness × area
  - [x] Sort by squareness × perimeter
- [x] Demo rectangles packing, writing output images
- [x] Picking best result
- [x] Atlas compacting
- [x] Atlas trimming
- [x] Writing sprites
- [x] Split code into two crates (bin and lib)
- [ ] Make lib crate extensible with custom heuristics
- [ ] Command-line argument processing
- [ ] Add metadata output
  - [ ] JSON
  - [ ] YAML
  - [ ] RON?
  - [ ] Protobuf?
- [ ] Use logger instead of `println!`
- [ ] Add manual and more details to `README.md`
- [x] Multi-threading
- [ ] Tests?
- [ ] Benchmarks?
- [ ] Add optional border and padding?
- [ ] Refactor until feel pride
