extern crate bluez;

use crate::parser;
use crate::publisher::Publisher;
use bluez::client::*;
use bluez::interface::controller::*;
use bluez::interface::event::Event;
use bluez::Address;
use bytes::Bytes;
use datadog_client::metrics::{Point, Serie, Type};
use log;
use std::error::Error;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn create_serie(
    address: &str,
    name: &Option<String>,
    timestamp: u64,
    key: &str,
    value: f64,
) -> Serie {
    let serie = Serie::new(key.to_string(), Type::Gauge)
        .set_host(address.to_string())
        .add_point(Point::new(timestamp, value));
    if let Some(name) = name {
        serie.add_tag(format!("sensor_name:{}", name))
    } else {
        serie
    }
}

fn create_series(address: &str, data: &Bytes) -> Vec<Serie> {
    let name = parser::read_name(data);
    log::debug!("create series for {} {:?}", address, name);
    let timestamp = now();
    vec![
        create_serie(
            address,
            &name,
            timestamp,
            "temperature",
            parser::read_temperature(data) as f64,
        ),
        create_serie(
            address,
            &name,
            timestamp,
            "humidity",
            parser::read_humidity(data) as f64,
        ),
        create_serie(
            address,
            &name,
            timestamp,
            "battery",
            parser::read_battery(data) as f64,
        ),
    ]
}

pub struct Watcher<'b> {
    client: BlueZClient<'b>,
    publisher: Publisher,
}

impl<'b> Watcher<'b> {
    pub fn new(publisher: Publisher) -> Self {
        Self {
            client: BlueZClient::new().unwrap(),
            publisher,
        }
    }

    async fn handle_device_found(&mut self, address: &Address, data: &Bytes) {
        if parser::is_sensor(data) {
            let series = create_series(&address.to_string(), data);
            log::info!("received {:?}", series);
            match self.publisher.send(series).await {
                Ok(_) => log::info!("measurement saved"),
                Err(err) => log::error!("error while saving: {:?}", err),
            };
        } else {
            log::debug!("{} is not a sensor...", address);
        }
    }

    async fn get_supported_controller(
        &mut self,
    ) -> Result<(Controller, ControllerInfo), Box<dyn Error>> {
        let controllers = self.client.get_controller_list().await?;
        for ctrl in controllers.into_iter() {
            let info = self.client.get_controller_info(ctrl).await?;
            if info.supported_settings.contains(ControllerSetting::Powered) {
                return Ok((ctrl, info));
            }
        }
        panic!("no usable controllers found");
    }

    pub async fn listen(&mut self) -> Result<(), Box<dyn Error>> {
        let (controller, info) = self.get_supported_controller().await?;

        if !info.current_settings.contains(ControllerSetting::Powered) {
            log::info!("powering on bluetooth controller {}", controller);
            self.client.set_powered(controller, true).await?;
        }

        // scan for some devices
        // to do this we'll need to listen for the Device Found event

        self.client
            .start_discovery(
                controller,
                AddressTypeFlag::BREDR | AddressTypeFlag::LEPublic | AddressTypeFlag::LERandom,
            )
            .await?;

        // just wait for discovery forever
        loop {
            // process() blocks until there is a response to be had
            let response = self.client.process().await?;

            match response.event {
                Event::DeviceFound {
                    address, eir_data, ..
                } => {
                    self.handle_device_found(&address, &eir_data).await;
                }
                Event::Discovering { discovering, .. } => {
                    log::debug!("discovering: {}", discovering);
                    // if discovery ended, turn it back on
                    if !discovering {
                        self.client
                            .start_discovery(
                                controller,
                                AddressTypeFlag::BREDR
                                    | AddressTypeFlag::LEPublic
                                    | AddressTypeFlag::LERandom,
                            )
                            .await?;
                    }
                }
                _ => (),
            }

            sleep(Duration::from_millis(50)).await;
        }
    }
}
