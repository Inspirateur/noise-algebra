use std::ops::Sub;
use crate::Signal;

use super::_is_same::IsSame;

impl<N> Sub<Signal<N>> for Signal<N>
    where N: Sub<N, Output = N> 
{
    type Output = Signal<N>;

    fn sub(self, rhs: Signal<N>) -> Self::Output {
        Signal {
            value: self.value - rhs.value,
            domain: (self.domain.start()-rhs.domain.end())..=(self.domain.end()-rhs.domain.start())
        }
    }
}

impl<N> Sub<f32> for Signal<N>
    where N: Sub<f32, Output = N> 
{
    type Output = Signal<N>;

    fn sub(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value - rhs,
            domain: (self.domain.start()-rhs)..=(self.domain.end()-rhs)
        }
    }
}

impl<N, __> Sub<Signal<N>> for f32
    where f32: Sub<N, Output = N>,
    f32: Sub<__>,
    N: IsSame<This = __>,
{
    type Output = Signal<N>;

    fn sub(self, rhs: Signal<N>) -> Self::Output {
        Signal {
            value: self - rhs.value,
            domain: (self-rhs.domain.start())..=(self-rhs.domain.end())
        }
    }
}
