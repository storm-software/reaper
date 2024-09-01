use std::fmt::Display;

use ansi_term::Colour::{Blue, Cyan, Purple, Red, Yellow};
use tracing::{subscriber::set_global_default, Level, Subscriber};
use tracing_log::LogTracer;
use tracing_subscriber::{
  filter::Directive,
  layer::{Layer, SubscriberExt},
  prelude::__tracing_subscriber_SubscriberExt,
  registry::LookupSpan,
  util::SubscriberInitExt,
  EnvFilter, Layer, Registry,
};

/// A boxed tracing Layer.
pub type BoxedLayer<S> = Box<dyn Layer<S> + Send + Sync>;

pub fn stdout<S>(default_directive: impl Display) -> BoxedLayer<S>
where
  S: Subscriber,
  for<'a> S: LookupSpan<'a>,
{
  let filter = EnvFilter::builder()
    .with_default_directive(default_directive.to_string().parse().unwrap())
    .from_env_lossy()
    .add_directive("hyper::proto::h1=off".parse().unwrap());

  tracing_subscriber::fmt::layer()
    .with_ansi(true)
    .with_target(true)
    .with_filter(filter)
    .boxed()
}

pub struct AnsiVisitor;

impl tracing::field::Visit for AnsiVisitor {
  fn record_f64(&mut self, _: &tracing::field::Field, value: f64) {
    println!("{value}")
  }

  fn record_i64(&mut self, _: &tracing::field::Field, value: i64) {
    println!("{value}")
  }

  fn record_u64(&mut self, _: &tracing::field::Field, value: u64) {
    println!("{value}")
  }

  fn record_bool(&mut self, _: &tracing::field::Field, value: bool) {
    println!("{value}")
  }

  fn record_str(&mut self, _: &tracing::field::Field, value: &str) {
    println!("{value}")
  }

  fn record_error(&mut self, _: &tracing::field::Field, value: &(dyn std::error::Error + 'static)) {
    println!("{value}")
  }

  fn record_debug(&mut self, _: &tracing::field::Field, value: &dyn std::fmt::Debug) {
    println!("{value:?}")
  }
}

/// An Ansi Term layer for tracing
pub struct AsniTermLayer;

impl<S> Layer<S> for AsniTermLayer
where
  S: tracing::Subscriber,
{
  fn on_event(&self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
    // Print the timestamp
    let utc: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
    print!("[{}] ", Cyan.paint(utc.to_rfc2822()));

    // Print the level prefix
    match *event.metadata().level() {
      Level::ERROR => {
        eprint!("{}: ", Red.paint("ERROR"));
      }
      Level::WARN => {
        print!("{}: ", Yellow.paint("WARN"));
      }
      Level::INFO => {
        print!("{}: ", Blue.paint("INFO"));
      }
      Level::DEBUG => {
        print!("DEBUG: ");
      }
      Level::TRACE => {
        print!("{}: ", Purple.paint("TRACE"));
      }
    }

    print!("{} ", Purple.paint(event.metadata().target()));
    print!(
      "at {} ",
      Cyan.paint(
        event
          .metadata()
          .name()
          .split(' ')
          .last()
          .unwrap_or_default()
      )
    );
    let mut visitor = AnsiVisitor;
    event.record(&mut visitor);
  }
}

/// ASNI (Console) Subscriber Composer
///
/// Builds a subscriber with multiple layers into a [tracing](https://crates.io/crates/tracing) subscriber.
pub fn get_subscriber(env_filter: String) -> impl Subscriber + Sync + Send {
  let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
  let formatting_layer = AsniTermLayer;

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
pub fn init_layers(layers: Vec<BoxedLayer<Registry>>) {
  let _ = tracing_subscriber::registry().with(layers).try_init();
}

/// Initializes a new [Subscriber] based on the given layers.
pub fn init_layers(layers: Vec<BoxedLayer<Registry>>) {
  let _ = tracing_subscriber::registry().with(layers).try_init();
}

/// Initialize telemetry with the given verbosity.
pub fn init(verbosity: Some<Directive>) {
  let layers = vec![
    reaper_telemetry::stdout(
      verbosity.unwrap_or_default(
        format!(EnvFilter::try_from_default_env())
          .parse()
          .unwrap_or_default(Level::INFO),
      ),
    ),
    reaper_eth_engine_metrics::error_layer::ReaperEthEngineErrorMetrics::default().boxed(),
  ];

  init_layers(layers)
}
