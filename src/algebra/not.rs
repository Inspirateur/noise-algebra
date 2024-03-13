use std::ops::Not;
use ndarray::{Array, Dimension};
use crate::Signal;

impl<D: Dimension> Not for Signal<Array<f32, D>>{
    type Output = Signal<Array<f32, D>>;

    fn not(self) -> Self::Output {
        Signal {
            value: self.amp - self.value,
            amp: self.amp
        }
    }
}
