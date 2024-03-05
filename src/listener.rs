use crate::model::Entry;
use bluez::management::interface::{Controller, ControllerSetting, Event};
use bluez::management::{
    get_controller_info, get_controller_list, set_powered, set_scan_parameters, start_discovery,
    AddressTypeFlag, ControllerInfo, ManagementStream,
};
use bluez::Address;
use bytes::Bytes;

use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

pub struct Listener(ManagementStream);

impl Listener {
    pub fn new() -> Self {
        Self(ManagementStream::open().unwrap())
    }

    async fn handle_device_found(&mut self, address: Address, data: Bytes) {
        if let Some(entry) = Entry::build(address, data) {
            entry.trace();
        } else {
            tracing::debug!("{} is not a sensor...", address);
        }
    }

    async fn get_supported_controller(
        &mut self,
    ) -> Result<(Controller, ControllerInfo), Box<dyn Error>> {
        let controllers = get_controller_list(&mut self.0, None).await?;
        for ctrl in controllers.into_iter() {
            let info = get_controller_info(&mut self.0, ctrl, None).await?;
            if info.supported_settings.contains(ControllerSetting::Powered) {
                return Ok((ctrl, info));
            }
        }
        panic!("no usable controllers found");
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let (controller, info) = self.get_supported_controller().await?;

        if !info.current_settings.contains(ControllerSetting::Powered) {
            tracing::info!("powering on bluetooth controller {}", controller);
            set_powered(&mut self.0, controller, true, None).await?;
        }

        // scan for some devices
        // to do this we'll need to listen for the Device Found event

        start_discovery(
            &mut self.0,
            controller,
            AddressTypeFlag::BREDR | AddressTypeFlag::LEPublic | AddressTypeFlag::LERandom,
            None,
        )
        .await?;

        // just wait for discovery forever
        loop {
            // process() blocks until there is a response to be had
            let response = self.0.receive().await?;

            match response.event {
                Event::DeviceFound {
                    address, eir_data, ..
                } => {
                    self.handle_device_found(address, eir_data).await;
                }
                Event::Discovering { discovering, .. } => {
                    tracing::debug!("discovering: {}", discovering);
                    // if discovery ended, turn it back on
                    if !discovering {
                        start_discovery(
                            &mut self.0,
                            controller,
                            AddressTypeFlag::BREDR
                                | AddressTypeFlag::LEPublic
                                | AddressTypeFlag::LERandom,
                            None,
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
