mod noise;
mod algebra;
pub use crate::noise::*;
use std::ops::Deref;

// Positive Signal with amplitude tracking for normalization
#[derive(Clone, Debug)]
pub struct Signal<N> {
    pub(crate) value: N,
    pub amp: f32
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

    const TOLERANCE: f32 = 0.05;

    fn assert_normalized(signal: impl IntoIterator<Item = f32>) {
        let (min, max) = signal.into_iter().fold(
            (f32::INFINITY, f32::NEG_INFINITY), 
            |(min, max), value| (min.min(value), max.max(value))
        );
        println!("Signal range [{:.3}, {:.3}]", min, max);
        assert!(min.abs() < TOLERANCE);
        assert!((max-1.).abs() < TOLERANCE);
    }

    #[test]
    fn test_range_noise() {
        let mut n = NoiseSource::<2>::new([0..=31, 0..=31], 1, 1);
        let sample = n.simplex(100.);
        assert_normalized(sample.value);
    }

    #[test]
    fn test_range_ridge() {
        let mut n = NoiseSource::<2>::new([0..=31, 0..=31], 1, 1);
        let sample = n.ridge(100.);
        assert_normalized(sample.value);
    }
}
