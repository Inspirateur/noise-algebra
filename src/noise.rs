use std::ops::RangeInclusive;
use ndarray::Array1;

pub trait Noise: Clone + Send + Sync + Sized {
    /// Sample the noise at a range of coordinates, with the specified step and seed
    /// the output is an unrolled array with samples for every combination of coordinates
    /// each range produces 1+(end-start)/step_by values, the length of the output array is the product of that for every range
    ///  
    /// Supports dimensions 1, 2, 3, 4
    /// 
    /// Note: Noise sample coordinates are usually in [0; 1] 
    /// but for convenience this method asks for i32 and scales them back to make it work 
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
