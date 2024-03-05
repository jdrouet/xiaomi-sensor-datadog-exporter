use crate::parser;
use bluez::Address;
use bytes::Bytes;

#[derive(Clone, Debug)]
pub(crate) struct Entry {
    address: String,
    name: Option<String>,
    temperature: f32,
    humidity: u8,
    battery: u8,
}

impl Entry {
    pub fn build(address: Address, data: Bytes) -> Option<Self> {
        if parser::is_sensor(&data) {
            Some(Self {
                address: address.to_string(),
                name: parser::read_name(&data),
                temperature: parser::read_temperature(&data),
                humidity: parser::read_humidity(&data),
                battery: parser::read_battery(&data),
            })
        } else {
            None
        }
    }

    pub fn trace(&self) {
        let name = self.name.as_deref().unwrap_or("");
        tracing::info!(
            "received address={:?} name={:?} temperature={} humidity={} battery={}",
            self.address,
            name,
            self.temperature,
            self.humidity,
            self.battery
        )
    }
}
