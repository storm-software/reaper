use sentry::{init, ClientOptions, release_name};

/// Initialize Sentry with the given DSN.
pub fn init_sentry(url: &str) {
    let _guard = init((url, ClientOptions {
        release: release_name!(),
        ..Default::default()
      }));
}
