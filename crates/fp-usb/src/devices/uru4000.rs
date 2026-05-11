use crate::FpError;
use async_trait::async_trait;
use crate::driver::{FingerprintDevice, GrayscaleImage, SensorInfo};
pub struct Uru4000 {
    interface: Option<nusb::Interface>,
    is_connected: bool
}

impl Uru4000 {
    pub fn open(device_info: nusb::DeviceInfo) -> Result<Self, FpError> {        let device: nusb::Device = device_info.open()?;
        let interface = device.claim_interface(0)?;

        Ok(Self {
            interface: Some(interface),
            is_connected: false,
        })
    }
}

#[async_trait]
impl FingerprintDevice for Uru4000 {
    async fn init(&mut self) -> Result<(), FpError> {
        self.is_connected = true;
        Ok(())
    }

    fn sensor_info(&self) -> SensorInfo {
        SensorInfo {
            vendor_id: 0x05ba,
            product_id: 0x000a,
            name: "DigitalPersona URU4500",
            image_width: 384,
            image_height: 290,
        }
    }

    async fn set_led(&mut self, on: bool) -> Result<(), FpError> {
        Ok(())
    }

    async fn capture_image(&mut self) -> Result<GrayscaleImage, FpError> {
        Err(FpError::DecryptError)
    }
}