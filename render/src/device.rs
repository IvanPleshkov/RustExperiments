
pub enum DeviceType {
    Unknown,
    Integrated,
    Discrete,
    Virtual,
    Software,
}

pub struct DeviceInfo {
    
    pub name: String,

    pub vendor: String,

    pub driver_vesrion: semver::Version,

    pub device_type: DeviceType,
}

pub trait Device {

    fn device_info(&self) -> &DeviceInfo;

    // fn create_comamnd_buffer(&mut self) -> RenderCommandBuffer;
    // fn submit_command_buffer(&mut self);
    // fn render_device_features(&self) -> &RenderDeviceFeatures;
}

impl DeviceInfo {
    pub fn log(&self) {

    }
}
