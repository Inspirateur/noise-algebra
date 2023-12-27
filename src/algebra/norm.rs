use std::ops::Div;
use crate::Signal;


impl<N: Div<f32, Output = N>> Signal<N> {
    pub fn normalize(self) -> Self {
        Signal { 
            value: self.value/(*self.domain.end()), 
            domain: self.domain.start()/self.domain.end()..=1.
        }
    }
}