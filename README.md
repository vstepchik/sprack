## Sprack

A tool for packing sprites/lightmaps/whatever textures into atlases, written in Rust.

Written while learning Rust. Inspired by [TeamHypersomnia/rectpack2D](https://github.com/TeamHypersomnia/rectpack2D)
and [@blackpawn's Packing Lightmaps](http://blackpawn.com/texts/lightmaps/default.html) text.

##### TODO
- [x] Base algorithm
- [x] Sprite flipping support (attempt fit rotated by 90°)
- [x] Heuristics
  - [x] Sort by area
  - [x] Sort by perimeter
  - [x] Sort by max side
  - [x] Sort by width
  - [x] Sort by height
  - [x] Sort by squareness * area
  - [x] Sort by squareness * perimeter
- [x] Demo rectangles packing, writing output images
- [ ] Picking best result
- [ ] Atlas trimming
- [ ] Reading&Writing actual sprites
- [ ] Command-line argument processing
- [ ] Add manual and more details to `README.md`
- [ ] Multi-threading
- [ ] Tests?
- [ ] Benchmarks?
