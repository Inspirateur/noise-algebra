use std::ops::RangeInclusive;
use crate::noise::{Noise, NoiseSource, len};
use itertools::iproduct;
use noise::Seedable;
use ::noise::{NoiseFn, Perlin as PerlinImpl};
use ndarray::Array1;
use crate::sources::C;

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
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], step_by: usize, seed: usize) -> Array1<f64> {
        match D {
            2 => {
                let source = self.source.set_seed(seed as u32);
                let mut res = Array1::from_elem(
                    len(ranges[0].clone(), step_by)*len(ranges[1].clone(), step_by), 0f64
                );
                for (i, (x, y)) in iproduct!(ranges[0].clone().step_by(step_by), ranges[1].clone().step_by(step_by)).enumerate() {
                    res[i] = source.get([
                        self.freq * (x as f64)/C, 
                        self.freq * (y as f64)/C
                    ])
                }
                res
            },
            _ => todo!()
        }
    }

    fn domain(&self) -> std::ops::RangeInclusive<f64> {
        -1f64..=1f64
    }
}

/// Uses the noise lib
#[inline]
pub fn perlin(freq: f64) -> NoiseSource<Perlin> {
    NoiseSource { noise: Perlin::new(freq) }
}