use crate::system::System;
use ash::version::DeviceV1_0;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use core::trace;
use core::trace::*;
use render;

pub struct Device {
    pub vk_instance: ash::Instance,

    pub vk_device: ash::vk::PhysicalDevice,

    pub vk_allocation_callbacks: Option<vk::AllocationCallbacks>,
}

impl Device {
    pub fn new(system: &System, vk_device: ash::vk::PhysicalDevice) -> Result<Device, ()> {
        trace!("Device", "new");

        Ok(Device {
            vk_instance: system.vk_instance.clone(),
            vk_device: vk_device,
            vk_allocation_callbacks: system.vk_allocation_callbacks,
        })
    }
}

impl render::Device for Device {}
