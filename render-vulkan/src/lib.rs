use render;

mod device;
mod system;
mod vk_utils;

pub fn create_vulkan_render_system(
    request: &render::SystemRequest,
) -> Option<Box<dyn render::System>> {
    system::System::new(request)
}
