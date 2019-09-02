use render;

mod system;
mod device;

pub fn create_vulkan_render_system(request: &render::SystemRequest) -> Option<Box<dyn render::System>> {
    system::System::new(request)
}
