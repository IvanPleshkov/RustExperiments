use crate::{
    command_buffer::CommandBuffer,
    device_info::DeviceInfo,
    gpu_buffer::{GpuBuffer, GpuBufferInfo},
    gpu_texture::{GpuTexture, GpuTextureInfo},
    material::Material,
    render_target::{RenderTarget, RenderTargetInfo},
};
use std::sync::Arc;

pub trait Device {
    fn device_info(&self) -> &DeviceInfo;

    fn create_buffer(&mut self, info: GpuBufferInfo) -> Arc<GpuBuffer>;

    fn create_texture(&mut self, info: GpuTextureInfo) -> Arc<GpuTexture>;

    fn create_render_target(&mut self, info: RenderTargetInfo) -> Arc<RenderTarget>;

    fn push_material(&mut self, config: &str);

    fn get_material(&self, material_name: &str) -> Option<Arc<Material>>;

    fn create_comamnd_buffer(&mut self) -> CommandBuffer;

    fn submit_command_buffer(&mut self, command_buffer: CommandBuffer);
}
