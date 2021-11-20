use crate::parser;
use bluez::Address;
use bytes::Bytes;
use std::time::{SystemTime, UNIX_EPOCH};

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Entry {
    time: u64,
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
                time: now(),
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
}
