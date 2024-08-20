

use super::OptionContract;
use crate::instruments::Payoff;

/// Supershare option.
#[derive(Debug, Clone)]
pub struct SupershareOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Lower strike price.
    pub strike_1: f64,

    /// Upper strike price.
    pub strike_2: f64,
}

impl Payoff for SupershareOption {
    type Underlying = f64;

    fn payoff(&self, underlying: Self::Underlying) -> f64 {
        match (self.strike_1..=self.strike_2).contains(&underlying) {
            true => underlying / self.strike_1,
            false => 0.0,
        }
    }
}
