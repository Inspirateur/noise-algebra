use std::ops::RangeInclusive;
use itertools::Itertools;
use ndarray::{Array2, Array3};
use simdnoise::NoiseBuilder;
use crate::Signal;
// empirical values to make the output nicer with default inputs
const C: f32 = 1.;
const FREQ_C: f32 = 0.001;
// manual scaling required because library oversight 
// https://github.com/verpeteren/rust-simd-noise/issues/23
const CORRECTION: f32 = 46.;

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

impl<const D: usize> NoiseSource<D> {
    pub fn new(area: [RangeInclusive<i32>; D], seed: i32, step_by: usize) -> Self {
        NoiseSource { 
            offsets: area.clone().map(|range| (range.start()/step_by as i32) as f32/C as f32),
            lens: area.map(|range| len(range, step_by)),
            seed, 
            step_by 
        }
    }
}

pub type Signal2d = Signal<Array2<f32>>;

impl NoiseSource<2> {
    pub fn simplex(&mut self, freq: f32) -> Signal2d {
        self.seed += 1;
        Signal {
            value: {
                let (res, _, _) = NoiseBuilder::gradient_2d_offset(
                    self.offsets[0], self.lens[0], 
                    self.offsets[1], self.lens[1]
                ).with_freq(FREQ_C * freq * self.step_by as f32).with_seed(self.seed as i32).generate();
                Array2::from_shape_vec(
                    (self.lens[0], self.lens[1]), 
                    res.into_iter().map(|v| (v*CORRECTION+1.).max(0.)/2.).collect_vec()
                ).unwrap()
            },
            amp: 1f32
        }
    }

    pub fn ridge(&mut self, freq: f32) -> Signal2d {
        self.seed += 1;
        Signal {
            value: {
                let (res, _, _) = NoiseBuilder::gradient_2d_offset(
                    self.offsets[0], self.lens[0], 
                    self.offsets[1], self.lens[1]
                ).with_freq(FREQ_C * freq * self.step_by as f32).with_seed(self.seed as i32).generate();
                Array2::from_shape_vec(
                    (self.lens[0], self.lens[1]), 
                    res.into_iter().map(|v| 1.-(v*CORRECTION).abs().min(1.)).collect_vec()
                ).unwrap()
            },
            amp: 1f32
        }
    }

    pub fn constant(&mut self, value: f32) -> Signal2d {
        Signal {
            value: Array2::from_shape_vec(
                (self.lens[0], self.lens[1]),
                vec![value; self.lens.iter()
                    .fold(1, |a, b| a*b)], 
            ).unwrap(),
            amp: value
        }
    }
}

pub type Signal3d = Signal<Array3<f32>>;

impl NoiseSource<3> {
    pub fn simplex(&mut self, freq: f32) -> Signal3d {
        self.seed += 1;
        Signal {
            value: {
                let (res, _, _) = NoiseBuilder::gradient_3d_offset(
                    self.offsets[0], self.lens[0], 
                    self.offsets[1], self.lens[1],
                    self.offsets[2], self.lens[2],
                ).with_freq(FREQ_C * freq * self.step_by as f32).with_seed(self.seed as i32).generate();
                Array3::from_shape_vec(
                    (self.lens[0], self.lens[1], self.lens[2]), 
                    res.into_iter().map(|v| (v*CORRECTION+1.).max(0.)/2.).collect_vec()
                ).unwrap()
            },
            amp: 1f32
        }
    }

    pub fn constant(&mut self, value: f32) -> Signal3d {
        Signal {
            value: Array3::from_shape_vec(
                (self.lens[0], self.lens[1], self.lens[2]),
                vec![value; self.lens.iter()
                    .fold(1, |a, b| a*b)], 
            ).unwrap(),
            amp: value
        }
    }
}