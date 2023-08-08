use std::ops::RangeInclusive;
use ndarray::Array1;

use crate::noise::{Noise, NoiseSource};

#[derive(Clone)]
pub struct NoiseNot<N: Noise>(N);

impl<N: Noise> Noise for NoiseNot<N> {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], steb_by: usize, seed: usize) -> Array1<f64> {
        let domain = self.domain();
        let start = domain.start();
        let end = domain.end();
        (*end + *start) - self.0.sample(ranges, steb_by, seed)
    }

    fn domain(&self) -> RangeInclusive<f64> {
        self.0.domain()
    }
}

impl<N: Noise> std::ops::Not for NoiseSource<N> {
    type Output = NoiseSource<NoiseNot<N>>;

    #[inline]
    fn not(self) -> Self::Output {
        NoiseSource { noise: NoiseNot(self.noise) }
    }
}
