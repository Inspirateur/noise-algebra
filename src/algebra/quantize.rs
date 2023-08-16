use crate::Signal;
use ndarray::Array1;

impl Signal<Array1<f64>> {
    pub fn quantize(self, step: f64) -> Self {
        Signal { 
            value: (self.value*step).map(|v| v.floor())/step, 
            domain: self.domain
        }
    }
}
