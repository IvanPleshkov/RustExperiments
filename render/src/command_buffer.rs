use crate::{
    gpu_buffer::{GpuBuffer, GpuBufferInfo},
    gpu_mesh::GpuMesh,
    gpu_texture::{GpuTexture, GpuTextureInfo},
    material::MaterialInstance,
    render_target::{RenderTarget, RenderTargetInfo},
};
use std::sync::Arc;

pub struct UpdateGpuBufferCommand {
    pub gpu_buffer: Arc<GpuBuffer>,
    pub info: GpuBufferInfo,
}

pub struct UpdateGpuTextureCommand {
    pub gpu_texture: Arc<GpuTexture>,
    pub info: GpuTextureInfo,
}

pub struct UpdateRenderTargetCommand {
    pub render_target: Arc<RenderTarget>,
    pub info: RenderTargetInfo,
}

pub struct FillGpuBufferCommand {
    pub gpu_buffer: Arc<GpuBuffer>,
    pub data: Vec<u8>,
}

pub struct FillGpuTextureCommand {
    pub gpu_texture: Arc<GpuTexture>,
    pub data: Vec<u8>,
}

pub struct ReadbackGpuBufferCommand {
    pub gpu_buffer: Arc<GpuBuffer>,
}

pub struct ReadbackGpuTextureCommand {
    pub gpu_texture: Arc<GpuTexture>,
}

pub struct DrawCommand {
    pub material_instance: Arc<MaterialInstance>,
    pub mesh: Arc<GpuMesh>,
}

pub struct SetRenderTargetCommand {
    pub render_target: Arc<RenderTarget>,
}

pub enum Command {
    UpdateGpuBuffer(UpdateGpuBufferCommand),
    UpdateGpuTexture(UpdateGpuTextureCommand),
    UpdateRenderTarget(UpdateRenderTargetCommand),
    FillGpuBuffer(FillGpuBufferCommand),
    FillGpuTexture(FillGpuTextureCommand),
    ReadbackGpuBuffer(ReadbackGpuBufferCommand),
    ReadbackGpuTexture(ReadbackGpuTextureCommand),
    Draw(DrawCommand),
    SetRenderTarget(SetRenderTargetCommand),
}

pub struct CommandBuffer {
    pub id: u64,

    pub unique_id_index: u64,

    pub commands: Vec<Command>,
}

impl CommandBuffer {
    pub fn new(id: u64) -> CommandBuffer {
        CommandBuffer {
            id: id,
            unique_id_index: 0,
            commands: Vec::new(),
        }
    }

    pub fn generate_unique_id(&mut self) -> u64 {
        self.unique_id_index = self.unique_id_index + 1;
        if self.unique_id_index == 64000 {
            panic!("Command buffer unique index overflow. Try to use more command buffers for resource creation.");
        }
        self.unique_id_index + (self.id << 32)
    }

    pub fn create_gpu_buffer(&mut self, info: GpuBufferInfo) -> Arc<GpuBuffer> {
        GpuBuffer::new(self, info)
    }

    pub fn create_gpu_texture(&mut self, info: GpuTextureInfo) -> Arc<GpuTexture> {
        GpuTexture::new(self, info)
    }

    pub fn create_render_target(&mut self, info: RenderTargetInfo) -> Arc<RenderTarget> {
        RenderTarget::new(self, info)
    }

    pub fn fill_gpu_buffer(&mut self, _gpu_buffer: Arc<GpuBuffer>, _data: Vec<u8>) {
        unimplemented!()
    }

    pub fn fill_gpu_texture(&mut self, _gpu_texture: Arc<GpuTexture>, _data: Vec<u8>) {
        unimplemented!()
    }

    pub fn readback_gpu_buffer(&mut self, _gpu_buffer: Arc<GpuBuffer>) {
        unimplemented!()
    }

    pub fn readback_gpu_texture(&mut self, _gpu_texture: Arc<GpuTexture>) {
        unimplemented!()
    }

    pub fn draw(&mut self, material_instance: Arc<MaterialInstance>, mesh: Arc<GpuMesh>) {
        self.commands.push(Command::Draw(DrawCommand {
            material_instance: material_instance,
            mesh: mesh,
        }));
    }

    pub fn set_render_target(&mut self, render_target: Arc<RenderTarget>) {
        self.commands
            .push(Command::SetRenderTarget(SetRenderTargetCommand {
                render_target: render_target,
            }));
    }
}
