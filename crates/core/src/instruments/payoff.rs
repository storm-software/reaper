

/// Generic payoff trait for derivatives.
pub trait Payoff {
    /// Underlying input type for the payoff function.
    type Underlying;

    /// Payoff function for the derivative.
    fn payoff(&self, underlying: Self::Underlying) -> f64;
}
