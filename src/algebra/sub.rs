use std::ops::Sub;
use ndarray::{Array, Dimension};

use crate::Signal;

impl<D: Dimension> Sub<Signal<Array<f32, D>>> for Signal<Array<f32, D>>{
    type Output = Signal<Array<f32, D>>;

    fn sub(self, rhs: Signal<Array<f32, D>>) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            amp: self.amp - rhs.amp
        }
    }
}

impl<D: Dimension> Sub<&Signal<Array<f32, D>>> for Signal<Array<f32, D>>{
    type Output = Signal<Array<f32, D>>;

    fn sub(self, rhs: &Signal<Array<f32, D>>) -> Self::Output {
        Signal {
            value: self.value - &rhs.value,
            amp: self.amp - rhs.amp
        }
    }
}