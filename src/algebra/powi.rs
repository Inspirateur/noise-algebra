use std::ops::RangeInclusive;
use crate::noise::{Noise, NoiseSource};
pub struct NoisePowi<L: Noise>(L, i32);

impl<L: Noise> Noise for NoisePowi<L> {
    #[inline]
    fn sample(&self, x: f64, y: f64, seed: usize) -> f64 {
        self.0.sample(x, y, seed).powi(self.1)
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
    pub fn powi(self, i: i32) -> NoiseSource<impl Noise> {
        let noise = NoisePowi(self.noise, i);
        NoiseSource { domain: noise.domain(), noise }
    }
}