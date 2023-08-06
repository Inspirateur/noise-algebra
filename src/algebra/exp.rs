use std::ops::RangeInclusive;
use ndarray::Array1;
use crate::noise::{Noise, NoiseSource, NoiseRange};

#[derive(Clone)]
pub struct NoiseExp<L: Noise>(L);

impl<L: Noise> Noise for NoiseExp<L> {
    #[inline]
    fn sample<I: NoiseRange>(&self, x_range: I, y_range: I, seed: usize) -> Array1<f64> {
        self.0.sample(x_range, y_range, seed).map(|v| v.exp())
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let domain = self.0.domain();
        domain.start().exp()..=domain.end().exp()
    }
}

impl<X: Noise> NoiseSource<X> {
    pub fn exp(self) -> NoiseSource<impl Noise> {
        NoiseSource { noise: NoiseExp(self.noise) }
    }
}