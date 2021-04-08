extern crate bluez;

use std::error::Error;

mod parser;
mod publisher;
mod watcher;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let publi = publisher::Publisher::from_env();
    let mut watcher = watcher::Watcher::new(publi);

    watcher.listen().await?;

    Ok(())
}
