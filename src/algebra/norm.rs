use ndarray::{Array, Dimension};
use crate::Signal;


impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn normalize(self) -> Self {
        Signal { 
            value: self.value/self.amp, 
            amp: 1f32
        }
    }
}