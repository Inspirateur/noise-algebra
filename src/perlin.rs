use crate::noise::{Noise, NoiseSource, NoiseRange};
use itertools::iproduct;
use noise::Seedable;
use ::noise::{NoiseFn, Perlin as PerlinImpl};
use ndarray::Array1;
const C: f64 = 3141.592653589793238462643383279502884197;

#[derive(Clone)]
pub struct Perlin {
    freq: f64,
    source: PerlinImpl,
}

impl Perlin {
    pub fn new(freq: f64) -> Self {
        Self {
            freq,
            source: PerlinImpl::new(0),
        }
    }
}

impl Noise for Perlin {
    #[inline]
    fn sample<R: NoiseRange>(&self, x_range: R, y_range: R, seed: usize) -> Array1<f64> {
        let source = self.source.set_seed(seed as u32);
        let mut res = Array1::from_elem(x_range.len()*y_range.len(), 0f64);
        for (i, (x, y)) in iproduct!(x_range, y_range).enumerate() {
            res[i] = source.get([
                self.freq * (x as f64)/C, 
                self.freq * (y as f64)/C
            ])
        }
        res
    }

    fn domain(&self) -> std::ops::RangeInclusive<f64> {
        -1f64..=1f64
    }
}

#[inline]
pub fn perlin(freq: f64) -> NoiseSource<Perlin> {
    NoiseSource { noise: Perlin::new(freq) }
}