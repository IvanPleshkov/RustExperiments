use render;

mod system;
mod device;

pub fn create_vulkan_render_system(params: &render::SystemInitializationParams) -> Option<Box<dyn render::System>> {
    system::System::new(params)
}
