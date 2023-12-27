use crate::Signal;
use ndarray::{Array, Dimension};

impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn threshold(self, t: f32) -> Self {
        Signal { 
            value: 1.0 / (1.0 + (-32. * (self.value - t + 0.15)).map(|v| v.exp())), 
            domain: 0f32..=1f32
        }
    }
}
