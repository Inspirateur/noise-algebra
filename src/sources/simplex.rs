use itertools::Itertools;
use ndarray::Array1;
use simdnoise::*;
use crate::{Noise, NoiseSource, noise::NoiseRange};
use crate::sources::C;

#[derive(Clone)]
pub struct Simplex {
    freq: f64
}


impl Simplex {
    fn new(freq: f64) -> Self {
        Self { freq }
    }
}

impl Noise for Simplex {
    #[inline]
    fn sample<R: NoiseRange>(&self, mut x_range: R, mut y_range: R, seed: usize) -> Array1<f64> {
        let x = x_range.next().unwrap_or(0);
        let y = y_range.next().unwrap_or(0);
        let (res, _, _) = NoiseBuilder::fbm_2d_offset(
            self.freq as f32*x as f32/C as f32, x_range.len(), 
            self.freq as f32*y as f32/C as f32, y_range.len()
        ).with_seed(seed as i32).generate();
        Array1::from_vec(res.into_iter().map(|v| v as f64).collect_vec())
    }

    fn domain(&self) -> std::ops::RangeInclusive<f64> {
        0f64..=1f64
    }
}

pub fn simplex(freq: f64) -> NoiseSource<Simplex> {
    NoiseSource { noise: Simplex::new(freq) }
}
