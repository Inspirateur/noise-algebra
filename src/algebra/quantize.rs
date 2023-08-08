use std::ops::RangeInclusive;
use ndarray::Array1;
use crate::noise::{Noise, NoiseSource};

#[derive(Clone)]
pub struct NoiseQuant<L: Noise>(L, f64);

impl<L: Noise> Noise for NoiseQuant<L> {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], steb_by: usize, seed: usize) -> Array1<f64> {
       let res = self.0.sample(ranges, steb_by, seed);
       (res*self.1).map(|v| v.floor())/self.1
    }

    fn domain(&self) -> RangeInclusive<f64> {
        self.0.domain()
    }
}

impl<X: Noise> NoiseSource<X> {
    #[inline]
    pub fn quantize(self, step: f64) -> NoiseSource<impl Noise> {
        let domain = self.noise.domain();
        NoiseSource { noise: NoiseQuant(self.noise, (domain.end()-domain.start())/step) }
    }
}