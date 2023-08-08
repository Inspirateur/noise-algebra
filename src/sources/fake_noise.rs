use std::ops::RangeInclusive;
use ndarray::Array1;
use crate::noise::{Noise, NoiseSource, len};

#[derive(Clone)]
struct FakeNoise { 
    value: f64
}

impl FakeNoise {
    fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Noise for FakeNoise {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], step_by: usize, _seed: usize) -> Array1<f64> {
        Array1::from_elem(
            ranges.iter()
                .map(|r| len(r.clone(), step_by))
                .fold(1, |a, b| a*b), 
            self.value
        )
    }

    fn domain(&self) -> std::ops::RangeInclusive<f64> {
        -1f64..=1f64
    }
}

#[inline]
pub fn fake_noise(value: f64) -> NoiseSource<impl Noise> {
    NoiseSource { noise: FakeNoise::new(value) }
}