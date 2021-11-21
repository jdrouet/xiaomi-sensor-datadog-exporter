use crate::model::Entry;
use std::io::{Error, Write};
use std::net::TcpStream;
use tokio::sync::mpsc::Receiver;

pub struct Publisher(TcpStream);

impl Publisher {
    pub fn new(address: &str) -> Self {
        Self(TcpStream::connect(address).expect("unable to connect tcp server"))
    }

    pub async fn run(&mut self, mut receiver: Receiver<Entry>) -> Result<(), Error> {
        log::debug!("waiting for events...");
        while let Some(event) = receiver.recv().await {
            log::debug!("received: {:?}", event);
            let payload = serde_json::to_string(&event).unwrap();
            writeln!(self.0, "{}", payload)?;
        }
        Ok(())
    }
}
