use tracing::{subscriber::set_global_default, Subscriber};
use tracing_log::LogTracer;
use tracing_subscriber::{
  filter::Directive,
  layer::{Layer, SubscriberExt},
  EnvFilter, Registry,
};

use crate::tracing_layers::{ReaperTelemetryErrorLayer, ReaperTelemetryAnsiTermLayer, get_stdout_layer};
use crate::types::BoxedLayer;

/// ANSI (Console) Subscriber Composer
///
/// Builds a subscriber with multiple layers into a [tracing](https://crates.io/crates/tracing) subscriber.
pub fn get_subscriber(env_filter: String) -> impl Subscriber + Sync + Send {
  let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
  let formatting_layer = ReaperTelemetryAnsiTermLayer;

  Registry::default().with(env_filter).with(formatting_layer)
}

/// Globally registers a subscriber.
///
/// ### Panics
/// If it is called more than once.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
  LogTracer::init().expect("Failed to set logger");
  set_global_default(subscriber).expect("Failed to set subscriber");
}

/// Builds and globally registers a subscriber.
///
/// ### Panics
/// If it is called more than once.
pub fn init_telemetry(env_filter: String) {
  init_subscriber(get_subscriber(env_filter));
}

/// Initializes a new [Subscriber] based on the given layers.
///
/// ### Panics
/// If it is called more than once.
pub fn init_layers(layers: Vec<BoxedLayer<Registry>>) {
  init_subscriber(tracing_subscriber::registry().with(layers));
}

/// Initialize telemetry with the given verbosity.
///
/// ### Panics
/// If it is called more than once.
pub fn init(verbosity: Option<Directive>) {
  let layers = vec![
    get_stdout_layer(
      verbosity.unwrap_or_else(||
        format!("{}", EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .parse()
        .expect("Failed to parse verbosity directive"),
      ),
    ),
    ReaperTelemetryErrorLayer::default().boxed(),
  ];

  init_layers(layers)
}
