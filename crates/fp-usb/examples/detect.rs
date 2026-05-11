use fp_usb::driver::SensorInfo;
use fp_usb::registry;
use fp_usb::devices::uru4000::Uru4000;
use fp_usb::driver::FingerprintDevice;

#[tokio::main]
async fn main() {
    match registry::detect() {
        None => {
            println!("Sensor no encontrado");
        }
        Some(device_info) => {
            match Uru4000::open(device_info) {
                Ok(mut sensor) => {
                    println!("Sensor abierto correctamente");
                    sensor.init().await;
                    let info = sensor.sensor_info();
                    println!("{:?}", info);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}