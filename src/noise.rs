use std::ops::RangeInclusive;
use num_traits::NumCast;

pub trait Noise {
    fn sample(&self, x: f64, y: f64, seed: usize) -> f64;

    fn domain(&self) -> RangeInclusive<f64>;
}

impl<Num: NumCast> Noise for Num {
    #[inline]
    fn sample(&self, _x: f64, _y: f64, _seed: usize) -> f64 {
        self.to_f64().unwrap()
    }

    fn domain(&self) -> RangeInclusive<f64> {
        let value = self.to_f64().unwrap();
        value..=value
    }
}

pub struct NoiseSource<X: Noise> {
    pub noise: X,
    pub domain: RangeInclusive<f64>
}


impl<X: Noise> std::ops::Deref for NoiseSource<X> {
    type Target = X;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.noise
    }
}
