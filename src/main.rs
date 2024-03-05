mod listener;
mod model;
mod parser;

use std::error::Error;

fn init_logs(directive: &str) {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(directive))
        .with(tracing_subscriber::fmt::layer().with_ansi(false))
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(level) = std::env::var("LOG") {
        init_logs(&level);
    } else {
        init_logs("debug");
    }

    listener::Listener::new().run().await
}
