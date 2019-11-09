use crate::system::System;
use crate::vk_utils;
use ash::version::DeviceV1_0;
// use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use ash::vk::Handle;
use common::trace;
use common::trace::*;
use nalgebra::Vector2;
use nalgebra::Vector3;
use render;
use std::sync::Arc;

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

    pub vk_allocation_callbacks: Option<vk::AllocationCallbacks>,

    pub vk_heaps: Vec<Heap>,

    pub vk_queues: Vec<Queue>,

    pub vk_queue_family_indices: Vec<u32>,

    pub buffers: Vec<Arc<render::GpuBuffer>>,

    pub textures: Vec<Arc<render::GpuTexture>>,
}

impl Device {
    pub fn new(
        system: &System,
        vk_physical_device: ash::vk::PhysicalDevice,
        request: &render::SystemRequest,
    ) -> Result<Device, ()> {
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
        device_info: render::DeviceInfo,
    ) -> Result<Device, ()> {
        trace!("Device", "create_vk_device");

        let heaps = Self::get_heaps(system, vk_physical_device);
        let queues = Self::get_queues(system, vk_physical_device);
        let mut vk_queue_family_indices: Vec<u32> = Vec::new();

        let mut queue_priorities: std::collections::HashMap<u32, Vec<f32>> =
            std::collections::HashMap::new();
        for queue in &queues {
            let key = queue.vk_queue_family_index as u32;
            if queue_priorities.contains_key(&key) {
                queue_priorities.get_mut(&key).unwrap().push(1.0);
            } else {
                queue_priorities.insert(key, vec![1.0]);
                vk_queue_family_indices.push(key);
            }
        }
        vk_queue_family_indices.sort();

        let mut vk_queue_create_infos: Vec<ash::vk::DeviceQueueCreateInfo> = Vec::new();
        for (queue_family, priorities) in &queue_priorities {
            vk_queue_create_infos.push(
                ash::vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(*queue_family)
                    .queue_priorities(priorities.as_slice())
                    .build(),
            );
        }

        let vk_device_features =
            Self::get_vk_enabled_features(system, vk_physical_device, request)?;
        let vk_device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(vk_queue_create_infos.as_slice())
            // .enabled_layer_names(enabled_layer_names: &'a [*const c_char])
            // .enabled_extension_names(enabled_extension_names: &'a [*const c_char])
            .enabled_features(&vk_device_features)
            .build();
        let vk_device = match unsafe {
            system.vk_instance.create_device(
                vk_physical_device,
                &vk_device_create_info,
                system.vk_allocation_callbacks.as_ref(),
            )
        } {
            Ok(vk_device) => {
                log::debug!("Vulkan device successfully created.");
                Ok(vk_device)
            }
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
            vk_queue_family_indices: vk_queue_family_indices,
            buffers: Vec::new(),
            textures: Vec::new(),
        })
    }

    fn get_vk_enabled_features(
        system: &System,
        vk_physical_device: ash::vk::PhysicalDevice,
        request: &render::SystemRequest,
    ) -> Result<ash::vk::PhysicalDeviceFeatures, ()> {
        let vk_device_features = unsafe {
            system
                .vk_instance
                .get_physical_device_features(vk_physical_device)
        };

        let f = |required: bool, enabled: bool, support: bool, feature_name: &str| {
            if required && !support {
                log::info!(
                    "Cannot init device because required feature is not supported: {}",
                    feature_name
                );
                return Err(());
            }

            if !support {
                if enabled {
                    log::info!("Device feature is not supported: {}", feature_name);
                }
                return Ok(false);
            }

            if required || enabled {
                log::info!("Enable device feature: {}", feature_name);
                Ok(true)
            } else {
                Ok(false)
            }
        };

        let builder = ash::vk::PhysicalDeviceFeatures::builder()
            .robust_buffer_access(f(
                request.required_device_features.robust_buffer_access,
                request.enabled_device_features.robust_buffer_access,
                vk_device_features.robust_buffer_access > 0,
                "Robust buffer access",
            )?)
            .full_draw_index_uint32(f(
                request.required_device_features.full_draw_index_uint32,
                request.enabled_device_features.full_draw_index_uint32,
                vk_device_features.full_draw_index_uint32 > 0,
                "Full draw index uint32",
            )?)
            .image_cube_array(f(
                request.required_device_features.image_cube_array,
                request.enabled_device_features.image_cube_array,
                vk_device_features.image_cube_array > 0,
                "Image cube array",
            )?)
            .independent_blend(f(
                request.required_device_features.independent_blend,
                request.enabled_device_features.independent_blend,
                vk_device_features.independent_blend > 0,
                "Independent blend",
            )?)
            .geometry_shader(f(
                request.required_device_features.geometry_shader,
                request.enabled_device_features.geometry_shader,
                vk_device_features.geometry_shader > 0,
                "Geometry shader",
            )?)
            .tessellation_shader(f(
                request.required_device_features.tessellation_shader,
                request.enabled_device_features.tessellation_shader,
                vk_device_features.tessellation_shader > 0,
                "Tessellation shader",
            )?)
            .sample_rate_shading(f(
                request.required_device_features.sample_rate_shading,
                request.enabled_device_features.sample_rate_shading,
                vk_device_features.sample_rate_shading > 0,
                "Sample rate shading",
            )?)
            .dual_src_blend(f(
                request.required_device_features.dual_src_blend,
                request.enabled_device_features.dual_src_blend,
                vk_device_features.dual_src_blend > 0,
                "Dual src blend",
            )?)
            .logic_op(f(
                request.required_device_features.logic_op,
                request.enabled_device_features.logic_op,
                vk_device_features.logic_op > 0,
                "Logic op",
            )?)
            .multi_draw_indirect(f(
                request.required_device_features.multi_draw_indirect,
                request.enabled_device_features.multi_draw_indirect,
                vk_device_features.multi_draw_indirect > 0,
                "Multi draw indirect",
            )?)
            .draw_indirect_first_instance(f(
                request
                    .required_device_features
                    .draw_indirect_first_instance,
                request.enabled_device_features.draw_indirect_first_instance,
                vk_device_features.draw_indirect_first_instance > 0,
                "Draw indirect first instance",
            )?)
            .depth_clamp(f(
                request.required_device_features.depth_clamp,
                request.enabled_device_features.depth_clamp,
                vk_device_features.depth_clamp > 0,
                "Depth clamp",
            )?)
            .depth_bias_clamp(f(
                request.required_device_features.depth_bias_clamp,
                request.enabled_device_features.depth_bias_clamp,
                vk_device_features.depth_bias_clamp > 0,
                "Depth bias clamp",
            )?)
            .fill_mode_non_solid(f(
                request.required_device_features.fill_mode_non_solid,
                request.enabled_device_features.fill_mode_non_solid,
                vk_device_features.fill_mode_non_solid > 0,
                "Fill mode non solid",
            )?)
            .depth_bounds(f(
                request.required_device_features.depth_bounds,
                request.enabled_device_features.depth_bounds,
                vk_device_features.depth_bounds > 0,
                "Depth bounds",
            )?)
            .wide_lines(f(
                request.required_device_features.wide_lines,
                request.enabled_device_features.wide_lines,
                vk_device_features.wide_lines > 0,
                "Wide lines",
            )?)
            .large_points(f(
                request.required_device_features.large_points,
                request.enabled_device_features.large_points,
                vk_device_features.large_points > 0,
                "Large points",
            )?)
            .alpha_to_one(f(
                request.required_device_features.alpha_to_one,
                request.enabled_device_features.alpha_to_one,
                vk_device_features.alpha_to_one > 0,
                "Alpha to one",
            )?)
            .multi_viewport(f(
                request.required_device_features.multi_viewport,
                request.enabled_device_features.multi_viewport,
                vk_device_features.multi_viewport > 0,
                "Multi viewport",
            )?)
            .sampler_anisotropy(f(
                request.required_device_features.sampler_anisotropy,
                request.enabled_device_features.sampler_anisotropy,
                vk_device_features.sampler_anisotropy > 0,
                "Sampler anisotropy",
            )?)
            .texture_compression_etc2(f(
                request.required_device_features.texture_compression_etc2,
                request.enabled_device_features.texture_compression_etc2,
                vk_device_features.texture_compression_etc2 > 0,
                "Texture compression etc2",
            )?)
            .texture_compression_astc_ldr(f(
                request
                    .required_device_features
                    .texture_compression_astc_ldr,
                request.enabled_device_features.texture_compression_astc_ldr,
                vk_device_features.texture_compression_astc_ldr > 0,
                "Texture compression astc ldr",
            )?)
            .texture_compression_bc(f(
                request.required_device_features.texture_compression_bc,
                request.enabled_device_features.texture_compression_bc,
                vk_device_features.texture_compression_bc > 0,
                "Texture compression bc",
            )?)
            .occlusion_query_precise(f(
                request.required_device_features.occlusion_query_precise,
                request.enabled_device_features.occlusion_query_precise,
                vk_device_features.occlusion_query_precise > 0,
                "Occlusion query precise",
            )?)
            .pipeline_statistics_query(f(
                request.required_device_features.pipeline_statistics_query,
                request.enabled_device_features.pipeline_statistics_query,
                vk_device_features.pipeline_statistics_query > 0,
                "Pipeline statistics query",
            )?)
            .vertex_pipeline_stores_and_atomics(f(
                request
                    .required_device_features
                    .vertex_pipeline_stores_and_atomics,
                request
                    .enabled_device_features
                    .vertex_pipeline_stores_and_atomics,
                vk_device_features.vertex_pipeline_stores_and_atomics > 0,
                "Vertex pipeline stores and atomics",
            )?)
            .fragment_stores_and_atomics(f(
                request.required_device_features.fragment_stores_and_atomics,
                request.enabled_device_features.fragment_stores_and_atomics,
                vk_device_features.fragment_stores_and_atomics > 0,
                "Fragment stores and atomics",
            )?)
            .shader_tessellation_and_geometry_point_size(f(
                request
                    .required_device_features
                    .shader_tessellation_and_geometry_point_size,
                request
                    .enabled_device_features
                    .shader_tessellation_and_geometry_point_size,
                vk_device_features.shader_tessellation_and_geometry_point_size > 0,
                "Shader tessellation and geometry point size",
            )?)
            .shader_image_gather_extended(f(
                request
                    .required_device_features
                    .shader_image_gather_extended,
                request.enabled_device_features.shader_image_gather_extended,
                vk_device_features.shader_image_gather_extended > 0,
                "Shader image gather extended",
            )?)
            .shader_storage_image_extended_formats(f(
                request
                    .required_device_features
                    .shader_storage_image_extended_formats,
                request
                    .enabled_device_features
                    .shader_storage_image_extended_formats,
                vk_device_features.shader_storage_image_extended_formats > 0,
                "Shader storage image extended formats",
            )?)
            .shader_storage_image_multisample(f(
                request
                    .required_device_features
                    .shader_storage_image_multisample,
                request
                    .enabled_device_features
                    .shader_storage_image_multisample,
                vk_device_features.shader_storage_image_multisample > 0,
                "Shader storage image multisample",
            )?)
            .shader_storage_image_read_without_format(f(
                request
                    .required_device_features
                    .shader_storage_image_read_without_format,
                request
                    .enabled_device_features
                    .shader_storage_image_read_without_format,
                vk_device_features.shader_storage_image_read_without_format > 0,
                "Shader storage image read without format",
            )?)
            .shader_storage_image_write_without_format(f(
                request
                    .required_device_features
                    .shader_storage_image_write_without_format,
                request
                    .enabled_device_features
                    .shader_storage_image_write_without_format,
                vk_device_features.shader_storage_image_write_without_format > 0,
                "Shader storage image write without format",
            )?)
            .shader_uniform_buffer_array_dynamic_indexing(f(
                request
                    .required_device_features
                    .shader_uniform_buffer_array_dynamic_indexing,
                request
                    .enabled_device_features
                    .shader_uniform_buffer_array_dynamic_indexing,
                vk_device_features.shader_uniform_buffer_array_dynamic_indexing > 0,
                "Shader uniform buffer array dynamic indexing",
            )?)
            .shader_sampled_image_array_dynamic_indexing(f(
                request
                    .required_device_features
                    .shader_sampled_image_array_dynamic_indexing,
                request
                    .enabled_device_features
                    .shader_sampled_image_array_dynamic_indexing,
                vk_device_features.shader_sampled_image_array_dynamic_indexing > 0,
                "Shader sampled image array dynamic indexing",
            )?)
            .shader_storage_buffer_array_dynamic_indexing(f(
                request
                    .required_device_features
                    .shader_storage_buffer_array_dynamic_indexing,
                request
                    .enabled_device_features
                    .shader_storage_buffer_array_dynamic_indexing,
                vk_device_features.shader_storage_buffer_array_dynamic_indexing > 0,
                "Shader storage buffer array dynamic indexing",
            )?)
            .shader_storage_image_array_dynamic_indexing(f(
                request
                    .required_device_features
                    .shader_storage_image_array_dynamic_indexing,
                request
                    .enabled_device_features
                    .shader_storage_image_array_dynamic_indexing,
                vk_device_features.shader_storage_image_array_dynamic_indexing > 0,
                "Shader storage image array dynamic indexing",
            )?)
            .shader_clip_distance(f(
                request.required_device_features.shader_clip_distance,
                request.enabled_device_features.shader_clip_distance,
                vk_device_features.shader_clip_distance > 0,
                "Shader clip distance",
            )?)
            .shader_cull_distance(f(
                request.required_device_features.shader_cull_distance,
                request.enabled_device_features.shader_cull_distance,
                vk_device_features.shader_cull_distance > 0,
                "Shader cull distance",
            )?)
            .shader_float64(f(
                request.required_device_features.shader_float64,
                request.enabled_device_features.shader_float64,
                vk_device_features.shader_float64 > 0,
                "Shader float64",
            )?)
            .shader_int64(f(
                request.required_device_features.shader_int64,
                request.enabled_device_features.shader_int64,
                vk_device_features.shader_int64 > 0,
                "Shader int64",
            )?)
            .shader_int16(f(
                request.required_device_features.shader_int16,
                request.enabled_device_features.shader_int16,
                vk_device_features.shader_int16 > 0,
                "Shader int16",
            )?)
            .shader_resource_residency(f(
                request.required_device_features.shader_resource_residency,
                request.enabled_device_features.shader_resource_residency,
                vk_device_features.shader_resource_residency > 0,
                "Shader resource residency",
            )?)
            .shader_resource_min_lod(f(
                request.required_device_features.shader_resource_min_lod,
                request.enabled_device_features.shader_resource_min_lod,
                vk_device_features.shader_resource_min_lod > 0,
                "Shader resource min lod",
            )?)
            .sparse_binding(f(
                request.required_device_features.sparse_binding,
                request.enabled_device_features.sparse_binding,
                vk_device_features.sparse_binding > 0,
                "Sparse binding",
            )?)
            .sparse_residency_buffer(f(
                request.required_device_features.sparse_residency_buffer,
                request.enabled_device_features.sparse_residency_buffer,
                vk_device_features.sparse_residency_buffer > 0,
                "Sparse residency buffer",
            )?)
            .sparse_residency_image2_d(f(
                request.required_device_features.sparse_residency_image2_d,
                request.enabled_device_features.sparse_residency_image2_d,
                vk_device_features.sparse_residency_image2_d > 0,
                "Sparse residency image2d",
            )?)
            .sparse_residency_image3_d(f(
                request.required_device_features.sparse_residency_image3_d,
                request.enabled_device_features.sparse_residency_image3_d,
                vk_device_features.sparse_residency_image3_d > 0,
                "Sparse residency image3d",
            )?)
            .sparse_residency2_samples(f(
                request.required_device_features.sparse_residency2_samples,
                request.enabled_device_features.sparse_residency2_samples,
                vk_device_features.sparse_residency2_samples > 0,
                "Sparse residency2 samples",
            )?)
            .sparse_residency4_samples(f(
                request.required_device_features.sparse_residency4_samples,
                request.enabled_device_features.sparse_residency4_samples,
                vk_device_features.sparse_residency4_samples > 0,
                "Sparse residency4  samples",
            )?)
            .sparse_residency8_samples(f(
                request.required_device_features.sparse_residency8_samples,
                request.enabled_device_features.sparse_residency8_samples,
                vk_device_features.sparse_residency8_samples > 0,
                "Sparse  residency8 samples",
            )?)
            .sparse_residency16_samples(f(
                request.required_device_features.sparse_residency16_samples,
                request.enabled_device_features.sparse_residency16_samples,
                vk_device_features.sparse_residency16_samples > 0,
                "Sparse residency16 samples",
            )?)
            .sparse_residency_aliased(f(
                request.required_device_features.sparse_residency_aliased,
                request.enabled_device_features.sparse_residency_aliased,
                vk_device_features.sparse_residency_aliased > 0,
                "Sparse residency aliased",
            )?)
            .variable_multisample_rate(f(
                request.required_device_features.variable_multisample_rate,
                request.enabled_device_features.variable_multisample_rate,
                vk_device_features.variable_multisample_rate > 0,
                "Variable multisample rate",
            )?)
            .inherited_queries(f(
                request.required_device_features.inherited_queries,
                request.enabled_device_features.inherited_queries,
                vk_device_features.inherited_queries > 0,
                "Inherited queries",
            )?);

        Ok(builder.build())
    }

    fn get_device_info(
        vk_physical_device_properties: &ash::vk::PhysicalDeviceProperties,
    ) -> render::DeviceInfo {
        let mut device_name_copy = vk_physical_device_properties.device_name.clone();
        let device_name_casted_slice =
            unsafe { &*(&mut device_name_copy as *mut [i8] as *mut [u8]) };
        let device_name = if let Ok(name) = std::str::from_utf8(device_name_casted_slice) {
            String::from(name)
        } else {
            String::new()
        };

        render::DeviceInfo {
            name: device_name,
            vendor: Self::get_vendor_name(vk_physical_device_properties.vendor_id),
            driver_vesrion: vk_utils::vk_version_to_semver(
                vk_physical_device_properties.driver_version,
            ),
            device_type: Self::get_device_type(&vk_physical_device_properties),
            limits: Self::get_device_limits(&vk_physical_device_properties),
        }
    }

    fn is_passed_vk_version(
        vk_physical_device_properties: &ash::vk::PhysicalDeviceProperties,
        request: &render::SystemRequest,
    ) -> bool {
        let device_vulkan_version =
            vk_utils::vk_version_to_semver(vk_physical_device_properties.api_version);
        log::info!("Device VK version: {}", device_vulkan_version);

        if device_vulkan_version >= request.min_supported_version
            && device_vulkan_version < request.first_unsupported_version
        {
            true
        } else {
            log::info!(
                "Device does not match reques version (from {} to {}).",
                request.min_supported_version,
                request.first_unsupported_version
            );
            false
        }
    }

    fn get_device_type(
        vk_physical_device_properties: &ash::vk::PhysicalDeviceProperties,
    ) -> render::DeviceType {
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

    fn get_device_limits(
        vk_physical_device_properties: &ash::vk::PhysicalDeviceProperties,
    ) -> render::DeviceLimits {
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
            max_per_stage_descriptor_uniform_buffers: limits
                .max_per_stage_descriptor_uniform_buffers,
            max_per_stage_descriptor_storage_buffers: limits
                .max_per_stage_descriptor_storage_buffers,
            max_per_stage_descriptor_sampled_images: limits.max_per_stage_descriptor_sampled_images,
            max_per_stage_descriptor_storage_images: limits.max_per_stage_descriptor_storage_images,
            max_per_stage_descriptor_input_attachments: limits
                .max_per_stage_descriptor_input_attachments,
            max_per_stage_resources: limits.max_per_stage_resources,
            max_descriptor_set_samplers: limits.max_descriptor_set_samplers,
            max_descriptor_set_uniform_buffers: limits.max_descriptor_set_uniform_buffers,
            max_descriptor_set_uniform_buffers_dynamic: limits
                .max_descriptor_set_uniform_buffers_dynamic,
            max_descriptor_set_storage_buffers: limits.max_descriptor_set_storage_buffers,
            max_descriptor_set_storage_buffers_dynamic: limits
                .max_descriptor_set_storage_buffers_dynamic,
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
            max_tessellation_control_per_vertex_input_components: limits
                .max_tessellation_control_per_vertex_input_components,
            max_tessellation_control_per_vertex_output_components: limits
                .max_tessellation_control_per_vertex_output_components,
            max_tessellation_control_per_patch_output_components: limits
                .max_tessellation_control_per_patch_output_components,
            max_tessellation_control_total_output_components: limits
                .max_tessellation_control_total_output_components,
            max_tessellation_evaluation_input_components: limits
                .max_tessellation_evaluation_input_components,
            max_tessellation_evaluation_output_components: limits
                .max_tessellation_evaluation_output_components,
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
                limits.max_compute_work_group_count[2],
            ),
            max_compute_work_group_invocations: limits.max_compute_work_group_invocations,
            max_compute_work_group_size: Vector3::new(
                limits.max_compute_work_group_size[0],
                limits.max_compute_work_group_size[1],
                limits.max_compute_work_group_size[2],
            ),
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
                limits.max_viewport_dimensions[1],
            ),
            viewport_bounds_range: Vector2::new(
                limits.viewport_bounds_range[0],
                limits.viewport_bounds_range[1],
            ),
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
            framebuffer_color_sample_counts: Self::vk_sample_count_flags_to_u32(
                limits.framebuffer_color_sample_counts,
            ),
            framebuffer_depth_sample_counts: Self::vk_sample_count_flags_to_u32(
                limits.framebuffer_depth_sample_counts,
            ),
            framebuffer_stencil_sample_counts: Self::vk_sample_count_flags_to_u32(
                limits.framebuffer_stencil_sample_counts,
            ),
            framebuffer_no_attachments_sample_counts: Self::vk_sample_count_flags_to_u32(
                limits.framebuffer_no_attachments_sample_counts,
            ),
            max_color_attachments: limits.max_color_attachments,
            sampled_image_color_sample_counts: Self::vk_sample_count_flags_to_u32(
                limits.sampled_image_color_sample_counts,
            ),
            sampled_image_integer_sample_counts: Self::vk_sample_count_flags_to_u32(
                limits.sampled_image_integer_sample_counts,
            ),
            sampled_image_depth_sample_counts: Self::vk_sample_count_flags_to_u32(
                limits.sampled_image_depth_sample_counts,
            ),
            sampled_image_stencil_sample_counts: Self::vk_sample_count_flags_to_u32(
                limits.sampled_image_stencil_sample_counts,
            ),
            storage_image_sample_counts: Self::vk_sample_count_flags_to_u32(
                limits.storage_image_sample_counts,
            ),
            max_sample_mask_words: limits.max_sample_mask_words,
            timestamp_compute_and_graphics: limits.timestamp_compute_and_graphics,
            timestamp_period: limits.timestamp_period,
            max_clip_distances: limits.max_clip_distances,
            max_cull_distances: limits.max_cull_distances,
            max_combined_clip_and_cull_distances: limits.max_combined_clip_and_cull_distances,
            discrete_queue_priorities: limits.discrete_queue_priorities,
            point_size_range: Vector2::new(limits.point_size_range[0], limits.point_size_range[1]),
            line_width_range: Vector2::new(limits.line_width_range[0], limits.line_width_range[1]),
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
        let mut heaps: Vec<Heap> = Vec::new();
        let vk_memory_properties = unsafe {
            system
                .vk_instance
                .get_physical_device_memory_properties(vk_physical_device)
        };
        for i in 0..vk_memory_properties.memory_type_count {
            let vk_memory = &vk_memory_properties.memory_types[i as usize];
            let vk_heap = &vk_memory_properties.memory_heaps[vk_memory.heap_index as usize];

            heaps.push(Heap {
                index: i as usize,
                size: vk_heap.size,
                is_device_local: vk_memory
                    .property_flags
                    .contains(ash::vk::MemoryPropertyFlags::DEVICE_LOCAL),
                is_host_visible: vk_memory
                    .property_flags
                    .contains(ash::vk::MemoryPropertyFlags::HOST_VISIBLE),
                is_host_coherent: vk_memory
                    .property_flags
                    .contains(ash::vk::MemoryPropertyFlags::HOST_COHERENT),
                is_host_cached: vk_memory
                    .property_flags
                    .contains(ash::vk::MemoryPropertyFlags::HOST_CACHED),
                is_lazily_allocated: vk_memory
                    .property_flags
                    .contains(ash::vk::MemoryPropertyFlags::LAZILY_ALLOCATED),
            });
        }

        heaps
    }

    fn get_queues(system: &System, vk_physical_device: ash::vk::PhysicalDevice) -> Vec<Queue> {
        let mut queues: Vec<Queue> = Vec::new();
        let vk_queue_family_properties = unsafe {
            system
                .vk_instance
                .get_physical_device_queue_family_properties(vk_physical_device)
        };
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
                    support_graphics: vk_queue_family
                        .queue_flags
                        .contains(ash::vk::QueueFlags::GRAPHICS),
                    support_compute: vk_queue_family
                        .queue_flags
                        .contains(ash::vk::QueueFlags::COMPUTE),
                    support_transfer: vk_queue_family
                        .queue_flags
                        .contains(ash::vk::QueueFlags::TRANSFER),
                    support_sparse_bindings: vk_queue_family
                        .queue_flags
                        .contains(ash::vk::QueueFlags::SPARSE_BINDING),
                    timestamp_valid_bits: timestamp_valid_bits,
                    min_image_transfer_granularity: Vector3::new(
                        vk_queue_family.min_image_transfer_granularity.width,
                        vk_queue_family.min_image_transfer_granularity.height,
                        vk_queue_family.min_image_transfer_granularity.depth,
                    ),
                });
            }
        }
        queues
    }

    pub fn delete_buffer(&mut self, buffer: &Arc<render::GpuBuffer>) {
        trace!("Device", "delete_buffer");
        // log::debug!("Delete buffer {}", buffer.name);
    }

    pub fn delete_texture(&mut self, texture: &Arc<render::GpuTexture>) {
        trace!("Device", "delete_texture");
        // log::debug!("Delete texture {}", texture.name);
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        trace!("Device", "drop");

        match unsafe { self.vk_device.device_wait_idle() } {
            Ok(_) => {}
            Err(error) => {
                log::error!("Error while idle device before drop. Error = {}.", error);
            }
        };

        let buffers = self.buffers.clone();
        for buffer in &buffers {
            //if !buffer.is_unique() {
            //    log::error!("Buffer {} is still used", buffer.name);
            //}
            self.delete_buffer(buffer);
        }

        let textures = self.textures.clone();
        for texture in &textures {
            //if !buffer.is_unique() {
            //    log::error!("Buffer {} is still used", buffer.name);
            //}
            self.delete_texture(texture);
        }

        unsafe {
            self.vk_device
                .destroy_device(self.vk_allocation_callbacks.as_ref())
        };
    }
}

impl render::Device for Device {
    fn device_info(&self) -> &render::DeviceInfo {
        &self.info
    }

    fn create_buffer(&mut self, info: render::GpuBufferInfo) -> Arc<render::GpuBuffer> {
        trace!("Device", "create_gpu_buffer");

        let mut usage_flags = ash::vk::BufferUsageFlags::empty();
        match info.buffer_type {
            render::GpuBufferType::Index => {
                usage_flags |= ash::vk::BufferUsageFlags::TRANSFER_SRC;
                usage_flags |= ash::vk::BufferUsageFlags::TRANSFER_DST;
                usage_flags |= ash::vk::BufferUsageFlags::UNIFORM_BUFFER;
                usage_flags |= ash::vk::BufferUsageFlags::STORAGE_BUFFER;
                usage_flags |= ash::vk::BufferUsageFlags::INDEX_BUFFER;
            }
            render::GpuBufferType::Vertex => {
                usage_flags |= ash::vk::BufferUsageFlags::TRANSFER_SRC;
                usage_flags |= ash::vk::BufferUsageFlags::TRANSFER_DST;
                usage_flags |= ash::vk::BufferUsageFlags::UNIFORM_BUFFER;
                usage_flags |= ash::vk::BufferUsageFlags::STORAGE_BUFFER;
                usage_flags |= ash::vk::BufferUsageFlags::VERTEX_BUFFER;
            }
        };

        let buffer_create_info = ash::vk::BufferCreateInfo::builder()
            .usage(usage_flags)
            .sharing_mode(ash::vk::SharingMode::CONCURRENT)
            .queue_family_indices(self.vk_queue_family_indices.as_slice())
            .size(info.size);

        let handle = match unsafe {
            self.vk_device
                .create_buffer(&buffer_create_info, self.vk_allocation_callbacks.as_ref())
        } {
            Ok(vk_buffer) => vk_buffer.as_raw(),
            Err(error) => {
                log::error!("Create gpu buffer error. Error = {}.", error);
                0
            }
        };

        let view_handle: u64 = 0;

        let gpu_buffer = render::GpuBuffer {
            handle: handle,
            view_handle: view_handle,
            info: info,
        };

        Arc::new(gpu_buffer)
    }

    fn create_texture(&mut self, info: render::GpuTextureInfo) -> Arc<render::GpuTexture> {
        trace!("Device", "create_gpu_texture");

        std::unimplemented!()
    }
}
