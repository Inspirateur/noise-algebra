use ndarray::{Array, Dimension};
use crate::Signal;


impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn exp(self) -> Self {
        Signal { 
            value: self.value.map(|v| v.exp()), 
            domain: self.domain.start().exp()..=self.domain.end().exp()
        }
    }
}
