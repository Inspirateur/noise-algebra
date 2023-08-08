# noise-algebra
Build your custom noise functions using plain algebra, and sample them however you like!

```rust
use noise_algebra::*;

fn main() {
    // Easily build your noise functions
    let ocean_threshold = 0.3;
    let elevation = (simplex(0.1) + simplex(0.5)*0.5 + simplex(1.)*0.1).normalize();
    // make temperature decrease with altitude
    let temperature = (simplex(0.2) - elevation.clone()*0.3).normalize();
    // make humidity sharply increase with proximity to ocean and decrease with temperature
    let humidity = temperature.clone() * (simplex(0.1) + elevation.clone().mask(ocean_threshold)).normalize();
    
    // Sample humidity in a 200x200 square centered on 0 with half precision (step_by = 2)
    let step_by = 2;
    let seed = 42;
    let samples = humidity.sample([-100..=100, -100..=100], step_by, seed);
    // All the individual noises used in the functions are given a different seed to avoid weird artefacts!
}
```

Designed for procedural generation, the API is focused on making noise building code easy to read and write while still being performant.  
*Big thanks to [Chris Janaqi](https://github.com/kokounet) for helping me with the implementation!*
## Benchmark
The same noise function is constructed using simd::simplex (from [simdnoise](https://crates.io/crates/simdnoise)), perlin (from [noise-rs](https://crates.io/crates/noise)) and a fake-noise function that just outputs a constant. We evaluate the sample speed of these functions for a grid of 32x32 points.

Results on my Intel(R) Xeon(R) CPU E5-1650 v3 (3.50GHz):
```
simd-generate-32^2      time:   [327.80 µs 328.66 µs 329.59 µs]
perlin-generate-32^2    time:   [508.20 µs 514.02 µs 522.55 µs]
const-generate-32^2     time:   [109.32 µs 109.99 µs 110.80 µs]
```

The current performances are not that great, because simd is not yet used for operations applied on the noises. 
The goal is to use simd everywhere but I haven't implemented it yet.

## Using your own noise source
You can easily use a custom base function by implementing the Noise trait which only has a sample and domain function.

```rust
pub trait Noise: Clone {
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], step_by: usize, seed: usize) -> Array1<f64>;

    fn domain(&self) -> RangeInclusive<f64>;
}
```

sample is expected to yield an unrolled array containing 1 sample for every coordinates in the provided ranges, and domain() defines the range of values that can be outputted by your custom noise, usually [-1; 1]. 

Check the doc or the implementation of the existing noise for more info!

## Todo
Work in progress:
- extend support to dimensions 1, 3 and 4
- use simd everywhere
- add more types of noise (cellular, gradient, etc.)
