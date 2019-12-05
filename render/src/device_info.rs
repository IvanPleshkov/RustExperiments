pub enum DeviceType {
    Unknown,
    Integrated,
    Discrete,
    Virtual,
    Software,
}

pub struct DeviceLimits {}

pub struct DeviceFeatures {}

pub struct DeviceInfo {
    pub name: String,

    pub vendor: String,

    pub driver_vesrion: semver::Version,

    pub device_type: DeviceType,

    pub features: DeviceFeatures,

    pub limits: DeviceLimits,
}

impl DeviceInfo {
    pub fn log(&self) {
        log::info!("Device info:");
        log::info!("Vendor: {}, Name: {}", self.vendor, self.name);
        log::info!("Driver version: {}", self.driver_vesrion);
    }
}
