use std::ops::Sub;
use crate::Signal;

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