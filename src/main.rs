use std::borrow::Cow;

mod metrics;
mod parser;
mod scanner;

fn init_logs() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let directive = std::env::var("LOG")
        .ok()
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("debug"));

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(directive))
        .with(tracing_subscriber::fmt::layer().with_ansi(false))
        .init();
}

#[tokio::main]
async fn main() -> bluer::Result<()> {
    init_logs();

    crate::metrics::register();
    crate::metrics::install();

    scanner::Scanner::new().await?.run().await
}
