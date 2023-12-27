use std::ops::Add;
use crate::Signal;

use super::_is_same::IsSame;

impl<N> Add<Signal<N>> for Signal<N>
    where N: Add<N, Output = N> 
{
    type Output = Signal<N>;

    fn add(self, rhs: Signal<N>) -> Self::Output {
        Signal {
            value: self.value + rhs.value,
            domain: (self.domain.start()+rhs.domain.start())..=(self.domain.end()+rhs.domain.end())
        }
    }
}

impl<N> Add<f32> for Signal<N>
    where N: Add<f32, Output = N> 
{
    type Output = Signal<N>;

    fn add(self, rhs: f32) -> Self::Output {
        Signal {
            value: self.value + rhs,
            domain: (self.domain.start()+rhs)..=(self.domain.end()+rhs)
        }
    }
}

impl<N, __> Add<Signal<N>> for f32
    where f32: Add<N, Output = N>,
    f32: Add<__>,
    N: IsSame<This = __>,
{
    type Output = Signal<N>;

    fn add(self, rhs: Signal<N>) -> Self::Output {
        Signal {
            value: self + rhs.value,
            domain: (self+rhs.domain.start())..=(self+rhs.domain.end())
        }
    }
}
