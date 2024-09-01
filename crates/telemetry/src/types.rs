use tracing_subscriber::layer::Layer;

/// A boxed tracing Layer.
pub type BoxedLayer<S> = Box<dyn Layer<S> + Send + Sync>;
