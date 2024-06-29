# Physically Based Pathtracer in Rust
Basic CPU pathtracer written from scratch in Rust.  
Written first from [Peter Shirley's book](https://raytracing.github.io/) and expanded ased on the [PBR Book](https://www.pbr-book.org/) and [Eric Veach's paper](https://graphics.stanford.edu/papers/veach_thesis/thesis.pdf). 

Suzanne rendered with a low sample rate
![Image of blender suzanne](https://imgur.com/66zPhKM.png)
## Compile and Run
[Get the Rust toolchain with cargo](https://www.rust-lang.org/learn/get-started).

Then, run `cargo run --release` (do not use the `debug` target as it is unoptimised and render time is 10x as long).  
This will generate an `image.ppm` file which you can view. In the future a window with egui will be implemented.
## TODO
- [x] Multithreading <- with rayon
    - [ ] Implement with `std::thread`s
- [ ] Write more idiomatic Rust code
    - [ ] Replace `Interval` with inbuilt `Range`
- [x] BufWrite to file
- [x] Triangle-ray intersection
- [ ] Soft shadows
- [ ] Accelerating structures
    - [ ] Bounding Volume Hierarchies
## Future Goals
- Wavefront GPU support with OpenCL
- Importance Sampling
- Simple Lua/RhaiScript interface for writing scenes to render
