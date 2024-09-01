use tracing::Level;
use tracing_subscriber::Layer;
use ansi_term::Colour::{Blue, Cyan, Purple, Red, Yellow};

pub struct ReaperTelemetryAnsiVisitor;

impl tracing::field::Visit for ReaperTelemetryAnsiVisitor {
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
pub struct ReaperTelemetryAnsiTermLayer;

impl<S> Layer<S> for ReaperTelemetryAnsiTermLayer
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
    let mut visitor = ReaperTelemetryAnsiVisitor;
    event.record(&mut visitor);
  }
}
