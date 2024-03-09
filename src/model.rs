use crate::parser;
use bluer::Address;

#[derive(Clone, Debug)]
pub(crate) struct Entry {
    address: Address,
    name: String,
    temperature: f32,
    humidity: u8,
    battery: u8,
}

impl Entry {
    pub fn build(address: Address, name: String, data: &[u8]) -> Option<Self> {
        Some(Self {
            address,
            name,
            temperature: parser::read_temperature(&data)?,
            humidity: parser::read_humidity(&data)?,
            battery: parser::read_battery(&data)?,
        })
    }

    pub fn trace(&self) {
        tracing::info!(
            "service data received address={:?} name={:?} temperature={} humidity={} battery={}",
            self.address,
            self.name,
            self.temperature,
            self.humidity,
            self.battery
        )
    }
}
