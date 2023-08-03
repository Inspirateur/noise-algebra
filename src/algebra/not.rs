use std::ops::RangeInclusive;
use crate::noise::{Noise, NoiseSource};

pub struct NoiseNot<N: Noise>(N);

impl<N: Noise> Noise for NoiseNot<N> {
    #[inline]
    fn sample(&self, x: f64, y: f64, seed: usize) -> f64 {
        let domain = self.domain();
        let start = domain.start();
        let end = domain.end();
        end - self.0.sample(x, y, seed) + start
    }

    fn domain(&self) -> RangeInclusive<f64> {
        self.0.domain()
    }
}

impl<N: Noise> std::ops::Not for NoiseSource<N> {
    type Output = NoiseSource<NoiseNot<N>>;

    #[inline]
    fn not(self) -> Self::Output {
        let noise = NoiseNot(self.noise);
        NoiseSource { 
            domain: noise.domain(),
            noise,
        }
    }
}
