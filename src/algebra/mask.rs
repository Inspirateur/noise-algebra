use crate::Signal;
use ndarray::{Array, Dimension};

impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn threshold(self, t: f32) -> Self {
        let t = 1. - 2.*t;
        Signal { 
            value: 1.0 / (1.0 + (-16. * (self.value - t)).map(|v| v.exp())), 
            domain: 0f32..=1f32
        }
    }
}
