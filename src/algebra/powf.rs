use crate::Signal;
use ndarray::{Dimension, Array};

impl<D: Dimension> Signal<Array<f32, D>> {
    pub fn powf(self, f: f32) -> Self {
        let start = self.domain.start();
        let end = self.domain.end();
        let min = start.signum()*start.abs().powf(f);
        let max = end.signum()*end.abs().powf(f);
        Signal { 
            value: self.value.map(|v| v.signum()*v.abs().powf(f)), 
            domain: min..=max
        }
    }
}
