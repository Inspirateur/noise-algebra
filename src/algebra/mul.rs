use std::ops::Mul;
use crate::Signal;

impl<N> Mul<Signal<N>> for Signal<N>
    where N: Mul<N, Output = N> 
{
    type Output = Signal<N>;

    fn mul(self, rhs: Signal<N>) -> Self::Output {
        let p1 = self.domain.start()*rhs.domain.start();
        let p2 = self.domain.start()*rhs.domain.end();
        let p3 = self.domain.end()*rhs.domain.start();
        let p4 = self.domain.end()*rhs.domain.end();
        let min = p1.min(p2).min(p3).min(p4);
        let max = p1.max(p2).max(p3).max(p4);
        Signal {
            value: self.value * rhs.value,
            domain: min..=max
        }
    }
}
