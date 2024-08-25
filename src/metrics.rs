use std::borrow::Cow;
use std::time::Duration;

use metrics::{describe_gauge, Unit};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;

const BATTERY: &str = "battery";
const HUMIDITY: &str = "humidity";
const TEMPERATURE: &str = "temperature";

const ADDRESS: &str = "address";
const NAME: &str = "name";

pub(crate) fn install() {
    let listener = std::env::var("LISTENER")
        .ok()
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed("./metrics.sock"));
    let builder = PrometheusBuilder::new().with_http_uds_listener(listener.as_ref());
    builder
        .idle_timeout(MetricKindMask::GAUGE, Some(Duration::from_secs(60)))
        .install()
        .expect("failed to install Prometheus recorder");
}

pub(crate) fn register() {
    describe_gauge!(BATTERY, Unit::Percent, "battery left in the sensor");
    describe_gauge!(HUMIDITY, Unit::Percent, "humidity in the room");
    describe_gauge!(TEMPERATURE, "temperature in the room");
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
