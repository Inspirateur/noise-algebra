use ndarray::Array1;
use crate::noise::{Noise, NoiseSource, NoiseRange};

#[derive(Clone)]
struct FakeNoise { 
    value: f64
}

impl FakeNoise {
    fn new(value: f64) -> Self {
        Self { value }
    }
}

impl Noise for FakeNoise {
    #[inline]
    fn sample<R: NoiseRange>(&self, x_range: R, y_range: R, _seed: usize) -> Array1<f64> {
        Array1::from_elem(x_range.len()*y_range.len(), self.value)
    }

    fn domain(&self) -> std::ops::RangeInclusive<f64> {
        -1f64..=1f64
    }
}

#[inline]
pub fn fake_noise(value: f64) -> NoiseSource<impl Noise> {
    NoiseSource { noise: FakeNoise::new(value) }
}