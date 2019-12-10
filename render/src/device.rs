use crate::{
    command_buffer::CommandBuffer,
    device_info::DeviceInfo,
    material::Material,
};
use std::sync::Arc;

pub trait Device {
    fn device_info(&self) -> &DeviceInfo;

    fn push_material(&mut self, config: &str);

    fn get_material(&self, material_name: &str) -> Option<Arc<Material>>;

    fn create_comamnd_buffer(&mut self) -> CommandBuffer;

    fn submit_command_buffer(&mut self, command_buffer: CommandBuffer);

    fn wait_idle(&mut self);
}
