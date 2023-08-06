use std::ops::{RangeInclusive, Sub};
use ndarray::Array1;
use crate::noise::{Noise, NoiseSource, NoiseRange};

#[derive(Clone)]
pub struct NoiseSub<L, R>(L, R);

impl<L: Noise, R: Noise> Noise for NoiseSub<L, R> {    
    #[inline]
    fn sample<I: NoiseRange>(&self, x_range: I, y_range: I, seed: usize) -> Array1<f64> {
        self.0.sample(x_range.clone(), y_range.clone(), seed.wrapping_mul(2))
            - self.1.sample(x_range, y_range, seed.wrapping_mul(2).wrapping_add(1))
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let ldomain = self.0.domain();
        let rdomain = self.1.domain();
        (ldomain.start()-rdomain.end())..=(ldomain.end()-rdomain.start())
    }
}

impl<L: Noise> Noise for NoiseSub<L, f64> {    
    #[inline]
    fn sample<I: NoiseRange>(&self, x_range: I, y_range: I, seed: usize) -> Array1<f64> {
        self.0.sample(x_range, y_range, seed)
            - self.1
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let ldomain = self.0.domain();
        (ldomain.start()-self.1)..=(ldomain.end()-self.1)
    }
}

impl<R: Noise> Noise for NoiseSub<f64, R> {    
    #[inline]
    fn sample<I: NoiseRange>(&self, x_range: I, y_range: I, seed: usize) -> Array1<f64> {
        self.0
            - self.1.sample(x_range, y_range, seed.wrapping_mul(2).wrapping_add(1))
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let rdomain = self.1.domain();
        (self.0-rdomain.end())..=(self.0-rdomain.start())
    }
}

impl<L: Noise, R: Noise> Sub<NoiseSource<R>> for NoiseSource<L> {
    type Output = NoiseSource<NoiseSub<L, R>>;

    #[inline]
    fn sub(self, rhs: NoiseSource<R>) -> Self::Output {
        NoiseSource { noise: NoiseSub(self.noise, rhs.noise) }
    }
}

impl<L: Noise> Sub<f64> for NoiseSource<L> {
    type Output = NoiseSource<NoiseSub<L, f64>>;

    #[inline]
    fn sub(self, rhs: f64) -> Self::Output {
        NoiseSource { noise: NoiseSub(self.noise, rhs) }
    }
}


impl<R: Noise> Sub<NoiseSource<R>> for f64 {
    type Output = NoiseSource<NoiseSub<f64, R>>;

    #[inline]
    fn sub(self, rhs: NoiseSource<R>) -> Self::Output {
        NoiseSource { noise: NoiseSub(self, rhs.noise) }
    }
}
