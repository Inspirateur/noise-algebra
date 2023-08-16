use ndarray::Array1;
use crate::Signal;


impl Signal<Array1<f32>> {
    pub fn exp(self) -> Self {
        Signal { 
            value: self.value.map(|v| v.exp()), 
            domain: self.domain.start().exp()..=self.domain.end().exp()
        }
    }
}