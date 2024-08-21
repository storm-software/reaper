//! `reaper_core` error handling module.
//! A custom error type `StormTradingError` is defined, along with a macro to create an error,
//! that propagates a `StormTradingError` with the text to include in the output.

use thiserror::Error;

/// Error type for `reaper_core`.
#[derive(Debug, Error)]
pub enum StormTradingError {
  /// This error indicates that an problem occurred in the computation.
  #[error("Computation error: {0}")]
  ComputationError(String),

  /// This error indicates an invalid parameter/argument was passed.
  #[error("Invalid argument: {0}")]
  InvalidArgument(String),

  /// This error indicates that a condition is violated.
  #[error("Condition violated: {0}")]
  ConditionViolated(String),

  /// This error indicates that a file operation failed.
  #[error("File operation failed: {0}")]
  FileOperationFailed(String),

  /// Error variant arising from missing inputs.
  #[error{"An input was missing: {0}"}]
  MissingInput(String),

  // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  // Data related errors
  // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  /// Error variant arising from the Yahoo! Finance API.
  #[error("Yahoo! Finance error: {0}")]
  YahooError(#[from] yahoo_finance_api::YahooError),

  /// Error variant arising from Polars.
  #[error("Polars error: {0}")]
  PolarsError(#[from] polars::error::PolarsError),

  /// Error variant arising from [`std::io`].
  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),

  // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  // Statistical distribution related errors
  // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  /// Error variant from constructing Bernoulli distribution.
  #[error("{0}")]
  Bernoulli(#[from] rand_distr::BernoulliError),

  /// Error variant from constructing Binomial distribution.
  #[error("{0}")]
  Binomial(#[from] rand_distr::BinomialError),

  /// Error variant from constructing ChiSquared distribution.
  #[error("{0}")]
  ChiSquared(#[from] rand_distr::ChiSquaredError),

  /// Error variant from constructing Exponential distribution.
  #[error("{0}")]
  Exponential(#[from] rand_distr::ExpError),

  /// Error variant from constructing Gamma distribution.
  #[error("{0}")]
  Gamma(#[from] rand_distr::GammaError),

  /// Error variant from constructing Gaussian distribution.
  #[error("{0}")]
  Gaussian(#[from] rand_distr::NormalError),

  /// Error variant from constructing Poisson distribution.
  #[error("{0}")]
  Poisson(#[from] rand_distr::PoissonError),

  // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  // Linear regression related errors
  // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  /// failed to invert matrix
  #[error("Matrix inversion failed")]
  MatrixInversionFailed,

  /// failed to perform SVD decomposition
  #[error("SVD decomposition failed: v_t is likely wrong type")]
  SvdDecompositionFailed,

  /// failed to compute u
  #[error("SVD decomposition failed: u is likely wrong type")]
  SvdDecompositionFailedOnU,

  // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  // Interpolation related errors
  // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  /// Unequal length of `xs` and `ys`.
  #[error("Unequal length of `xs` and `ys` inputs in interpolator.")]
  UnequalLength,

  /// Interpolator has not been fitted.
  #[error("Interpolator has not been fitted.")]
  Unfitted,

  /// Outside of interpolation range.
  #[error("Outside of interpolation range.")]
  OutsideOfRange,
}

/// Curve error enum.
#[derive(Debug, Clone, Copy)]
pub enum CurveError {
  /// The date is outside the curve's range.
  DateOutsideRange,

  /// The curve has no points.
  NoPoints,
}

/// Create a `StormTradingError` with the text to include in the output.
/// You would use it as follows:
///
/// ```ignore
/// return Err(
///     error!(
///         ComputationError,
///         "Linear Regression: Singular Value Decomposition failed."
///     )
/// );
/// ```
#[macro_export]
macro_rules! error {
  ($error_type:ident, $msg:expr) => {
    $crate::reaper_core::$error_type { text: $msg.to_string() }.into()
  };
}
