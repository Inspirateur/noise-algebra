use std::ops::RangeInclusive;
use itertools::Itertools;
use ndarray::Array1;
use simdnoise::*;
use crate::noise::len;
use crate::{Noise, NoiseSource};
use crate::sources::C;

#[derive(Clone)]
pub struct Simplex {
    freq: f32
}


impl Simplex {
    fn new(freq: f32) -> Self {
        Self { freq }
    }
}

impl Noise for Simplex {
    #[inline]
    fn sample<const D: usize>(&self, ranges: [RangeInclusive<i32>; D], step_by: usize, seed: usize) -> Array1<f64> {
        match D {
            2 => {
                let x_start = *ranges[0].start();
                let y_start = *ranges[1].start();
                let (res, _, _) = NoiseBuilder::fbm_2d_offset(
                    x_start as f32/C as f32, len(ranges[0].clone(), step_by), 
                    y_start as f32/C as f32, len(ranges[1].clone(), step_by)
                ).with_freq(self.freq * step_by as f32).with_seed(seed as i32).generate();
                Array1::from_vec(res.into_iter().map(|v| v as f64).collect_vec())        
            },
            _ => todo!()
        }
    }

    fn domain(&self) -> std::ops::RangeInclusive<f64> {
        0f64..=1f64
    }
}

/// Uses the simdnoise lib, subsequent operations are NOT simd (yet)
pub fn simplex(freq: f32) -> NoiseSource<Simplex> {
    NoiseSource { noise: Simplex::new(freq) }
}
