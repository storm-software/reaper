use std::fmt::Display;

use tracing::{Subscriber};
use tracing_subscriber::{
  layer::{Layer},
  registry::LookupSpan,
  EnvFilter,
};

use crate::types::BoxedLayer;

pub fn get_stdout_layer<S>(default_directive: impl Display) -> BoxedLayer<S>
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
