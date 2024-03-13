use std::ops::Add;
use ndarray::{Array, Dimension};
use crate::Signal;

impl<D: Dimension> Add<Signal<Array<f32, D>>> for Signal<Array<f32, D>>{
    type Output = Signal<Array<f32, D>>;

    fn add(self, rhs: Signal<Array<f32, D>>) -> Self::Output {
        Signal {
            value: self.value + rhs.value,
            amp: self.amp+rhs.amp
        }
    }
}

impl<D: Dimension> Add<&Signal<Array<f32, D>>> for Signal<Array<f32, D>>{
    type Output = Signal<Array<f32, D>>;

    fn add(self, rhs: &Signal<Array<f32, D>>) -> Self::Output {
        Signal {
            value: self.value + &rhs.value,
            amp: self.amp+rhs.amp
        }
    }
}
