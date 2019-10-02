use crate::system::System;
use crate::vk_utils;
use ash::version::DeviceV1_0;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use core::trace;
use core::trace::*;
use render;

pub struct Device {
    pub vk_instance: ash::Instance,

    pub vk_physical_device: ash::vk::PhysicalDevice,

    // pub vk_device: ash::Device;
    pub vk_allocation_callbacks: Option<vk::AllocationCallbacks>,
}

impl Device {
    pub fn new(
        system: &System,
        vk_physical_device: ash::vk::PhysicalDevice,
        request: &render::SystemRequest,
    ) -> Result<Device, ()> {
        trace!("Device", "new");

        let physical_device_properties = unsafe {
            system
                .vk_instance
                .get_physical_device_properties(vk_physical_device)
        };
        let device_vulkan_version =
            vk_utils::vk_version_to_semver(physical_device_properties.api_version);
        if device_vulkan_version >= request.first_unsupported_version {
            return Err(());
        }

        Ok(Device {
            vk_instance: system.vk_instance.clone(),
            vk_physical_device: vk_physical_device,
            vk_allocation_callbacks: system.vk_allocation_callbacks,
        })
    }
}

impl Drop for Device {
    fn drop(&mut self) {}
}

impl render::Device for Device {}
