use crate::system::System;
use crate::vk_utils;
use ash::version::DeviceV1_0;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use common::trace;
use common::trace::*;
use render;

pub struct Device {

    pub info: render::DeviceInfo,

    pub vk_instance: ash::Instance,

    pub vk_physical_device: ash::vk::PhysicalDevice,

    // pub vk_device: ash::Device;
    pub vk_allocation_callbacks: Option<vk::AllocationCallbacks>,

}

impl Device {
    pub fn new(system: &System, vk_physical_device: ash::vk::PhysicalDevice, request: &render::SystemRequest) -> Result<Device, ()> {
        trace!("Device", "new");

        let vk_physical_device_properties = unsafe {
            system
                .vk_instance
                .get_physical_device_properties(vk_physical_device)
        };

        let device_info = Self::get_device_info(&vk_physical_device_properties);
        device_info.log();

        if !Self::is_passed_vk_version(&vk_physical_device_properties, request) {
            return Err(());
        }

        Ok(Device {
            info: device_info,
            vk_instance: system.vk_instance.clone(),
            vk_physical_device: vk_physical_device,
            vk_allocation_callbacks: system.vk_allocation_callbacks,
        })
    }

    fn get_device_info(vk_physical_device_properties: &ash::vk::PhysicalDeviceProperties) -> render::DeviceInfo {

        render::DeviceInfo {
            name: String::new(),
            vendor: Self::get_vendor_name(vk_physical_device_properties.vendor_id),
            driver_vesrion: vk_utils::vk_version_to_semver(vk_physical_device_properties.driver_version),
            device_type: Self::get_device_type(&vk_physical_device_properties),
        }
    }

    fn is_passed_vk_version(vk_physical_device_properties: &ash::vk::PhysicalDeviceProperties, request: &render::SystemRequest) -> bool {
        let device_vulkan_version =
            vk_utils::vk_version_to_semver(vk_physical_device_properties.api_version);
        log::info!("Device VK version: {}", device_vulkan_version);

        if device_vulkan_version >= request.min_supported_version && device_vulkan_version < request.first_unsupported_version {
            true
        } else {
            log::info!("Device does not match reques version (from {} to {}).", request.min_supported_version, request.first_unsupported_version);
            false
        }
    }

    fn get_device_type(vk_physical_device_properties: &ash::vk::PhysicalDeviceProperties) -> render::DeviceType {
        match vk_physical_device_properties.device_type {
            ash::vk::PhysicalDeviceType::DISCRETE_GPU => render::DeviceType::Discrete,
            ash::vk::PhysicalDeviceType::INTEGRATED_GPU => render::DeviceType::Integrated,
            ash::vk::PhysicalDeviceType::VIRTUAL_GPU => render::DeviceType::Virtual,
            ash::vk::PhysicalDeviceType::CPU => render::DeviceType::Software,
            _ => render::DeviceType::Unknown,
        }
    }

    fn get_vendor_name(vendor_id: u32) -> String {
        match vendor_id {
            0x1002 => String::from("AMD"),
            0x1010 => String::from("ImgTec"),
            0x10DE => String::from("NVIDIA"),
            0x13B5 => String::from("ARM"),
            0x5143 => String::from("Qualcomm"),
            0x8086 => String::from("INTEL"),
            _ => String::from("Unknown"),
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        trace!("Device", "Drop");
    }
}

impl render::Device for Device {

    fn device_info(&self) -> &render::DeviceInfo {
        &self.info
    }

}
