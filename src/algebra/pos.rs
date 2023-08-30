use ndarray::Array1;
use crate::Signal;


impl Signal<Array1<f32>> {
    pub fn pos(self) -> Self {
        // shortcut to turn a -1;1 value into 0;1
        Signal { 
            value: 0.5+self.value/2., 
            domain: (0.5+self.domain.start()/2.)..=(0.5+self.domain.end()/2.)
        }
    }
}