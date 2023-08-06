use std::{marker::PhantomData, ops::RangeInclusive};
use ndarray::Array1;
use simdnoise::simplex::simplex_2d;
use simdeez::*;
use simdeez::sse41::Sse2;
use crate::{Noise, NoiseSource, noise::NoiseRange};

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
    fn sample<R: NoiseRange>(&self, x_range: R, y_range: R, seed: usize) -> Array1<f64> {
        todo!()
    }

    fn domain(&self) -> std::ops::RangeInclusive<f64> {
        todo!()
    }
}

pub fn simplex(freq: f64) -> NoiseSource<Simplex> {
    NoiseSource { noise: Simplex::new(freq) }
}
