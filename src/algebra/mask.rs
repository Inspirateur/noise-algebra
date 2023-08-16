use crate::Signal;
use ndarray::Array1;

impl Signal<Array1<f64>> {
    pub fn mask(self, t: f64) -> Self {
        Signal { 
            value: 1.0 / (1.0 + (-16. * (self.value - t)).map(|v| v.exp())), 
            domain: 0f64..=1f64
        }
    }
}
