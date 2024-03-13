use crate::Signal;
use ndarray::Array1;

impl Signal<Array1<f32>> {
    pub fn quantize(self, step: f32) -> Self {
        Signal { 
            value: (self.value*step).map(|v| v.floor())/step, 
            amp: self.amp
        }
    }
}
