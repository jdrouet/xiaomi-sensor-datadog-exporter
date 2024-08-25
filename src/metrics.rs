const BATTERY: &str = "battery";
const HUMIDITY: &str = "humidity";
const TEMPERATURE: &str = "temperature";

const ADDRESS: &str = "address";
const NAME: &str = "name";

pub(crate) fn register() {
    ::metrics::describe_gauge!(
        BATTERY,
        ::metrics::Unit::Percent,
        "amount of battery left in the sensor"
    );
    ::metrics::describe_gauge!(
        HUMIDITY,
        ::metrics::Unit::Percent,
        "amount of humidity in the room"
    );
}

#[inline(always)]
pub(crate) fn push_battery(address: String, name: String, value: u8) {
    metrics::gauge!(BATTERY, ADDRESS => address, NAME => name).set(value as f32);
}

#[inline(always)]
pub(crate) fn push_humidity(address: String, name: String, value: u8) {
    metrics::gauge!(HUMIDITY, ADDRESS => address, NAME => name).set(value as f32);
}

#[inline(always)]
pub(crate) fn push_temperature(address: String, name: String, value: f32) {
    metrics::gauge!(TEMPERATURE, ADDRESS => address, NAME => name).set(value);
}
