use crate::noise::{Noise, NoiseSource};
use noise::Seedable;
use ::noise::{NoiseFn, Perlin as PerlinImpl};

#[derive(Clone)]
struct FakeNoise { }

impl FakeNoise {
    fn new() -> Self {
        Self { }
    }
}

impl Noise for FakeNoise {
    type Sample = f64;

    #[inline]
    fn sample(&self, x: f64, y: f64, seed: usize) -> Self::Sample {
        0.5
    }

    fn domain(&self) -> std::ops::RangeInclusive<f64> {
        0f64..=1f64
    }
}

#[inline]
pub fn fake_noise() -> NoiseSource<impl Noise> {
    let source = FakeNoise::new();
    NoiseSource {
        domain: source.domain().clone(),
        noise: source,
    }
}