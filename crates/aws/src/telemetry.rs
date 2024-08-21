use tracing_subscriber::{fmt, EnvFilter};

/// Builds and globally registers a subscriber.
///
/// ### Panics
///
/// If it is called more than once.
pub fn init_telemetry(env_filter: String) {
  fmt()
    .json()
    .with_env_filter(
      EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter)),
    )
    // .with_current_span(false)
    .without_time()
    .with_target(false)
    .with_file(true)
    .with_line_number(true)
    .init()
}
