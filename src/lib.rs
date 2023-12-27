mod noise;
mod algebra;
pub use crate::noise::NoiseSource;
use std::ops::{RangeInclusive, Deref};

#[derive(Clone, Debug)]
pub struct Signal<N> {
    pub(crate) value: N,
    pub domain: RangeInclusive<f32>
}

impl<N> Deref for Signal<N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        let mut n = NoiseSource::new([0..=31, 0..=31], 10, 1);
        let sample = n.simplex(1.);
        let (min, max) = sample.value.iter().fold(
            (f32::INFINITY, f32::NEG_INFINITY), 
            |(min, max), value| (min.min(*value), max.max(*value))
        );
        println!("{:?}", sample.value);
        println!("[{:.3}, {:.3}]", min, max);
    }
}
