use crate::system::System;
use crate::vk_utils;
// use ash::version::DeviceV1_0;
// use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use common::trace;
use common::trace::*;
use render;
use nalgebra::Vector2;
use nalgebra::Vector3;

pub struct Heap {

    pub index: usize,

    pub size: u64,

    pub is_device_local: bool,

    pub is_host_visible: bool,

    pub is_host_coherent: bool,

    pub is_host_cached: bool,

    pub is_lazily_allocated: bool,
}

pub struct Queue {

    pub vk_queue_family_index: usize,

    pub vk_queue_index: usize,
    
    pub support_graphics: bool,

    pub support_compute: bool,

    pub support_transfer: bool,

    pub support_sparse_bindings: bool,

    pub timestamp_valid_bits: Option<u32>,

    pub min_image_transfer_granularity: Vector3<u32>,
}

pub struct Device {

    pub info: render::DeviceInfo,

    pub vk_device: ash::Device,

    pub vk_instance: ash::Instance,

    pub vk_physical_device: ash::vk::PhysicalDevice,

    // pub vk_device: ash::Device;
    pub vk_allocation_callbacks: Option<vk::AllocationCallbacks>,

    pub vk_heaps: Vec<Heap>,

    pub vk_queues: Vec<Queue>,
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

        Self::create_vk_device(system, vk_physical_device, request, device_info)
    }

    fn create_vk_device(
        system: &System,
        vk_physical_device: ash::vk::PhysicalDevice,
        request: &render::SystemRequest,
        device_info: render::DeviceInfo) -> Result<Device, ()>
    {
        trace!("Device", "create_vk_device");

        let heaps = Self::get_heaps(system, vk_physical_device);
        let queues = Self::get_queues(system, vk_physical_device);

        let queue_create_infos = Self::get_vk_create_queue_create_infos(&queues);
        let vk_device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(queue_create_infos.as_slice())
            // .enabled_layer_names(enabled_layer_names: &'a [*const c_char])
            // .enabled_extension_names(enabled_extension_names: &'a [*const c_char])
            // .enabled_features(enabled_features: &'a PhysicalDeviceFeatures)
            .build();
        let vk_device = match unsafe { system.vk_instance.create_device(
            vk_physical_device,
            &vk_device_create_info,
            system.vk_allocation_callbacks.as_ref(),
        )} {
            Ok(vk_device) => {
                log::debug!("Vulkan device successfully created.");
                Ok(vk_device)
            },
            Err(error) => {
                log::error!("Cannot create vulkan device. Error = {}.", error);
                Err(())
            }
        }?;
        
        Ok(Device {
            info: device_info,
            vk_device: vk_device,
            vk_instance: system.vk_instance.clone(),
            vk_physical_device: vk_physical_device,
            vk_allocation_callbacks: system.vk_allocation_callbacks,
            vk_heaps: heaps,
            vk_queues: queues,
        })
    }

    fn get_vk_create_queue_create_infos(queues: &Vec<Queue>) -> Vec<ash::vk::DeviceQueueCreateInfo> {
        let mut infos = Vec::new();
        infos
    }

    fn get_device_info(vk_physical_device_properties: &ash::vk::PhysicalDeviceProperties) -> render::DeviceInfo {

        let mut device_name_copy = vk_physical_device_properties.device_name.clone();
        let device_name_casted_slice = unsafe { &*(&mut device_name_copy as *mut [i8] as *mut [u8]) };
        let device_name = if let Ok(name) = std::str::from_utf8(device_name_casted_slice) {
            String::from(name)
         } else {
            String::new()
         };

        render::DeviceInfo {
            name: device_name,
            vendor: Self::get_vendor_name(vk_physical_device_properties.vendor_id),
            driver_vesrion: vk_utils::vk_version_to_semver(vk_physical_device_properties.driver_version),
            device_type: Self::get_device_type(&vk_physical_device_properties),
            limits: Self::get_device_limits(&vk_physical_device_properties),
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

    fn get_device_limits(vk_physical_device_properties: &ash::vk::PhysicalDeviceProperties) -> render::DeviceLimits {
        let limits = &vk_physical_device_properties.limits;
        render::DeviceLimits {
            max_image_dimension1_d: limits.max_image_dimension1_d,
            max_image_dimension2_d: limits.max_image_dimension2_d,
            max_image_dimension3_d: limits.max_image_dimension3_d,
            max_image_dimension_cube: limits.max_image_dimension_cube,
            max_image_array_layers: limits.max_image_array_layers,
            max_texel_buffer_elements: limits.max_texel_buffer_elements,
            max_uniform_buffer_range: limits.max_uniform_buffer_range,
            max_storage_buffer_range: limits.max_storage_buffer_range,
            max_push_constants_size: limits.max_push_constants_size,
            max_memory_allocation_count: limits.max_memory_allocation_count,
            max_sampler_allocation_count: limits.max_sampler_allocation_count,
            buffer_image_granularity: limits.buffer_image_granularity,
            sparse_address_space_size: limits.sparse_address_space_size,
            max_bound_descriptor_sets: limits.max_bound_descriptor_sets,
            max_per_stage_descriptor_samplers: limits.max_per_stage_descriptor_samplers,
            max_per_stage_descriptor_uniform_buffers: limits.max_per_stage_descriptor_uniform_buffers,
            max_per_stage_descriptor_storage_buffers: limits.max_per_stage_descriptor_storage_buffers,
            max_per_stage_descriptor_sampled_images: limits.max_per_stage_descriptor_sampled_images,
            max_per_stage_descriptor_storage_images: limits.max_per_stage_descriptor_storage_images,
            max_per_stage_descriptor_input_attachments: limits.max_per_stage_descriptor_input_attachments,
            max_per_stage_resources: limits.max_per_stage_resources,
            max_descriptor_set_samplers: limits.max_descriptor_set_samplers,
            max_descriptor_set_uniform_buffers: limits.max_descriptor_set_uniform_buffers,
            max_descriptor_set_uniform_buffers_dynamic: limits.max_descriptor_set_uniform_buffers_dynamic,
            max_descriptor_set_storage_buffers: limits.max_descriptor_set_storage_buffers,
            max_descriptor_set_storage_buffers_dynamic: limits.max_descriptor_set_storage_buffers_dynamic,
            max_descriptor_set_sampled_images: limits.max_descriptor_set_sampled_images,
            max_descriptor_set_storage_images: limits.max_descriptor_set_storage_images,
            max_descriptor_set_input_attachments: limits.max_descriptor_set_input_attachments,
            max_vertex_input_attributes: limits.max_vertex_input_attributes,
            max_vertex_input_bindings: limits.max_vertex_input_bindings,
            max_vertex_input_attribute_offset: limits.max_vertex_input_attribute_offset,
            max_vertex_input_binding_stride: limits.max_vertex_input_binding_stride,
            max_vertex_output_components: limits.max_vertex_output_components,
            max_tessellation_generation_level: limits.max_tessellation_generation_level,
            max_tessellation_patch_size: limits.max_tessellation_patch_size,
            max_tessellation_control_per_vertex_input_components: limits.max_tessellation_control_per_vertex_input_components,
            max_tessellation_control_per_vertex_output_components: limits.max_tessellation_control_per_vertex_output_components,
            max_tessellation_control_per_patch_output_components: limits.max_tessellation_control_per_patch_output_components,
            max_tessellation_control_total_output_components: limits.max_tessellation_control_total_output_components,
            max_tessellation_evaluation_input_components: limits.max_tessellation_evaluation_input_components,
            max_tessellation_evaluation_output_components: limits.max_tessellation_evaluation_output_components,
            max_geometry_shader_invocations: limits.max_geometry_shader_invocations,
            max_geometry_input_components: limits.max_geometry_input_components,
            max_geometry_output_components: limits.max_geometry_output_components,
            max_geometry_output_vertices: limits.max_geometry_output_vertices,
            max_geometry_total_output_components: limits.max_geometry_total_output_components,
            max_fragment_input_components: limits.max_fragment_input_components,
            max_fragment_output_attachments: limits.max_fragment_output_attachments,
            max_fragment_dual_src_attachments: limits.max_fragment_dual_src_attachments,
            max_fragment_combined_output_resources: limits.max_fragment_combined_output_resources,
            max_compute_shared_memory_size: limits.max_compute_shared_memory_size,
            max_compute_work_group_count: Vector3::new(
                limits.max_compute_work_group_count[0],
                limits.max_compute_work_group_count[1],
                limits.max_compute_work_group_count[2]),
            max_compute_work_group_invocations: limits.max_compute_work_group_invocations,
            max_compute_work_group_size: Vector3::new(
                limits.max_compute_work_group_size[0],
                limits.max_compute_work_group_size[1],
                limits.max_compute_work_group_size[2]),
            sub_pixel_precision_bits: limits.sub_pixel_precision_bits,
            sub_texel_precision_bits: limits.sub_texel_precision_bits,
            mipmap_precision_bits: limits.mipmap_precision_bits,
            max_draw_indexed_index_value: limits.max_draw_indexed_index_value,
            max_draw_indirect_count: limits.max_draw_indirect_count,
            max_sampler_lod_bias: limits.max_sampler_lod_bias,
            max_sampler_anisotropy: limits.max_sampler_anisotropy,
            max_viewports: limits.max_viewports,
            max_viewport_dimensions: Vector2::new(
                limits.max_viewport_dimensions[0],
                limits.max_viewport_dimensions[1]),
            viewport_bounds_range: Vector2::new(
                limits.viewport_bounds_range[0],
                limits.viewport_bounds_range[1]),
            viewport_sub_pixel_bits: limits.viewport_sub_pixel_bits,
            min_memory_map_alignment: limits.min_memory_map_alignment,
            min_texel_buffer_offset_alignment: limits.min_texel_buffer_offset_alignment,
            min_uniform_buffer_offset_alignment: limits.min_uniform_buffer_offset_alignment,
            min_storage_buffer_offset_alignment: limits.min_storage_buffer_offset_alignment,
            min_texel_offset: limits.min_texel_offset,
            max_texel_offset: limits.max_texel_offset,
            min_texel_gather_offset: limits.min_texel_gather_offset,
            max_texel_gather_offset: limits.max_texel_gather_offset,
            min_interpolation_offset: limits.min_interpolation_offset,
            max_interpolation_offset: limits.max_interpolation_offset,
            sub_pixel_interpolation_offset_bits: limits.sub_pixel_interpolation_offset_bits,
            max_framebuffer_width: limits.max_framebuffer_width,
            max_framebuffer_height: limits.max_framebuffer_height,
            max_framebuffer_layers: limits.max_framebuffer_layers,
            framebuffer_color_sample_counts: Self::vk_sample_count_flags_to_u32(limits.framebuffer_color_sample_counts),
            framebuffer_depth_sample_counts: Self::vk_sample_count_flags_to_u32(limits.framebuffer_depth_sample_counts),
            framebuffer_stencil_sample_counts: Self::vk_sample_count_flags_to_u32(limits.framebuffer_stencil_sample_counts),
            framebuffer_no_attachments_sample_counts: Self::vk_sample_count_flags_to_u32(limits.framebuffer_no_attachments_sample_counts),
            max_color_attachments: limits.max_color_attachments,
            sampled_image_color_sample_counts: Self::vk_sample_count_flags_to_u32(limits.sampled_image_color_sample_counts),
            sampled_image_integer_sample_counts: Self::vk_sample_count_flags_to_u32(limits.sampled_image_integer_sample_counts),
            sampled_image_depth_sample_counts: Self::vk_sample_count_flags_to_u32(limits.sampled_image_depth_sample_counts),
            sampled_image_stencil_sample_counts: Self::vk_sample_count_flags_to_u32(limits.sampled_image_stencil_sample_counts),
            storage_image_sample_counts: Self::vk_sample_count_flags_to_u32(limits.storage_image_sample_counts),
            max_sample_mask_words: limits.max_sample_mask_words,
            timestamp_compute_and_graphics: limits.timestamp_compute_and_graphics,
            timestamp_period: limits.timestamp_period,
            max_clip_distances: limits.max_clip_distances,
            max_cull_distances: limits.max_cull_distances,
            max_combined_clip_and_cull_distances: limits.max_combined_clip_and_cull_distances,
            discrete_queue_priorities: limits.discrete_queue_priorities,
            point_size_range: Vector2::new(
                limits.point_size_range[0],
                limits.point_size_range[1]),
            line_width_range: Vector2::new(
                limits.line_width_range[0],
                limits.line_width_range[1]),
            point_size_granularity: limits.point_size_granularity,
            line_width_granularity: limits.line_width_granularity,
            strict_lines: limits.strict_lines,
            standard_sample_locations: limits.standard_sample_locations,
            optimal_buffer_copy_offset_alignment: limits.optimal_buffer_copy_offset_alignment,
            optimal_buffer_copy_row_pitch_alignment: limits.optimal_buffer_copy_row_pitch_alignment,
            non_coherent_atom_size: limits.non_coherent_atom_size,
        }
    }

    fn vk_sample_count_flags_to_u32(count: ash::vk::SampleCountFlags) -> u32 {
        match count {
            ash::vk::SampleCountFlags::TYPE_1 => 1,
            ash::vk::SampleCountFlags::TYPE_2 => 2,
            ash::vk::SampleCountFlags::TYPE_4 => 4,
            ash::vk::SampleCountFlags::TYPE_8 => 8,
            ash::vk::SampleCountFlags::TYPE_16 => 16,
            ash::vk::SampleCountFlags::TYPE_32 => 32,
            ash::vk::SampleCountFlags::TYPE_64 => 64,
            _ => 0,
        }
    }

    fn get_heaps(system: &System, vk_physical_device: ash::vk::PhysicalDevice) -> Vec<Heap> {
        let mut heaps : Vec<Heap> = Vec::new();
        let vk_memory_properties = unsafe { system.vk_instance.get_physical_device_memory_properties(vk_physical_device) };
        for i in 0..vk_memory_properties.memory_type_count {
            let vk_memory = &vk_memory_properties.memory_types[i as usize];
            let vk_heap = &vk_memory_properties.memory_heaps[vk_memory.heap_index as usize];
            
            heaps.push(Heap {
                index: i as usize,
                size: vk_heap.size,
                is_device_local: vk_memory.property_flags.contains(ash::vk::MemoryPropertyFlags::DEVICE_LOCAL),
                is_host_visible: vk_memory.property_flags.contains(ash::vk::MemoryPropertyFlags::HOST_VISIBLE),
                is_host_coherent: vk_memory.property_flags.contains(ash::vk::MemoryPropertyFlags::HOST_COHERENT),
                is_host_cached: vk_memory.property_flags.contains(ash::vk::MemoryPropertyFlags::HOST_CACHED),
                is_lazily_allocated: vk_memory.property_flags.contains(ash::vk::MemoryPropertyFlags::LAZILY_ALLOCATED),
            });
        }

        heaps
    }

    fn get_queues(system: &System, vk_physical_device: ash::vk::PhysicalDevice) -> Vec<Queue> {
        let mut queues : Vec<Queue> = Vec::new();
        let vk_queue_family_properties = unsafe { system.vk_instance.get_physical_device_queue_family_properties(vk_physical_device) };
        for q in 0..vk_queue_family_properties.len() {
            let vk_queue_family = &vk_queue_family_properties[q];
            let timestamp_valid_bits = if vk_queue_family.timestamp_valid_bits > 0 {
                Some(vk_queue_family.timestamp_valid_bits)
            } else {
                None
            };
            for i in 0..vk_queue_family.queue_count {
                queues.push(Queue {
                    vk_queue_family_index: q,
                    vk_queue_index: i as usize,
                    support_graphics: vk_queue_family.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS),
                    support_compute: vk_queue_family.queue_flags.contains(ash::vk::QueueFlags::COMPUTE),
                    support_transfer: vk_queue_family.queue_flags.contains(ash::vk::QueueFlags::TRANSFER),
                    support_sparse_bindings: vk_queue_family.queue_flags.contains(ash::vk::QueueFlags::SPARSE_BINDING),
                    timestamp_valid_bits: timestamp_valid_bits,
                    min_image_transfer_granularity: Vector3::new(
                        vk_queue_family.min_image_transfer_granularity.width,
                        vk_queue_family.min_image_transfer_granularity.height,
                        vk_queue_family.min_image_transfer_granularity.depth),
                });
            };
        };
        queues
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
