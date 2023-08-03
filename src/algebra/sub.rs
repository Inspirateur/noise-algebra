use std::ops::RangeInclusive;
use crate::noise::{Noise, NoiseSource};

pub struct NoiseSub<L: Noise, R: Noise>(L, R);

impl<L: Noise, R: Noise> Noise for NoiseSub<L, R> {
    #[inline]
    fn sample(&self, x: f64, y: f64, seed: usize) -> f64 {
        self.0.sample(x, y, seed.wrapping_mul(2))
            - self.1.sample(x, y, seed.wrapping_mul(2).wrapping_add(1))
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let ldomain = self.0.domain();
        let rdomain = self.1.domain();
        (ldomain.start()-rdomain.end())..=(ldomain.end()-rdomain.start())
    }
}

impl<L: Noise, R: Noise> std::ops::Sub<NoiseSource<R>> for NoiseSource<L> {
    type Output = NoiseSource<NoiseSub<L, R>>;

    #[inline]
    fn sub(self, rhs: NoiseSource<R>) -> Self::Output {
        let noise = NoiseSub(self.noise, rhs.noise);
        NoiseSource { 
            domain: noise.domain(),
            noise,
        }
    }
}

impl<L: Noise, R: Noise> std::ops::Sub<R> for NoiseSource<L> {
    type Output = NoiseSource<NoiseSub<L, R>>;

    #[inline]
    fn sub(self, rhs: R) -> Self::Output {
        let noise = NoiseSub(self.noise, rhs);
        NoiseSource { 
            domain: noise.domain(),
            noise,
        }
    }
}

impl<R: Noise> std::ops::Sub<NoiseSource<R>> for f64 {
    type Output = NoiseSource<NoiseSub<f64, R>>;

    #[inline]
    fn sub(self, rhs: NoiseSource<R>) -> Self::Output {
        let noise = NoiseSub(self, rhs.noise);
        NoiseSource { 
            domain: noise.domain(),
            noise,
        }
    }
}
