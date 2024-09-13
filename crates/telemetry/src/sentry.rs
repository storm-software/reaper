/// Initialize Sentry with the given DSN.
pub fn init_sentry(url: &str) {
    let _guard = sentry::init((url, sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
      }));
}
