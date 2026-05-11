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
                    match sensor.set_led(true).await {
                        Ok(_) => println!("LED encendido"),
                        Err(e) => println!("Error LED: {}", e),
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    match sensor.set_led(false).await {
                        Ok(_) => println!("LED apagado"),
                        Err(e) => println!("Error apagando LED: {}", e),
                    }
                    
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}