use std::ops::RangeInclusive;
use ndarray::Array1;
use crate::noise::{Noise, NoiseSource};

#[derive(Clone)]
pub struct NoisePowi<L: Noise>(L, i32);

impl<L: Noise> Noise for NoisePowi<L> {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], steb_by: usize, seed: usize) -> Array1<f64> {
        self.0.sample(ranges, steb_by, seed).map(|v| v.powi(self.1))
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let domain = self.0.domain();
        let start = domain.start();
        let end = domain.end();
        let min = start.powi(self.1);
        let max = end.powi(self.1);
        let crosses_zero = start*end < 0.;
        if crosses_zero && self.1 % 2 == 0 {
            0f64..=max
        } else {
            min..=max
        }
    }
}

impl<X: Noise> NoiseSource<X> {
    #[inline]
    pub fn powi(self, i: i32) -> NoiseSource<impl Noise> {
        NoiseSource { noise: NoisePowi(self.noise, i) }
    }
}