use crate::gpu_mesh::GpuMesh;
use crate::material::MaterialInstance;
use crate::render_target::RenderTarget;
use std::sync::Arc;

pub struct DrawCommand {
    pub material_instance: Arc<MaterialInstance>,
    pub mesh: Arc<GpuMesh>,
}

pub struct SetRenderTargetCommand {
    pub render_target: Arc<RenderTarget>,
}

pub enum Command {
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
