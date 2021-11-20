use crate::model::Entry;
use bluez::client::*;
use bluez::interface::controller::*;
use bluez::interface::event::Event;
use bluez::Address;
use bytes::Bytes;
use log;
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

pub struct Listener<'b>(BlueZClient<'b>);

impl<'b> Listener<'b> {
    pub fn new() -> Self {
        Self(BlueZClient::new().unwrap())
    }

    async fn handle_device_found(&mut self, address: Address, data: Bytes, sender: Sender<Entry>) {
        if let Some(entry) = Entry::build(address, data) {
            log::info!("received {:?}", entry);
            match sender.send(entry).await {
                Ok(_) => log::info!("entry sent"),
                Err(err) => log::error!("error sending: {:?}", err),
            };
        } else {
            log::debug!("{} is not a sensor...", address);
        }
    }

    async fn get_supported_controller(
        &mut self,
    ) -> Result<(Controller, ControllerInfo), Box<dyn Error>> {
        let controllers = self.0.get_controller_list().await?;
        for ctrl in controllers.into_iter() {
            let info = self.0.get_controller_info(ctrl).await?;
            if info.supported_settings.contains(ControllerSetting::Powered) {
                return Ok((ctrl, info));
            }
        }
        panic!("no usable controllers found");
    }

    pub async fn run(&mut self, sender: Sender<Entry>) -> Result<(), Box<dyn Error>> {
        let (controller, info) = self.get_supported_controller().await?;

        if !info.current_settings.contains(ControllerSetting::Powered) {
            log::info!("powering on bluetooth controller {}", controller);
            self.0.set_powered(controller, true).await?;
        }

        // scan for some devices
        // to do this we'll need to listen for the Device Found event

        self.0
            .start_discovery(
                controller,
                AddressTypeFlag::BREDR | AddressTypeFlag::LEPublic | AddressTypeFlag::LERandom,
            )
            .await?;

        // just wait for discovery forever
        loop {
            // process() blocks until there is a response to be had
            let response = self.0.process().await?;

            match response.event {
                Event::DeviceFound {
                    address, eir_data, ..
                } => {
                    self.handle_device_found(address, eir_data, sender.clone())
                        .await;
                }
                Event::Discovering { discovering, .. } => {
                    log::debug!("discovering: {}", discovering);
                    // if discovery ended, turn it back on
                    if !discovering {
                        self.0
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
