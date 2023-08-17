use std::ops::Mul;
use crate::Signal;

use super::_is_same::IsSame;

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

impl<N> Mul<f64> for Signal<N>
    where N: Mul<f64, Output = N> 
{
    type Output = Signal<N>;

    fn mul(self, rhs: f64) -> Self::Output {
        let p1 = self.domain.start()*rhs;
        let p2 = self.domain.end()*rhs;
        let min = p1.min(p2);
        let max = p2.max(p1);
        Signal {
            value: self.value * rhs,
            domain: min..=max
        }
    }
}

impl<N, __> Mul<Signal<N>> for f64
    where f64: Mul<N, Output = N>,
    f64: Mul<__>,
    N: IsSame<This = __>,
{
    type Output = Signal<N>;

    fn mul(self, rhs: Signal<N>) -> Self::Output {
        let p1 = self*rhs.domain.start();
        let p2 = self*rhs.domain.end();
        let min = p1.min(p2);
        let max = p2.max(p1);
        Signal {
            value: self * rhs.value,
            domain: min..=max
        }
    }
}
