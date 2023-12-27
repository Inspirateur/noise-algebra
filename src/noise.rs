use std::ops::RangeInclusive;
use itertools::Itertools;
use ndarray::Array2;
use simdnoise::NoiseBuilder;
use crate::Signal;
// empirical values to make the output nicer with default inputs
const C: f32 = 1.;
const FREQ_C: f32 = 0.001;

fn len(range: RangeInclusive<i32>, step_by: usize) -> usize {
    1 + (range.end() - range.start()) as usize/step_by
}

#[derive(Clone)]
pub struct NoiseSource<const D: usize> {
    seed: i32,
    offsets: [f32; D],
    lens: [usize; D],
    step_by: usize
}

impl NoiseSource<2> {
    pub fn new(area: [RangeInclusive<i32>; 2], seed: i32, step_by: usize) -> Self {
        NoiseSource { 
            offsets: area.clone().map(|range| (range.start()/step_by as i32) as f32/C as f32),
            lens: area.map(|range| len(range, step_by)),
            seed, 
            step_by 
        }
    }

    pub fn simplex(&mut self, freq: f32) -> Signal<Array2<f32>> {
        self.seed += 1;
        Signal {
            value: {
                // *46 is due to a manual scaling required because library oversight 
                // https://github.com/verpeteren/rust-simd-noise/issues/23
                let (res, _, _) = NoiseBuilder::gradient_2d_offset(
                    self.offsets[0], self.lens[0], 
                    self.offsets[1], self.lens[1]
                ).with_freq(FREQ_C * freq * self.step_by as f32).with_seed(self.seed as i32).generate();
                Array2::from_shape_vec(
                    (self.lens[0], self.lens[1]), 
                    res.into_iter().map(|v| v).collect_vec()
                ).unwrap()*46.
            },
            domain: -1f32..=1f32
        }
    }

    pub fn gavoronoise(&mut self, freq: f32) -> Signal<Array2<f32>> {
        // TODO: https://www.shadertoy.com/view/llsGWl
        todo!()
    }

    pub fn constant(&mut self, value: f32) -> Signal<Array2<f32>> {
        Signal {
            value: Array2::from_shape_vec(
                (self.lens[0], self.lens[1]),
                vec![value; self.lens.iter()
                    .fold(1, |a, b| a*b)], 
            ).unwrap(),
            domain: -1f32..=1f32
        }
    }
}