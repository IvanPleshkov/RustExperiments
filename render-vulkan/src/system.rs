use render;
use crate::device::VulkanRenderDevice;
use ash::vk;
use ash::version::EntryV1_0;

pub struct System {

    pub vk_instance: ash::Instance,

    pub devices: Vec<Box<VulkanRenderDevice>>,
}

impl System {

    pub fn new(request: &render::SystemRequest) -> Option<Box<dyn render::System>> {
        log::trace!("render_vulkan::System::new");

        match ash::Entry::new() {
            Ok(entry) => Self::init_render_system(entry, request),
            Err(error) => {
                log::error!("Cannot load vulkan entry. Error = {}.", error);
                None
            },
        }
    }

    fn init_render_system(entry: ash::Entry, request: &render::SystemRequest) -> Option<Box<dyn render::System>> {
        log::trace!("render_vulkan::System::init_render_system");

        let app_info = vk::ApplicationInfo::builder()
            .api_version(ash::vk_make_version!(1, 1, 0))
            .build();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .build();

        match unsafe { entry.create_instance(&create_info, None) } {
            Ok(instance) => Self::init_vk_render_devices(instance, request),
            Err(error) => {
                log::error!("Cannot create vulkan instance. Error = {}.", error);
                None
            },
        }
    }

    fn init_vk_render_devices(instance: ash::Instance, request: &render::SystemRequest) -> Option<Box<dyn render::System>> {
        Some(Box::new(System {
            vk_instance: instance,
            devices: vec![],
        }))
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

        let default_params = render::SystemRequest::request_vulkan_debug();
        let render_system = System::new(&default_params);
        assert_eq!(render_system.is_some(), true);
    }
}
