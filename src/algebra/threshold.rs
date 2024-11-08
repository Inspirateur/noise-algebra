use crate::Signal;
use ndarray::{Array, Dimension};

impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn threshold(self, t: f32) -> Self {
        Signal { 
            value: 1.0 / (1.0 + (-32. * (self.value - t + 0.1)).map(|v| v.exp())), 
            amp: 1f32
        }
    }
}
