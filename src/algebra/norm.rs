use std::ops::Div;
use crate::Signal;


impl<N: Div<f64, Output = N>> Signal<N> {
    pub fn normalize(self) -> Self {
        Signal { 
            value: self.value/(*self.domain.end()), 
            domain: self.domain.start()/self.domain.end()..=1.
        }
    }
}