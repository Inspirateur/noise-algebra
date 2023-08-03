use std::ops::RangeInclusive;
use crate::noise::{Noise, NoiseSource};
pub struct NoisePowf<L: Noise>(L, f64);

impl<L: Noise> Noise for NoisePowf<L> {
    #[inline]
    fn sample(&self, x: f64, y: f64, seed: usize) -> f64 {
        let res = self.0.sample(x, y, seed);
        res.signum()*res.abs().powf(self.1)
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let domain = self.0.domain();
        let start = domain.start();
        let end = domain.end();
        let min = start.signum()*start.abs().powf(self.1);
        let max = end.signum()*end.abs().powf(self.1);
        min..=max
    }
}

impl<X: Noise> NoiseSource<X> {
    pub fn powf(self, f: f64) -> NoiseSource<impl Noise> {
        let noise = NoisePowf(self.noise, f);
        NoiseSource { domain: noise.domain(), noise }
    }
}