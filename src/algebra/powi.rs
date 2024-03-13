use crate::Signal;
use ndarray::{Dimension, Array};

impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn powi(self, i: i32) -> Self {
        Signal { 
            value: self.value.map(|v| v.powi(i)), 
            amp: self.amp.powi(i)
        }
    }
}
