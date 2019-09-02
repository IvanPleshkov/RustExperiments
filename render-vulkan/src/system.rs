use render;
use crate::device::VulkanRenderDevice;
use ash::vk;
use ash::version::EntryV1_0;

pub struct System {
    vk_instance: ash::Instance,
    devices: Vec<Box<VulkanRenderDevice>>,
}

impl System {

    pub fn new(params: &render::SystemInitializationParams) -> Option<Box<dyn render::System>> {
        log::trace!("render_vulkan::System::new");

        match ash::Entry::new() {
            Ok(entry) => Self::init_render_system(entry, params),
            Err(error) => {
                log::error!("Cannot load vulkan entry. Error = {}.", error);
                None
            },
        }
    }

    fn init_render_system(entry: ash::Entry, params: &render::SystemInitializationParams) -> Option<Box<dyn render::System>> {
        log::trace!("render_vulkan::System::init_render_system");

        let app_info = vk::ApplicationInfo::builder()
            .api_version(ash::vk_make_version!(1, 1, 0))
            .build();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .build();

        match unsafe { entry.create_instance(&create_info, None) } {
            Ok(instance) => Self::init_vk_render_devices(instance, params),
            Err(error) => {
                log::error!("Cannot create vulkan instance. Error = {}.", error);
                None
            },
        }
    }

    fn init_vk_render_devices(instance: ash::Instance, params: &render::SystemInitializationParams) -> Option<Box<dyn render::System>> {
        None
    }
}

impl render::System for System {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vulkan_render_system() {
        log::warn!("Start test");

        let default_params = render::SystemInitializationParams::default();
        let render_system = System::new(&default_params);
        assert_eq!(render_system.is_some(), true);
    }
}
