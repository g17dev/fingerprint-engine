use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FpError {
    #[error("Sensor no encontrado.")]
    SensorNotFound,

    #[error("Error USB: {0}")]
    UsbError(#[from] nusb::Error),

    #[error("Error durante la transferiacia por USB: {0}")]
    UsbTransferError(#[from] nusb::transfer::TransferError),

    #[error("Error al decifrar la huella obtenida.")]
    DecryptError,

    #[error(
        "La calidad obtenida fue de {quality}, se necesita minimo {minimum}, intentalo de nuevo."
    )]
    LowQuality { quality: u8, minimum: u8 },

    #[error("Timeout agotado {0} ms esperando leer la huella")]
    Timeout(u64),
}

#[derive(Debug)]
pub struct SensorInfo {
    pub vendor_id: u16,
    pub product_id: u16,
    pub name: &'static str,
    pub image_width: u32,
    pub image_height: u32,
}

pub struct GrayscaleImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

#[async_trait]
pub trait FingerprintDevice {
    async fn init(&mut self) -> Result<(), FpError>;

    fn sensor_info(&self) -> SensorInfo;

    async fn set_led(&mut self, on: bool) -> Result<(), FpError>;

    async fn capture_image(&mut self) -> Result<GrayscaleImage, FpError>;
}
