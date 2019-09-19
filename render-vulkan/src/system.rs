use render;
use crate::device::VulkanRenderDevice;
use ash::vk;
use ash::version::EntryV1_0;

pub struct System {

    pub vk_instance: ash::Instance,

    pub vk_allocation_callbacks: Option<vk::AllocationCallbacks>,

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

    fn get_vk_allocation_callbacks() -> Option<vk::AllocationCallbacks> {
        None
    }

    fn get_vk_instance_create_info(request: &render::SystemRequest) -> vk::InstanceCreateInfo {
        let app_info = vk::ApplicationInfo::builder()
            .application_name(std::ffi::CString::new(
                request.application_name.clone()).unwrap().as_c_str())
            .application_version(ash::vk_make_version!(
                request.application_version.major,
                request.application_version.minor,
                request.application_version.patch))
            .engine_name(std::ffi::CString::new(
                request.engine_name.clone()).unwrap().as_c_str())
            .engine_version(ash::vk_make_version!(
                request.application_version.major,
                request.application_version.minor,
                request.application_version.patch))
            .api_version(ash::vk_make_version!(
                request.min_supported_version.major,
                request.min_supported_version.minor,
                request.min_supported_version.patch))
            .build();

        vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            //.enabled_extension_names(enabled_extension_names: &'a [*const c_char])
            //.enabled_layer_names(enabled_layer_names: &'a [*const c_char])
            .build()
    }

    fn init_render_system(entry: ash::Entry, request: &render::SystemRequest) -> Option<Box<dyn render::System>> {
        log::trace!("render_vulkan::System::init_render_system");

        let create_info = Self::get_vk_instance_create_info(request);
        let vk_allocation_callbacks = Self::get_vk_allocation_callbacks();
        match unsafe { entry.create_instance(&create_info, vk_allocation_callbacks.as_ref()) } {
            Ok(instance) => {
                let mut system = Box::new(System {
                    vk_instance: instance,
                    vk_allocation_callbacks: vk_allocation_callbacks,
                    devices: vec![],
                });
                if let Ok(_) = system.init_vk_render_devices(request) {
                    Some(system)
                } else {
                    None
                }
            },
            Err(error) => {
                log::error!("Cannot create vulkan instance. Error = {}.", error);
                None
            },
        }
    }

    fn init_vk_render_devices(&mut self, _request: &render::SystemRequest) -> Result<(),()> {
        Ok(())
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
