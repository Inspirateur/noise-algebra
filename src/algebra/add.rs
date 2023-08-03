use std::ops::RangeInclusive;
use crate::noise::{Noise, NoiseSource};

pub struct NoiseAdd<L: Noise, R: Noise>(L, R);

impl<L: Noise, R: Noise> Noise for NoiseAdd<L, R> {
    #[inline]
    fn sample(&self, x: f64, y: f64, seed: usize) -> f64 {
        self.0.sample(x, y, seed.wrapping_mul(2))
            + self.1.sample(x, y, seed.wrapping_mul(2).wrapping_add(1))
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let ldomain = self.0.domain();
        let rdomain = self.1.domain();
        (ldomain.start()+rdomain.start())..=(ldomain.end()+rdomain.end())
    }
}

impl<L: Noise, R: Noise> std::ops::Add<NoiseSource<R>> for NoiseSource<L> {
    type Output = NoiseSource<NoiseAdd<L, R>>;

    #[inline]
    fn add(self, rhs: NoiseSource<R>) -> Self::Output {
        let noise = NoiseAdd(self.noise, rhs.noise);
        NoiseSource { 
            domain: noise.domain(),
            noise,
        }
    }
}

impl<L: Noise, R: Noise> std::ops::Add<R> for NoiseSource<L> {
    type Output = NoiseSource<NoiseAdd<L, R>>;

    #[inline]
    fn add(self, rhs: R) -> Self::Output {
        let noise = NoiseAdd(self.noise, rhs);
        NoiseSource { 
            domain: noise.domain(),
            noise,
        }
    }
}

impl<R: Noise> std::ops::Add<NoiseSource<R>> for f64 {
    type Output = NoiseSource<NoiseAdd<f64, R>>;

    #[inline]
    fn add(self, rhs: NoiseSource<R>) -> Self::Output {
        let noise = NoiseAdd(self, rhs.noise);
        NoiseSource { 
            domain: noise.domain(),
            noise,
        }
    }
}
