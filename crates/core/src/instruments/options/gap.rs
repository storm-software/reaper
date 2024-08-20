

/// Gap option.
#[derive(Debug, Clone)]
pub struct GapOption {
    /// The option contract.
    pub contract: OptionContract,

    /// First strike price (barrier strike).
    pub strike_1: f64,

    /// Second strike price (payoff strike).
    pub strike_2: f64,
}
