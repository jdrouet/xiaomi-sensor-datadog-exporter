mod listener;
mod model;
mod parser;
mod publisher;

use clap::Parser;
use std::error::Error;
use tokio::sync::mpsc;

#[derive(Parser)]
struct Command {
    #[clap(long, about = "Size of the buffer", default_value = "10")]
    buffer_size: usize,
    address: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let cmd = Command::parse();

    let (sender, receiver) = mpsc::channel(cmd.buffer_size);

    tokio::spawn(async {
        listener::Listener::new()
            .run(sender)
            .await
            .map_err(|err| err.to_string())
    });

    publisher::Publisher::new(&cmd.address)
        .run(receiver)
        .await?;

    Ok(())
}
