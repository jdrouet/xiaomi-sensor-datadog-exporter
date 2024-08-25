mod metrics;
mod parser;
mod scanner;

fn init_logs(directive: &str) {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(directive))
        .with(tracing_subscriber::fmt::layer().with_ansi(false))
        .init();
}

#[tokio::main]
async fn main() -> bluer::Result<()> {
    if let Ok(level) = std::env::var("LOG") {
        init_logs(&level);
    } else {
        init_logs("debug");
    }

    crate::metrics::register();
    scanner::Scanner::new().await?.run().await
}
