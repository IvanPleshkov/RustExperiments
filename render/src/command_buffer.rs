use crate::{
    gpu_buffer::{GpuBuffer, GpuBufferInfo},
    gpu_texture::{GpuTexture, GpuTextureInfo},
    gpu_mesh::GpuMesh,
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
    CreateGpuBuffer(CreateGpuBufferCommand),
    CreateGpuTexture(CreateGpuTextureCommand),
    CreateRenderTarget(CreateRenderTargetCommand),
    FillGpuBuffer(FillGpuBufferCommand),
    FillGpuTexture(FillGpuTextureCommand),
    ReadbackGpuBuffer(ReadbackGpuBufferCommand),
    ReadbackGpuTexture(ReadbackGpuTextureCommand),
    Draw(DrawCommand),
    SetRenderTarget(SetRenderTargetCommand),
}

pub struct CommandBuffer {
    pub commands: Vec<Command>,
}

impl CommandBuffer {
    pub fn new() -> CommandBuffer {
        CommandBuffer {
            commands: Vec::new(),
        }
    }

    pub fn create_gpu_buffer(gpu_buffer: Arc<GpuBuffer>, info: GpuBufferInfo) -> Arc<GpuBuffer> {
        unimplemented!()
    }

    pub fn create_gpu_texture(gpu_texture: Arc<GpuTexture>, info: GpuTextureInfo) -> Arc<GpuTexture> {
        unimplemented!()
    }

    pub fn create_render_target(render_target: Arc<RenderTarget>, info: RenderTargetInfo) -> Arc<RenderTarget> {
        unimplemented!()
    }

    pub fn fill_gpu_buffer(gpu_buffer: Arc<GpuBuffer>, data: Vec<u8>) {
        unimplemented!()
    }

    pub fn fill_gpu_texture(gpu_texture: Arc<GpuTexture>, data: Vec<u8>) {
        unimplemented!()
    }

    pub fn readback_gpu_buffer(gpu_buffer: Arc<GpuBuffer>) {
        unimplemented!()
    }

    pub fn readback_gpu_texture(gpu_texture: Arc<GpuTexture>) {
        unimplemented!()
    }

    pub fn draw(&mut self, material_instance: Arc<MaterialInstance>, mesh: Arc<GpuMesh>) {
        self.commands.push(Command::Draw(DrawCommand {
            material_instance: material_instance,
            mesh: mesh,
        }));
    }

    pub fn set_render_target(&mut self, render_target: Arc<RenderTarget>) {
        self.commands.push(Command::SetRenderTarget(SetRenderTargetCommand {
            render_target: render_target,
        }));
    }
}
