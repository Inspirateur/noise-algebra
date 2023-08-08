use std::ops::RangeInclusive;
use ndarray::Array1;
use crate::noise::{Noise, NoiseSource};

#[derive(Clone)]
pub struct NoiseExp<L: Noise>(L);

impl<L: Noise> Noise for NoiseExp<L> {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], steb_by: usize, seed: usize) -> Array1<f64> {
        self.0.sample(ranges, steb_by, seed).map(|v| v.exp())
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let domain = self.0.domain();
        domain.start().exp()..=domain.end().exp()
    }
}

impl<X: Noise> NoiseSource<X> {
    #[inline]
    pub fn exp(self) -> NoiseSource<impl Noise> {
        NoiseSource { noise: NoiseExp(self.noise) }
    }
}