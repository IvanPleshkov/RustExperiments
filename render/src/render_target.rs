use crate::command_buffer::CommandBuffer;
use crate::gpu_texture::GpuTexture;
use std::sync::Arc;

pub struct RenderTarget {
    pub id: u64,

    pub color_texture: Option<Arc<GpuTexture>>,

    pub depth_texture: Option<Arc<GpuTexture>>,

    pub stencil_texture: Option<Arc<GpuTexture>>,

    pub info: RenderTargetInfo,
}

pub struct RenderTargetInfo {}

impl RenderTarget {
    pub fn new(command_buffer: &mut CommandBuffer, info: RenderTargetInfo) -> Arc<RenderTarget> {
        Arc::new(RenderTarget {
            id: command_buffer.generate_unique_id(),
            color_texture: None,
            depth_texture: None,
            stencil_texture: None,
            info: info,
        })
    }
}
