use crate::device::Device;
use crate::vk_utils;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use common::trace;
use common::trace::*;
use render;
//use ash::version::DeviceV1_0;

pub struct System {

    pub vk_instance: ash::Instance,

    pub vk_allocation_callbacks: Option<vk::AllocationCallbacks>,

    pub devices: Vec<Device>,
}

impl System {
    pub fn new(request: &render::SystemRequest) -> Option<Box<dyn render::System>> {
        trace!("System", "new");

        if request.api_name != render::SystemRequest::vulkan_name() || request.first_unsupported_version.major < 2
        {
            return None;
        }

        match ash::Entry::new() {
            Ok(entry) => Self::init_render_system(entry, request),
            Err(error) => {
                log::error!("Cannot load vulkan entry. Error = {}.", error);
                None
            }
        }
    }

    fn get_vk_allocation_callbacks() -> Option<vk::AllocationCallbacks> {
        trace!("System", "get_vk_allocation_callbacks");

        None
    }

    fn get_vk_instance_create_info(request: &render::SystemRequest) -> vk::InstanceCreateInfo {
        trace!("System", "get_vk_instance_create_info");

        let app_info = vk::ApplicationInfo::builder()
            .application_name(
                std::ffi::CString::new(request.application_name.clone())
                    .unwrap()
                    .as_c_str(),
            )
            .application_version(vk_utils::semver_to_vk_version(&request.application_version))
            .engine_name(
                std::ffi::CString::new(request.engine_name.clone())
                    .unwrap()
                    .as_c_str(),
            )
            .engine_version(vk_utils::semver_to_vk_version(&request.engine_version))
            .api_version(vk_utils::semver_to_vk_version(
                &request.min_supported_version,
            ))
            .build();

        let mut extensions: Vec<std::ffi::CString> = Vec::new();
        let mut extension_ptrs: Vec<*const std::os::raw::c_char> = Vec::new();
        for extension_name in &request.extensions {
            extensions.push(std::ffi::CString::new(extension_name.clone()).unwrap());
            extension_ptrs.push(extensions.last().unwrap().as_ptr())
        }

        let mut layers: Vec<std::ffi::CString> = Vec::new();
        let mut layer_ptrs: Vec<*const std::os::raw::c_char> = Vec::new();
        for layer_name in &request.layers {
            layers.push(std::ffi::CString::new(layer_name.clone()).unwrap());
            layer_ptrs.push(extensions.last().unwrap().as_ptr())
        }

        vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(extension_ptrs.as_slice())
            .enabled_layer_names(layer_ptrs.as_slice())
            .build()
    }

    fn init_render_system(
        entry: ash::Entry,
        request: &render::SystemRequest,
    ) -> Option<Box<dyn render::System>> {
        trace!("System", "init_render_system");

        let create_info = Self::get_vk_instance_create_info(request);
        let vk_allocation_callbacks = Self::get_vk_allocation_callbacks();
        match unsafe { entry.create_instance(&create_info, vk_allocation_callbacks.as_ref()) } {
            Ok(instance) => {
                let mut system = Box::new(System {
                    vk_instance: instance,
                    vk_allocation_callbacks,
                    devices: vec![],
                });
                if let Ok(_) = system.init_vk_render_devices(request) {
                    Some(system)
                } else {
                    None
                }
            }
            Err(error) => {
                log::error!("Cannot create vulkan instance. Error = {}.", error);
                None
            }
        }
    }

    fn init_vk_render_devices(&mut self, request: &render::SystemRequest) -> Result<(), ()> {
        trace!("System", "init_vk_render_devices");

        match unsafe { self.vk_instance.enumerate_physical_devices() } {
            Ok(vk_physical_devices) => {
                for vk_device in vk_physical_devices {
                    if let Ok(device) = Device::new(&self, vk_device, request) {
                        self.devices.push(device);
                    }
                }
            }
            Err(error) => {
                log::error!("Cannot enumerate physical devices. Error = {}.", error);
            }
        }

        if self.devices.is_empty() {
            log::info!("No passed VK devices.");
            Err(())
        } else {
            Ok(())
        }
    }
}

impl Drop for System {
    fn drop(&mut self) {
        trace!("System", "Drop");
        self.devices.clear();
    }
}

impl render::System for System {}

#[cfg(test)]
mod tests {
    use super::*;
    // use common::init_logger;
    // use common::init_logger::*;

    #[test]
    fn create_vulkan_render_system() {
        common::init_test_logger!("create_vulkan_render_system");

        let render_system = System::new(&render::SystemRequest::request_vulkan_debug());
        assert_eq!(render_system.is_some(), true);
    }
}
