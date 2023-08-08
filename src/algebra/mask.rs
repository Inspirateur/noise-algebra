use std::ops::RangeInclusive;
use ndarray::Array1;
use crate::noise::{Noise, NoiseSource};

#[derive(Clone)]
pub struct NoiseMask<L: Noise>(L, f64);

impl<L: Noise> Noise for NoiseMask<L> {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], steb_by: usize, seed: usize) -> Array1<f64> {
        1.0 / (1.0 + (-16. * (self.0.sample(ranges, steb_by, seed) - self.1)).map(|v| v.exp()))
    }

    fn domain(&self) -> RangeInclusive<f64> {
        0f64..=1f64
    }
}

impl<X: Noise> NoiseSource<X> {
    #[inline]
    pub fn mask(self, t: f64) -> NoiseSource<impl Noise> {
        NoiseSource { noise: NoiseMask(self.noise, 1. - 2.*t) }
    }
}