

/// Forward start option.
#[derive(Debug, Clone)]
pub struct ForwardStartOption {
    /// The option contract.
    pub contract: OptionContract,

    /// Strike price of the option.
    pub strike: f64,

    /// Forward start date.
    pub start_date: Date,
}
