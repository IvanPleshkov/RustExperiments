use render;

mod device;
mod system;

pub fn create_vulkan_render_system(
    request: &render::SystemRequest,
) -> Option<Box<dyn render::System>> {
    system::System::new(request)
}
