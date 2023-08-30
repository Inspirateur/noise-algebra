use std::ops::{Add, Mul};
use crate::Signal;


impl<N: Add<f64, Output = N> + Mul<f64, Output = N>> Signal<N> {
    pub fn pos(self) -> Self {
        // shortcut to turn a -1;1 value into 0;1
        Signal { 
            value: self.value*0.5 + 0.5, 
            domain: (self.domain.start()*0.5 + 0.5)..=(self.domain.end()*0.5 + 0.5)
        }
    }
}