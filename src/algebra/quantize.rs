use crate::Signal;
use ndarray::{Array, Dimension};

impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn quantize(self, step: f32) -> Self {
        Signal { 
            value: (self.value*step).map(|v| v.floor())/step, 
            amp: self.amp
        }
    }
}
