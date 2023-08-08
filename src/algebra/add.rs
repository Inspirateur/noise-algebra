use std::ops::RangeInclusive;
use ndarray::Array1;
use crate::noise::{Noise, NoiseSource};

#[derive(Clone)]
pub struct NoiseAdd<L, R>(L, R);

impl<L: Noise, R: Noise> Noise for NoiseAdd<L, R> {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], step_by: usize, seed: usize) -> Array1<f64> {
        self.0.sample(ranges.clone(), step_by, seed.wrapping_mul(2))
            + self.1.sample(ranges, step_by, seed.wrapping_mul(2).wrapping_add(1))
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let ldomain = self.0.domain();
        let rdomain = self.1.domain();
        (ldomain.start()+rdomain.start())..=(ldomain.end()+rdomain.end())
    }
}

impl<L: Noise> Noise for NoiseAdd<L, f64> {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], step_by: usize, seed: usize) -> Array1<f64> {
        self.0.sample(ranges, step_by, seed)
            + self.1
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let ldomain = self.0.domain();
        (ldomain.start()+self.1)..=(ldomain.end()+self.1)
    }
}

impl<R: Noise> Noise for NoiseAdd<f64, R> {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], step_by: usize, seed: usize) -> Array1<f64> {
        self.0
            + self.1.sample(ranges, step_by, seed)
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let rdomain = self.1.domain();
        (self.0+rdomain.start())..=(self.0+rdomain.end())
    }
}

impl<L: Noise, R: Noise> std::ops::Add<NoiseSource<R>> for NoiseSource<L> {
    type Output = NoiseSource<NoiseAdd<L, R>>;
    #[inline]
    fn add(self, rhs: NoiseSource<R>) -> Self::Output {
        NoiseSource { noise: NoiseAdd(self.noise, rhs.noise) }
    }
}

impl<L: Noise> std::ops::Add<f64> for NoiseSource<L> {
    type Output = NoiseSource<NoiseAdd<L, f64>>;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        NoiseSource { noise: NoiseAdd(self.noise, rhs) }
    }
}

impl<R: Noise> std::ops::Add<NoiseSource<R>> for f64 {
    type Output = NoiseSource<NoiseAdd<f64, R>>;

    #[inline]
    fn add(self, rhs: NoiseSource<R>) -> Self::Output {
        NoiseSource { noise: NoiseAdd(self, rhs.noise) }
    }
}
