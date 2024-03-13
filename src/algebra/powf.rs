use crate::Signal;
use ndarray::{Dimension, Array};

impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn powf(&self, f: f32) -> Self {
        Signal { 
            value: self.value.map(|v| v.powf(f)), 
            amp: self.amp.powf(f)
        }
    }
}
