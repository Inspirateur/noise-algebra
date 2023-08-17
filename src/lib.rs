mod noise;
mod algebra;
pub use crate::noise::NoiseSource;
use std::ops::{RangeInclusive, Deref};

#[derive(Clone)]
pub struct Signal<N> {
    pub(crate) value: N,
    pub(crate) domain: RangeInclusive<f64>
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
}
