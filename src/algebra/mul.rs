use std::ops::Mul;
use crate::Signal;

use super::_is_same::IsSame;

impl<N> Mul<Signal<N>> for Signal<N>
    where N: Mul<N, Output = N> 
{
    type Output = Signal<N>;

    fn mul(self, rhs: Signal<N>) -> Self::Output {
        Signal {
            value: self.value * rhs.value,
            amp: self.amp * rhs.amp
        }
    }
}

impl<N> Mul<f32> for Signal<N>
    where N: Mul<f32, Output = N> 
{
    type Output = Signal<N>;

    fn mul(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value * rhs,
            amp: self.amp*rhs
        }
    }
}

impl<N, __> Mul<Signal<N>> for f32
    where f32: Mul<N, Output = N>,
    f32: Mul<__>,
    N: IsSame<This = __>,
{
    type Output = Signal<N>;

    fn mul(self, rhs: Signal<N>) -> Self::Output {
        Signal {
            value: self * rhs.value,
            amp: self*rhs.amp
        }
    }
}
