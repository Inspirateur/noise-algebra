use std::ops::Add;
use crate::Signal;

impl<N> Add<Signal<N>> for Signal<N>
    where N: Add<N, Output = N> 
{
    type Output = Signal<N>;

    fn add(self, rhs: Signal<N>) -> Self::Output {
        Signal {
            value: self.value + rhs.value,
            amp: self.amp+rhs.amp
        }
    }
}
