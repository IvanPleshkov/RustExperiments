use crate::render_device::RenderDevice;

pub trait RenderSystem {
    fn render_devices_count() -> usize;
    fn get_render_device(&self) -> &Box<RenderDevice>;
}
