use std::ops::RangeInclusive;
use itertools::Itertools;
use ndarray::Array1;
use simdnoise::NoiseBuilder;

use crate::Signal;
const C: f32 = 3.14159265359;

fn len(range: RangeInclusive<i32>, step_by: usize) -> usize {
    1 + (range.end() - range.start()) as usize/step_by
}

pub struct NoiseSource<const D: usize> {
    seed: i32,
    offsets: [f32; D],
    lens: [usize; D],
    step_by: usize
}

impl<const D: usize> NoiseSource<D> {
    pub fn new(area: [RangeInclusive<i32>; D], seed: i32, step_by: usize) -> Self {
        NoiseSource { 
            offsets: area.clone().map(|range| *range.start() as f32/C as f32),
            lens: area.map(|range| len(range, step_by)),
            seed, 
            step_by 
        }
    }

    pub fn simplex(&mut self, freq: f32) -> Signal<Array1<f64>> {
        self.seed += 1;
        Signal {
            value: match D {
                2 => {
                    let (res, _, _) = NoiseBuilder::fbm_2d_offset(
                        self.offsets[0], self.lens[0], 
                        self.offsets[1], self.lens[1]
                    ).with_freq(freq * self.step_by as f32).with_seed(self.seed as i32).generate();
                    Array1::from_vec(res.into_iter().map(|v| v as f64).collect_vec())        
                },
                _ => todo!()
            },
            domain: -1f64..=1f64
        }

    }

    pub fn constant(&self, value: f64) -> Signal<Array1<f64>> {
        Signal {
            value: Array1::from_elem(
                self.lens.iter()
                    .fold(1, |a, b| a*b), 
                value
            ),
            domain: -1f64..=1f64
        }

    }
}