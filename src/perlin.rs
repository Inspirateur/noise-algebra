use crate::noise::{Noise, NoiseSource};
use ::noise::{NoiseFn, Perlin as PerlinImpl};

struct Perlin {
    freq: f64,
    source: PerlinImpl,
}

impl Perlin {
    fn new(freq: f64) -> Self {
        Self {
            freq,
            source: PerlinImpl::new(0),
        }
    }
}

impl Noise for Perlin {
    #[inline]
    fn sample(&self, x: f64, y: f64, seed: usize) -> f64 {
        self.source.get([self.freq * x, self.freq * y])
    }

    fn domain(&self) -> std::ops::RangeInclusive<f64> {
        todo!()
    }
}

#[inline]
pub fn perlin(freq: f64) -> NoiseSource<impl Noise> {
    let source = Perlin::new(freq);
    NoiseSource {
        domain: source.domain().clone(),
        noise: source,
    }
}