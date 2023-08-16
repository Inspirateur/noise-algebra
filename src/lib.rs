mod noise;
mod algebra;
pub use crate::noise::NoiseSource;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Signal<N> {
    pub(crate) value: N,
    pub(crate) domain: RangeInclusive<f64>
}


#[cfg(test)]
mod tests {
    use super::*;
}
