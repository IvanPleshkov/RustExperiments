
pub enum DeviceType {
    Unknown,
    Integrated,
    Discrete,
    Virtual,
    Software,
}

pub trait Device {
    // fn create_comamnd_buffer(&mut self) -> RenderCommandBuffer;
    // fn submit_command_buffer(&mut self);
    // fn render_device_features(&self) -> &RenderDeviceFeatures;
}
