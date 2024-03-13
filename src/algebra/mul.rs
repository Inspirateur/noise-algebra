use std::ops::Mul;
use ndarray::{Array, Dimension};
use crate::Signal;

impl<D: Dimension> Mul<Signal<Array<f32, D>>> for Signal<Array<f32, D>>{
    type Output = Signal<Array<f32, D>>;

    fn mul(self, rhs: Signal<Array<f32, D>>) -> Self::Output {
        Signal {
            value: self.value * rhs.value,
            amp: self.amp * rhs.amp
        }
    }
}

impl<D: Dimension> Mul<&Signal<Array<f32, D>>> for Signal<Array<f32, D>>{
    type Output = Signal<Array<f32, D>>;

    fn mul(self, rhs: &Signal<Array<f32, D>>) -> Self::Output {
        Signal {
            value: self.value * &rhs.value,
            amp: self.amp * rhs.amp
        }
    }
}

impl<D: Dimension> Mul<f32> for Signal<Array<f32, D>>{
    type Output = Signal<Array<f32, D>>;

    fn mul(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value * rhs,
            amp: self.amp*rhs
        }
    }
}

impl<D: Dimension> Mul<Signal<Array<f32, D>>> for f32 {
    type Output = Signal<Array<f32, D>>;

    fn mul(self, rhs: Signal<Array<f32, D>>) -> Self::Output {
        Signal {
            value: self * rhs.value,
            amp: self*rhs.amp
        }
    }
}
