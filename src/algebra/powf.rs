use std::ops::RangeInclusive;
use crate::noise::{Noise, NoiseSource, NoiseRange};
use ndarray::Array1;

#[derive(Clone)]
pub struct NoisePowf<L: Noise>(L, f64);

impl<L: Noise> Noise for NoisePowf<L> {
    #[inline]
    fn sample<I: NoiseRange>(&self, x_range: I, y_range: I, seed: usize) -> Array1<f64> {
        let res = self.0.sample(x_range, y_range, seed);
        res.map(|v| v.signum()*v.abs().powf(self.1))
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
        NoiseSource { noise: NoisePowf(self.noise, f) }
    }
}