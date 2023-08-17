/// This is used in a workaround for this bug https://github.com/rust-lang/rust/issues/24066
///
/// Implementing the addition of f64 + signal fails and we need another fake where clause to make it work (weird I know)
/// 
/// ```
/// impl<N> Add<Signal<N>> for f64
///     where f64: Add<N, Output = N>
/// {
///     type Output = Signal<N>;
///     fn add(self, rhs: Signal<N>) -> Self::Output {
///         Signal {
///             value: self + rhs.value,
///             domain: (self+rhs.domain.start())..=(self+rhs.domain.end())
///         }
///     }
/// }
/// ```
///  
pub trait IsSame {
    type This;
}

impl<T> IsSame for T {
    type This = T;
}
