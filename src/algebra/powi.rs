use crate::Signal;
use ndarray::Array1;

impl Signal<Array1<f64>> {
    pub fn powi(self, i: i32) -> Self {
        let max = self.domain.end().powi(i);
        let crosses_zero = self.domain.start()*self.domain.end() < 0.;
        let min = if crosses_zero && i % 2 == 0 {
            0f64
        } else {
            self.domain.start().powi(i)
        };
        Signal { 
            value: self.value.map(|v| v.signum()*v.abs().powi(i)), 
            domain: min..=max
        }
    }
}
