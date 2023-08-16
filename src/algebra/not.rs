use std::ops::{Not, Add, Sub};
use crate::Signal;

impl<N> Not for Signal<N>
    where N: Add<N, Output = N>,
    f64: Sub<N, Output = N>
{
    type Output = Signal<N>;

    fn not(self) -> Self::Output {
        Signal {
            value: self.domain.start()+self.domain.end() - self.value,
            domain: self.domain
        }
    }
}
