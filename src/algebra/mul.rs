use std::ops::{RangeInclusive, Mul};
use ndarray::Array1;
use crate::noise::{Noise, NoiseSource, NoiseRange};

#[derive(Clone)]
pub struct NoiseMul<L, R>(L, R);

impl<L: Noise, R: Noise> Noise for NoiseMul<L, R> {
    #[inline]
    fn sample<I: NoiseRange>(&self, x_range: I, y_range: I, seed: usize) -> Array1<f64> {
        self.0.sample(x_range.clone(), y_range.clone(), seed.wrapping_mul(2))
            * self.1.sample(x_range, y_range, seed.wrapping_mul(2).wrapping_add(1))
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

impl<L: Noise> Noise for NoiseMul<L, f64> {
    #[inline]
    fn sample<I: NoiseRange>(&self, x_range: I, y_range: I, seed: usize) -> Array1<f64> {
        self.0.sample(x_range, y_range, seed.wrapping_mul(2))
            * self.1
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let ldomain = self.0.domain();
        let p1 = ldomain.start()*self.1;
        let p2 = ldomain.end()*self.1;
        let min = p1.min(p2);
        let max = p1.max(p2);
        min..=max
    }
}

impl<R: Noise> Noise for NoiseMul<f64, R> {
    #[inline]
    fn sample<I: NoiseRange>(&self, x_range: I, y_range: I, seed: usize) -> Array1<f64> {
        self.0
            * self.1.sample(x_range, y_range, seed.wrapping_mul(2))
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let rdomain = self.1.domain();
        let p1 = rdomain.start()*self.0;
        let p2 = rdomain.end()*self.0;
        let min = p1.min(p2);
        let max = p1.max(p2);
        min..=max
    }
}

impl<L: Noise, R: Noise> Mul<NoiseSource<R>> for NoiseSource<L> {
    type Output = NoiseSource<NoiseMul<L, R>>;

    #[inline]
    fn mul(self, rhs: NoiseSource<R>) -> Self::Output {
        NoiseSource { noise: NoiseMul(self.noise, rhs.noise) }
    }
}

impl<L: Noise> std::ops::Mul<f64> for NoiseSource<L> {
    type Output = NoiseSource<NoiseMul<L, f64>>;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        NoiseSource { noise: NoiseMul(self.noise, rhs) }
    }
}


impl<R: Noise> std::ops::Mul<NoiseSource<R>> for f64 {
    type Output = NoiseSource<NoiseMul<f64, R>>;

    #[inline]
    fn mul(self, rhs: NoiseSource<R>) -> Self::Output {
        NoiseSource { noise: NoiseMul(self, rhs.noise) }
    }
}
