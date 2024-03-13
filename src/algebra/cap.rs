use crate::Signal;
use ndarray::{Dimension, Array};

impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn cap(self, f: f32) -> Self {
        Signal { 
            value: self.value.map(|v| v.min(f)), 
            amp: self.amp.min(f)
        }
    }
}
