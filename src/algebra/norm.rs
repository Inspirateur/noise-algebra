use std::ops::RangeInclusive;
use ndarray::Array1;
use crate::noise::{Noise, NoiseSource};

#[derive(Clone)]
pub struct NoiseNorm<N: Noise>(N, f64);

impl<N: Noise> Noise for NoiseNorm<N> {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], steb_by: usize, seed: usize) -> Array1<f64> {
        // assumes a domain of type [-x; x] or [0; x] with x non-zero
        self.0.sample(ranges, steb_by, seed)/self.1
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let domain = self.0.domain();
        domain.start()/domain.end()..=1.
    }
}

impl<X: Noise> NoiseSource<X> {
    #[inline]
    pub fn normalize(self) -> NoiseSource<impl Noise> {
        let mut ampl = *(self.noise.domain().end());
        if ampl == 0. {
            ampl = 1.;
        }
        NoiseSource { noise: NoiseNorm(self.noise, ampl) }
    }
}