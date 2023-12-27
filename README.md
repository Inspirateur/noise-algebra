# noise-algebra
Build your custom noise functions using plain algebra, and sample them however you like!

```rust
use noise_algebra::*;

fn main() {
    // prepare a noise source object;
    // it will yield samples for a 200x200 square centered on 0 with half precision (step_by = 2)
    let step_by = 2;
    let seed = 42;
    let n = NoiseSource::new([-100..=100, -100..=100], seed, step_by)
    // Easily compose your noise
    let ocean_threshold = 0.3;
    let elevation = (n.simplex(0.1) + n.simplex(0.5)*0.5 + n.simplex(1.)*0.1).normalize();
    // make temperature decrease with altitude
    let temperature = (n.simplex(0.2) - elevation.clone()*0.3).normalize();
    // make humidity sharply increase with proximity to ocean and decrease with temperature
    let humidity = temperature.clone() * (n.simplex(0.1) + elevation.clone().mask(ocean_threshold)).normalize();
    // All the individual noises used in the functions are given a different seed to avoid weird artefacts;
    // and the noise samples keep track of their range so that normalize magically works!
}
```

Designed for procedural generation, the API is focused on making noise building code easy to read and write while still being performant.

*Big thanks to [Chris Janaqi](https://github.com/kokounet) for helping me with the implementation!*
## Benchmark
The same noise is constructed using simd::simplex (from [simdnoise](https://crates.io/crates/simdnoise)) and a fake-noise function that just outputs a constant. 
We evaluate the sample speed of these functions for a grid of 32x32 points.

Results on my Intel(R) Xeon(R) CPU E5-1650 v3 (3.50GHz):
```
simplex-generate-32^2      time:   [93.603 µs 94.057 µs 94.697 µs]
const-generate-32^2     time:   [42.411 µs 43.501 µs 44.717 µs]
```

The current performances could be better, simd is not yet used for operations applied on the noises. 
The goal is to use simd everywhere but I haven't implemented it yet.

## Todo
Work in progress:
- extend support to dimensions 1, 3 and 4
- use simd everywhere
- add more types of noise (cellular, gradient, etc.)
