use std::ops::RangeInclusive;
use crate::noise::{Noise, NoiseSource};
pub struct NoiseMul<L: Noise, R: Noise>(L, R);

impl<L: Noise, R: Noise> Noise for NoiseMul<L, R> {
    #[inline]
    fn sample(&self, x: f64, y: f64, seed: usize) -> f64 {
        self.0.sample(x, y, seed.wrapping_mul(2))
            * self.1.sample(x, y, seed.wrapping_mul(2).wrapping_add(1))
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let ldomain = self.0.domain();
        let rdomain = self.1.domain();
        let p1 = ldomain.start()*rdomain.start();
        let p2 = ldomain.start()*rdomain.end();
        let p3 = ldomain.end()*rdomain.start();
        let p4 = ldomain.end()*rdomain.end();
        let min = p1.min(p2).min(p3).min(p4);
        let max = p1.max(p2).max(p3).max(p4);
        min..=max
    }
}

impl<L: Noise, R: Noise> std::ops::Mul<NoiseSource<R>> for NoiseSource<L> {
    type Output = NoiseSource<NoiseMul<L, R>>;

    #[inline]
    fn mul(self, rhs: NoiseSource<R>) -> Self::Output {
        let noise = NoiseMul(self.noise, rhs.noise);
        NoiseSource { 
            domain: noise.domain(),
            noise,
        }
    }
}

impl<L: Noise, R: Noise> std::ops::Mul<R> for NoiseSource<L> {
    type Output = NoiseSource<NoiseMul<L, R>>;

    #[inline]
    fn mul(self, rhs: R) -> Self::Output {
        let noise = NoiseMul(self.noise, rhs);
        NoiseSource { 
            domain: noise.domain(),
            noise,
        }
    }
}

impl<R: Noise> std::ops::Mul<NoiseSource<R>> for f64 {
    type Output = NoiseSource<NoiseMul<f64, R>>;

    #[inline]
    fn mul(self, rhs: NoiseSource<R>) -> Self::Output {
        let noise = NoiseMul(self, rhs.noise);
        NoiseSource { 
            domain: noise.domain(),
            noise,
        }
    }
}
