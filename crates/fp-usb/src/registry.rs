// Listado de sensores soportados con VID, PID y nombre
const SUPPORTED_SENSORS: &[(u16, u16, &str)] = &[(0x05ba, 0x000a, "DigitalPersona URU4500")];

pub fn detect() -> Option<nusb::DeviceInfo> {
    let mut devices = nusb::list_devices().ok()?;

    devices.find(|device| {
        let vid = device.vendor_id();
        let pid = device.product_id();

        SUPPORTED_SENSORS.iter().any(|&(v, p, _)| {
            v == vid && p == pid
        })
    })
}
