use crate::Signal;
use ndarray::{Dimension, Array};

impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn powi(self, i: i32) -> Self {
        let max = self.domain.end().powi(i);
        let crosses_zero = self.domain.start()*self.domain.end() < 0.;
        let min = if crosses_zero && i % 2 == 0 {
            0f32
        } else {
            self.domain.start().powi(i)
        };
        Signal { 
            value: self.value.map(|v| v.signum()*v.abs().powi(i)), 
            domain: min..=max
        }
    }
}
