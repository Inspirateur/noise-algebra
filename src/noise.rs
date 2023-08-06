use std::ops::RangeInclusive;
use ndarray::Array1;

pub trait NoiseRange: ExactSizeIterator<Item = i32> + Clone {}
impl<R: ExactSizeIterator<Item = i32> + Clone> NoiseRange for R {}

pub trait Noise: Clone {
    fn sample<R: NoiseRange>(&self, x_range: R, y_range: R, seed: usize) -> Array1<f64>;

    fn domain(&self) -> RangeInclusive<f64>;
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
