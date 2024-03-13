use crate::Signal;
use ndarray::{Dimension, Array};

impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn max(self, f: f32) -> Self {
        Signal { 
            value: self.value.map(|v| v.max(f)), 
            amp: self.amp.max(f)
        }
    }
}
