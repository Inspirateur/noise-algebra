use std::ops::RangeInclusive;
use ndarray::Array1;

pub trait Noise: Clone {
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], step_by: usize, seed: usize) -> Array1<f64>;

    fn domain(&self) -> RangeInclusive<f64>;
}

pub(crate) fn len(range: RangeInclusive<i32>, step_by: usize) -> usize {
    1 + (range.end() - range.start()) as usize/step_by
}

#[derive(Clone)]
pub struct NoiseSource<X: Noise> {
    pub noise: X
}


impl<X: Noise> std::ops::Deref for NoiseSource<X> {
    type Target = X;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.noise
    }
}
